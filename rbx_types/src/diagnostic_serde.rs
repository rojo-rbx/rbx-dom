use std::fmt::Display;

use serde::Serializer;

pub(crate) fn decimal<T: Display, S: Serializer>(
    value: &T,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&value.to_string())
}

pub(crate) fn hex64<S: Serializer>(bits: &u64, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format!("{bits:016x}"))
}
