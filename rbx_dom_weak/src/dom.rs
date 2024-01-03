use std::collections::{HashMap, HashSet, VecDeque};

use rbx_types::{Ref, UniqueId, Variant};

use crate::instance::{Instance, InstanceBuilder};

/// Represents a DOM containing one or more Roblox instances.
///
/// Instances are described by [`Instance`] objects and have a referent, a class
/// name, a name, properties, and an ordered list of children.
///
/// When constructing instances, you'll want to create [`InstanceBuilder`]
/// objects and insert them into the tree.
#[derive(Debug)]
pub struct WeakDom {
    instances: HashMap<Ref, Instance>,
    root_ref: Ref,
    unique_ids: HashSet<UniqueId>,
}

impl WeakDom {
    /// Construct a new `WeakDom` described by the given [`InstanceBuilder`].
    pub fn new(builder: InstanceBuilder) -> WeakDom {
        let mut dom = WeakDom {
            instances: HashMap::new(),
            root_ref: builder.referent,
            unique_ids: HashSet::new(),
        };

        dom.insert(Ref::none(), builder);
        dom
    }

    /// Consumes the WeakDom, returning its underlying root ref and backing
    /// storage. This method is useful when tree-preserving operations are too
    /// slow.
    pub fn into_raw(self) -> (Ref, HashMap<Ref, Instance>) {
        (self.root_ref, self.instances)
    }

    /// Returns the referent of the root instance of the `WeakDom`.
    pub fn root_ref(&self) -> Ref {
        self.root_ref
    }

    /// Returns a reference to the root instance of the `WeakDom`.
    pub fn root(&self) -> &Instance {
        self.instances.get(&self.root_ref).unwrap()
    }

    /// Returns a _mutable_ reference to the root instance of the `WeakDom`.
    pub fn root_mut(&mut self) -> &mut Instance {
        self.instances.get_mut(&self.root_ref).unwrap()
    }

    /// Returns a reference to an instance by referent, or `None` if it is not
    /// found.
    pub fn get_by_ref(&self, referent: Ref) -> Option<&Instance> {
        self.instances.get(&referent)
    }

    /// Returns a _mutable_ reference to an instance by referent, or `None` if
    /// it is not found.
    pub fn get_by_ref_mut(&mut self, referent: Ref) -> Option<&mut Instance> {
        self.instances.get_mut(&referent)
    }

    /// Insert a new instance into the DOM with the given parent. The parent is allowed to
    /// be the none Ref.
    ///
    /// ## Panics
    /// Panics if `parent_ref` is some and does not refer to an instance in the DOM.
    pub fn insert(&mut self, parent_ref: Ref, root_builder: InstanceBuilder) -> Ref {
        let root_referent = root_builder.referent;

        // Rather than performing this movement recursively, we instead use a
        // queue that we load the children of each `InstanceBuilder` into.
        // Then we can just iter through that.
        let mut queue = VecDeque::with_capacity(1);
        queue.push_back((parent_ref, root_builder));

        while let Some((parent, builder)) = queue.pop_front() {
            self.inner_insert(
                builder.referent,
                Instance {
                    referent: builder.referent,
                    children: Vec::with_capacity(builder.children.len()),
                    parent,
                    name: builder.name,
                    class: builder.class,
                    properties: builder.properties,
                },
            );

            if parent.is_some() {
                self.instances
                    .get_mut(&parent)
                    .unwrap_or_else(|| panic!("cannot insert into parent that does not exist"))
                    .children
                    .push(builder.referent);
            }

            for child in builder.children {
                queue.push_back((builder.referent, child));
            }
        }

        root_referent
    }

    /// Destroy the instance with the given referent.
    ///
    /// ## Panics
    /// Panics if `referent` does not refer to an instance in the DOM.
    ///
    /// Will also panic if `referent` refers to the root instance in this
    /// `WeakDom`.
    pub fn destroy(&mut self, referent: Ref) {
        if referent == self.root_ref {
            panic!("cannot destroy the root instance of a WeakDom");
        }

        let instance = self
            .instances
            .get(&referent)
            .unwrap_or_else(|| panic!("cannot destroy an instance that does not exist"));

        let parent_ref = instance.parent;
        if parent_ref.is_some() {
            let parent = self.instances.get_mut(&parent_ref).unwrap();
            parent.children.retain(|&child| child != referent);
        }

        let mut to_remove = VecDeque::new();
        to_remove.push_back(referent);

        while let Some(referent) = to_remove.pop_front() {
            let instance = self.inner_remove(referent);
            to_remove.extend(instance.children);
        }
    }

    /// Move the instance with the given referent to a new `WeakDom`, parenting
    /// it to the given ref. To move to within the same DOM, use
    /// [`WeakDom::transfer_within`].
    ///
    /// This function would be called `move`, but that's a Rust keyword!
    ///
    /// ## Panics
    /// Panics if `referent` does not refer to an instance in `self` or if
    /// `dest_parent_ref` does not refer to an instance in `other_dom`.
    ///
    /// Will also panic if `referent` refers to the root instance in this
    /// `WeakDom`.
    pub fn transfer(&mut self, referent: Ref, dest: &mut WeakDom, dest_parent_ref: Ref) {
        if referent == self.root_ref {
            panic!("cannot transfer the root instance of WeakDom");
        }

        let mut instance = self.inner_remove(referent);

        // Remove the instance being moved from its parent's list of children.
        // If we care about panic tolerance in the future, doing this first is
        // important to ensure this link is the one severed first.
        let parent_ref = instance.parent;
        if parent_ref.is_some() {
            let parent = self.instances.get_mut(&parent_ref).unwrap();
            parent.children.retain(|&child| child != referent);
        }

        // We'll start tracking all of the instances that we're moving in a
        // queue. We're about to move the moving instance, so we need to do this
        // now.
        let mut to_move = VecDeque::new();
        to_move.extend(instance.children.iter().copied());

        // Instance was released.
        // Bye-bye, instance!
        instance.parent = dest_parent_ref;
        dest.inner_insert(referent, instance);

        // Transfer all of the descendants of the moving instance breadth-first.
        while let Some(referent) = to_move.pop_front() {
            let instance = self.inner_remove(referent);

            to_move.extend(instance.children.iter().copied());
            dest.inner_insert(referent, instance);
        }

        // Finally, notify the new parent instance that their adoption is
        // complete. Enjoy!
        let dest_parent = dest.instances.get_mut(&dest_parent_ref).unwrap_or_else(|| {
            panic!("cannot move an instance into an instance that does not exist")
        });
        dest_parent.children.push(referent);
    }

    /// Move the instance with the given referent to a new parent within the
    /// same `WeakDom`. To move to another DOM, use [`WeakDom::transfer`].
    ///
    /// This function would be called `move_within`, but `move` is a Rust
    /// keyword and consistency with `transfer` is valuable.
    ///
    /// ## Panics
    /// Panics if `referent` or `dest_parent_ref` do not refer to instances in
    /// `self`.
    ///
    /// Will also panic if `referent` refers to the root instance in this
    /// `WeakDom`.
    pub fn transfer_within(&mut self, referent: Ref, dest_parent_ref: Ref) {
        if referent == self.root_ref {
            panic!("cannot transfer the root instance of WeakDom");
        }

        let instance = self
            .instances
            .get_mut(&referent)
            .unwrap_or_else(|| panic!("cannot move an instance that does not exist"));

        // Tell the instance who its new parent is.
        let parent_ref = instance.parent;
        instance.parent = dest_parent_ref;

        // Remove the instance's referent from its parent's list of children.
        if parent_ref.is_some() {
            let parent = self.instances.get_mut(&parent_ref).unwrap();
            parent.children.retain(|&child| child != referent);
        }

        // Add the instance's referent to its new parent's list of children.
        let dest_parent = self
            .instances
            .get_mut(&dest_parent_ref)
            .unwrap_or_else(|| panic!("cannot move into an instance that does not exist"));
        dest_parent.children.push(referent);
    }

    /// Clone the instance with the given `referent` and all its descendants
    /// (i.e. the entire subtree) into the same WeakDom.
    ///
    /// After the operation, the root of the cloned subtree has no parent.
    ///
    /// Any Ref properties that point to instances contained in the subtree are
    /// rewritten to point to the cloned instances.
    pub fn clone_within(&mut self, referent: Ref) -> Ref {
        let mut ctx = CloneContext::default();
        let root_builder = ctx.clone_ref_as_builder(self, referent);
        let root_ref = self.insert(Ref::none(), root_builder);

        while let Some((cloned_parent, uncloned_child)) = ctx.queue.pop_front() {
            let builder = ctx.clone_ref_as_builder(self, uncloned_child);
            self.insert(cloned_parent, builder);
        }

        ctx.rewrite_refs(self);
        root_ref
    }

    /// Clone the instance with the given `referent` and all its descendants (i.e. the
    /// entire subtree) into the given WeakDom.
    ///
    /// After the operation, the root of the cloned subtree has no parent.
    ///
    /// Any Ref properties that point to instances contained in the subtree are
    /// rewritten to point to the cloned instances. Any other Ref properties
    /// would be invalid  in `dest` and are thus rewritten to be `Ref::none()`
    ///
    /// This means that if you call this method on multiple different instances, Ref
    /// properties will not necessarily be preserved in the destination dom. If you're
    /// cloning multiple instances, prefer `clone_multiple_into_external` instead!
    pub fn clone_into_external(&self, referent: Ref, dest: &mut WeakDom) -> Ref {
        let mut ctx = CloneContext::default();
        let root_builder = ctx.clone_ref_as_builder(self, referent);
        let root_ref = dest.insert(Ref::none(), root_builder);

        while let Some((cloned_parent, uncloned_child)) = ctx.queue.pop_front() {
            let builder = ctx.clone_ref_as_builder(self, uncloned_child);
            dest.insert(cloned_parent, builder);
        }

        ctx.rewrite_refs(dest);
        root_ref
    }

    /// Similar to `clone_into_external`, but clones multiple subtrees all at once. This
    /// method will preserve Ref properties that point across the cloned subtrees.
    pub fn clone_multiple_into_external(&self, referents: &[Ref], dest: &mut WeakDom) -> Vec<Ref> {
        let mut ctx = CloneContext::default();
        let mut root_refs = Vec::with_capacity(referents.len());

        for referent in referents {
            let builder = ctx.clone_ref_as_builder(self, *referent);
            root_refs.push(dest.insert(Ref::none(), builder));
        }

        while let Some((cloned_parent, uncloned_child)) = ctx.queue.pop_front() {
            let builder = ctx.clone_ref_as_builder(self, uncloned_child);
            dest.insert(cloned_parent, builder);
        }

        ctx.rewrite_refs(dest);
        root_refs
    }

    fn inner_insert(&mut self, referent: Ref, instance: Instance) {
        self.instances.insert(referent, instance);

        // We need to ensure that the value of the Instance.UniqueId property does
        // not collide with another instance. If it does, we must regenerate
        // it. If we *don't* do this, it's possible to use WeakDom::insert to
        // insert UniqueId properties that collide with other instances in the
        // dom, violating the invariant that every UniqueId is unique.

        // Unwrap is safe because we just inserted this referent into the instance map
        let instance = self.instances.get_mut(&referent).unwrap();
        if let Some(Variant::UniqueId(unique_id)) = instance.properties.get("UniqueId") {
            if self.unique_ids.contains(unique_id) {
                // We found a collision! We need to replace the UniqueId property with
                // a new value.

                // Unwrap is probably ok. Likely not worth making this method fallible
                // just because the system clock might be out whack, so panicking is fine
                let new_unique_id = UniqueId::now().unwrap();

                self.unique_ids.insert(new_unique_id);
                instance
                    .properties
                    .insert("UniqueId".to_string(), Variant::UniqueId(new_unique_id));
            } else {
                self.unique_ids.insert(*unique_id);
            };
        }
    }

    fn inner_remove(&mut self, referent: Ref) -> Instance {
        let instance = self
            .instances
            .remove(&referent)
            .unwrap_or_else(|| panic!("cannot remove an instance that does not exist"));

        if let Some(Variant::UniqueId(unique_id)) = instance.properties.get("UniqueId") {
            self.unique_ids.remove(unique_id);
        }

        instance
    }
}

impl Default for WeakDom {
    fn default() -> WeakDom {
        WeakDom {
            instances: HashMap::new(),
            root_ref: Ref::none(),
            unique_ids: HashSet::new(),
        }
    }
}

#[derive(Debug, Default)]
struct CloneContext {
    queue: VecDeque<(Ref, Ref)>,
    ref_rewrites: HashMap<Ref, Ref>,
}

impl CloneContext {
    /// On any instances cloned during the operation, rewrite any Ref properties that
    /// point to instances that were also cloned.
    fn rewrite_refs(self, dest: &mut WeakDom) {
        let mut existing_dest_refs = HashSet::new();

        for (_, new_ref) in self.ref_rewrites.iter() {
            let instance = dest
                .get_by_ref(*new_ref)
                .expect("Cannot rewrite refs on an instance that does not exist");

            for prop_value in instance.properties.values() {
                if let Variant::Ref(value) = prop_value {
                    if dest.instances.contains_key(value) {
                        existing_dest_refs.insert(*value);
                    }
                }
            }
        }

        for (_, new_ref) in self.ref_rewrites.iter() {
            let instance = dest
                .get_by_ref_mut(*new_ref)
                .expect("Cannot rewrite refs on an instance that does not exist");

            for prop_value in instance.properties.values_mut() {
                if let Variant::Ref(original_ref) = prop_value {
                    if let Some(new_ref) = self.ref_rewrites.get(original_ref) {
                        // If the ref points to an instance contained within the
                        // cloned subtree, rewrite it as the corresponding new ref
                        *prop_value = Variant::Ref(*new_ref);
                    } else if !existing_dest_refs.contains(original_ref) {
                        // If the ref points to an instance that does not exist
                        // in the destination WeakDom, rewrite it as none
                        *prop_value = Variant::Ref(Ref::none())
                    }
                }
            }
        }
    }

    /// Clone the instance with the given `referent` and `source` WeakDom into a new
    /// InstanceBuilder, and record the mapping of the original referent to the new
    /// referent.
    ///
    /// This method only clones the instance's class name, name, and properties; it
    /// does not clone any children.
    fn clone_ref_as_builder(&mut self, source: &WeakDom, original_ref: Ref) -> InstanceBuilder {
        let instance = source
            .get_by_ref(original_ref)
            .expect("Cannot clone an instance that does not exist");

        let builder = InstanceBuilder::new(instance.class.to_string())
            .with_name(instance.name.to_string())
            .with_properties(instance.properties.clone());

        let new_ref = builder.referent;

        for uncloned_child in instance.children.iter() {
            self.queue.push_back((new_ref, *uncloned_child))
        }

        self.ref_rewrites.insert(original_ref, new_ref);
        builder
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::DomViewer;
    use rbx_types::{UniqueId, Variant};

    #[test]
    fn transfer() {
        let target = InstanceBuilder::new("Folder")
            .with_name("Target")
            .with_child(InstanceBuilder::new("Part").with_name("Some Child"));
        let target_ref = target.referent;

        let mut source = WeakDom::new(InstanceBuilder::new("Folder").with_child(target));
        let mut dest = WeakDom::new(InstanceBuilder::new("DataModel"));

        let mut viewer = DomViewer::new();

        // This snapshot should contain Target and Some Child
        insta::assert_yaml_snapshot!(viewer.view_children(&source));

        let dest_root = dest.root_ref();
        source.transfer(target_ref, &mut dest, dest_root);

        // This snapshot should be empty
        insta::assert_yaml_snapshot!(viewer.view_children(&source));

        // This snapshot should be exactly the same as the first snapshot,
        // containing Target and Child.
        insta::assert_yaml_snapshot!(viewer.view_children(&dest));
    }

    #[test]
    fn transfer_within() {
        let subject = InstanceBuilder::new("Folder")
            .with_name("Root")
            .with_child(InstanceBuilder::new("SpawnLocation"));
        let subject_ref = subject.referent;

        let source_parent = InstanceBuilder::new("Folder")
            .with_name("Source")
            .with_child(subject);

        let dest_parent = InstanceBuilder::new("Folder").with_name("Dest");
        let dest_parent_ref = dest_parent.referent;

        let mut dom = WeakDom::new(
            InstanceBuilder::new("Folder")
                .with_child(source_parent)
                .with_child(dest_parent),
        );

        let mut viewer = DomViewer::new();

        // This snapshot should have Root and SpawnLocation contained in Source.
        insta::assert_yaml_snapshot!(viewer.view_children(&dom));

        dom.transfer_within(subject_ref, dest_parent_ref);

        // This snapshot should have Root and SpawnLocation contained in Dest.
        insta::assert_yaml_snapshot!(viewer.view_children(&dom));
    }

    #[test]
    fn clone_within() {
        let mut child1 = InstanceBuilder::new("Part").with_name("Child1");
        let child1_ref = child1.referent;

        let mut dom = {
            let root = InstanceBuilder::new("Folder").with_name("Root");
            let mut child2 = InstanceBuilder::new("Part").with_name("Child2");

            child1 = child1.with_property("RefProp", root.referent);
            child2 = child2.with_property("RefProp", child1.referent);

            WeakDom::new(root.with_child(child1.with_child(child2)))
        };

        let cloned_child1_ref = dom.clone_within(child1_ref);

        assert!(
            dom.get_by_ref(cloned_child1_ref).unwrap().parent.is_none(),
            "parent of cloned subtree root should be none directly after a clone"
        );

        dom.transfer_within(cloned_child1_ref, dom.root_ref);

        // This snapshot should have a clone of the Child1 subtree under the
        // root Folder, with Child2's ref property pointing to the cloned
        // Child1, and Child1's ref property pointing to the root Folder.
        let mut viewer = DomViewer::new();
        insta::assert_yaml_snapshot!(viewer.view(&dom));
    }

    #[test]
    fn clone_into_external() {
        let dom = {
            let mut child1 = InstanceBuilder::new("Part").with_name("Child1");
            let mut child2 = InstanceBuilder::new("Part").with_name("Child2");
            let mut child3 = InstanceBuilder::new("Part").with_name("Child3");

            child1 = child1.with_property("RefProp", child2.referent);
            child2 = child2.with_property("RefProp", child1.referent);
            child3 = child3.with_property("RefProp", Ref::new());

            WeakDom::new(
                InstanceBuilder::new("Folder")
                    .with_name("Root")
                    .with_children([child1, child2, child3]),
            )
        };

        let mut other_dom = WeakDom::new(InstanceBuilder::new("DataModel"));
        let cloned_root = dom.clone_into_external(dom.root_ref, &mut other_dom);

        assert!(
            other_dom.get_by_ref(cloned_root).unwrap().parent.is_none(),
            "parent of cloned subtree root should be none directly after a clone"
        );

        other_dom.transfer_within(cloned_root, other_dom.root_ref);

        let mut viewer = DomViewer::new();

        // This snapshot is here just to show that the ref props are rewritten after being
        // cloned into the other dom. It should contain a Folder at the root with the three
        // Parts as children
        insta::assert_yaml_snapshot!(viewer.view(&dom));

        // This snapshot should have a clone of the root Folder under the other
        // dom's DataModel, with Child1's and Child2's ref properties rewritten to point
        // to the newly cloned instances, and Child3's ref property rewritten to none.
        insta::assert_yaml_snapshot!(viewer.view(&other_dom));
    }

    #[test]
    fn clone_multiple_into_external() {
        let dom = {
            let mut child1 = InstanceBuilder::new("Part").with_name("Child1");
            let mut child2 = InstanceBuilder::new("Part").with_name("Child2");

            child1 = child1.with_property("RefProp", child2.referent);
            child2 = child2.with_property("RefProp", child1.referent);

            WeakDom::new(
                InstanceBuilder::new("Folder")
                    .with_name("Root")
                    .with_children([child1, child2]),
            )
        };

        let mut other_dom = WeakDom::new(InstanceBuilder::new("DataModel"));
        let cloned = dom.clone_multiple_into_external(dom.root().children(), &mut other_dom);

        assert!(
            other_dom.get_by_ref(cloned[0]).unwrap().parent.is_none(),
            "parent of cloned subtree root should be none directly after a clone"
        );

        assert!(
            other_dom.get_by_ref(cloned[1]).unwrap().parent.is_none(),
            "parent of cloned subtree root should be none directly after a clone"
        );

        other_dom.transfer_within(cloned[0], other_dom.root_ref);
        other_dom.transfer_within(cloned[1], other_dom.root_ref);

        let mut viewer = DomViewer::new();

        // This snapshot should contain Child1 and Child2, with Child1's and Child2's ref
        // properties rewritten to point to the newly cloned instances
        insta::assert_yaml_snapshot!(viewer.view(&other_dom));
    }

    #[test]
    fn large_depth_tree() {
        // We've had issues with stack overflows when creating WeakDoms with
        // particularly deep trees, so this test is simply to ensure that does
        // not happen. `i16::MAX` is arbitrary but very large for recursion.
        const N: usize = i16::MAX as usize;

        let mut refs = Vec::with_capacity(N + 1);
        let mut base = InstanceBuilder::new("Folder");
        refs.push(base.referent());
        for _ in 0..N {
            base = InstanceBuilder::new("Folder").with_child(base);
            refs.push(base.referent());
        }
        let _ = WeakDom::new(base);
    }

    #[test]
    fn unique_id_collision_weakdom_new() {
        let unique_id: UniqueId = UniqueId::now().unwrap();
        let builder =
            InstanceBuilder::new("Folder").with_property("UniqueId", Variant::UniqueId(unique_id));

        // Should avoid a collision even if dom was created from a builder containing a
        // UniqueId prop at the root
        let mut dom = WeakDom::new(builder);

        // Try to make a collision!
        let child_ref = dom.insert(
            dom.root_ref(),
            InstanceBuilder::new("Folder").with_property("UniqueId", Variant::UniqueId(unique_id)),
        );

        let child = dom.get_by_ref(child_ref).unwrap();
        if let Some(Variant::UniqueId(actual_unique_id)) = child.properties.get("UniqueId") {
            assert_ne!(
                unique_id,
                *actual_unique_id,
                "child should have a different UniqueId than the root ({unique_id}), but it was the same."
            )
        } else {
            panic!("UniqueId property must exist and contain a Variant::UniqueId")
        };
    }

    #[test]
    fn unique_id_collision() {
        let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));
        let unique_id: UniqueId = UniqueId::now().unwrap();
        let parent_ref = dom.insert(
            dom.root_ref(),
            InstanceBuilder::new("Folder").with_property("UniqueId", Variant::UniqueId(unique_id)),
        );

        // Try to make a collision!
        let child_ref = dom.insert(
            parent_ref,
            InstanceBuilder::new("Folder").with_property("UniqueId", Variant::UniqueId(unique_id)),
        );

        let child = dom.get_by_ref(child_ref).unwrap();
        if let Some(Variant::UniqueId(actual_unique_id)) = child.properties.get("UniqueId") {
            assert_ne!(
                unique_id,
                *actual_unique_id,
                "child should have a different UniqueId than the parent ({unique_id}), but it was the same."
            )
        } else {
            panic!("UniqueId property must exist and contain a Variant::UniqueId")
        }
    }

    #[test]
    fn unique_id_no_collision() {
        let unique_id = UniqueId::now().unwrap();
        let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));

        let child_ref = dom.insert(
            dom.root_ref(),
            InstanceBuilder::new("Folder").with_property("UniqueId", Variant::UniqueId(unique_id)),
        );

        let child = dom.get_by_ref(child_ref).unwrap();
        if let Some(Variant::UniqueId(actual_unique_id)) = child.properties.get("UniqueId") {
            assert_eq!(
                unique_id,
                *actual_unique_id,
                "if there is no collision, UniqueId should remain the same after passing it to WeakDom::insert."
            )
        } else {
            panic!("UniqueId property must exist and contain a Variant::UniqueId")
        };
    }

    #[test]
    fn unique_id_collision_transfer() {
        let unique_id = UniqueId::now().unwrap();
        let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));
        let mut other_dom = WeakDom::new(InstanceBuilder::new("DataModel"));

        let folder_ref = dom.insert(
            dom.root_ref(),
            InstanceBuilder::new("Folder").with_property("UniqueId", Variant::UniqueId(unique_id)),
        );

        other_dom.insert(
            other_dom.root_ref(),
            InstanceBuilder::new("Folder").with_property("UniqueId", Variant::UniqueId(unique_id)),
        );

        let other_root_ref = other_dom.root_ref();
        dom.transfer(folder_ref, &mut other_dom, other_root_ref);

        let folder = other_dom.get_by_ref(folder_ref).unwrap();
        if let Some(Variant::UniqueId(actual_unique_id)) = folder.properties.get("UniqueId") {
            assert_ne!(
                unique_id, *actual_unique_id,
                "WeakDom::transfer caused a UniqueId collision."
            )
        } else {
            panic!("UniqueId property must exist and contain a Variant::UniqueId")
        };
    }
}
