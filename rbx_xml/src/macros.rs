#[macro_export]
macro_rules! read_event {
    {$reader:expr, $xmlevent:pat => $body:expr} => {
        loop {
            match $reader.next().ok_or(crate::deserializer::DecodeError::MalformedDocument)?? {
                $xmlevent => break $body,
                ::xml::reader::XmlEvent::Whitespace(_) => {},
                _ => return Err(crate::deserializer::DecodeError::MalformedDocument),
            }
        }
    };
}