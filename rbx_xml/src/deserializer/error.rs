use rbx_dom_weak::types::VariantType;
use thiserror::Error;

#[derive(Debug, Error)]
#[error(transparent)]
pub struct DecodeError(Box<ErrorKind>);

#[derive(Debug, Error)]
pub(crate) enum ErrorKind {
    #[error("unexpected end of file")]
    UnexpectedEof,
    #[error("unexpected token type in file")]
    UnexpectedToken,
    #[error("unexpectedly got element '{got}' when expecting '{expected}'")]
    UnexpectedElement { expected: String, got: String },
    #[error("unknown element {0}")]
    UnknownElement(String),
    #[error("cannot be strict with {0} without a database")]
    StrictWithoutDatabase(&'static str),

    #[error("property 'Name' must be a string")]
    NameNotString(VariantType),
    #[error("unknown class name '{0}'")]
    UnknownClass(String),

    #[error("property of type {0} without 'name' attribute")]
    UnnamedProperty(String),
    #[error("duplicate property {0} on Instance")]
    DuplicateProperty(String),
    #[error("unknown property {0}.{1}")]
    UnknownProperty(String, String),
    #[error("could not deserialize property of type '{0}'")]
    UnknownType(String),

    #[error("property could not be converted: {0}")]
    BadConversion(#[from] super::conversions::ConversionError),

    #[error("not a valid Roblox file because: {0}")]
    InvalidFile(&'static str),
    #[error("missing attribute {name} on element {element}")]
    MissingAttribute { name: String, element: String },
    #[error("invalid Roblox file version {0}, expected 4")]
    InvalidVersion(String),

    #[error("could not convert text to utf8: {0}")]
    NonUtf8Text(#[from] std::string::FromUtf8Error),
    #[error("XML parsing error: {0}")]
    XmlParsing(#[from] quick_xml::Error),
    #[error("XML attribute parsing error: {0}")]
    XmlAttribute(#[from] quick_xml::events::attributes::AttrError),
    #[error("invalid base64 string: {0}")]
    InvalidBase64(#[from] base64::DecodeError),

    #[error("error when reading property at character {offset}: {message}")]
    InvalidData { offset: usize, message: String },
}

impl ErrorKind {
    pub fn err(self) -> DecodeError {
        DecodeError(Box::from(self))
    }
}

impl DecodeError {
    pub(crate) fn new(kind: ErrorKind) -> Self {
        Self(Box::from(kind))
    }
}

impl<T> From<T> for DecodeError
where
    ErrorKind: From<T>,
{
    fn from(value: T) -> Self {
        Self(Box::from(ErrorKind::from(value)))
    }
}
