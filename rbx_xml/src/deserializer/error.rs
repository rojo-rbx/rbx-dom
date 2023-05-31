use std::fmt::Display;

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

    #[error("could not convert text to utf8: {0}")]
    NonUtf8Text(#[from] std::string::FromUtf8Error),
    #[error("XML parsing error: {0}")]
    XmlParsing(#[from] quick_xml::Error),
    #[error("XML attribute parsing error: {0}")]
    XmlAttribute(#[from] quick_xml::events::attributes::AttrError),

    #[error("{0}")]
    Custom(String),
}

impl ErrorKind {
    pub fn err(self) -> DecodeError {
        DecodeError(Box::from(self))
    }
}

impl DecodeError {
    fn custom<M: Display + Send + Sync + 'static>(message: M) -> Self {
        Self(Box::from(ErrorKind::Custom(message.to_string())))
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
