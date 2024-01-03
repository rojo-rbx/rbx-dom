use std::{
    borrow::Cow,
    collections::{HashSet, VecDeque},
    fs::File,
    io::BufReader,
    path::PathBuf,
};

use anyhow::Context;
use rbx_dom_weak::Instance;
use rbx_reflection::ReflectionDatabase;
use rbx_types::VariantType;

pub fn apply_defaults(
    database: &mut ReflectionDatabase,
    defaults_place: &PathBuf,
) -> anyhow::Result<()> {
    let file = BufReader::new(File::open(defaults_place).context("Could not find defaults place")?);

    let decode_options = rbx_xml::DecodeOptions::new()
        .property_behavior(rbx_xml::DecodePropertyBehavior::IgnoreUnknown)
        .reflection_database(database);

    let tree =
        rbx_xml::from_reader(file, decode_options).context("Could not decode defaults place")?;

    let mut found_classes = HashSet::new();
    let mut to_visit = VecDeque::from_iter(tree.root().children());

    while let Some(referent) = to_visit.pop_front() {
        let instance = tree.get_by_ref(*referent).unwrap();

        to_visit.extend(instance.children());

        // Classes like Camera can exist twice since it is added to Workspace on save.
        if found_classes.contains(&instance.class) {
            continue;
        }

        found_classes.insert(instance.class.clone());

        apply_instance_defaults(database, instance);
    }

    Ok(())
}

fn apply_instance_defaults(database: &mut ReflectionDatabase, instance: &Instance) {
    let class = match database.classes.get_mut(instance.class.as_str()) {
        Some(class_descriptor) => class_descriptor,
        None => {
            log::warn!(
                "Class {} found in default place but not reflection database",
                instance.class
            );

            return;
        }
    };

    for (property_name, property_value) in &instance.properties {
        let property_name = Cow::Owned(property_name.clone());

        match property_value.ty() {
            // We skip these types because their defaults aren't useful.
            VariantType::Ref | VariantType::SharedString => continue,

            _ => class
                .default_properties
                .insert(property_name, property_value.clone()),
        };
    }
}
