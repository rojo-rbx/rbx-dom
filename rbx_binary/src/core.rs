use std::io::{self, Read, Write};

use rbx_reflection::{
    ClassDescriptor, PropertyDescriptor, PropertyKind, PropertySerialization, ReflectionDatabase,
};

use crate::chunk::ChunkBuilder;

pub static FILE_MAGIC_HEADER: &[u8] = b"<roblox!";
pub static FILE_SIGNATURE: &[u8] = b"\x89\xff\x0d\x0a\x1a\x0a";
pub const FILE_VERSION: u16 = 0;

pub struct ReadInterleavedBytesIter<'a, const N: usize> {
    bytes: &'a [u8],
    index: usize,
    len: usize,
}

impl<'a, const N: usize> ReadInterleavedBytesIter<'a, N> {
    fn new(bytes: &'a [u8], len: usize) -> Self {
        let index = 0;
        Self { bytes, index, len }
    }
}

impl<'a, const N: usize> Iterator for ReadInterleavedBytesIter<'a, N> {
    type Item = [u8; N];
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            let output = core::array::from_fn(|i| self.bytes[self.index + self.len * i]);
            self.index += 1;
            Some(output)
        } else {
            None
        }
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

pub trait RbxReadExt: Read {
    fn read_le_u32(&mut self) -> io::Result<u32> {
        let mut buffer = [0; 4];
        self.read_exact(&mut buffer)?;

        Ok(u32::from_le_bytes(buffer))
    }

    fn read_le_u16(&mut self) -> io::Result<u16> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;

        Ok(u16::from_le_bytes(bytes))
    }

    fn read_le_i16(&mut self) -> io::Result<i16> {
        let mut bytes = [0; 2];
        self.read_exact(&mut bytes)?;

        Ok(i16::from_le_bytes(bytes))
    }

    fn read_le_f32(&mut self) -> io::Result<f32> {
        let mut buffer = [0u8; 4];
        self.read_exact(&mut buffer)?;

        Ok(f32::from_le_bytes(buffer))
    }

    fn read_le_f64(&mut self) -> io::Result<f64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;

        Ok(f64::from_le_bytes(bytes))
    }

    fn read_be_u32(&mut self) -> io::Result<u32> {
        let mut bytes = [0; 4];
        self.read_exact(&mut bytes)?;

        Ok(u32::from_be_bytes(bytes))
    }

    fn read_be_i64(&mut self) -> io::Result<i64> {
        let mut bytes = [0; 8];
        self.read_exact(&mut bytes)?;

        Ok(i64::from_be_bytes(bytes))
    }

    fn read_u8(&mut self) -> io::Result<u8> {
        let mut buffer = [0u8];
        self.read_exact(&mut buffer)?;

        Ok(buffer[0])
    }

    /// Read a binary "string" in the format that Roblox's model files use.
    ///
    /// This function is safer than read_string because Roblox generally makes
    /// no guarantees about encoding of things it calls strings. rbx_binary
    /// makes a semantic differentiation between strings and binary buffers,
    /// which makes it more strict than Roblox but more likely to be correct.
    fn read_binary_string<'a>(&mut self) -> io::Result<&'a [u8]>
    where
        Self: ReadSlice<'a>,
    {
        let length = self.read_le_u32()?;
        let out = self.read_slice(length as usize)?;
        Ok(out)
    }

    /// Read a UTF-8 encoded string encoded how Roblox model files encode
    /// strings. This function isn't always appropriate because Roblox's formats
    /// generally aren't dilligent about data being valid Unicode.
    fn read_string<'a>(&mut self) -> io::Result<&'a str>
    where
        Self: ReadSlice<'a>,
    {
        let out = self.read_binary_string()?;

        core::str::from_utf8(out).map_err(|_| {
            io::Error::new(
                io::ErrorKind::InvalidData,
                "stream did not contain valid UTF-8",
            )
        })
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_u8()? != 0)
    }
}

pub trait RbxReadInterleaved<'a>: ReadSlice<'a> {
    /// Create an iterator that reads chunks of N interleaved bytes.
    /// Consumes `N * len` bytes from Self.
    fn read_interleaved_bytes<const N: usize>(
        &mut self,
        len: usize,
    ) -> io::Result<ReadInterleavedBytesIter<'a, N>> {
        let out = self.read_slice(len * N)?;

        Ok(ReadInterleavedBytesIter::new(out, len))
    }

    /// Creates an iterator of `len` big-endian i32 values.
    /// The values are transformed during iteration.
    fn read_interleaved_i32_array(
        &mut self,
        len: usize,
    ) -> io::Result<impl Iterator<Item = i32> + 'a> {
        Ok(self
            .read_interleaved_bytes(len)?
            .map(|out| untransform_i32(i32::from_be_bytes(out))))
    }

    /// Creates an iterator of `len` big-endian u32 values.
    /// The values are transformed during iteration.
    fn read_interleaved_u32_array(
        &mut self,
        len: usize,
    ) -> io::Result<impl Iterator<Item = u32> + 'a> {
        Ok(self.read_interleaved_bytes(len)?.map(u32::from_be_bytes))
    }

    /// Creates an iterator of `len` big-endian f32 values.
    /// The values are properly unrotated during iteration.
    fn read_interleaved_f32_array(
        &mut self,
        len: usize,
    ) -> io::Result<impl Iterator<Item = f32> + 'a> {
        Ok(self
            .read_interleaved_bytes(len)?
            .map(|out| f32::from_bits(u32::from_be_bytes(out).rotate_right(1))))
    }

    /// Creates an iterator of `len` big-endian i32 values.
    /// The values are properly untransformed and accumulated
    /// so as to properly read arrays of referent values.
    fn read_referent_array(&mut self, len: usize) -> io::Result<impl Iterator<Item = i32> + 'a> {
        let mut last = 0;
        Ok(self
            .read_interleaved_i32_array(len)?
            .map(move |mut referent| {
                referent += last;
                last = referent;
                referent
            }))
    }

    /// Creates an iterator of `len` big-endian i64 values.
    /// The values are transformed during iteration.
    fn read_interleaved_i64_array(
        &mut self,
        len: usize,
    ) -> io::Result<impl Iterator<Item = i64> + 'a> {
        Ok(self
            .read_interleaved_bytes(len)?
            .map(|out| untransform_i64(i64::from_be_bytes(out))))
    }
}

impl<R> RbxReadExt for R where R: Read {}
impl<'a, R> RbxReadInterleaved<'a> for R where R: ReadSlice<'a> {}

pub trait ReadSlice<'a> {
    /// Read a slice of length `len`, or return
    /// an error if the length overruns the source data.
    fn read_slice(&mut self, len: usize) -> io::Result<&'a [u8]>;
}

#[cold]
fn unexpected_eof() -> io::Error {
    io::Error::new(io::ErrorKind::UnexpectedEof, "failed to fill whole buffer")
}

impl<'a> ReadSlice<'a> for &'a [u8] {
    fn read_slice(&mut self, len: usize) -> io::Result<&'a [u8]> {
        let out;

        (out, *self) = self.split_at_checked(len).ok_or_else(unexpected_eof)?;

        Ok(out)
    }
}

pub trait RbxWriteExt: Write {
    fn write_le_u32(&mut self, value: u32) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_le_u16(&mut self, value: u16) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_le_i16(&mut self, value: i16) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_le_f32(&mut self, value: f32) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_le_f64(&mut self, value: f64) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;

        Ok(())
    }

    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.write_all(&[value])?;

        Ok(())
    }

    fn write_binary_string(&mut self, value: &[u8]) -> io::Result<()> {
        self.write_le_u32(value.len() as u32)?;
        self.write_all(value)?;

        Ok(())
    }

    fn write_string(&mut self, value: &str) -> io::Result<()> {
        self.write_binary_string(value.as_bytes())
    }

    fn write_bool(&mut self, value: bool) -> io::Result<()> {
        self.write_u8(value as u8)
    }
}

impl ChunkBuilder {
    /// Takes `values` and writes it as a blob of data with each value
    /// interleaved by `N` bytes.
    pub fn write_interleaved_bytes<const N: usize, I>(&mut self, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = [u8; N]>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        let values = values.into_iter();
        let values_len = values.len();
        let bytes_len = values_len * N;

        let initialize_bytes = |buffer: &mut [u8]| {
            for (i, bytes) in values.enumerate() {
                for (b, byte) in IntoIterator::into_iter(bytes).enumerate() {
                    buffer[i + b * values_len] = byte;
                }
            }
        };

        self.initialize_bytes_with(bytes_len, initialize_bytes);

        Ok(())
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes. Transformation is applied to the values as they're written.
    pub fn write_interleaved_i32_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = i32>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        self.write_interleaved_bytes(values.into_iter().map(|v| transform_i32(v).to_be_bytes()))
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes.
    pub fn write_interleaved_u32_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = u32>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        self.write_interleaved_bytes(values.into_iter().map(|v| v.to_be_bytes()))
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes. Rotation is applied to the values as they're written.
    pub fn write_interleaved_f32_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = f32>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        self.write_interleaved_bytes(
            values
                .into_iter()
                .map(|v| v.to_bits().rotate_left(1).to_be_bytes()),
        )
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes. The appropriate transformation and de-accumulation is done as
    /// values are written.
    pub fn write_referent_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = i32>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        let mut last_value = 0;
        let delta_encoded = values.into_iter().map(|value| {
            let encoded = value - last_value;
            last_value = value;
            encoded
        });

        self.write_interleaved_i32_array(delta_encoded)
    }

    /// Writes all items from `values` into the buffer as a blob of interleaved
    /// bytes. Transformation is applied to the values as they're written.
    pub fn write_interleaved_i64_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: IntoIterator<Item = i64>,
        <I as IntoIterator>::IntoIter: ExactSizeIterator,
    {
        self.write_interleaved_bytes(values.into_iter().map(|v| transform_i64(v).to_be_bytes()))
    }
}

impl<W> RbxWriteExt for W where W: Write {}

/// Applies the 'zigzag' transformation done by Roblox to many `i32` values.
pub fn transform_i32(value: i32) -> i32 {
    (value << 1) ^ (value >> 31)
}

/// Inverses the 'zigzag' encoding transformation done by Roblox to many
/// `i32` values.
pub fn untransform_i32(value: i32) -> i32 {
    ((value as u32) >> 1) as i32 ^ -(value & 1)
}

/// Applies the 'zigzag' transformation done by Roblox to many `i64` values.
pub fn transform_i64(value: i64) -> i64 {
    (value << 1) ^ (value >> 63)
}

/// Inverses the 'zigzag' encoding transformation done by Roblox to many
/// `i64` values.
pub fn untransform_i64(value: i64) -> i64 {
    ((value as u64) >> 1) as i64 ^ -(value & 1)
}

pub struct PropertyDescriptors<'db> {
    pub canonical: &'db PropertyDescriptor<'db>,
    pub serialized: Option<&'db PropertyDescriptor<'db>>,
}

impl<'db> PropertyDescriptors<'db> {
    /// Get both the canonical and serialized property descriptors for a given
    /// class and property descriptor. The canonical and serialized descriptors
    /// might be the same descriptor!
    pub fn new(
        class_descriptor: &'db ClassDescriptor<'db>,
        property_descriptor: &'db PropertyDescriptor<'db>,
    ) -> Option<PropertyDescriptors<'db>> {
        match &property_descriptor.kind {
            // This property descriptor is the canonical form of this
            // logical property.
            PropertyKind::Canonical { serialization } => {
                let serialized = find_serialized_from_canonical(
                    class_descriptor,
                    property_descriptor,
                    serialization,
                );

                Some(PropertyDescriptors {
                    canonical: property_descriptor,
                    serialized,
                })
            }

            // This descriptor is an alias for another property. While this
            // descriptor might be one of the two descriptors we need to
            // return, it's possible that both the canonical and serialized
            // forms are different.
            PropertyKind::Alias { alias_for } => {
                let canonical = class_descriptor.properties.get(*alias_for).unwrap();

                if let PropertyKind::Canonical { serialization } = &canonical.kind {
                    let serialized =
                        find_serialized_from_canonical(class_descriptor, canonical, serialization);

                    Some(PropertyDescriptors {
                        canonical,
                        serialized,
                    })
                } else {
                    // If one property in the database calls itself an alias
                    // of another property, that property must be canonical.
                    log::error!(
                        "Property {}.{} is marked as an alias for {}.{}, but the latter is not canonical.",
                        class_descriptor.name,
                        property_descriptor.name,
                        class_descriptor.name,
                        alias_for
                    );

                    None
                }
            }

            // This descriptor is of an unknown kind and we don't know how
            // to deal with it -- maybe rbx_binary is out of date?
            _ => None,
        }
    }
}

/// Find the superclass which contains the specified property,
/// extract the canonical and serialized property descriptors,
/// and return both.
pub fn find_property_descriptors<'db>(
    database: &'db ReflectionDatabase<'db>,
    class_descriptor: Option<&'db ClassDescriptor<'db>>,
    property_name: &str,
) -> Option<(&'db ClassDescriptor<'db>, PropertyDescriptors<'db>)> {
    // Checking the class descriptor is ugly without an optional
    // return value, and all the call sites need this precise logic.
    let class_descriptor = class_descriptor?;

    // We need to find the canonical property descriptor associated with
    // the property we're working with. Walk superclasses and
    // find a class descriptor which knows about this property name.
    let (class, prop) = database
        .superclasses_iter(class_descriptor)
        .find_map(|class| {
            let prop = class.properties.get(property_name)?;
            Some((class, prop))
        })?;

    // Extract the canonical and serialized property descriptors
    // from the class and property descriptors
    let descriptors = PropertyDescriptors::new(class, prop)?;
    Some((class, descriptors))
}

/// Given the canonical property descriptor for a logical property along with
/// its serialization, returns the serialized form of the logical property if
/// this property is serializable.
fn find_serialized_from_canonical<'db>(
    class: &'db ClassDescriptor<'db>,
    canonical: &'db PropertyDescriptor<'db>,
    serialization: &'db PropertySerialization<'db>,
) -> Option<&'db PropertyDescriptor<'db>> {
    match serialization {
        // This property serializes as-is. This is the happiest path: both the
        // canonical and serialized descriptors are the same!
        PropertySerialization::Serializes | PropertySerialization::Migrate { .. } => {
            Some(canonical)
        }

        // This property serializes under an alias. That property should have a
        // corresponding property descriptor within the same class descriptor.
        PropertySerialization::SerializesAs(serialized_name) => {
            let serialized_descriptor = class.properties.get(*serialized_name).unwrap();

            Some(serialized_descriptor)
        }

        // If this property does not serialize, there is no serialized
        // descriptor to use.
        PropertySerialization::DoesNotSerialize => None,

        // This case will be hit if a new form of property serialization is
        // introduced to the reflection database. This might happen if the
        // database starts including more complex aliasing rules, like for the
        // Grip properties of Tool, or the Rotation and Orientation properties
        // of BasePart.
        _ => None,
    }
}
