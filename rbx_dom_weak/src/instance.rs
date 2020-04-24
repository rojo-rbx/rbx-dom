use std::collections::HashMap;

use rbx_types::{Ref, Variant};

/// Represents an instance that can be turned into a new `WeakDom`, or inserted
/// into an existing one.
#[derive(Debug)]
pub struct InstanceBuilder {
    pub(crate) referent: Ref,
    pub(crate) name: String,
    pub(crate) class: String,
    pub(crate) properties: HashMap<String, Variant>,
    pub(crate) children: Vec<InstanceBuilder>,
}

impl InstanceBuilder {
    /// Create a new `InstanceBuilder` with the given ClassName. This is also
    /// used as the instance's name, unless overwritten later.
    pub fn new<S: Into<String>>(class: S) -> Self {
        let class = class.into();
        let name = class.clone();

        InstanceBuilder {
            referent: Ref::new(),
            name,
            class,
            properties: HashMap::new(),
            children: Vec::new(),
        }
    }

    /// Change the name of the `InstanceBuilder`.
    pub fn with_name<S: Into<String>>(self, name: S) -> Self {
        Self {
            name: name.into(),
            ..self
        }
    }

    /// Add a new property to the `InstanceBuilder`.
    pub fn with_property<K: Into<String>, V: Into<Variant>>(mut self, key: K, value: V) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Add multiple properties to the `InstanceBuilder` at once.
    pub fn with_properties<K, V, I>(mut self, props: I) -> Self
    where
        K: Into<String>,
        V: Into<Variant>,
        I: IntoIterator<Item = (K, V)>,
    {
        for (key, value) in props {
            self.properties.insert(key.into(), value.into());
        }

        self
    }

    /// Add a new child to the `InstanceBuilder`.
    pub fn with_child(mut self, child: InstanceBuilder) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple children to the `InstanceBuilder` at once.
    ///
    /// Order of the children will be preserved.
    pub fn with_children<I: IntoIterator<Item = InstanceBuilder>>(mut self, children: I) -> Self {
        self.children.extend(children.into_iter());
        self
    }
}

/// An instance contained inside of a `WeakDom`.
///
/// Operations that could affect other instances contained in the `WeakDom`
/// cannot be performed on an `Instance` correctly.
#[derive(Debug)]
pub struct Instance {
    pub(crate) referent: Ref,
    pub(crate) children: Vec<Ref>,
    pub(crate) parent: Ref,

    pub name: String,
    pub class: String,
    pub properties: HashMap<String, Variant>,
}

impl Instance {
    #[inline]
    pub fn referent(&self) -> Ref {
        self.referent
    }

    #[inline]
    pub fn children(&self) -> &[Ref] {
        &self.children
    }

    #[inline]
    pub fn parent(&self) -> Ref {
        self.parent
    }
}
