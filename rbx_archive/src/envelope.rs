use std::collections::BTreeMap;

use crate::{manifest::validate_json, Error, Result};

/// Borrowed view of an opaque object's version-0 runtime envelope.
#[derive(Debug)]
pub struct ContentEnvelope<'a> {
    /// Raw payload, which may be binary.
    pub payload: &'a [u8],
    /// Original JSON metadata object, or an empty string when absent.
    pub metadata: &'a str,
}

impl<'a> ContentEnvelope<'a> {
    /// Validates an envelope, bounding JSON metadata to one MiB.
    pub fn from_bytes(bytes: &'a [u8]) -> Result<Self> {
        Self::parse(bytes, 1024 * 1024)
    }

    pub(crate) fn parse(mut bytes: &'a [u8], max_metadata: u64) -> Result<Self> {
        if integer(&mut bytes)? != 0 {
            return Err(Error::Invalid("unsupported runtime envelope version"));
        }
        let payload = field(&mut bytes)?;
        let metadata = field(&mut bytes)?;
        if !bytes.is_empty() {
            return Err(Error::Invalid("trailing runtime envelope bytes"));
        }
        if metadata.len() as u64 > max_metadata {
            return Err(Error::Limit("runtime metadata bytes"));
        }
        let metadata = std::str::from_utf8(metadata)
            .map_err(|_| Error::Invalid("runtime metadata is not UTF-8"))?;
        let values = if metadata.is_empty() {
            BTreeMap::new()
        } else {
            if metadata.contains('\0') {
                return Err(Error::Invalid("NUL in runtime metadata"));
            }
            validate_json(metadata)?;
            serde_json::from_str::<BTreeMap<String, String>>(metadata)?
        };
        if payload.is_empty() && values.get("INTERNAL_TYPE").map(String::as_str) != Some("Text") {
            return Err(Error::Invalid(
                "only Text envelopes can have empty payloads",
            ));
        }
        Ok(Self { payload, metadata })
    }
}

fn integer(bytes: &mut &[u8]) -> Result<i32> {
    let raw: [u8; 4] = bytes
        .get(..4)
        .ok_or(Error::Invalid("truncated runtime envelope"))?
        .try_into()
        .unwrap();
    *bytes = &bytes[4..];
    Ok(i32::from_le_bytes(raw))
}
fn field<'a>(bytes: &mut &'a [u8]) -> Result<&'a [u8]> {
    let len = usize::try_from(integer(bytes)?)
        .map_err(|_| Error::Invalid("negative envelope field length"))?;
    let value = bytes
        .get(..len)
        .ok_or(Error::Invalid("truncated envelope field"))?;
    *bytes = &bytes[len..];
    Ok(value)
}

pub(crate) fn validate_identity(identity: &str) -> Result<()> {
    let suffix = identity
        .strip_prefix("rbxruntime://")
        .ok_or(Error::Invalid("invalid opaque identity prefix"))?;
    if suffix.is_empty()
        || suffix.len() > 256
        || suffix.bytes().filter(|byte| *byte == b'_').count() >= 3
        || !suffix
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"_+=(.".contains(&byte))
    {
        return Err(Error::Invalid("invalid opaque identity"));
    }
    Ok(())
}
