//! Convenience functions for finding the canonical and serialized names
//! for properties.

// Large portions of thise code are ripped straight from the old implementation
// of rbx_xml as the underlying implementation works just fine and doesn't need
// to be changed too much. Some cleanup has been done and the database is
// passed as a parameter rather than hardcoding `rbx_reflection_database` but
// this code is otherwise unchanged.

// Documentation for public facing functions has also been added.
// You're welcome.

use rbx_reflection::{
    ClassDescriptor, PropertyDescriptor, PropertyKind, PropertySerialization, ReflectionDatabase,
};

/// Find the 'canonical' data for a property. This is the data that a property
/// should be deserialized with and generally corresponds to what users would
/// expect the property to be referred to as.
pub fn find_canonical_property_descriptor<'db>(
    database: &'db ReflectionDatabase<'db>,
    class_name: &str,
    property_name: &str,
) -> Option<&'db PropertyDescriptor<'db>> {
    find_property_descriptors(database, class_name, property_name).map(|(canonical, _)| canonical)
}

/// Find the 'serialized' data for a property. This is the data that a property
/// should be serialized with and may differ completely from what users expect
/// a property to be. Essentially, these properties are 1:1 matches with what
/// Roblox writes and reads in Roblox Studio.
pub fn find_serialized_property_descriptor<'db>(
    database: &'db ReflectionDatabase<'db>,
    class_name: &str,
    property_name: &str,
) -> Option<&'db PropertyDescriptor<'db>> {
    find_property_descriptors(database, class_name, property_name).map(|(_, serialized)| serialized)
}

/// Find both the canonical and serialized property descriptors for a given
/// class and property name pair. These might be the same descriptor!
fn find_property_descriptors<'db>(
    database: &'db ReflectionDatabase<'db>,
    class_name: &str,
    property_name: &str,
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
            return discriptor_from_kind(property_descriptor, current_class_descriptor);
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

/// Utilized by `find_property_descriptors` to find the canonical descriptor
/// if given a serialized one. This is in its own function because aliases
/// cause a rerun of the search and may do so theoretically infinitely.
fn discriptor_from_kind<'db>(
    property_descriptor: &'db PropertyDescriptor<'db>,
    class_descriptor: &'db ClassDescriptor<'db>,
) -> Option<(&'db PropertyDescriptor<'db>, &'db PropertyDescriptor<'db>)> {
    match &property_descriptor.kind {
        PropertyKind::Canonical { serialization } => match serialization {
            PropertySerialization::Serializes => Some((property_descriptor, property_descriptor)),
            PropertySerialization::DoesNotSerialize => {
                // This hasn't caused any issues for 3 years
                None
            }
            PropertySerialization::SerializesAs(serialized_name) => {
                let serialized_descriptor = class_descriptor
                    .properties
                    .get(serialized_name.as_ref())
                    .expect("database SerializesAs entries should point to a real property");

                Some((property_descriptor, serialized_descriptor))
            }
            // The lifetime on 'PropertyKind' makes us do this.
            // I don't know if we can fix it.
            _ => unimplemented!(),
        },
        PropertyKind::Alias { alias_for } => {
            let canonical_descriptor = class_descriptor.properties.get(alias_for.as_ref()).unwrap();
            // This *could* recurse infinitely but I don't imagine that's a realistic scenario
            return discriptor_from_kind(canonical_descriptor, class_descriptor);
        }
        _ => unimplemented!(),
    }
}
