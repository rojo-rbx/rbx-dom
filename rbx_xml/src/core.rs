use std::io::{Read, Write};

use rbx_dom_weak::RbxValue;

use crate::{
    deserializer::{DecodeError, EventIterator},
    serializer::{EncodeError, XmlEventWriter},
};

pub trait XmlType<T: ?Sized> {
    const XML_NAME: &'static str;

    fn write_xml<W: Write>(
        writer: &mut XmlEventWriter<W>,
        name: &str,
        value: &T,
    ) -> Result<(), EncodeError>;

    fn read_xml<R: Read>(
        reader: &mut EventIterator<R>,
    ) -> Result<RbxValue, DecodeError>;
}