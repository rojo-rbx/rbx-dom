use serde::Serialize;
use serde_value::Value;

use crate::{DiagnosticSerializationError, Variant};

/// The human-readable canonical serde value tree with JSON-exact numeric leaves.
///
/// This is outgoing evidence, not a deserializer or a second Roblox value table.
/// The canonical `Variant` serializer owns structure and human-readable forms:
/// referents and unique IDs are hexadecimal, and shared strings are base64.
/// `serde-value` owns traversal; this tree retains its typed data-model cases.
#[derive(Debug, PartialEq, Serialize)]
#[cfg_attr(feature = "typescript", derive(ts_rs::TS))]
pub enum DiagnosticValue {
    /// A boolean from the canonical serializer.
    Bool(bool),
    /// An unsigned eight-bit integer.
    U8(u8),
    /// An unsigned sixteen-bit integer.
    U16(u16),
    /// An unsigned thirty-two-bit integer.
    U32(u32),
    /// A full unsigned domain, emitted as decimal text for JavaScript.
    U64(
        #[serde(serialize_with = "crate::diagnostic_serde::decimal")]
        #[cfg_attr(feature = "typescript", ts(as = "String"))]
        u64,
    ),
    /// A signed eight-bit integer.
    I8(i8),
    /// A signed sixteen-bit integer.
    I16(i16),
    /// A signed thirty-two-bit integer.
    I32(i32),
    /// A full signed domain, emitted as decimal text for JavaScript.
    I64(
        #[serde(serialize_with = "crate::diagnostic_serde::decimal")]
        #[cfg_attr(feature = "typescript", ts(as = "String"))]
        i64,
    ),
    /// Exact original IEEE 754 single-precision bits.
    F32(u32),
    /// Exact original IEEE 754 double-precision bits, emitted as sixteen hex digits.
    F64(
        #[serde(serialize_with = "crate::diagnostic_serde::hex64")]
        #[cfg_attr(feature = "typescript", ts(as = "String"))]
        u64,
    ),
    /// A Unicode scalar value.
    Char(char),
    /// A canonical textual value or map key.
    String(String),
    /// A unit value, distinct from an absent option.
    Unit,
    /// An optional value, preserving its serde case.
    Option(Option<Box<DiagnosticValue>>),
    /// A newtype wrapper retained by serde-value.
    Newtype(Box<DiagnosticValue>),
    /// An ordered sequence.
    Seq(Vec<DiagnosticValue>),
    /// Typed key/value pairs; JSON object-key coercion is not used.
    Map(Vec<(DiagnosticValue, DiagnosticValue)>),
    /// Raw bytes emitted by the canonical serializer.
    Bytes(Vec<u8>),
}

/// Projects existing native values without changing their ordinary serde contract.
impl DiagnosticValue {
    /// Runs the canonical human-readable serializer, preserving all numeric bits.
    ///
    /// `serde-value` uses Serde's human-readable mode. Current native maps use
    /// string or enum keys; this entry does not accept arbitrary serde map types.
    /// Returns the original canonical serialization failure when traversal fails.
    pub fn from_variant(value: &Variant) -> Result<Self, DiagnosticSerializationError> {
        serde_value::to_value(value)
            .map(Self::from_value)
            .map_err(|source| DiagnosticSerializationError::Value { source })
    }

    fn from_value(value: Value) -> Self {
        match value {
            Value::Bool(value) => Self::Bool(value),
            Value::U8(value) => Self::U8(value),
            Value::U16(value) => Self::U16(value),
            Value::U32(value) => Self::U32(value),
            Value::U64(value) => Self::U64(value),
            Value::I8(value) => Self::I8(value),
            Value::I16(value) => Self::I16(value),
            Value::I32(value) => Self::I32(value),
            Value::I64(value) => Self::I64(value),
            Value::F32(value) => Self::F32(value.to_bits()),
            Value::F64(value) => Self::F64(value.to_bits()),
            Value::Char(value) => Self::Char(value),
            Value::String(value) => Self::String(value),
            Value::Unit => Self::Unit,
            Value::Option(value) => {
                Self::Option(value.map(|value| Box::new(Self::from_value(*value))))
            }
            Value::Newtype(value) => Self::Newtype(Box::new(Self::from_value(*value))),
            Value::Seq(value) => Self::Seq(value.into_iter().map(Self::from_value).collect()),
            Value::Map(value) => Self::Map(
                value
                    .into_iter()
                    .map(|(key, value)| (Self::from_value(key), Self::from_value(value)))
                    .collect(),
            ),
            Value::Bytes(value) => Self::Bytes(value),
        }
    }
}
