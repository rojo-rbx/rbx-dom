use std::collections::HashMap;

use rbx_types::Variant;
use thunderdome::Index;

use crate::{temp_id, SmallString};

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
    pub(crate) temp_id: u64,
    pub(crate) name: SmallString,
    pub(crate) class: SmallString,
    pub(crate) properties: HashMap<SmallString, Variant>,
    pub(crate) children: Vec<InstanceBuilder>,
}

impl InstanceBuilder {
    /// Create a new `InstanceBuilder` with the given ClassName. This is also
    /// used as the instance's Name, unless overwritten later.
    pub fn new<S: Into<SmallString>>(class: S) -> Self {
        let class = class.into();
        let name = class.clone();

        InstanceBuilder {
            temp_id: temp_id::get(),
            name,
            class,
            properties: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Create a new `InstanceBuilder` with all values set to empty.
    pub fn empty() -> Self {
        InstanceBuilder {
            temp_id: temp_id::get(),
            name: SmallString::new(""),
            class: SmallString::new(""),
            properties: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// TODO
    pub fn temp_id(&self) -> u64 {
        self.temp_id
    }

    /// Change the name of the `InstanceBuilder`.
    pub fn with_name<S: Into<SmallString>>(self, name: S) -> Self {
        Self {
            name: name.into(),
            ..self
        }
    }

    /// Change the name of the `InstanceBuilder`.
    pub fn set_name<S: Into<SmallString>>(&mut self, name: S) {
        self.name = name.into();
    }

    /// Change the class of the `InstanceBuilder`.
    pub fn with_class<S: Into<SmallString>>(self, class: S) -> Self {
        Self {
            class: class.into(),
            ..self
        }
    }

    /// Change the class of the `InstanceBuilder`.
    pub fn set_class<S: Into<SmallString>>(&mut self, class: S) {
        self.class = class.into();
    }

    /// Add a new property to the `InstanceBuilder`.
    pub fn with_property<K: Into<SmallString>, V: Into<Variant>>(
        mut self,
        key: K,
        value: V,
    ) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Add a new property to the `InstanceBuilder`.
    pub fn add_property<K: Into<SmallString>, V: Into<Variant>>(&mut self, key: K, value: V) {
        self.properties.insert(key.into(), value.into());
    }

    /// Add multiple properties to the `InstanceBuilder` at once.
    pub fn with_properties<K, V, I>(mut self, props: I) -> Self
    where
        K: Into<SmallString>,
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
        K: Into<SmallString>,
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
        self.children.extend(children.into_iter());
        self
    }

    /// Add multiple children to the `InstanceBuilder` at once.
    ///
    /// Order of the children will be preserved.
    pub fn add_children<I>(&mut self, children: I)
    where
        I: IntoIterator<Item = InstanceBuilder>,
    {
        self.children.extend(children.into_iter());
    }
}

/// An instance contained inside of a [`WeakDom`][crate::WeakDom].
///
/// Operations that could affect other instances contained in the
/// [`WeakDom`][crate::WeakDom] cannot be performed on an `Instance` correctly.
#[derive(Debug)]
pub struct Instance {
    pub(crate) index: Index,
    pub(crate) children: Vec<Index>,
    pub(crate) parent: Option<Index>,

    /// The instance's name, corresponding to the `Name` property.
    pub name: SmallString,

    /// The instance's class, corresponding to the `ClassName` property.
    pub class: SmallString,

    /// Any properties stored on the object that are not `Name` or `ClassName`.
    pub properties: HashMap<SmallString, Variant>,
}

impl Instance {
    /// TODO
    #[inline]
    pub fn index(&self) -> Index {
        self.index
    }

    /// Returns a list of the referents corresponding to the instance's
    /// children. All referents returned will be non-null and point to valid
    /// instances in the same [`WeakDom`][crate::WeakDom].
    #[inline]
    pub fn children(&self) -> &[Index] {
        self.children.as_ref()
    }

    /// Returns the referent corresponding to this instance's parent. This
    /// referent will either point to an instance in the same
    /// [`WeakDom`][crate::WeakDom] or be null.
    #[inline]
    pub fn parent(&self) -> Option<Index> {
        self.parent
    }
}
