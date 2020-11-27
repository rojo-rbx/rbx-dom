use std::{
    io::{self, Read, Write},
    mem,
};

use rbx_reflection::{ClassDescriptor, PropertyDescriptor, PropertyKind, PropertySerialization};

pub static FILE_MAGIC_HEADER: &[u8] = b"<roblox!";
pub static FILE_SIGNATURE: &[u8] = b"\x89\xff\x0d\x0a\x1a\x0a";
pub const FILE_VERSION: u16 = 0;

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
    fn read_binary_string(&mut self) -> io::Result<Vec<u8>> {
        let length = self.read_le_u32()?;

        let mut value = Vec::with_capacity(length as usize);
        self.take(length as u64).read_to_end(&mut value)?;

        Ok(value)
    }

    /// Read a UTF-8 encoded string encoded how Roblox model files encode
    /// strings. This function isn't always appropriate because Roblox's formats
    /// generally aren't dilligent about data being valid Unicode.
    fn read_string(&mut self) -> io::Result<String> {
        let length = self.read_le_u32()?;
        let mut value = String::with_capacity(length as usize);
        self.take(length as u64).read_to_string(&mut value)?;

        Ok(value)
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(self.read_u8()? != 0)
    }

    fn read_interleaved_i32_array(&mut self, output: &mut [i32]) -> io::Result<()> {
        let mut buffer = vec![0; output.len() * mem::size_of::<i32>()];
        self.read_exact(&mut buffer)?;

        for i in 0..output.len() {
            let v0 = buffer[i] as i32;
            let v1 = buffer[i + output.len()] as i32;
            let v2 = buffer[i + output.len() * 2] as i32;
            let v3 = buffer[i + output.len() * 3] as i32;

            output[i] = untransform_i32((v0 << 24) | (v1 << 16) | (v2 << 8) | v3);
        }

        Ok(())
    }

    fn read_interleaved_u32_array(&mut self, output: &mut [u32]) -> io::Result<()> {
        let mut buffer = vec![0; output.len() * mem::size_of::<u32>()];
        self.read_exact(&mut buffer)?;

        for i in 0..output.len() {
            let bytes = [
                buffer[i],
                buffer[i + output.len()],
                buffer[i + output.len() * 2],
                buffer[i + output.len() * 3],
            ];

            output[i] = u32::from_be_bytes(bytes);
        }

        Ok(())
    }

    fn read_interleaved_f32_array(&mut self, output: &mut [f32]) -> io::Result<()> {
        let mut buf = vec![0; output.len() * mem::size_of::<f32>()];
        self.read_exact(&mut buf)?;

        for i in 0..output.len() {
            let v0 = buf[i] as u32;
            let v1 = buf[i + output.len()] as u32;
            let v2 = buf[i + output.len() * 2] as u32;
            let v3 = buf[i + output.len() * 3] as u32;

            output[i] = f32::from_bits(((v0 << 24) | (v1 << 16) | (v2 << 8) | v3).rotate_right(1));
        }
        Ok(())
    }

    fn read_referent_array(&mut self, output: &mut [i32]) -> io::Result<()> {
        self.read_interleaved_i32_array(output)?;

        let mut last = 0;

        for referent in output.iter_mut() {
            *referent += last;
            last = *referent;
        }

        Ok(())
    }

    fn read_interleaved_i64_array(&mut self, output: &mut [i64]) -> io::Result<()> {
        let mut buf = vec![0; output.len() * mem::size_of::<i64>()];
        self.read_exact(&mut buf)?;

        for i in 0..output.len() {
            let z0 = buf[i] as i64;
            let z1 = buf[i + output.len()] as i64;
            let z2 = buf[i + output.len() * 2] as i64;
            let z3 = buf[i + output.len() * 3] as i64;
            let z4 = buf[i + output.len() * 4] as i64;
            let z5 = buf[i + output.len() * 5] as i64;
            let z6 = buf[i + output.len() * 6] as i64;
            let z7 = buf[i + output.len() * 7] as i64;

            output[i] = untransform_i64(
                (z0 << 56)
                    | (z1 << 48)
                    | (z2 << 40)
                    | (z3 << 32)
                    | (z4 << 24)
                    | (z5 << 16)
                    | (z6 << 8)
                    | z7,
            );
        }

        Ok(())
    }
}

impl<R> RbxReadExt for R where R: Read {}

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

    fn write_interleaved_i32_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: Iterator<Item = i32>,
    {
        let values: Vec<_> = values.collect();

        for shift in &[24, 16, 8, 0] {
            for value in values.iter().copied() {
                let encoded = transform_i32(value) >> shift;
                self.write_u8(encoded as u8)?;
            }
        }

        Ok(())
    }

    fn write_interleaved_u32_array(&mut self, values: &[u32]) -> io::Result<()> {
        for shift in &[24, 16, 8, 0] {
            for value in values.iter().copied() {
                let encoded = value >> shift;
                self.write_u8(encoded as u8)?;
            }
        }

        Ok(())
    }

    fn write_interleaved_f32_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: Iterator<Item = f32>,
    {
        let values: Vec<_> = values.collect();

        for shift in &[24, 16, 8, 0] {
            for value in values.iter().copied() {
                let encoded = value.to_bits().rotate_left(1) >> shift;
                self.write_u8(encoded as u8)?;
            }
        }

        Ok(())
    }

    fn write_referent_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: Iterator<Item = i32>,
    {
        let mut last_value = 0;
        let delta_encoded = values.map(|value| {
            let encoded = value - last_value;
            last_value = value;
            encoded
        });

        self.write_interleaved_i32_array(delta_encoded)
    }

    fn write_interleaved_i64_array<I>(&mut self, values: I) -> io::Result<()>
    where
        I: Iterator<Item = i64>,
    {
        let values: Vec<_> = values.collect();

        for shift in &[56, 48, 40, 32, 24, 16, 8, 0] {
            for value in values.iter().copied() {
                let encoded = transform_i64(value) >> shift;
                self.write_u8(encoded as u8)?;
            }
        }

        Ok(())
    }
}

impl<W> RbxWriteExt for W where W: Write {}

/// Applies the integer transformation generally used in property data in the
/// Roblox binary format.
pub fn transform_i32(value: i32) -> i32 {
    (value << 1) ^ (value >> 31)
}

/// The inverse of `transform_i32`.
pub fn untransform_i32(value: i32) -> i32 {
    ((value as u32) >> 1) as i32 ^ -(value & 1)
}

pub fn transform_i64(value: i64) -> i64 {
    (value << 1) ^ (value >> 63)
}

pub fn untransform_i64(value: i64) -> i64 {
    ((value as u64) >> 1) as i64 ^ -(value & 1)
}

pub fn find_canonical_property_descriptor(
    class_name: &str,
    property_name: &str,
) -> Option<&'static PropertyDescriptor<'static>> {
    find_property_descriptors(class_name, property_name).map(|descriptors| descriptors.canonical)
}

pub struct PropertyDescriptors {
    pub canonical: &'static PropertyDescriptor<'static>,
    pub serialized: Option<&'static PropertyDescriptor<'static>>,
}

/// Find both the canonical and serialized property descriptors for a given
/// class and property name pair. These might be the same descriptor!
pub fn find_property_descriptors(
    class_name: &str,
    property_name: &str,
) -> Option<PropertyDescriptors> {
    let mut class_descriptor = rbx_reflection_database::get().classes.get(class_name)?;

    // We need to find the canonical property descriptor associated with
    // the property we're working with.
    //
    // At each step of the loop, we're checking a new class descriptor to see if
    // it has an entry for the property name we're looking for. If that class
    // doesn't have the property, we'll check its superclass until we reach the
    // root.
    loop {
        // If this class descriptor knows about this property name, we're pretty
        // much done!
        if let Some(property_descriptor) = class_descriptor.properties.get(property_name) {
            match &property_descriptor.kind {
                // This property descriptor is the canonical form of this
                // logical property. That means we've found one of the two
                // descriptors we're looking for!
                PropertyKind::Canonical { serialization } => {
                    let serialized = find_serialized_from_canonical(
                        class_descriptor,
                        property_descriptor,
                        serialization,
                    );

                    return Some(PropertyDescriptors {
                        canonical: property_descriptor,
                        serialized,
                    });
                }

                // This descriptor is an alias for another property. While this
                // descriptor might be one of the two descriptors we need to
                // return, it's possible that both the canonical and serialized
                // forms are different.
                PropertyKind::Alias { alias_for } => {
                    let canonical = class_descriptor.properties.get(alias_for.as_ref()).unwrap();

                    if let PropertyKind::Canonical { serialization } = &canonical.kind {
                        let serialized = find_serialized_from_canonical(
                            class_descriptor,
                            canonical,
                            serialization,
                        );

                        return Some(PropertyDescriptors {
                            canonical,
                            serialized,
                        });
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

                        return None;
                    }
                }

                // This descriptor is of an unknown kind and we don't know how
                // to deal with it -- maybe rbx_binary is out of date?
                _ => return None,
            }
        }

        if let Some(superclass_name) = &class_descriptor.superclass {
            // If a property descriptor isn't found in our class, check our
            // superclass.

            class_descriptor = rbx_reflection_database::get()
                .classes
                .get(superclass_name)
                .expect("Superclass in reflection database didn't exist");
        } else {
            // This property isn't known by any class in the reflection
            // database.

            return None;
        }
    }
}

/// Given the canonical property descriptor for a logical property along with
/// its serialization, returns the serialized form of the logical property if
/// this property is serializable.
fn find_serialized_from_canonical(
    class: &'static ClassDescriptor<'static>,
    canonical: &'static PropertyDescriptor<'static>,
    serialization: &'static PropertySerialization<'static>,
) -> Option<&'static PropertyDescriptor<'static>> {
    match serialization {
        // This property serializes as-is. This is the happiest path: both the
        // canonical and serialized descriptors are the same!
        PropertySerialization::Serializes => Some(canonical),

        // This property serializes under an alias. That property should have a
        // corresponding property descriptor within the same class descriptor.
        PropertySerialization::SerializesAs(serialized_name) => {
            let serialized_descriptor = class.properties.get(serialized_name.as_ref()).unwrap();

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
