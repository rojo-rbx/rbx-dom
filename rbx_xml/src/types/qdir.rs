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

// This type is only implemented here because generate_reflection needs to set
// Roblox Studio's auto-recovery path in the global settings file (which is
// basically a .rbxmx). It's probably not useful otherwise.
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
