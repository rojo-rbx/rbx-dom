use std::{collections::HashMap, io};

use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Reader,
};

use base64::Engine;

use super::error::{DecodeError, ErrorKind};

pub type XmlReadResult = Result<XmlData, DecodeError>;

pub struct XmlReader<R: io::BufRead> {
    reader: Reader<R>,
    event_buffer: Vec<u8>,
    peeked: Option<XmlReadResult>,
    finished: bool,
}

/// Represents an owned piece of XML data
#[derive(Debug, PartialEq)]
pub enum XmlData {
    Text(String),
    ElementStart {
        name: String,
        attributes: HashMap<String, String>,
    },
    ElementEnd {
        name: String,
    },
}
pub struct ElementStart(String, HashMap<String, String>);
pub struct ElementEnd(String);

impl ElementStart {
    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn attributes(&self) -> &HashMap<String, String> {
        &self.1
    }

    pub fn get_attribute(&mut self, name: &str) -> Result<String, DecodeError> {
        match self.1.get(name) {
            Some(_) => Ok(self.1.remove(name).unwrap()),
            None => Err(ErrorKind::MissingAttribute {
                name: name.into(),
                element: self.0.clone(),
            }
            .err()),
        }
    }
}

impl ElementEnd {
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl<'a> XmlReader<io::BufReader<&'a [u8]>> {
    pub(crate) fn from_str(string: &'a str) -> Self {
        let mut inner = Reader::from_reader(io::BufReader::new(string.as_bytes()));
        inner.trim_text(true);
        Self {
            reader: inner,
            event_buffer: Vec::new(),
            peeked: None,
            finished: false,
        }
    }
}

impl<R: io::Read> XmlReader<io::BufReader<R>> {
    /// Creates a new `XmlReader` from the provided argument.
    pub(crate) fn from_reader(reader: R) -> Self {
        let mut inner = Reader::from_reader(io::BufReader::new(reader));
        inner.trim_text(true);
        Self {
            reader: inner,
            event_buffer: Vec::new(),
            peeked: None,
            finished: false,
        }
    }
}

impl<R: io::BufRead> XmlReader<R> {
    pub fn peek(&mut self) -> Option<&XmlReadResult> {
        if self.peeked.is_some() {
            return self.peeked.as_ref();
        }
        self.peeked = self.next();
        self.peeked.as_ref()
    }

    pub fn expect_next(&mut self) -> XmlReadResult {
        match self.next() {
            None => Err(ErrorKind::UnexpectedEof.err()),
            Some(Err(error)) => Err(error),
            Some(Ok(data)) => Ok(data),
        }
    }

    pub fn expect_start_with_name(
        &mut self,
        expected_name: &str,
    ) -> Result<ElementStart, DecodeError> {
        match self.expect_next()? {
            XmlData::ElementStart { name, attributes } if name == expected_name => {
                Ok(ElementStart(name, attributes))
            }
            XmlData::ElementStart { name, .. } => Err(ErrorKind::UnexpectedElement {
                expected: expected_name.into(),
                got: name,
            }
            .err()),
            XmlData::ElementEnd { name } => Err(ErrorKind::ExpectingStartGotEnd {
                expected: expected_name.into(),
                got: name,
            }
            .err()),
            XmlData::Text(_) => Err(ErrorKind::UnexpectedText(self.offset()).err()),
        }
    }

    pub fn expect_end_with_name(&mut self, expected_name: &str) -> Result<ElementEnd, DecodeError> {
        match self.expect_next()? {
            XmlData::ElementEnd { name } if name == expected_name => Ok(ElementEnd(name)),
            XmlData::ElementEnd { name } => Err(ErrorKind::UnexpectedElement {
                expected: expected_name.into(),
                got: name,
            }
            .err()),
            XmlData::ElementStart { name, .. } => Err(ErrorKind::ExpectingEndGotStart {
                expected: expected_name.into(),
                got: name,
            }
            .err()),
            XmlData::Text(_) => Err(ErrorKind::UnexpectedText(self.offset()).err()),
        }
    }

    /// Deserializes a `T` using the provided function, expecting it to be
    /// contained inside of an element with the provided `name`.
    pub fn read_named_with<T, F>(&mut self, name: &str, mut with: F) -> Result<T, DecodeError>
    where
        F: Fn(&mut Self) -> Result<T, DecodeError>,
    {
        self.expect_start_with_name(name)?;
        let value = with(self)
            .map_err(|err| self.error(format!("error reading value for element {name}: {err}")))?;
        self.expect_end_with_name(name)?;
        Ok(value)
    }

    pub fn eat_text(&mut self) -> Result<String, DecodeError> {
        let mut buffer = String::new();
        loop {
            match self.peek() {
                Some(Ok(XmlData::Text(text))) => {
                    buffer.push_str(text);
                    // We want this to panic if we're wrong anyway
                    self.next().unwrap().unwrap();
                }
                _ => return Ok(buffer),
            }
        }
    }

    pub fn eat_base64(&mut self) -> Result<Vec<u8>, DecodeError> {
        log::trace!("converting string from base64");
        let mut buffer = self.eat_text()?;
        // The maintainer of the base64 library is adamantly opposed to adding
        // any support for whitespace, so we simply have to filter it out using
        // `retain`.
        buffer.retain(|b| !b.is_ascii_whitespace());
        base64::prelude::BASE64_STANDARD
            .decode(&mut buffer)
            .map_err(DecodeError::from)
    }

    pub fn skip_element(&mut self) -> Result<(), DecodeError> {
        let mut depth = 0;
        loop {
            match self.expect_next()? {
                XmlData::ElementEnd { .. } => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                XmlData::ElementStart { .. } => depth += 1,
                _ => {}
            }
        }
    }

    pub fn error<M: Into<String>>(&self, message: M) -> DecodeError {
        ErrorKind::InvalidData {
            offset: self.offset(),
            message: message.into(),
        }
        .err()
    }

    pub fn offset(&self) -> usize {
        self.reader.buffer_position()
    }
}

impl<R: io::BufRead> Iterator for XmlReader<R> {
    type Item = XmlReadResult;

    fn next(&mut self) -> Option<Self::Item> {
        if self.peeked.is_some() {
            return self.peeked.take();
        }
        if self.finished {
            return None;
        }

        let xml_event = self.reader.read_event_into(&mut self.event_buffer);
        let ret = match xml_event {
            Ok(event) => match event {
                // We need to eventually own this data but right now we don't have to
                Event::CData(data) => parse_text(&data),
                Event::Text(data) => parse_text(&data),
                Event::Start(data) => parse_start(data),
                // AFAIK we don't have any of these right now, so we should probably just error when we encounter them.
                // Event::Empty(data) => parse_start(data),
                Event::End(data) => parse_end(data),
                Event::Eof => {
                    self.finished = true;
                    return None;
                }
                _ => Err(ErrorKind::UnexpectedToken(self.reader.buffer_position()).err()),
            },
            Err(error) => Err(error.into()),
        };

        if matches!(ret, Err(_)) {
            self.finished = true;
        }
        Some(ret)
    }
}

fn parse_start(data: BytesStart) -> Result<XmlData, DecodeError> {
    let name = to_str_helper(data.name().into_inner())?;
    let mut attributes = HashMap::new();
    for attribute in data.attributes() {
        match attribute {
            Ok(attr) => {
                attributes.insert(
                    to_str_helper(attr.key.into_inner())?,
                    to_str_helper(&attr.value)?,
                );
            }
            Err(err) => return Err(err.into()),
        }
    }

    Ok(XmlData::ElementStart { name, attributes })
}

fn parse_end(data: BytesEnd) -> Result<XmlData, DecodeError> {
    to_str_helper(data.name().into_inner()).map(|name| XmlData::ElementEnd { name })
}

fn parse_text(data: &[u8]) -> Result<XmlData, DecodeError> {
    String::from_utf8(data.to_vec())
        .map_err(DecodeError::from)
        .map(XmlData::Text) // Tuple enums are essentially functions
}

fn to_str_helper(slice: &[u8]) -> Result<String, DecodeError> {
    String::from_utf8(slice.to_vec()).map_err(DecodeError::from)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn iter_does_work() {
        let document = r#"
        <tag1 att1 = "FOO">
            <tag2 att2 = "BAR">Tag2 Content</tag2>
            <tag3><![CDATA[   CData Content   ]]></tag3>
            <tag2>   Tag2 Content No Space   </tag2>
            <tag3 att3 = "BAZ"></tag3>
        </tag1>"#;
        for event in XmlReader::from_str(document) {
            event.unwrap();
        }
    }

    #[test]
    fn read_bool() {
        let document = r#"
        <bool name = "Test">true</bool>
        "#;
        let mut reader = XmlReader::from_str(document);
        let mut start = reader.expect_start_with_name("bool").unwrap();
        let content = reader.eat_text().unwrap();
        let end = reader.expect_end_with_name("bool").unwrap();

        assert_eq!(start.name(), "bool");
        assert_eq!(start.get_attribute("name").unwrap(), "Test".to_owned());
        assert_eq!(content, "true");
        assert_eq!(end.name(), "bool");
    }
}
