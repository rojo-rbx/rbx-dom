use rbx_dom_weak::types::VariantType;
use thiserror::Error;

// The redirection used for `DecodeError` where it contains a boxed enum
// is so that the size `DecodeError` occupies on the stack is kept small.
// This is important because the type is included in the return type of most
// functions in the deserializer.

#[derive(Debug, Error)]
#[error(transparent)]
/// An error that may be raised when deserializing a file.
pub struct DecodeError(Box<ErrorKind>);

#[derive(Debug, Error)]
#[non_exhaustive]
/// The type of error that a DecodeError is. This includes parsing errors,
/// logic errors, and database failures.
pub(crate) enum ErrorKind {
    /// An element that the parser doesn't know how to read was encountered.
    #[error("unknown element {0}")]
    UnknownElement(String),

    /// The 'Name' property on an Instance was not a string.
    #[error("property 'Name' must be a string")]
    NameNotString(VariantType),

    /// A property was duplicated on an Instance
    #[error("duplicate property {0} on Instance")]
    DuplicateProperty(String),
    /// A property that isn't known to the database was present on an Instance.
    /// This will only be raised when the corresponding configuration option is
    /// set.
    #[error("unknown property {0}.{1}")]
    UnknownProperty(String, String),
    /// A property with a type that's not known to the parser was encountered.
    /// This will only be raised when the corresponding configuration option is
    /// set.
    #[error("could not deserialize property of type '{0}'")]
    UnknownType(String),

    /// A conversion between two data types wasn't possible. This occurs when
    /// a property's type does not match the type it is meant to be
    /// and it cannot be converted to the correct one. As such, this will only
    /// be raised when a database is used.
    #[error(
        "property {class}.{name} could not be converted from {from:?} to {to:?} because: {error}"
    )]
    ConversionFail {
        class: String,
        name: String,
        from: VariantType,
        to: VariantType,
        error: super::conversions::ConversionError,
    },

    /// A specific attribute should be present on an element but was not.
    #[error("missing attribute {name} on element {element}")]
    MissingAttribute { name: String, element: String },
    /// The wrong version was specified in the opening tag of a file.
    #[error("invalid Roblox file version {0}, expected 4")]
    InvalidVersion(String),

    /// The end of the file was unexpectedly reached
    #[error("unexpected end of file")]
    UnexpectedEof,
    /// The next token that was read when parsing the file did not line up with
    /// what was expected.
    /// Examples include reading a tag when text was expected.
    #[error("unexpected token type at offset {0}")]
    UnexpectedToken(usize),
    /// A particular element name was expected but the next one was named
    /// something else.
    #[error("got element '{got}' when expecting '{expected}'")]
    UnexpectedElement { expected: String, got: String },
    /// When expecting the start of an element, an ending was encountered
    /// instead.
    #[error("got end of element '{got}' when expecting start of element '{expected}'")]
    ExpectingStartGotEnd { expected: String, got: String },
    /// When expecting the end of an element, a start was encountered instead.
    #[error("got start of element '{got}' when expecting end of element '{expected}'")]
    ExpectingEndGotStart { expected: String, got: String },
    #[error("got text when expecting an element start or end at offset {0}")]
    UnexpectedText(usize),

    /// When reading an attribute, element name, or text file, it could not be
    /// converted to valid UTF-8.
    #[error("could not convert text to utf8: {0}")]
    NonUtf8Text(#[from] std::string::FromUtf8Error),
    /// There was an error raised while parsing the file using `quick_xml`.
    #[error("XML parsing error: {0}")]
    XmlParsing(#[from] quick_xml::Error),
    /// There was an error raised while parsing an attribute using `quick_xml`.
    #[error("XML attribute parsing error: {0}")]
    XmlAttribute(#[from] quick_xml::events::attributes::AttrError),
    /// There was an error while decoding a Base64 string.
    #[error("invalid base64 string: {0}")]
    InvalidBase64(#[from] base64::DecodeError),
    #[error("could not migrate property: {0}")]
    MigrationFail(#[from] rbx_reflection::MigrationError),

    /// A property could not be read because of invalid data inside it.
    #[error("{0}")]
    InvalidData(String),
    #[error("property {name} at offset {offset} could not be read: {message}")]
    PropertyNotReadable {
        offset: usize,
        name: String,
        message: String,
    },
}

impl ErrorKind {
    /// A convenience function for turning an `ErrorKind` into a `DecodeError`.
    pub fn err(self) -> DecodeError {
        DecodeError(Box::from(self))
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
