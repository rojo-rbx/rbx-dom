use std::{
    io::{Read, Write},
    path::PathBuf,
};

use rbx_dom_weak::types::QDir;

use crate::{
    core::XmlType,
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::XmlEventWriter,
};

impl XmlType for QDir {
    const XML_TAG_NAME: &'static str = "QDir";

    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError> {
        writer.write_string(self.path_buf().to_str().unwrap_or_default())
    }

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        Ok(QDir::from(PathBuf::from(reader.read_characters()?)))
    }
}

#[cfg(test)]
mod test {
    use crate::test_util;

    #[test]
    fn round_trip_windows_style() {
        test_util::test_xml_round_trip(
            &"C:\\Users\\lebronjames\\Documents\\ROBLOX\\AutoSaves".to_owned(),
        );
    }

    #[test]
    fn round_trip_unix_style() {
        test_util::test_xml_round_trip(&"/Users/jiminycricket/Pictures/.real".to_owned());
    }
}
