use std::io;

use rbx_dom_weak::types::{Content, Font, FontStyle};

use super::{EncodeError, XmlWriter};

pub fn font_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Font,
) -> Result<(), EncodeError> {
    // FIXME Font uses String instead of Content for Family and CacheFaceId
    // as a result, we cannot reuse the content serializer here without owning
    // the value we're serializing. So, we have to either reimplement
    // serializing here or just clone the Strings. Cloning is the better
    // option to me since it makes fixing it faster in the future.
    writer.write_rbx("Family", Content::from(value.family.clone()))?;
    writer.write_element("Weight", value.weight.as_u16())?;
    writer.write_element(
        "Style",
        match &value.style {
            FontStyle::Normal => "Normal",
            FontStyle::Italic => "Italic",
        },
    )?;
    if let Some(ref cached_face_id) = value.cached_face_id {
        writer.write_rbx("CachedFaceId", Content::from(cached_face_id.clone()))?;
    }

    Ok(())
}
