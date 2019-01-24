#[macro_export]
macro_rules! read_event {
    {$reader:expr, $xmlevent:pat => $body:expr} => {
        loop {
            match $reader.next().ok_or(crate::deserializer::DecodeError::Message("Unexpected EOF"))?? {
                $xmlevent => break $body,
                ::xml::reader::XmlEvent::Whitespace(_) => {},
                invalid => {
                    ::log::trace!("Expected event {}, got event {:?}", stringify!($xmlevent), invalid);
                    return Err(crate::deserializer::DecodeError::MalformedDocument);
                },
            }
        }
    };
}