use std::io;

use thiserror::Error;

use crate::types::InvalidTypeError;

/// Represents an error that occurred during deserialization.
#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    source: Box<InnerError>,
}

impl From<InnerError> for Error {
    fn from(inner: InnerError) -> Self {
        Self {
            source: Box::new(inner),
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum InnerError {
    #[error(transparent)]
    Io {
        #[from]
        source: io::Error,
    },

    #[error("Invalid file header")]
    BadHeader,

    #[error("Unknown file version {version}. Known versions are: 0")]
    UnknownFileVersion { version: u16 },

    #[error("Unknown version {version} for chunk {chunk_name}")]
    UnknownChunkVersion {
        chunk_name: &'static str,
        version: u32,
    },

    #[error(transparent)]
    InvalidTypeError {
        #[from]
        source: InvalidTypeError,
    },

    #[error(
        "Type mismatch: Property {type_name}.{prop_name} should be {valid_type_names}, but it was {actual_type_name}",
    )]
    PropTypeMismatch {
        type_name: String,
        prop_name: String,
        valid_type_names: &'static str,
        actual_type_name: String,
    },

    #[error("Invalid property data: Property {type_name}.{prop_name} was expected to be {valid_value}, but it was {actual_value}")]
    InvalidPropData {
        type_name: String,
        prop_name: String,
        valid_value: &'static str,
        actual_value: String,
    },

    #[error("File referred to type ID {type_id}, which was not declared")]
    InvalidTypeId { type_id: u32 },

    #[error("Invalid property data: CFrame property {type_name}.{prop_name} had an invalid rotation ID {id:02x}")]
    BadRotationId {
        type_name: String,
        prop_name: String,
        id: u8,
    },

    #[error("Expected type id for {expected_type_name} ({expected_type_id:02x}) when reading OptionalCFrame; got {actual_type_id:02x}")]
    BadOptionalCFrameFormat {
        expected_type_name: String,
        expected_type_id: u8,
        actual_type_id: u8,
    },

    #[error("Failed to deserialize {class_name}.{prop_name} because {source}")]
    BadPropertyValue {
        source: rbx_dom_weak::types::Error,
        prop_name: String,
        class_name: String,
    },
}
