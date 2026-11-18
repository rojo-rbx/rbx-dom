use rbx_dom_weak::WeakDom;

use crate::DecodeDiagnostic;

/// The DOM and information-loss evidence produced by one native decode pass.
///
/// An empty diagnostic slice means that none of the documented diagnostic
/// conditions were encountered. This is not validation of arbitrary native
/// format extensions or a guarantee of byte-for-byte round-trip equivalence.
#[derive(Debug)]
pub struct DecodeReport {
    dom: WeakDom,
    diagnostics: Vec<DecodeDiagnostic>,
}

/// Keeps decoded data and its evidence together until the caller consumes it.
impl DecodeReport {
    pub(crate) fn new(dom: WeakDom, diagnostics: Vec<DecodeDiagnostic>) -> Self {
        Self { dom, diagnostics }
    }

    /// Borrows the actual DOM constructed by the decoder, without another pass.
    pub fn dom(&self) -> &WeakDom {
        &self.dom
    }

    /// Borrows every recorded loss, including repeated occurrences.
    ///
    /// Chunk diagnostics precede property-value diagnostics, which are grouped
    /// in document instance order, then decode order within an instance.
    /// Unparented-instance diagnostics follow both, in document instance order.
    pub fn diagnostics(&self) -> &[DecodeDiagnostic] {
        &self.diagnostics
    }

    /// Consumes the report when only its permissively decoded DOM is needed.
    pub fn into_dom(self) -> WeakDom {
        self.dom
    }

    /// Moves out the DOM and its evidence without copying either collection.
    pub fn into_parts(self) -> (WeakDom, Vec<DecodeDiagnostic>) {
        (self.dom, self.diagnostics)
    }
}
