use std::collections::{HashMap, VecDeque};

use thunderdome::{Arena, Index};

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
    storage: Arena<Instance>,
    roots: Vec<Index>,
}

impl WeakDom {
    /// Construct a new `WeakDom` described by the given [`InstanceBuilder`].
    pub fn new(builder: InstanceBuilder) -> WeakDom {
        let mut dom = WeakDom {
            storage: Arena::new(),
            roots: Vec::new(),
        };
        dom.insert(None, builder);
        dom
    }

    /// Returns the referent of the root instances of the `WeakDom`.
    #[inline]
    pub fn roots(&self) -> &[Index] {
        self.roots.as_slice()
    }

    /// Returns a reference to an instance by referent, or `None` if it is not
    /// found.
    #[inline]
    pub fn get(&self, index: Index) -> Option<&Instance> {
        self.storage.get(index)
    }

    /// Returns a _mutable_ reference to an instance by referent, or `None` if
    /// it is not found.
    #[inline]
    pub fn get_mut(&mut self, index: Index) -> Option<&mut Instance> {
        self.storage.get_mut(index)
    }

    /// Insert a new instance into the DOM with the given parent.
    pub fn insert(&mut self, parent: Option<Index>, builder: InstanceBuilder) -> Index {
        let root_builder_id = builder.temp_id;

        let mut id_to_index = HashMap::new();
        let mut to_insert = VecDeque::new();
        to_insert.push_back((parent, builder));

        while let Some((parent, builder)) = to_insert.pop_front() {
            let temp_id = builder.temp_id;
            let children = builder.children;

            let instance = Instance {
                index: temp_index_will_be_reassigned(),
                children: Vec::with_capacity(children.len()),
                parent,
                name: builder.name,
                class: builder.class,
                properties: builder.properties,
            };

            let index = self.storage.insert(instance);
            self.storage.get_mut(index).unwrap().index = index;
            id_to_index.insert(temp_id, index);

            let parent = parent.and_then(|index| self.storage.get_mut(index));
            if let Some(parent) = parent {
                parent.children.push(index);
            }

            let new_elements = children.into_iter().map(|child| (Some(index), child));
            to_insert.extend(new_elements);
        }

        id_to_index.get(&root_builder_id).copied().unwrap()
    }

    /// Destroy the instance with the given referent.
    ///
    /// ## Panics
    /// Panics if `referent` does not refer to an instance in the DOM.
    ///
    /// Will also panic if `referent` refers to the root instance in this
    /// `WeakDom`.
    pub fn destroy(&mut self, _index: Index) {
        todo!()
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
    pub fn transfer(&mut self, _index: Index, _dest: &mut WeakDom, _dest_parent: Index) {
        todo!()
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
    pub fn transfer_within(&mut self, _index: Index, _dest_parent: Index) {
        todo!()
    }
}

/// Creates an Index to be used temporarily.
fn temp_index_will_be_reassigned() -> Index {
    Index::from_bits(1 << 32).unwrap()
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
