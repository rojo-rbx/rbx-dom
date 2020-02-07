use std::io::{Read, Write};

use rbx_reflection::PropertyDescriptor;

use crate::{
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::XmlEventWriter,
};

pub trait XmlType: Sized {
    const XML_TAG_NAME: &'static str;

    fn write_xml<W: Write>(
        &self,
        writer: &mut XmlEventWriter<W>,
        name: &str,
    ) -> Result<(), EncodeError>;

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError>;
}

pub fn find_canonical_property_descriptor(
    class_name: &str,
    property_name: &str,
) -> Option<&'static PropertyDescriptor<'static>> {
    find_property_descriptors(class_name, property_name).map(|(canonical, _serialized)| canonical)
}

pub fn find_serialized_property_descriptor(
    class_name: &str,
    property_name: &str,
) -> Option<&'static PropertyDescriptor<'static>> {
    find_property_descriptors(class_name, property_name).map(|(_canonical, serialized)| serialized)
}

/// Find both the canonical and serialized property descriptors for a given
/// class and property name pair. These might be the same descriptor!
fn find_property_descriptors(
    class_name: &str,
    property_name: &str,
) -> Option<(
    &'static PropertyDescriptor<'static>,
    &'static PropertyDescriptor<'static>,
)> {
    let class_descriptor = rbx_reflection_database::get().classes.get(class_name)?;

    let mut current_class_descriptor = class_descriptor;

    // We need to find the canonical property descriptor associated with
    // the property we're trying to deserialize.
    //
    // At each step of the loop, we're checking a new class descriptor
    // to see if it has an entry for the property name we're looking for.
    loop {
        // If this class descriptor knows about this property name,
        // we're pretty much done!
        if let Some(property_descriptor) = current_class_descriptor.properties.get(property_name) {
            if property_descriptor.alias_for.is_none() {
                // The property name in the XML was the canonical name
                // and also the serialized name, hooray!

                let serialized_descriptor = property_descriptor
                    .serializes_as
                    .as_ref()
                    .map(|name| current_class_descriptor.properties.get(name).unwrap())
                    .unwrap_or(property_descriptor);

                return Some((property_descriptor, serialized_descriptor));
            }

            if let Some(canonical_name) = &property_descriptor.alias_for {
                // This property has a canonical form that we'll map
                // from the XML name.

                let canonical_descriptor = current_class_descriptor
                    .properties
                    .get(canonical_name)
                    .unwrap();

                let serialized_descriptor = canonical_descriptor
                    .serializes_as
                    .as_ref()
                    .map(|name| current_class_descriptor.properties.get(name).unwrap())
                    .unwrap_or(canonical_descriptor);

                return Some((canonical_descriptor, serialized_descriptor));
            } else {
                // This property doesn't have a canonical form, we we'll
                // skip serializing it by declaring there isn't a
                // canonical property descriptor for it.

                return None;
            }
        }

        if let Some(superclass_name) = &current_class_descriptor.superclass {
            // If a property descriptor isn't found in our class, check
            // our superclass.

            rbx_reflection_database::get()
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
