use std::io::BufRead;

use rbx_dom_weak::types::{Font, FontStyle, FontWeight};

use crate::deserializer::{
    data_types::{content_deserializer, string_deserializer},
    error::DecodeError,
    reader::{XmlData, XmlReader},
};

pub fn font_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Font, DecodeError> {
    if matches!(reader.peek(), Some(Ok(XmlData::ElementEnd{name})) if name == "Font") {
        return Ok(Font::default());
    }

    let family = reader
        .read_named_with("Family", content_deserializer)?
        .into_string();
    let weight = FontWeight::from_u16(reader.read_named_with("Weight", u16_deserializer)?)
        .unwrap_or_default();

    let style = match reader
        .read_named_with("Style", string_deserializer)?
        .as_str()
    {
        "Normal" => FontStyle::Normal,
        "Italic" => FontStyle::Italic,
        _ => FontStyle::Normal,
    };

    let cached_face_id = match reader.peek() {
        Some(Ok(XmlData::ElementStart { name, .. })) if name == "CachedFaceId" => Some(
            reader
                .read_named_with("CachedFaceId", content_deserializer)?
                .into_string(),
        ),
        _ => None,
    };

    Ok(Font {
        family,
        weight,
        style,
        cached_face_id,
    })
}

fn u16_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<u16, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not read 16-bit uint from `{content}` because {err}"
        ))
    })
}
