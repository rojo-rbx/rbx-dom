use std::{
    fmt,
    io::{self, Read, Write},
};

use rbx_dom_weak::RbxValueType;

#[derive(Debug)]
pub struct DecodeError {
    // This indirection drops the size of the error type substantially (~150
    // bytes to 8 on 64-bit), which is important since it's passed around every
    // function!
    inner: Box<DecodeErrorImpl>,
}

impl DecodeError {
    pub(crate) fn new_from_reader<R: Read>(kind: DecodeErrorKind, reader: &xml::EventReader<R>) -> DecodeError {
        use xml::common::Position;

        let pos = reader.position();

        DecodeError {
            inner: Box::new(DecodeErrorImpl {
                kind,
                line: (pos.row + 1) as usize,
                column: pos.column as usize,
            }),
        }
    }

    pub fn line(&self) -> usize {
        self.inner.line
    }

    pub fn column(&self) -> usize {
        self.inner.column
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        write!(output, "line {}, column {}: {}", self.inner.line, self.inner.column, self.inner.kind)
    }
}

impl std::error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.inner.kind.source()
    }
}

#[derive(Debug)]
struct DecodeErrorImpl {
    kind: DecodeErrorKind,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub(crate) enum DecodeErrorKind {
    // Errors from other crates
    Xml(xml::reader::Error),
    ParseFloat(std::num::ParseFloatError),
    ParseInt(std::num::ParseIntError),
    DecodeBase64(base64::DecodeError),

    // Errors specific to rbx_xml
    WrongDocVersion(String),
    UnexpectedEof,
    UnexpectedXmlEvent(xml::reader::XmlEvent),
    MissingAttribute(&'static str),
    UnknownPropertyType(String),
    InvalidContent(&'static str),
    NameMustBeString(RbxValueType),
}

impl fmt::Display for DecodeErrorKind {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        use self::DecodeErrorKind::*;

        match self {
            Xml(err) => write!(output, "{}", err),
            ParseFloat(err) => write!(output, "{}", err),
            ParseInt(err) => write!(output, "{}", err),
            DecodeBase64(err) => write!(output, "{}", err),

            WrongDocVersion(version) => write!(output, "Invalid version '{}', expected version 4", version),
            UnexpectedEof => write!(output, "Unexpected end-of-file"),
            UnexpectedXmlEvent(event) => write!(output, "Unexpected XML event {:?}", event),
            MissingAttribute(attribute_name) => write!(output, "Missing attribute '{}'", attribute_name),
            UnknownPropertyType(prop_name) => write!(output, "Unknown property type '{}'", prop_name),
            InvalidContent(explain) => write!(output, "Invalid text content: {}", explain),
            NameMustBeString(ty) => write!(output, "The 'Name' property must be of type String, but it was {:?}", ty),
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

            WrongDocVersion(_)
            | UnexpectedEof
            | UnexpectedXmlEvent(_)
            | MissingAttribute(_)
            | UnknownPropertyType(_)
            | InvalidContent(_)
            | NameMustBeString(_) => None,
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
    // This Box helps reduce the size of EncodeError a lot, which is important.
    kind: Box<EncodeErrorKind>,
}

impl EncodeError {
    pub(crate) fn new_from_writer<W: Write>(kind: EncodeErrorKind, _writer: &xml::EventWriter<W>) -> EncodeError {
        EncodeError { kind: Box::new(kind) }
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

    UnsupportedPropertyType(RbxValueType),
}

impl fmt::Display for EncodeErrorKind {
    fn fmt(&self, output: &mut fmt::Formatter) -> fmt::Result {
        use self::EncodeErrorKind::*;

        match self {
            Io(err) => write!(output, "{}", err),
            Xml(err) => write!(output, "{}", err),

            UnsupportedPropertyType(ty) => write!(output, "Properties of type {:?} cannot be encoded yet", ty),
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