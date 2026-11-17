# rbx_archive

Reads and writes the version-1 JSONL embedded-place archive profile (`.rbx`).
Uses the existing `rbx_binary` codec and returns an `rbx_dom_weak::WeakDom` with a
DataModel root, just like `rbx_binary::from_reader`.

```rust,no_run
use std::{fs::File, io::{BufReader, BufWriter}};

let dom = rbx_archive::from_reader(BufReader::new(File::open("place.rbx")?))?;
rbx_archive::to_writer(
    BufWriter::new(File::create("saved.rbx")?),
    &dom,
    dom.root().children(),
)?;
# Ok::<(), Box<dyn std::error::Error>>(())
```

## Supported profile

The container is an unencrypted STORE ZIP with `manifest.jsonl` and objects named
`objects/<32 lowercase BLAKE3-128 hex digits>`. ZIP64 is accepted within configured
limits. The manifest declares a version-1 `header`, one `package` of type `place`,
and `content` records. The package's `place` is a logical path, resolved through a
content record. Objects and manifest may occur in any ZIP order. Logical aliases,
pathless content records, and arbitrary JSON-object metadata are supported.

The binary scene uses normal binary value encodings, plus embedded PROP metadata:

- Tag `0x82` introduces one metadata value per instance; signed little-endian i32
  lengths precede values, and negative lengths skip inline instances.
- Version 1 metadata contains a version byte, 16 digest bytes, and a little-endian
  u64 object size. It restores String, BinaryString, and SharedString values from
  empty placeholders (string or shared-string wire encoding).
- Version 2 appends a length-prefixed file-local opaque identity. A Content value
  has a None placeholder and resolves to `ContentType::Embedded`. Its shared
  bytes contain the complete runtime envelope, including payload and metadata.

`ContentEnvelope::from_bytes` validates and borrows the version-0 runtime envelope:
little-endian i32 version, i32 payload length, payload, i32 metadata length, metadata.
Metadata is absent or a JSON object with unique string-valued members. Payloads
may be binary. Empty payloads require `INTERNAL_TYPE` set to `Text`.
Opaque identities are file-local sharing tokens, not fetchable URIs or live engine
handles. Equal identities must reference equal objects; distinct identities may
share identical envelope bytes.

The writer leaves ordinary strings inline and stores opaque Content in deduplicated
objects. Binary and XML writers reject embedded Content instead of discarding it.
Archive support does not change which ordinary property types `rbx_binary` supports.
The format codec does not encode an engine-version-specific list of properties
eligible for externalization; it validates the wire type and placeholder instead.

Only embedded place packages are supported. Legacy package layouts, model packages,
remote manifest records, and other PROP metadata kinds are rejected. The library
performs no HTTP requests; ordinary asset URIs and inline AssetContentMap bundle
IDs keep their existing binary representation.

## Container access and memory

`Reader::new(input, limits)` validates the directory, manifest, CRCs, and object
digests. It hashes objects through a 64 KiB buffer and retains the directory and
manifest, rather than all object bodies. `read_object` loads an individual object.
`deserialize` buffers the scene once and builds a complete DOM, loading referenced
objects on demand. This is not a streaming instance visitor API. DOM properties,
scene decompression, and restored values consume additional memory.

Default limits are 512 MiB archive bytes, 512 MiB aggregate object bytes, 256 MiB
per object/scene, 16 MiB manifest bytes, 1 MiB per record and runtime metadata, and
65,534 entries. Aggregate restored bytes are separately capped at 512 MiB, counting
repeated references. These are configurable container/object budgets, not a bound
on total process memory or a guarantee that the underlying binary parser accepts
all malformed input without allocation or panic.

The writer retains the scene and shared references to unique opaque envelopes,
then writes the ZIP to a seekable output. Callers should use a temporary output
when replacing an existing file, since encode errors can leave partial output.

`Reader::copy_to` copies the container without a DOM round trip, preserving raw
manifest metadata, logical aliases, and unattached objects. A DOM read/edit/write
retains represented instance properties but does not retain unattached objects or
arbitrary manifest metadata.

See [`examples/read_archive.rs`](examples/read_archive.rs) for reading opaque
payloads. `rbx-util convert` and `remove-prop` accept `.rbx` paths. The Rojo CLI is a
separate consumer and needs its own format dispatch integration.
