use rbx_types::{Ref, Variant};
use ustr::{Ustr, UstrMap};

/**
Represents an instance that can be turned into a new
[`WeakDom`][crate::WeakDom], or inserted into an existing one.

## Examples
Instances have the given ClassName and Name and no properties by default.

```
use rbx_dom_weak::{InstanceBuilder, WeakDom};

let data_model = InstanceBuilder::new("DataModel");
let dom = WeakDom::new(data_model);
```

Properties and children can be added to the builder.

```
use rbx_dom_weak::{InstanceBuilder, WeakDom};
use rbx_dom_weak::types::Color3;

let data_model = InstanceBuilder::new("DataModel")
    .with_child(InstanceBuilder::new("Workspace")
        .with_property("FilteringEnabled", true))
    .with_child(InstanceBuilder::new("Lighting")
        .with_property("Ambient", Color3::new(1.0, 0.0, 0.0)));

let dom = WeakDom::new(data_model);
```
*/
#[derive(Debug)]
pub struct InstanceBuilder {
    referent: Ref,
    name: String,
    class: Ustr,
    properties: Vec<(Ustr, Variant)>,
    children: Vec<InstanceBuilder>,
}

impl InstanceBuilder {
    /// Create a new `InstanceBuilder` with the given ClassName. This is also
    /// used as the instance's Name, unless overwritten later.
    pub fn new<S: Into<Ustr>>(class: S) -> Self {
        let class = class.into();
        let name = class.to_string();

        InstanceBuilder {
            referent: Ref::new(),
            name,
            class,
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Create a new `InstanceBuilder` with the given ClassName and with a
    /// property table with at least enough space for the given capacity.
    pub fn with_property_capacity<S: Into<Ustr>>(class: S, capacity: usize) -> Self {
        let class = class.into();
        let name = class.to_string();

        InstanceBuilder {
            referent: Ref::new(),
            name,
            class,
            properties: Vec::with_capacity(capacity),
            children: Vec::new(),
        }
    }

    /// Create a new `InstanceBuilder` with all values set to empty.
    pub fn empty() -> Self {
        InstanceBuilder {
            referent: Ref::new(),
            name: String::new(),
            class: Ustr::default(),
            properties: Vec::new(),
            children: Vec::new(),
        }
    }

    /// Return the referent of the instance that the `InstanceBuilder` refers to.
    pub fn referent(&self) -> Ref {
        self.referent
    }

    /// Change the referent of the `InstanceBuilder`.
    pub fn with_referent<R: Into<Ref>>(self, referent: R) -> Self {
        Self {
            referent: referent.into(),
            ..self
        }
    }

    /// Change the name of the `InstanceBuilder`.
    pub fn with_name<S: Into<String>>(self, name: S) -> Self {
        Self {
            name: name.into(),
            ..self
        }
    }

    /// Change the name of the `InstanceBuilder`.
    pub fn set_name<S: Into<String>>(&mut self, name: S) {
        self.name = name.into();
    }

    /// Change the class of the `InstanceBuilder`.
    pub fn with_class<S: Into<Ustr>>(self, class: S) -> Self {
        Self {
            class: class.into(),
            ..self
        }
    }

    /// Change the class of the `InstanceBuilder`.
    pub fn set_class<S: Into<Ustr>>(&mut self, class: S) {
        self.class = class.into();
    }

    /// Add a new property to the `InstanceBuilder`.
    pub fn with_property<K: Into<Ustr>, V: Into<Variant>>(mut self, key: K, value: V) -> Self {
        self.properties.push((key.into(), value.into()));
        self
    }

    /// Add a new property to the `InstanceBuilder`.
    pub fn add_property<K: Into<Ustr>, V: Into<Variant>>(&mut self, key: K, value: V) {
        self.properties.push((key.into(), value.into()));
    }

    /// Check if the `InstanceBuilder` already has a property with the given key.
    pub fn has_property<K: Into<Ustr>>(&self, key: K) -> bool {
        let key = key.into();
        self.properties.iter().any(|(k, _)| *k == key)
    }

    /// Add multiple properties to the `InstanceBuilder` at once.
    pub fn with_properties<K, V, I>(mut self, props: I) -> Self
    where
        K: Into<Ustr>,
        V: Into<Variant>,
        I: IntoIterator<Item = (K, V)>,
    {
        let props = props.into_iter().map(|(k, v)| (k.into(), v.into()));
        self.properties.extend(props);

        self
    }

    /// Add multiple properties to the `InstanceBuilder` at once.
    pub fn add_properties<K, V, I>(&mut self, props: I)
    where
        K: Into<Ustr>,
        V: Into<Variant>,
        I: IntoIterator<Item = (K, V)>,
    {
        let props = props.into_iter().map(|(k, v)| (k.into(), v.into()));
        self.properties.extend(props);
    }

    /// Add a new child to the `InstanceBuilder`.
    pub fn with_child(mut self, child: InstanceBuilder) -> Self {
        self.children.push(child);
        self
    }

    /// Add a new child to the `InstanceBuilder`.
    pub fn add_child(&mut self, child: InstanceBuilder) {
        self.children.push(child);
    }

    /// Add multiple children to the `InstanceBuilder` at once.
    ///
    /// Order of the children will be preserved.
    pub fn with_children<I>(mut self, children: I) -> Self
    where
        I: IntoIterator<Item = InstanceBuilder>,
    {
        self.children.extend(children);
        self
    }

    /// Add multiple children to the `InstanceBuilder` at once.
    ///
    /// Order of the children will be preserved.
    pub fn add_children<I>(&mut self, children: I)
    where
        I: IntoIterator<Item = InstanceBuilder>,
    {
        self.children.extend(children);
    }

    pub(crate) fn children(&self) -> &[InstanceBuilder] {
        &self.children
    }
}

/// An instance contained inside of a [`WeakDom`][crate::WeakDom].
///
/// Operations that could affect other instances contained in the
/// [`WeakDom`][crate::WeakDom] cannot be performed on an `Instance` correctly.
#[derive(Debug)]
pub struct Instance {
    referent: Ref,
    children: Vec<Ref>,
    parent: Ref,

    /// The instance's name, corresponding to the `Name` property.
    pub name: String,

    /// The instance's class, corresponding to the `ClassName` property.
    pub class: Ustr,

    /// Any properties stored on the object that are not `Name` or `ClassName`.
    pub properties: UstrMap<Variant>,
}

impl Instance {
    /// Returns this instance's referent. It will always be non-null.
    #[inline]
    pub fn referent(&self) -> Ref {
        self.referent
    }

    /// Returns a list of the referents corresponding to the instance's
    /// children. All referents returned will be non-null and point to valid
    /// instances in the same [`WeakDom`][crate::WeakDom].
    #[inline]
    pub fn children(&self) -> &[Ref] {
        &self.children
    }

    /// Returns the referent corresponding to this instance's parent. This
    /// referent will either point to an instance in the same
    /// [`WeakDom`][crate::WeakDom] or be null.
    #[inline]
    pub fn parent(&self) -> Ref {
        self.parent
    }

    pub(crate) const fn set_parent(&mut self, parent: Ref) {
        self.parent = parent;
    }
    pub(crate) const fn children_mut(&mut self) -> &mut Vec<Ref> {
        &mut self.children
    }
    pub(crate) fn from_builder(
        parent: Ref,
        builder: InstanceBuilder,
    ) -> (Self, Vec<InstanceBuilder>) {
        (
            Instance {
                referent: builder.referent,
                children: Vec::with_capacity(builder.children.len()),
                parent,
                class: builder.class,
                name: builder.name,
                properties: builder.properties.into_iter().collect(),
            },
            builder.children,
        )
    }
}
