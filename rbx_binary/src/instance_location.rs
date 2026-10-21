use rbx_dom_weak::types::Ref;

/// Correlates a native document instance with the identity allocated in this pass.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InstanceLocation {
    document_referent: i32,
    dom_ref: Ref,
}

/// Preserves the pair without assuming native and DOM referents share numbering.
impl InstanceLocation {
    pub(crate) fn new(document_referent: i32, dom_ref: Ref) -> Self {
        Self {
            document_referent,
            dom_ref,
        }
    }

    /// Returns the signed instance ID read from the native document.
    pub fn document_referent(&self) -> i32 {
        self.document_referent
    }

    /// Returns the decoder-assigned identity, including for omitted nodes.
    ///
    /// Unparented instances have allocated identities but are absent from the
    /// returned DOM. A diagnostic's presence does not imply this ref resolves.
    pub fn dom_ref(&self) -> Ref {
        self.dom_ref
    }
}
