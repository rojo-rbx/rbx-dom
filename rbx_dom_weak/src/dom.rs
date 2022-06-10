use std::collections::{HashMap, VecDeque};

use rbx_types::Ref;

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
}

impl WeakDom {
    /// Construct a new `WeakDom` described by the given [`InstanceBuilder`].
    pub fn new(builder: InstanceBuilder) -> WeakDom {
        let root_ref = builder.referent;

        let mut instances = HashMap::new();
        instances.insert(
            root_ref,
            Instance {
                referent: root_ref,
                children: Vec::with_capacity(builder.children.len()),
                parent: Ref::none(),
                name: builder.name,
                class: builder.class,
                properties: builder.properties,
            },
        );

        let mut dom = WeakDom {
            instances,
            root_ref,
        };

        for child in builder.children {
            dom.insert(root_ref, child);
        }

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

    /// Insert a new instance into the DOM with the given parent.
    ///
    /// ## Panics
    /// Panics if `parent_ref` does not refer to an instance in the DOM.
    pub fn insert(&mut self, parent_ref: Ref, builder: InstanceBuilder) -> Ref {
        let referent = builder.referent;

        self.instances.insert(
            referent,
            Instance {
                referent,
                children: Vec::with_capacity(builder.children.len()),
                parent: parent_ref,
                name: builder.name,
                class: builder.class,
                properties: builder.properties,
            },
        );

        let parent = self
            .instances
            .get_mut(&parent_ref)
            .unwrap_or_else(|| panic!("cannot insert into parent that does not exist"));

        parent.children.push(referent);

        for child in builder.children {
            self.insert(referent, child);
        }

        referent
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
        let parent = self.instances.get_mut(&parent_ref).unwrap();
        parent.children.retain(|&child| child != referent);

        let mut to_remove = VecDeque::new();
        to_remove.push_back(referent);

        while let Some(referent) = to_remove.pop_front() {
            let instance = self.instances.remove(&referent).unwrap();
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

        let mut instance = self
            .instances
            .remove(&referent)
            .unwrap_or_else(|| panic!("cannot move an instance that does not exist"));

        // Remove the instance being moved from its parent's list of children.
        // If we care about panic tolerance in the future, doing this first is
        // important to ensure this link is the one severed first.
        let parent = self.instances.get_mut(&instance.parent).unwrap();
        parent.children.retain(|&child| child != referent);

        // We'll start tracking all of the instances that we're moving in a
        // queue. We're about to move the moving instance, so we need to do this
        // now.
        let mut to_move = VecDeque::new();
        to_move.extend(instance.children.iter().copied());

        // Instance was released.
        // Bye-bye, instance!
        instance.parent = dest_parent_ref;
        dest.instances.insert(referent, instance);

        // Transfer all of the descendants of the moving instance breadth-first.
        while let Some(referent) = to_move.pop_front() {
            let instance = self.instances.remove(&referent).unwrap();
            to_move.extend(instance.children.iter().copied());
            dest.instances.insert(referent, instance);
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

        let mut instance = self
            .instances
            .get_mut(&referent)
            .unwrap_or_else(|| panic!("cannot move an instance that does not exist"));

        // Tell the instance who its new parent is.
        let parent_ref = instance.parent;
        instance.parent = dest_parent_ref;

        // Remove the instance's referent from its parent's list of children.
        let parent = self.instances.get_mut(&parent_ref).unwrap();
        parent.children.retain(|&child| child != referent);

        // Add the instance's referent to its new parent's list of children.
        let dest_parent = self
            .instances
            .get_mut(&dest_parent_ref)
            .unwrap_or_else(|| panic!("cannot move into an instance that does not exist"));
        dest_parent.children.push(referent);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use crate::DomViewer;

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
}
