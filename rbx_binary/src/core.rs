use std::{
    io::{self, Read, Write},
    mem,
};

use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use rbx_reflection::RbxPropertyDescriptor;

pub static FILE_MAGIC_HEADER: &[u8] = b"<roblox!";
pub static FILE_SIGNATURE: &[u8] = b"\x89\xff\x0d\x0a\x1a\x0a";
pub const FILE_VERSION: u16 = 0;

pub trait RbxReadExt: Read {
    fn read_string(&mut self) -> io::Result<String> {
        let length = self.read_u32::<LittleEndian>()?;

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

    fn read_referent_array(&mut self, output: &mut [i32]) -> io::Result<()> {
        self.read_interleaved_i32_array(output)?;

        let mut last = 0;

        for i in 0..output.len() {
            output[i] += last;
            last = output[i];
        }

        Ok(())
    }
}

impl<R> RbxReadExt for R where R: Read {}

pub trait RbxWriteExt: Write {
    fn write_string(&mut self, value: &str) -> io::Result<()> {
        self.write_u32::<LittleEndian>(value.len() as u32)?;
        write!(self, "{}", value)?;

        Ok(())
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

    fn write_referents<I>(&mut self, values: I) -> io::Result<()>
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

pub fn find_canonical_property_descriptor(
    class_name: &str,
    property_name: &str,
) -> Option<&'static RbxPropertyDescriptor> {
    find_property_descriptors(class_name, property_name).map(|(canonical, _serialized)| canonical)
}

pub fn find_serialized_property_descriptor(
    class_name: &str,
    property_name: &str,
) -> Option<&'static RbxPropertyDescriptor> {
    find_property_descriptors(class_name, property_name).map(|(_canonical, serialized)| serialized)
}

/// Find both the canonical and serialized property descriptors for a given
/// class and property name pair. These might be the same descriptor!
fn find_property_descriptors(
    class_name: &str,
    property_name: &str,
) -> Option<(
    &'static RbxPropertyDescriptor,
    &'static RbxPropertyDescriptor,
)> {
    let class_descriptor = rbx_reflection::get_class_descriptor(class_name)?;

    let mut current_class_descriptor = class_descriptor;

    // We need to find the canonical property descriptor associated with the
    // property we're trying to deserialize.
    //
    // At each step of the loop, we're checking a new class descriptor to see if
    // it has an entry for the property name we're looking for.
    loop {
        // If this class descriptor knows about this property name, we're pretty
        // much done!
        if let Some(property_descriptor) =
            current_class_descriptor.get_property_descriptor(property_name)
        {
            if property_descriptor.is_canonical() {
                // The property name in the file was the canonical name and also
                // the serialized name, hooray!

                let serialized_descriptor = property_descriptor
                    .serialized_name()
                    .map(|name| {
                        current_class_descriptor
                            .get_property_descriptor(name)
                            .unwrap()
                    })
                    .unwrap_or(property_descriptor);

                return Some((property_descriptor, serialized_descriptor));
            }

            if let Some(canonical_name) = property_descriptor.canonical_name() {
                // This property has a canonical form that we'll map from the
                // property name in the file.

                let canonical_descriptor = current_class_descriptor
                    .get_property_descriptor(canonical_name)
                    .unwrap();

                let serialized_descriptor = canonical_descriptor
                    .serialized_name()
                    .map(|name| {
                        current_class_descriptor
                            .get_property_descriptor(name)
                            .unwrap()
                    })
                    .unwrap_or(canonical_descriptor);

                return Some((canonical_descriptor, serialized_descriptor));
            } else {
                // This property doesn't have a canonical form, we we'll skip
                // serializing it by declaring there isn't a canonical property
                // descriptor for it.

                return None;
            }
        }

        if let Some(superclass_name) = current_class_descriptor.superclass() {
            // If a property descriptor isn't found in our class, check our
            // superclass.

            current_class_descriptor = rbx_reflection::get_class_descriptor(superclass_name)
                .expect("Superclass in rbx_reflection didn't exist");
        } else {
            // This property isn't known by any class in the reflection
            // database.

            return None;
        }
    }
}
