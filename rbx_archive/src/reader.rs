use std::{
    cell::{Cell, RefCell},
    collections::{BTreeMap, BTreeSet},
    io::{self, Read, Seek, SeekFrom, Write},
};

use rbx_binary::{EmbeddedContentResolver, EmbeddedReference};
use rbx_dom_weak::{types::SharedString, WeakDom};
use zip::{read::Config, unstable::read::ZipIterable, CompressionMethod};

use crate::{
    envelope::validate_identity, invalid_io, manifest::valid_digest, ContentEnvelope, Error,
    Limits, Manifest, Result,
};

#[derive(Clone, Copy)]
struct Member {
    offset: u64,
    size: u64,
}

struct PhysicalMember {
    start: u64,
    end: u64,
    size: u64,
    crc: u32,
    descriptor: bool,
}

/// Validated archive directory and manifest, backed by a seekable input.
///
/// Opening scans and hashes all objects with a fixed-size scratch buffer.
/// Object bodies are loaded only by `read_object` or when restoring scene
/// properties. Do not modify the underlying input during this reader's lifetime.
pub struct Reader<R> {
    input: R,
    manifest: Manifest,
    members: BTreeMap<String, Member>,
    limits: Limits,
}

impl<R: Read + Seek> Reader<R> {
    /// Validates STORE ZIP members, CRCs, BLAKE3 digests, and the JSONL manifest.
    /// ZIP64 is accepted within the configured budgets. Encrypted entries,
    /// compressed ZIP members, and duplicate names are rejected.
    pub fn new(mut input: R, limits: Limits) -> Result<Self> {
        let length = input.seek(SeekFrom::End(0))?;
        if length > limits.max_archive_bytes {
            return Err(Error::Limit("archive bytes"));
        }
        input.seek(SeekFrom::Start(0))?;
        let mut magic = [0; 4];
        input.read_exact(&mut magic)?;
        if magic != *b"PK\x03\x04" {
            return Err(Error::Invalid("missing ZIP local header"));
        }
        let mut members = BTreeMap::new();
        let mut names = BTreeSet::new();
        let mut manifest = None;
        let mut total = 0u64;
        let mut ranges = Vec::new();
        let mut central_start = length;
        {
            let mut zip = ZipIterable::try_new(&mut input, Config::default())?
                .enforce_header_consistency(true);
            if zip.len() > limits.max_entries {
                return Err(Error::Limit("ZIP entries"));
            }
            let entries = zip.files()?;
            while let Some(entry) = entries.next() {
                let entry = entry?;
                let name = std::str::from_utf8(entry.name_raw())
                    .map_err(|_| Error::Invalid("non-UTF-8 member name"))?
                    .to_owned();
                if !names.insert(name.clone()) {
                    return Err(Error::Invalid("duplicate ZIP member"));
                }
                if entry.encrypted()
                    || entry.compression() != CompressionMethod::Stored
                    || entry.size() != entry.compressed_size()
                {
                    return Err(Error::Invalid(
                        "archive members must be unencrypted STORE files",
                    ));
                }
                let object_digest = if name == "manifest.jsonl" {
                    None
                } else {
                    let value = name
                        .strip_prefix("objects/")
                        .filter(|value| valid_digest(value))
                        .ok_or(Error::Invalid("unsupported ZIP member"))?;
                    Some(value.to_owned())
                };
                let size = entry.size();
                let limit = if object_digest.is_some() {
                    limits.max_object_bytes
                } else {
                    limits.max_manifest_bytes
                };
                if size > limit {
                    return Err(Error::Limit("member bytes"));
                }
                if object_digest.is_some() {
                    total = total
                        .checked_add(size)
                        .ok_or(Error::Limit("aggregate object bytes"))?;
                    if total > limits.max_total_object_bytes {
                        return Err(Error::Limit("aggregate object bytes"));
                    }
                }
                let start = entry.header_start();
                let crc = entry.crc32();
                let descriptor = entry.flags().is_using_data_descriptor();
                central_start = central_start.min(entry.central_header_start());
                let mut file = entry.with_reader(entries.reader_mut(), Default::default())?;
                let offset = file
                    .data_start()
                    .ok_or(Error::Invalid("missing member data offset"))?;
                let end = offset
                    .checked_add(size)
                    .filter(|end| *end <= length)
                    .ok_or(Error::Invalid("member outside archive"))?;
                ranges.push(PhysicalMember {
                    start,
                    end,
                    size,
                    crc,
                    descriptor,
                });
                if let Some(digest) = object_digest {
                    let mut hasher = blake3::Hasher::new();
                    let mut buffer = [0; 64 * 1024];
                    let mut read = 0;
                    loop {
                        let count = file.read(&mut buffer)?;
                        if count == 0 {
                            break;
                        }
                        read += count as u64;
                        hasher.update(&buffer[..count]);
                    }
                    if read != size || hasher.finalize().to_hex()[..32] != digest {
                        return Err(Error::Invalid("object digest or size mismatch"));
                    }
                    members.insert(digest, Member { offset, size });
                } else {
                    let mut bytes = Vec::new();
                    file.read_to_end(&mut bytes)?;
                    if bytes.len() as u64 != size {
                        return Err(Error::Invalid("truncated manifest"));
                    }
                    manifest = Some(Manifest::parse(bytes, &limits)?);
                }
            }
        }
        ranges.sort_unstable_by_key(|range| range.start);
        if ranges.first().is_none_or(|range| range.start != 0) {
            return Err(Error::Invalid("unindexed ZIP prefix"));
        }
        for (index, member) in ranges.iter().enumerate() {
            let next = ranges
                .get(index + 1)
                .map_or(central_start, |member| member.start);
            validate_member_end(&mut input, member, next)?;
        }
        let manifest = manifest.ok_or(Error::Invalid("missing manifest.jsonl"))?;
        let declared: BTreeSet<_> = manifest
            .content
            .iter()
            .map(|record| record.object_digest.as_str())
            .collect();
        if declared.len() != members.len()
            || !members.keys().all(|key| declared.contains(key.as_str()))
        {
            return Err(Error::Invalid("manifest and physical objects disagree"));
        }
        Ok(Self {
            input,
            manifest,
            members,
            limits,
        })
    }

    /// Logical content records and the place root.
    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Reads a validated object into memory, checking its digest again.
    pub fn read_object(&mut self, digest: &str) -> Result<Vec<u8>> {
        let member = self
            .members
            .get(digest)
            .ok_or(Error::Invalid("object not found"))?;
        self.input.seek(SeekFrom::Start(member.offset))?;
        let mut bytes = Vec::new();
        (&mut self.input)
            .take(member.size)
            .read_to_end(&mut bytes)?;
        if bytes.len() as u64 != member.size || crate::digest(&bytes) != digest {
            return Err(Error::Invalid("object changed or truncated"));
        }
        Ok(bytes)
    }

    /// Decodes the scene and restores embedded values into a DOM.
    /// The binary scene is buffered once; other objects are loaded on demand.
    pub fn deserialize(&mut self) -> Result<WeakDom> {
        self.deserialize_with(&rbx_binary::Deserializer::new())
    }

    /// Decodes using an existing binary configuration, including its reflection
    /// database. The object resolver belongs only to this operation.
    pub fn deserialize_with(
        &mut self,
        deserializer: &rbx_binary::Deserializer<'_>,
    ) -> Result<WeakDom> {
        let scene_digest = self.manifest.scene_digest().to_owned();
        let scene = self.read_object(&scene_digest)?;
        let resolver = Resolver {
            reader: RefCell::new(self),
            restored: Cell::new(0),
            identities: RefCell::new(BTreeMap::new()),
        };
        Ok(deserializer.deserialize_with_resolver(scene.as_slice(), &resolver)?)
    }

    /// Copies validated container content, preserving logical records, pathless
    /// objects, aliases, and raw metadata. ZIP timestamps/order may change.
    pub fn copy_to<W: Write + Seek>(&mut self, output: W) -> Result<()> {
        let mut zip = zip::ZipWriter::new(output);
        let options =
            zip::write::SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        zip.start_file("manifest.jsonl", options)?;
        zip.write_all(self.manifest.as_bytes())?;
        for (digest, member) in &self.members {
            zip.start_file(
                format!("objects/{digest}"),
                options.large_file(member.size >= u32::MAX as u64),
            )?;
            self.input.seek(SeekFrom::Start(member.offset))?;
            if io::copy(&mut (&mut self.input).take(member.size), &mut zip)? != member.size {
                return Err(Error::Invalid("truncated object"));
            }
        }
        zip.finish()?.flush()?;
        Ok(())
    }
}

struct Resolver<'a, R> {
    reader: RefCell<&'a mut Reader<R>>,
    restored: Cell<u64>,
    identities: RefCell<BTreeMap<String, [u8; 16]>>,
}
impl<R: Read + Seek> EmbeddedContentResolver for Resolver<'_, R> {
    fn resolve(&self, reference: &EmbeddedReference) -> io::Result<SharedString> {
        let mut reader = self.reader.borrow_mut();
        let digest: String = reference
            .digest
            .iter()
            .map(|byte| format!("{byte:02x}"))
            .collect();
        if reader.manifest.scene_digest() == digest {
            return Err(invalid_io("property references the scene object"));
        }
        let member = reader
            .members
            .get(&digest)
            .ok_or_else(|| invalid_io("missing embedded object"))?;
        if member.size != reference.size {
            return Err(invalid_io("embedded size mismatch"));
        }
        let restored = self
            .restored
            .get()
            .checked_add(reference.size)
            .ok_or_else(|| invalid_io("restored bytes overflow"))?;
        if restored > reader.limits.max_restored_bytes {
            return Err(invalid_io("restored bytes limit exceeded"));
        }
        self.restored.set(restored);
        if let Some(identity) = &reference.identity {
            validate_identity(identity).map_err(invalid_io)?;
            let previous = self
                .identities
                .borrow_mut()
                .insert(identity.clone(), reference.digest);
            if previous.is_some_and(|digest| digest != reference.digest) {
                return Err(invalid_io("conflicting opaque identity"));
            }
        }
        let bytes = reader.read_object(&digest).map_err(invalid_io)?;
        if reference.identity.is_some() {
            ContentEnvelope::parse(&bytes, reader.limits.max_runtime_metadata_bytes)
                .map_err(invalid_io)?;
        }
        Ok(SharedString::new(bytes))
    }
}

// A data descriptor is the only permitted gap between STORE members;
// validating it excludes hidden local entries outside the directory index.
fn validate_member_end(
    input: &mut (impl Read + Seek),
    member: &PhysicalMember,
    next: u64,
) -> Result<()> {
    let gap = next
        .checked_sub(member.end)
        .ok_or(Error::Invalid("overlapping ZIP members"))?;
    if !member.descriptor {
        return if gap == 0 {
            Ok(())
        } else {
            Err(Error::Invalid("unindexed bytes between ZIP members"))
        };
    }
    if ![12, 16, 20, 24].contains(&gap) {
        return Err(Error::Invalid("invalid ZIP data descriptor length"));
    }
    input.seek(SeekFrom::Start(member.end))?;
    let mut buffer = [0; 24];
    input.read_exact(&mut buffer[..gap as usize])?;
    let mut bytes = &buffer[..gap as usize];
    if gap == 16 || gap == 24 {
        if !bytes.starts_with(b"PK\x07\x08") {
            return Err(Error::Invalid("invalid ZIP data descriptor signature"));
        }
        bytes = &bytes[4..];
    }
    let crc = u32::from_le_bytes(bytes[..4].try_into().unwrap());
    let (compressed, size) = if bytes.len() == 12 {
        (
            u32::from_le_bytes(bytes[4..8].try_into().unwrap()) as u64,
            u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as u64,
        )
    } else {
        (
            u64::from_le_bytes(bytes[4..12].try_into().unwrap()),
            u64::from_le_bytes(bytes[12..20].try_into().unwrap()),
        )
    };
    if crc != member.crc || size != member.size || compressed != member.size {
        return Err(Error::Invalid("ZIP data descriptor mismatch"));
    }
    Ok(())
}
