use rbx_dom_weak::WeakDom;

use crate::{DecodeDiagnostic, DecodeReport};

/// Selects evidence retention once for a complete decoding pass.
pub(super) trait DiagnosticSink: Default {
    /// Prevents an entry that discards evidence from producing an audited report.
    type Output;

    /// Evaluates evidence construction only when the entry requested it.
    fn record(&mut self, make: impl FnOnce() -> DecodeDiagnostic);

    /// Preserves instance grouping without allowing mixed retention modes.
    fn append(&mut self, other: &mut Self);

    /// Combines the DOM with retained evidence only for an audited entry.
    fn finish(self, dom: WeakDom) -> Self::Output;
}

/// Owns evidence only for the entry that returns it to its caller.
#[derive(Default)]
pub(super) struct CollectDiagnostics {
    diagnostics: Vec<DecodeDiagnostic>,
}

impl DiagnosticSink for CollectDiagnostics {
    type Output = DecodeReport;

    fn record(&mut self, make: impl FnOnce() -> DecodeDiagnostic) {
        self.diagnostics.push(make());
    }

    fn append(&mut self, other: &mut Self) {
        self.diagnostics.append(&mut other.diagnostics);
    }

    fn finish(self, dom: WeakDom) -> DecodeReport {
        DecodeReport::new(dom, self.diagnostics)
    }
}

/// Drops lazy evidence inputs at their source point, without constructing it.
#[derive(Default)]
pub(super) struct DiscardDiagnostics;

impl DiagnosticSink for DiscardDiagnostics {
    type Output = WeakDom;

    fn record(&mut self, _make: impl FnOnce() -> DecodeDiagnostic) {}

    fn append(&mut self, _other: &mut Self) {}

    fn finish(self, dom: WeakDom) -> WeakDom {
        dom
    }
}
