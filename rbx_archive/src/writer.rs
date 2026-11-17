use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    io::{self, Seek, SeekFrom, Write},
};

use rbx_binary::EmbeddedContentSink;
use rbx_dom_weak::{
    types::{EmbeddedContent, Ref, SharedString},
    WeakDom,
};
use zip::{write::SimpleFileOptions, CompressionMethod, ZipWriter};

use crate::{
    envelope::validate_identity, invalid_io, ContentEnvelope, Error, Limits, Manifest, Result,
};

/// Configurable DOM-to-archive writer. The scene and unique opaque envelopes are
/// retained during encoding; ordinary string properties remain inline.
#[derive(Default)]
pub struct Writer {
    limits: Limits,
}
impl Writer {
    /// Creates a writer with explicit budgets.
    pub fn new(limits: Limits) -> Self {
        Self { limits }
    }

    /// Encodes selected instances and descendants as an embedded place archive.
    /// Errors leave a potentially incomplete output, as with the binary writer.
    pub fn serialize<W: Write + Seek>(&self, output: W, dom: &WeakDom, refs: &[Ref]) -> Result<()> {
        self.serialize_with(output, dom, refs, &rbx_binary::Serializer::new())
    }

    /// Encodes using an existing binary configuration, including its reflection
    /// database and chunk compression setting.
    pub fn serialize_with<W: Write + Seek>(
        &self,
        output: W,
        dom: &WeakDom,
        refs: &[Ref],
        serializer: &rbx_binary::Serializer<'_>,
    ) -> Result<()> {
        let sink = ObjectSink {
            limits: &self.limits,
            objects: RefCell::new(BTreeMap::new()),
            identities: RefCell::new(BTreeMap::new()),
            total: Cell::new(0),
        };
        let mut scene =
            BoundedOutput::new(io::Cursor::new(Vec::new()), self.limits.max_object_bytes);
        serializer.serialize_with_sink(&mut scene, dom, refs, &sink)?;
        let scene = scene.inner.into_inner();
        let scene_digest = crate::digest(&scene);
        let mut objects = sink.objects.into_inner();
        if objects.contains_key(&scene_digest) {
            return Err(Error::Invalid("opaque object equals scene object"));
        }
        objects.insert(scene_digest.clone(), SharedString::new(scene));
        let total: u64 = objects
            .values()
            .map(|bytes| bytes.data().len() as u64)
            .sum();
        if total > self.limits.max_total_object_bytes {
            return Err(Error::Limit("aggregate object bytes"));
        }
        if objects.len() >= self.limits.max_entries {
            return Err(Error::Limit("ZIP entries"));
        }
        let mut manifest = Vec::new();
        for record in [
            serde_json::json!({"kind":"header", "version":1, "digestAlgorithm":"blake3-128"}),
            serde_json::json!({"kind":"package", "type":"place", "place":"Place.rbxl"}),
            serde_json::json!({"kind":"content", "path":"Place.rbxl", "objectDigest":scene_digest}),
        ] {
            serde_json::to_writer(&mut manifest, &record)?;
            manifest.push(b'\n');
        }
        for digest in objects.keys().filter(|digest| **digest != scene_digest) {
            serde_json::to_writer(
                &mut manifest,
                &serde_json::json!({"kind":"content", "objectDigest":digest}),
            )?;
            manifest.push(b'\n');
        }
        let manifest = Manifest::parse(manifest, &self.limits)?;
        let options = SimpleFileOptions::default().compression_method(CompressionMethod::Stored);
        let mut zip = ZipWriter::new(BoundedOutput::new(output, self.limits.max_archive_bytes));
        zip.start_file("manifest.jsonl", options)?;
        zip.write_all(manifest.as_bytes())?;
        for (digest, bytes) in objects {
            zip.start_file(
                format!("objects/{digest}"),
                options.large_file(bytes.data().len() as u64 >= u32::MAX as u64),
            )?;
            zip.write_all(bytes.data())?;
        }
        zip.finish()?.flush()?;
        Ok(())
    }
}

struct ObjectSink<'a> {
    limits: &'a Limits,
    objects: RefCell<BTreeMap<String, SharedString>>,
    identities: RefCell<BTreeMap<String, String>>,
    total: Cell<u64>,
}
impl EmbeddedContentSink for ObjectSink<'_> {
    fn store(&self, content: &EmbeddedContent) -> io::Result<()> {
        validate_identity(&content.identity).map_err(invalid_io)?;
        ContentEnvelope::parse(content.data.data(), self.limits.max_runtime_metadata_bytes)
            .map_err(invalid_io)?;
        if content.data.data().len() as u64 > self.limits.max_object_bytes {
            return Err(invalid_io("object byte limit exceeded"));
        }
        let digest = crate::digest(content.data.data());
        if self
            .identities
            .borrow_mut()
            .insert(content.identity.clone(), digest.clone())
            .is_some_and(|previous| previous != digest)
        {
            return Err(invalid_io("conflicting opaque identity"));
        }
        let mut objects = self.objects.borrow_mut();
        if !objects.contains_key(&digest) {
            if objects.len() >= self.limits.max_entries {
                return Err(invalid_io("object count limit exceeded"));
            }
            let total = self
                .total
                .get()
                .checked_add(content.data.data().len() as u64);
            if total.is_none_or(|size| size > self.limits.max_total_object_bytes) {
                return Err(invalid_io("aggregate object byte limit exceeded"));
            }
            self.total.set(total.unwrap());
            objects.insert(digest, content.data.clone());
        }
        Ok(())
    }
}

struct BoundedOutput<W> {
    inner: W,
    position: u64,
    limit: u64,
}
impl<W> BoundedOutput<W> {
    fn new(inner: W, limit: u64) -> Self {
        Self {
            inner,
            position: 0,
            limit,
        }
    }
}
impl<W: Write> Write for BoundedOutput<W> {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        if bytes.len() as u64 > self.limit.saturating_sub(self.position) {
            return Err(invalid_io("output byte limit exceeded"));
        }
        let written = self.inner.write(bytes)?;
        self.position += written as u64;
        Ok(written)
    }
    fn flush(&mut self) -> io::Result<()> {
        self.inner.flush()
    }
}
impl<W: Seek> Seek for BoundedOutput<W> {
    fn seek(&mut self, offset: SeekFrom) -> io::Result<u64> {
        let position = self.inner.seek(offset)?;
        if position > self.limit {
            return Err(invalid_io("output seek exceeds byte limit"));
        }
        self.position = position;
        Ok(position)
    }
}
