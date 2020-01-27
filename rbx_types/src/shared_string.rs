/// A version of `BinaryString` used for data that's commonly repeated.
/// `rbx_types` automatically deduplicates data as it's loaded into
/// `SharedString` values.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SharedString;
