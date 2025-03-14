//! rbx_dom_weak is a common representation of the Roblox DOM for Rust. It's
//! designed to play nicely with the borrow checker and allows accessing
//! instances by ID in constant time.
//!
//! Constructing a new tree of instances is accomplished by first creating an
//! [`InstanceBuilder`] object that describes a tree of instances and then
//! wrapping it with an [`WeakDom`]:
//!
//! ```
//! use rbx_dom_weak::{InstanceBuilder, WeakDom};
//!
//! let dm = InstanceBuilder::new("DataModel");
//!
//! let mut dom = WeakDom::new(dm);
//!
//! println!("ID of DOM root is {:?}", dom.root_ref());
//! ```
//!
//! Once we have a tree, we can use [`WeakDom::insert`] and
//! [`WeakDom::get_by_ref`] to add instances to the tree and retrieve them.
//!
//! ```
//! use rbx_dom_weak::{InstanceBuilder, WeakDom};
//!
//! let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));
//!
//! // We can define properties using any type that can be converted to an
//! // rbx_dom_weak::types::Variant.
//! let http_service = InstanceBuilder::new("HttpService")
//!     .with_property("HttpEnabled", true);
//!
//! let http_service_id = dom.insert(dom.root_ref(), http_service);
//!
//! println!("HttpService has ID {:?}", http_service_id);
//! ```
//!
//! To change properties on an instance that's already present in the tree, use
//! [`WeakDom::get_by_ref_mut`]. Note that it isn't possible to add or remove
//! children through this method, use [`WeakDom::insert`] and
//! [`WeakDom::destroy`] instead.

#![deny(missing_docs)]

mod dom;
mod instance;
mod viewer;

pub use rbx_types as types;

pub use ahash::AHashMap;
pub use ustr::{ustr, Ustr, UstrMap, UstrSet};

pub use crate::{
    dom::WeakDom as GenericWeakDom,
    instance::{Instance, InstanceBuilder},
    viewer::{DomViewer, ViewedInstance},
};
/// Convenience type of GenericWeakDom<Instance>
pub type WeakDom = GenericWeakDom<Instance>;

/// Helper trait that provides convenience methods for `AHashMap` and `UstrMap`.
pub trait HashMapExt {
    /// Constructs an empty map.
    fn new() -> Self;

    /// Constructs an empty map with at least the specified capacity.
    fn with_capacity(capacity: usize) -> Self;
}

impl<V> HashMapExt for UstrMap<V> {
    /// Creates an empty `UstrMap` using the default value for its hasher.
    fn new() -> Self {
        UstrMap::default()
    }

    /// Creates an empty `UstrMap` with at least the specified capacity using
    /// the default value for its hasher.
    fn with_capacity(capacity: usize) -> Self {
        UstrMap::with_capacity_and_hasher(capacity, Default::default())
    }
}

impl<K, V> HashMapExt for AHashMap<K, V> {
    /// Creates an empty `AHashMap` using the default value for its hasher.
    fn new() -> Self {
        AHashMap::default()
    }

    /// Creates an empty `AHashMap` with at least the specified capacity using
    /// the default value for its hasher.
    fn with_capacity(capacity: usize) -> Self {
        AHashMap::with_capacity_and_hasher(capacity, Default::default())
    }
}
