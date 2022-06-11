use std::io;

use rbx_dom_weak::types::Ref;
use thiserror::Error;

/// Represents an error that occurred during serialization.
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
pub(super) enum InnerError {
    #[error(transparent)]
    Io {
        #[from]
        source: io::Error,
    },

    #[error(
        "Property type mismatch: Expected {type_name}.{prop_name} to be of type {valid_type_names}, \
        but it was of type {actual_type_name} on instance {instance_full_name}",
    )]
    PropTypeMismatch {
        type_name: String,
        prop_name: String,
        valid_type_names: &'static str,
        actual_type_name: String,
        instance_full_name: String,
    },

    #[error("Unsupported property type: {type_name}.{prop_name} is of type {prop_type}")]
    UnsupportedPropType {
        type_name: String,
        prop_name: String,
        prop_type: String,
    },

    #[error(
        "Invalid property value: The instance {instance_full_name} had a property \
        ({type_name}.{prop_name}) of type {prop_type} with a value that could \
        not be written."
    )]
    InvalidPropValue {
        instance_full_name: String,
        type_name: String,
        prop_name: String,
        prop_type: String,
    },

    #[error("The instance with referent {referent:?} was not present in the dom.")]
    InvalidInstanceId { referent: Ref },
}
