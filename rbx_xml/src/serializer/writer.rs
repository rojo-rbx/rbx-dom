//! Wrapper type for a quick_xml writer. This is just a few convenience
//! functions, as the actual API is rather straightforward.
use std::io;

use base64::Engine;
use quick_xml::{
    events::{BytesCData, BytesEnd, BytesStart, BytesText, Event},
    Writer,
};
use rbx_dom_weak::types::Variant;

use super::data_types::try_serialize_value;
use super::error::EncodeError;

pub struct XmlWriter<W: io::Write> {
    inner: Writer<W>,
}

impl<W: io::Write> XmlWriter<W> {
    pub fn into_inner(self) -> W {
        self.inner.into_inner()
    }

    pub fn new(writer: W, indent: Option<(u8, usize)>) -> XmlWriter<W> {
        XmlWriter {
            inner: if let Some((indent_char, indent_size)) = indent {
                Writer::new_with_indent(writer, indent_char, indent_size)
            } else {
                Writer::new(writer)
            },
        }
    }

    #[must_use]
    pub fn start_element<'a>(&'a mut self, name: &'a str) -> StartBuilder<'a, W> {
        StartBuilder {
            writer: self,
            event: BytesStart::new(name),
        }
    }

    pub fn end_element(&mut self, name: &str) -> Result<(), EncodeError> {
        self.inner.write_event(Event::End(BytesEnd::new(name)))?;
        Ok(())
    }

    /// Writes a string of text as the contents of the current element.
    /// This will automatically escape text but may trim whitespace.
    /// Use `write_raw_text` to preserve the exact text.
    pub fn write_text(&mut self, text: &str) -> Result<(), EncodeError> {
        self.inner.write_event(Event::Text(BytesText::new(text)))?;
        Ok(())
    }

    /// Writes a raw string as the contents of the current element.
    /// This differs from `write_text` in that it preserves whitespace and
    /// wraps the text in `CDATA`. As a result however, this will raise an
    /// error if `text` contains the sequence `]]>`.
    pub fn write_raw_text(&mut self, text: &str) -> Result<(), EncodeError> {
        self.inner
            .write_event(Event::CData(BytesCData::new(text)))?;
        Ok(())
    }

    /// Writes a sequence of bytes as a Base64 string. The passed bytes do not
    /// need to be valid UTF-8.
    pub fn write_base64(&mut self, bytes: &[u8]) -> Result<(), EncodeError> {
        // The old implementation didn't wrap base64, so we also don't
        self.write_text(&base64::prelude::BASE64_STANDARD.encode(bytes))
    }

    /// Writes an element named `name` and writes `value` into it. This is a
    /// convenience function that combines `start_element`, `write_text` and
    /// `end_element` without any attributes.
    pub fn write_element<V: Into<String>>(
        &mut self,
        name: &str,
        value: V,
    ) -> Result<(), EncodeError> {
        self.inner
            .create_element(name)
            .write_text_content(BytesText::new(&value.into()))?;
        Ok(())
    }

    pub fn write_rbx(&mut self, value: impl Into<Variant>) -> Result<(), EncodeError> {
        try_serialize_value(self, &value.into())
    }
}

pub struct StartBuilder<'a, W: io::Write> {
    writer: &'a mut XmlWriter<W>,
    event: BytesStart<'a>,
}

impl<'a, W: io::Write> StartBuilder<'a, W> {
    #[must_use]
    pub fn attribute(mut self, name: &'a str, value: &'a str) -> Self {
        self.event.push_attribute((name, value));
        self
    }

    pub fn finalize(self) -> Result<(), EncodeError> {
        self.writer.inner.write_event(Event::Start(self.event))?;
        Ok(())
    }
}