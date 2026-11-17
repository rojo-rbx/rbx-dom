use std::{collections::BTreeSet, fmt};

use serde::{
    de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor},
    Deserialize,
};
use serde_json::value::RawValue;

use crate::{Error, Limits, Result};

/// A logical content record. Several records may name the same physical object.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ContentRecord {
    #[serde(rename = "kind")]
    _kind: String,
    /// Optional relative logical path, independent of the ZIP member name.
    #[serde(default, deserialize_with = "present")]
    pub path: Option<String>,
    /// Lowercase BLAKE3-128 digest of the object's bytes.
    pub object_digest: String,
    /// Uninterpreted JSON object. Its original token spellings are retained.
    #[serde(default, deserialize_with = "present")]
    pub metadata: Option<Box<RawValue>>,
}

fn present<'de, D: de::Deserializer<'de>, T: Deserialize<'de>>(
    decoder: D,
) -> std::result::Result<Option<T>, D::Error> {
    T::deserialize(decoder).map(Some)
}

/// The version-1 embedded place profile of a JSONL manifest.
#[derive(Debug)]
pub struct Manifest {
    /// Logical path identifying the binary place scene.
    pub place: String,
    /// Records in manifest order, including pathless records and aliases.
    pub content: Vec<ContentRecord>,
    pub(crate) raw: Vec<u8>,
}

#[derive(Deserialize)]
struct Kind {
    kind: String,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct Header {
    #[serde(rename = "kind")]
    _kind: String,
    version: u32,
    digest_algorithm: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Package {
    #[serde(rename = "kind")]
    _kind: String,
    #[serde(rename = "type")]
    package_type: String,
    place: String,
}

impl Manifest {
    pub(crate) fn parse(bytes: Vec<u8>, limits: &Limits) -> Result<Self> {
        if bytes.len() as u64 > limits.max_manifest_bytes {
            return Err(Error::Limit("manifest bytes"));
        }
        let text =
            std::str::from_utf8(&bytes).map_err(|_| Error::Invalid("manifest is not UTF-8"))?;
        if text.starts_with('\u{feff}') || text.contains('\0') {
            return Err(Error::Invalid("invalid manifest text"));
        }
        let mut header = false;
        let mut place = None;
        let mut content = Vec::new();
        let mut paths = BTreeSet::new();
        for (index, line) in text.lines().enumerate() {
            if line.trim().is_empty() {
                return Err(Error::Invalid("blank manifest record"));
            }
            if line.len() as u64 > limits.max_record_bytes {
                return Err(Error::Limit("manifest record bytes"));
            }
            validate_json(line)?;
            match serde_json::from_str::<Kind>(line)?.kind.as_str() {
                "header" => {
                    let Header {
                        version,
                        digest_algorithm,
                        ..
                    } = serde_json::from_str(line)?;
                    if index != 0 || header || version != 1 || digest_algorithm != "blake3-128" {
                        return Err(Error::Invalid("unsupported or misplaced manifest header"));
                    }
                    header = true;
                }
                "package" => {
                    let Package {
                        package_type,
                        place: path,
                        ..
                    } = serde_json::from_str(line)?;
                    if place.is_some() || package_type != "place" || !valid_path(&path) {
                        return Err(Error::Invalid("unsupported or duplicate package"));
                    }
                    place = Some(path);
                }
                "content" => {
                    let record: ContentRecord = serde_json::from_str(line)?;
                    if content.len() >= limits.max_entries {
                        return Err(Error::Limit("content records"));
                    }
                    if !valid_digest(&record.object_digest) {
                        return Err(Error::Invalid("invalid object digest"));
                    }
                    if let Some(path) = &record.path {
                        if !valid_path(path) || !paths.insert(path.clone()) {
                            return Err(Error::Invalid("invalid or duplicate logical path"));
                        }
                    }
                    if let Some(metadata) = &record.metadata {
                        if !metadata.get().trim_start().starts_with('{') {
                            return Err(Error::Invalid("content metadata must be an object"));
                        }
                    }
                    content.push(record);
                }
                _ => return Err(Error::Invalid("unsupported manifest record")),
            }
        }
        if !header {
            return Err(Error::Invalid("missing manifest header"));
        }
        let place = place.ok_or(Error::Invalid("missing place package"))?;
        if !paths.contains(&place) {
            return Err(Error::Invalid("place path is not declared"));
        }
        Ok(Self {
            place,
            content,
            raw: bytes,
        })
    }

    /// Original validated JSONL bytes, including record order and metadata.
    pub fn as_bytes(&self) -> &[u8] {
        &self.raw
    }

    /// Resolves the package's logical place path to its object digest.
    pub fn scene_digest(&self) -> &str {
        &self
            .content
            .iter()
            .find(|record| record.path.as_ref() == Some(&self.place))
            .expect("validated place path")
            .object_digest
    }
}

pub(crate) fn valid_digest(value: &str) -> bool {
    value.len() == 32
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn valid_path(path: &str) -> bool {
    !path.contains(['\\', ':', '\0'])
        && path.split('/').all(|part| !matches!(part, "" | "." | ".."))
}

// Validate without normalizing numbers or allocating a second JSON tree.
// serde_json normally accepts duplicate map keys, which is ambiguous for readers.
pub(crate) fn validate_json(text: &str) -> Result<()> {
    let mut decoder = serde_json::Deserializer::from_str(text);
    UniqueJson(0).deserialize(&mut decoder)?;
    decoder.end()?;
    Ok(())
}

struct UniqueJson(usize);
impl<'de> DeserializeSeed<'de> for UniqueJson {
    type Value = ();
    fn deserialize<D: de::Deserializer<'de>>(
        self,
        decoder: D,
    ) -> std::result::Result<(), D::Error> {
        decoder.deserialize_any(self)
    }
}
impl<'de> Visitor<'de> for UniqueJson {
    type Value = ();
    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("JSON with unique keys and at most 16 container levels")
    }
    fn visit_bool<E: de::Error>(self, _: bool) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_i64<E: de::Error>(self, _: i64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_u64<E: de::Error>(self, _: u64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_f64<E: de::Error>(self, _: f64) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_str<E: de::Error>(self, _: &str) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_unit<E: de::Error>(self) -> std::result::Result<(), E> {
        Ok(())
    }
    fn visit_seq<A: SeqAccess<'de>>(self, mut seq: A) -> std::result::Result<(), A::Error> {
        if self.0 >= 16 {
            return Err(de::Error::custom("JSON nesting exceeds 16 containers"));
        }
        while seq.next_element_seed(UniqueJson(self.0 + 1))?.is_some() {}
        Ok(())
    }
    fn visit_map<A: MapAccess<'de>>(self, mut map: A) -> std::result::Result<(), A::Error> {
        if self.0 >= 16 {
            return Err(de::Error::custom("JSON nesting exceeds 16 containers"));
        }
        let mut names = BTreeSet::new();
        while let Some(name) = map.next_key::<String>()? {
            if !names.insert(name) {
                return Err(de::Error::custom("duplicate JSON key"));
            }
            map.next_value_seed(UniqueJson(self.0 + 1))?;
        }
        Ok(())
    }
}
