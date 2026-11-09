/// Borrows the exact source location until the selected sink decides retention.
pub(super) struct ColorSequenceEnvelopeSource<'attribute> {
    pub(super) attribute: &'attribute str,
    pub(super) index: u32,
    pub(super) keypoint_index: u32,
    pub(super) found_bits: u32,
}
