#![recursion_limit="128"]

mod roblox_install;
mod api_dump;
mod run_in_roblox;

use std::{
    fs::File,
    io::Write,
    path::PathBuf,
    error::Error,
};

use serde_derive::Deserialize;
use rbx_dom_weak::{RbxTree, RbxInstanceProperties};
use quote::quote;
use proc_macro2::{TokenStream, Literal};
use lazy_static::lazy_static;

use crate::api_dump::{Dump, DumpClassMember, ValueType, ValueCategory};

static PLUGIN_MODEL: &[u8] = include_bytes!(concat!(env!("OUT_DIR"), "/plugin.rbxmx"));

lazy_static! {
    static ref OUTPUT_DIR: PathBuf = {
        let mut output = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        output.pop();
        output.push("rbx_reflection");
        output.push("src");
        output
    };
}

fn resolve_value_type(value_type: &ValueType) -> TokenStream {
    let name = Literal::string(&value_type.name);

    match value_type.category {
        ValueCategory::Primitive => {
            let data_kind = match value_type.name.as_str() {
                "bool" => quote!(RbxValueType::Bool),
                "string" => quote!(RbxValueType::String),
                "int" => quote!(RbxValueType::Int32),
                "float" => quote!(RbxValueType::Float32),

                // These aren't quite right:
                "double" => quote!(RbxValueType::Float32),
                "int64" => quote!(RbxValueType::Int32),

                unknown => {
                    println!("Can't emit primitives of type {}", unknown);

                    let unknown_name = Literal::string(unknown);
                    return quote!(RbxPropertyType::UnimplementedType(#unknown_name));
                },
            };

            quote!(RbxPropertyType::Data(#data_kind))
        },
        ValueCategory::DataType => {
            let data_kind = match value_type.name.as_str() {
                "Vector3" => quote!(RbxValueType::Vector3),
                "Vector2" => quote!(RbxValueType::Vector2),
                "Color3" => quote!(RbxValueType::Color3),
                "CFrame" => quote!(RbxValueType::CFrame),
                "PhysicalProperties" => quote!(RbxValueType::PhysicalProperties),
                "BinaryString" => quote!(RbxValueType::BinaryString),

                unknown => {
                    println!("Can't emit data of type {}", unknown);

                    let unknown_name = Literal::string(unknown);
                    return quote!(RbxPropertyType::UnimplementedType(#unknown_name));
                },
            };

            quote!(RbxPropertyType::Data(#data_kind))
        },
        ValueCategory::Enum => quote!(RbxPropertyType::Enum(#name)),
        ValueCategory::Class => {
            println!("Can't emit class references yet!");
            quote!(RbxPropertyType::UnimplementedType("Ref"))
        },
    }
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum Message {
    Version {
        version: [u32; 4],
    },
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Output at {}", OUTPUT_DIR.display());

    let dump = Dump::read()?;

    let plugin_tree = {
        let mut plugin_tree = RbxTree::new(RbxInstanceProperties {
            name: String::from("generate_rbx_reflection plugin"),
            class_name: String::from("Folder"),
            properties: Default::default(),
        });

        let root_id = plugin_tree.get_root_id();

        rbx_xml::decode(&mut plugin_tree, root_id, PLUGIN_MODEL)
            .expect("Couldn't deserialize built-in plugin");

        plugin_tree
    };

    let messages = run_in_roblox::run_in_roblox(plugin_tree);

    let mut studio_version = [0, 0, 0, 0];

    for message in &messages {
        let deserialized = serde_json::from_slice(&message)
            .expect("Couldn't deserialize message");

        match deserialized {
            Message::Version { version } => {
                studio_version = version;
            },
        }
    }

    let classes = dump.classes.iter().map(|class| {
        let class_name = Literal::string(&class.name);

        let superclass_value = if class.superclass == "<<<ROOT>>>" {
            quote!(None)
        } else {
            let superclass_literal = Literal::string(&class.superclass);
            quote!(Some(#superclass_literal))
        };

        let properties = class.members.iter().filter_map(|member|
            match member {
                DumpClassMember::Property { name, value_type } => {
                    let member_name = Literal::string(&name);
                    let resolved_type = resolve_value_type(value_type);

                    Some(quote! {
                        properties.insert(#member_name, RbxInstanceProperty {
                            name: #member_name,
                            value_type: #resolved_type,
                        });
                    })
                },
                _ => None,
            }
        );

        quote! {
            output.insert(#class_name, RbxInstanceClass {
                name: #class_name,
                superclass: #superclass_value,
                properties: {
                    #[allow(unused_mut)]
                    let mut properties = HashMap::new();
                    #(#properties)*
                    properties
                },
            });
        }
    });

    let enums = dump.enums.iter().map(|rbx_enum| {
        let enum_name = Literal::string(&rbx_enum.name);

        let items = rbx_enum.items.iter().map(|item| {
            let item_name = Literal::string(&item.name);
            let item_value = Literal::u32_unsuffixed(item.value);

            quote! {
                items.insert(#item_name, #item_value);
            }
        });

        quote! {
            output.insert(#enum_name, RbxEnum {
                name: #enum_name,
                items: {
                    #[allow(unused_mut)]
                    let mut items = HashMap::new();
                    #(#items)*
                    items
                },
            });
        }
    });

    let output = quote! {
        #![allow(unused_mut)]
        use std::collections::HashMap;
        use rbx_dom_weak::RbxValueType;
        use crate::types::*;

        pub fn generate_classes() -> HashMap<&'static str, RbxInstanceClass> {
            let mut output = HashMap::new();

            #(#classes)*

            output
        }

        pub fn generate_enums() -> HashMap<&'static str, RbxEnum> {
            let mut output = HashMap::new();

            #(#enums)*

            output
        }
    };

    let mut dump = File::create(OUTPUT_DIR.join("dump.rs"))?;
    writeln!(dump, "//! This file is automatically generated by generate_rbx_reflection.")?;
    writeln!(dump, "//! To update it, make sure you have Roblox Studio installed and run")?;
    writeln!(dump, "//! `gen-reflection` in the root.")?;
    write!(dump, "{}", output)?;

    let mut version = File::create(OUTPUT_DIR.join("version.rs"))?;
    writeln!(version, "pub const VERSION_MAJOR: u32 = {};", studio_version[0])?;
    writeln!(version, "pub const VERSION_MINOR: u32 = {};", studio_version[1])?;
    writeln!(version, "pub const VERSION_PATCH: u32 = {};", studio_version[2])?;
    writeln!(version, "pub const VERSION_BUILD: u32 = {};", studio_version[3])?;

    Ok(())
}