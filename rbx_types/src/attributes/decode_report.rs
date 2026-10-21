use super::{AttributeDecodeDiagnostic, Attributes};

/// The attributes and loss evidence produced together by one reader pass.
///
/// An empty diagnostic slice excludes the documented diagnostic conditions;
/// it is not a guarantee of byte-for-byte round-trip equivalence.
#[derive(Debug)]
pub struct AttributesDecodeReport {
    attributes: Attributes,
    diagnostics: Vec<AttributeDecodeDiagnostic>,
}

/// Keeps permissively decoded values paired with their original substitutions.
impl AttributesDecodeReport {
    pub(super) fn new(attributes: Attributes, diagnostics: Vec<AttributeDecodeDiagnostic>) -> Self {
        Self {
            attributes,
            diagnostics,
        }
    }

    /// Borrows the attributes constructed by the same pass as the evidence.
    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    /// Borrows each substitution in serialized entry and field order.
    pub fn diagnostics(&self) -> &[AttributeDecodeDiagnostic] {
        &self.diagnostics
    }

    /// Consumes the report when only the permissive attributes are needed.
    pub fn into_attributes(self) -> Attributes {
        self.attributes
    }

    /// Moves out the attributes and evidence without copying either.
    pub fn into_parts(self) -> (Attributes, Vec<AttributeDecodeDiagnostic>) {
        (self.attributes, self.diagnostics)
    }
}
