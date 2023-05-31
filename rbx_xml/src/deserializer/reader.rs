use std::{collections::HashMap, io};

use quick_xml::{
    events::{BytesEnd, BytesStart, Event},
    Reader,
};

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
        // We really don't need to use a hashmap for this but I'm feeling lazy
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

    pub fn get_attribute(&self, name: &str) -> Option<&str> {
        match self.1.get(name) {
            Some(value) => Some(&value),
            None => None,
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

impl<R: io::BufRead> XmlReader<R> {
    /// Creates a new `XmlReader` from the provided argument.
    pub(crate) fn from_reader(reader: R) -> Self {
        let mut inner = Reader::from_reader(reader);
        inner.trim_text(true);
        Self {
            reader: inner,
            event_buffer: Vec::new(),
            peeked: None,
            finished: false,
        }
    }

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

    // pub fn expect_peek(&mut self) -> Result<&XmlData, DecodeError> {
    //     match self.peek() {
    //         None => Err(ErrorKind::UnexpectedEof.err()),
    //         Some(Err(error)) => Err(error.clone()),
    //         Some(Ok(data)) => Ok(data),
    //     }
    // }

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
            _ => Err(ErrorKind::UnexpectedToken.err()),
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
            _ => Err(ErrorKind::UnexpectedToken.err()),
        }
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
                Some(Ok(_)) => return Ok(buffer),
                // `peek` returns `&Result` so we have to use `next`
                Some(Err(_)) => return Err(self.next().unwrap().unwrap_err()),
                None => return Ok(buffer),
            }
        }
    }

    pub fn eat_unknown_element(&mut self) -> Result<(), DecodeError> {
        let mut depth = 0;
        log::debug!("eating unknown element");

        loop {
            match self.expect_next()? {
                XmlData::ElementStart { .. } => {
                    depth += 1;
                }
                XmlData::ElementEnd { .. } => {
                    depth -= 1;
                    if depth == 0 {
                        return Ok(());
                    }
                }
                _ => {}
            }
        }
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
            Ok(event) => match event.into_owned() {
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
                _ => Err(ErrorKind::UnexpectedToken.into()),
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
        let start = reader.expect_start_with_name("bool").unwrap();
        let content = reader.eat_text().unwrap();
        let end = reader.expect_end_with_name("bool").unwrap();

        assert_eq!(start.name(), "bool");
        assert!(matches!(start.get_attribute("name"), Some("Test")));
        assert_eq!(content, "true");
        assert_eq!(end.name(), "bool");
    }
}
