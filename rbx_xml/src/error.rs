use std::{
    fmt,
    io::{self, Write},
};

use rbx_dom_weak::types::VariantType;

/// An error that can occur when serializing an XML-format model or place.
#[derive(Debug)]
pub struct EncodeError {
    // This Box helps reduce the size of EncodeError a lot, which is important.
    kind: Box<EncodeErrorKind>,
}

impl EncodeError {
    pub(crate) fn new_from_writer<W: Write>(
        kind: EncodeErrorKind,
        _writer: &xml::EventWriter<W>,
    ) -> EncodeError {
        EncodeError {
            kind: Box::new(kind),
        }
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        write!(output, "{}", self.kind)
    }
}

impl std::error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

#[derive(Debug)]
pub(crate) enum EncodeErrorKind {
    Io(io::Error),
    Xml(xml::writer::Error),
    Type(rbx_dom_weak::types::Error),

    UnknownProperty {
        class_name: String,
        property_name: String,
    },
    UnsupportedPropertyType(VariantType),
    UnsupportedPropertyConversion {
        class_name: String,
        property_name: String,
        expected_type: VariantType,
        actual_type: VariantType,
        message: String,
    },
}

impl fmt::Display for EncodeErrorKind {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        use self::EncodeErrorKind::*;

        match self {
            Io(err) => write!(output, "{}", err),
            Xml(err) => write!(output, "{}", err),
            Type(err) => write!(output, "{}", err),

            UnknownProperty {
                class_name,
                property_name,
            } => write!(
                output,
                "Property {}.{} is unknown",
                class_name, property_name
            ),
            UnsupportedPropertyType(ty) => {
                write!(output, "Properties of type {:?} cannot be encoded yet", ty)
            }
            UnsupportedPropertyConversion {
                class_name,
                property_name,
                expected_type,
                actual_type,
                message,
            } => write!(
                output,
                "Property {}.{} is expected to be of type {:?}, but it was of type {:?} \
                 When trying to convert the value, this error occured: {}",
                class_name, property_name, expected_type, actual_type, message
            ),
        }
    }
}

impl std::error::Error for EncodeErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::EncodeErrorKind::*;

        match self {
            Io(err) => Some(err),
            Xml(err) => Some(err),
            Type(err) => Some(err),

            UnknownProperty { .. }
            | UnsupportedPropertyType(_)
            | UnsupportedPropertyConversion { .. } => None,
        }
    }
}

impl From<xml::writer::Error> for EncodeErrorKind {
    fn from(error: xml::writer::Error) -> EncodeErrorKind {
        EncodeErrorKind::Xml(error)
    }
}

impl From<io::Error> for EncodeErrorKind {
    fn from(error: io::Error) -> EncodeErrorKind {
        EncodeErrorKind::Io(error)
    }
}

impl From<rbx_dom_weak::types::Error> for EncodeErrorKind {
    fn from(error: rbx_dom_weak::types::Error) -> EncodeErrorKind {
        EncodeErrorKind::Type(error)
    }
}
