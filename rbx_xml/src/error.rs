use std::{
    fmt,
    io::{self, Read, Write},
};

use rbx_dom_weak::RbxValueType;

use crate::deserializer_core::DecodeError as OldDecodeError;
use crate::serializer_core::EncodeError as OldEncodeError;

#[derive(Debug)]
pub struct DecodeError {
    kind: DecodeErrorKind,
    line: u64,
    column: u64,
}

impl DecodeError {
    pub(crate) fn new_from_reader<R: Read>(kind: DecodeErrorKind, reader: &xml::EventReader<R>) -> DecodeError {
        use xml::common::Position;

        let pos = reader.position();

        DecodeError {
            kind,
            line: pos.row + 1,
            column: pos.column,
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        write!(output, "line {}, column {}: {}", self.line, self.column, self.kind)
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

// FIXME: This is temporarily while we transition error types
impl From<OldDecodeError> for DecodeError {
    fn from(value: OldDecodeError) -> DecodeError {
        DecodeError {
            kind: DecodeErrorKind::Old(value),
            line: 1,
            column: 1,
        }
    }
}

#[derive(Debug)]
pub(crate) enum DecodeErrorKind {
    // Errors from other crates
    Xml(xml::reader::Error),
    ParseFloat(std::num::ParseFloatError),
    ParseInt(std::num::ParseIntError),
    DecodeBase64(base64::DecodeError),

    // Errors specific to rbx_xml
    UnexpectedEof,
    UnexpectedXmlEvent(xml::reader::XmlEvent),
    MissingAttribute(&'static str),
    UnknownPropertyType(String),
    InvalidContent(&'static str),

    // FIXME: Temporary variant while we have two error types
    Old(OldDecodeError),
}

impl fmt::Display for DecodeErrorKind {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        use self::DecodeErrorKind::*;

        match self {
            Xml(err) => write!(output, "{}", err),
            ParseFloat(err) => write!(output, "{}", err),
            ParseInt(err) => write!(output, "{}", err),
            DecodeBase64(err) => write!(output, "{}", err),

            UnexpectedEof => write!(output, "Unexpected end-of-file"),
            UnexpectedXmlEvent(event) => write!(output, "Unexpected XML event {:?}", event),
            MissingAttribute(attribute_name) => write!(output, "Missing attribute '{}'", attribute_name),
            UnknownPropertyType(prop_name) => write!(output, "Unknown property type '{}'", prop_name),
            InvalidContent(explain) => write!(output, "Invalid text content: {}", explain),

            Old(old_error) => write!(output, "{}", old_error),
        }
    }
}

impl std::error::Error for DecodeErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::DecodeErrorKind::*;

        match self {
            Xml(err) => Some(err),
            ParseFloat(err) => Some(err),
            ParseInt(err) => Some(err),
            DecodeBase64(err) => Some(err),

            UnexpectedEof
            | UnexpectedXmlEvent(_)
            | MissingAttribute(_)
            | UnknownPropertyType(_)
            | InvalidContent(_) => None,

            Old(_) => None,
        }
    }
}

impl From<xml::reader::Error> for DecodeErrorKind {
    fn from(error: xml::reader::Error) -> DecodeErrorKind {
        DecodeErrorKind::Xml(error)
    }
}

impl From<std::num::ParseFloatError> for DecodeErrorKind {
    fn from(error: std::num::ParseFloatError) -> DecodeErrorKind {
        DecodeErrorKind::ParseFloat(error)
    }
}

impl From<std::num::ParseIntError> for DecodeErrorKind {
    fn from(error: std::num::ParseIntError) -> DecodeErrorKind {
        DecodeErrorKind::ParseInt(error)
    }
}

impl From<base64::DecodeError> for DecodeErrorKind {
    fn from(error: base64::DecodeError) -> DecodeErrorKind {
        DecodeErrorKind::DecodeBase64(error)
    }
}

#[derive(Debug)]
pub struct EncodeError {
    kind: EncodeErrorKind,
}

impl EncodeError {
    pub(crate) fn new_from_writer<W: Write>(kind: EncodeErrorKind, _writer: &xml::EventWriter<W>) -> EncodeError {
        EncodeError { kind }
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

// FIXME: This is temporarily while we transition error types
impl From<OldEncodeError> for EncodeError {
    fn from(error: OldEncodeError) -> EncodeError {
        EncodeError {
            kind: EncodeErrorKind::Old(error)
        }
    }
}

#[derive(Debug)]
pub(crate) enum EncodeErrorKind {
    Io(io::Error),
    Xml(xml::writer::Error),

    UnsupportedPropertyType(RbxValueType),

    Old(OldEncodeError),
}

impl fmt::Display for EncodeErrorKind {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        use self::EncodeErrorKind::*;

        match self {
            Io(err) => write!(output, "{}", err),
            Xml(err) => write!(output, "{}", err),

            UnsupportedPropertyType(ty) => write!(output, "Properties of type {:?} cannot be encoded yet", ty),

            Old(old_error) => write!(output, "{}", old_error),
        }
    }
}

impl std::error::Error for EncodeErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        use self::EncodeErrorKind::*;

        match self {
            Io(err) => Some(err),
            Xml(err) => Some(err),

            UnsupportedPropertyType(_) => None,

            Old(_) => None,
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