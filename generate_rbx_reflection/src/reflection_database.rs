use quote::quote;
use proc_macro2::{TokenStream, Literal};

use crate::{
    api_dump::{Dump, DumpClassMember, ValueType, ValueCategory},
};

pub fn generate(dump: &Dump) -> TokenStream {
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

    quote! {
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
    }
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
        ValueCategory::Class => quote!(RbxPropertyType::Data(RbxValueType::Ref)),
    }
}