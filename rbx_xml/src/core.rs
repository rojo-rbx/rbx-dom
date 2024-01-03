use std::io::{Read, Write};

use rbx_reflection::{PropertyDescriptor, PropertyKind, PropertySerialization, ReflectionDatabase};

use crate::{
    deserializer_core::XmlEventReader,
    error::{DecodeError, EncodeError},
    serializer_core::{XmlEventWriter, XmlWriteEvent},
};

/// Trait that defines how to read and write a given type into the XML model
/// format.
///
/// This trait is based on the assumption that any given type has only one
/// representation in the format. For cases where that isn't the case, newtype
/// wrappers are the expected solution.
pub trait XmlType: Sized {
    const XML_TAG_NAME: &'static str;

    fn read_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError>;
    fn write_xml<W: Write>(&self, writer: &mut XmlEventWriter<W>) -> Result<(), EncodeError>;

    fn read_outer_xml<R: Read>(reader: &mut XmlEventReader<R>) -> Result<Self, DecodeError> {
        reader.expect_start_with_name(Self::XML_TAG_NAME)?;
        let value = Self::read_xml(reader)?;
        reader.expect_end_with_name(Self::XML_TAG_NAME)?;

        Ok(value)
    }

    fn write_outer_xml<W: Write>(
        &self,
        name: &str,
        writer: &mut XmlEventWriter<W>,
    ) -> Result<(), EncodeError> {
        writer.write(XmlWriteEvent::start_element(Self::XML_TAG_NAME).attr("name", name))?;
        self.write_xml(writer)?;
        writer.write(XmlWriteEvent::end_element())
    }
}

pub fn find_canonical_property_descriptor<'db>(
    class_name: &str,
    property_name: &str,
    database: &'db ReflectionDatabase<'db>,
) -> Option<&'db PropertyDescriptor<'db>> {
    find_property_descriptors(class_name, property_name, database)
        .map(|(canonical, _serialized)| canonical)
}

pub fn find_serialized_property_descriptor<'db>(
    class_name: &str,
    property_name: &str,
    database: &'db ReflectionDatabase<'db>,
) -> Option<&'db PropertyDescriptor<'db>> {
    find_property_descriptors(class_name, property_name, database)
        .map(|(_canonical, serialized)| serialized)
}

/// Find both the canonical and serialized property descriptors for a given
/// class and property name pair. These might be the same descriptor!
fn find_property_descriptors<'db>(
    class_name: &str,
    property_name: &str,
    database: &'db ReflectionDatabase<'db>,
) -> Option<(&'db PropertyDescriptor<'db>, &'db PropertyDescriptor<'db>)> {
    let class_descriptor = database.classes.get(class_name)?;

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
            match &property_descriptor.kind {
                PropertyKind::Canonical { serialization } => match serialization {
                    PropertySerialization::Serializes | PropertySerialization::Migrate { .. } => {
                        return Some((property_descriptor, property_descriptor))
                    }
                    PropertySerialization::DoesNotSerialize => {
                        // FIXME: Is this the correct solution?
                        return None;
                    }
                    PropertySerialization::SerializesAs(serialized_name) => {
                        let serialized_descriptor = current_class_descriptor
                            .properties
                            .get(serialized_name.as_ref())
                            .unwrap();

                        return Some((property_descriptor, serialized_descriptor));
                    }
                    _ => unimplemented!(),
                },
                PropertyKind::Alias { alias_for } => {
                    let canonical_descriptor = current_class_descriptor
                        .properties
                        .get(alias_for.as_ref())
                        .unwrap();

                    // FIXME: This code is duplicated with above.
                    match &canonical_descriptor.kind {
                        PropertyKind::Canonical { serialization } => match serialization {
                            PropertySerialization::Serializes
                            | PropertySerialization::Migrate { .. } => {
                                return Some((canonical_descriptor, canonical_descriptor))
                            }
                            PropertySerialization::DoesNotSerialize => {
                                // FIXME: Is this the correct solution?
                                return None;
                            }
                            PropertySerialization::SerializesAs(serialized_name) => {
                                let serialized_descriptor = current_class_descriptor
                                    .properties
                                    .get(serialized_name.as_ref())
                                    .unwrap();

                                return Some((canonical_descriptor, serialized_descriptor));
                            }
                            _ => unimplemented!(),
                        },
                        _ => return None,
                    }
                }
                // FIXME
                _ => unimplemented!(),
            }
        }

        if let Some(superclass_name) = &current_class_descriptor.superclass {
            // If a property descriptor isn't found in our class, check
            // our superclass.

            current_class_descriptor = database
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
