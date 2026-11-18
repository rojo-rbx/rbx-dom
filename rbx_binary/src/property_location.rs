/// Identifies a serialized property before reflection renaming or migration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub struct PropertyLocation {
    class_name: String,
    property_name: String,
    type_id: u32,
}

/// Exposes source identity without allowing it to drift from a diagnostic.
impl PropertyLocation {
    pub(crate) fn new(class_name: String, property_name: String, type_id: u32) -> Self {
        Self {
            class_name,
            property_name,
            type_id,
        }
    }

    /// Returns the class name from the property's INST declaration.
    pub fn class_name(&self) -> &str {
        &self.class_name
    }

    /// Returns the original PROP name, which can differ from its canonical name.
    pub fn property_name(&self) -> &str {
        &self.property_name
    }

    /// Returns the document's INST type ID, not a reflection database ID.
    pub fn type_id(&self) -> u32 {
        self.type_id
    }
}
