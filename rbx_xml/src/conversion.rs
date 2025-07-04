//! Describes conversions that are allowed when deserializing properties from
//! the XML format.

use std::borrow::{Borrow, Cow};
use std::convert::TryInto;

use rbx_dom_weak::{
    types::{
        Attributes, BrickColor, Color3uint8, ContentId, ContentType, Enum, MaterialColors,
        SmoothGrid, Tags, Variant, VariantType,
    },
    Ustr,
};

pub trait ConvertVariant: Clone + Sized {
    fn try_convert(self, class_name: Ustr, target_type: VariantType) -> Result<Self, String> {
        Self::try_convert_cow(class_name, Cow::Owned(self), target_type)
            .map(|value| value.into_owned())
    }

    fn try_convert_ref(
        &self,
        class_name: Ustr,
        target_type: VariantType,
    ) -> Result<Cow<'_, Self>, String> {
        Self::try_convert_cow(class_name, Cow::Borrowed(self), target_type)
    }

    fn try_convert_cow(
        class_name: Ustr,
        value: Cow<'_, Self>,
        target_type: VariantType,
    ) -> Result<Cow<'_, Self>, String>;
}

impl ConvertVariant for Variant {
    fn try_convert_cow(
        class_name: Ustr,
        value: Cow<'_, Self>,
        target_type: VariantType,
    ) -> Result<Cow<'_, Self>, String> {
        match (value.borrow(), target_type) {
            // Older files may not have their number types moved to 64-bit yet,
            // which can cause problems. See issue #301.
            (Variant::Int32(value), VariantType::Int64) => {
                Ok(Cow::Owned((i64::from(*value)).into()))
            }
            (Variant::Float32(value), VariantType::Float64) => {
                Ok(Cow::Owned((f64::from(*value)).into()))
            }
            (Variant::Int32(value), VariantType::BrickColor) => {
                let narrowed: u16 = (*value).try_into().map_err(|_| {
                    format!("Value {} is not in the range of a valid BrickColor", value)
                })?;

                BrickColor::from_number(narrowed)
                    .ok_or_else(|| format!("{} is not a valid BrickColor number", value))
                    .map(Into::into)
                    .map(Cow::Owned)
            }
            (Variant::Color3(value), VariantType::Color3uint8) => {
                Ok(Cow::Owned(Color3uint8::from(*value).into()))
            }
            (Variant::BinaryString(value), VariantType::Tags) => Ok(Cow::Owned(
                Tags::decode(value.as_ref())
                    .map_err(|_| "Tags contain invalid UTF-8")?
                    .into(),
            )),
            (Variant::BinaryString(value), VariantType::Attributes) => {
                let bytes: &[u8] = value.as_ref();
                match Attributes::from_reader(bytes) {
                    Ok(attributes) => Ok(Cow::Owned(attributes.into())),
                    Err(err) => {
                        log::warn!(
                            "Failed to parse Attributes on {} because {:?}; falling back to BinaryString.

rbx-dom may require changes to fully support this property. Please open an issue at https://github.com/rojo-rbx/rbx-dom/issues and show this warning.",
                             class_name,
                             err
                        );

                        Ok(Cow::Owned(value.clone().into()))
                    }
                }
            }
            (Variant::BinaryString(value), VariantType::MaterialColors) => {
                match MaterialColors::decode(value.as_ref()) {
                    Ok(material_colors) => Ok(Cow::Owned(material_colors.into())),
                    Err(err) => {
                        log::warn!(
                            "Failed to parse MaterialColors on {} because {:?}; falling back to BinaryString.

rbx-dom may require changes to fully support this property. Please open an issue at https://github.com/rojo-rbx/rbx-dom/issues and show this warning.",
                            class_name,
                            err
                        );

                        Ok(Cow::Owned(value.clone().into()))
                    }
                }
            }
            (Variant::BinaryString(value), VariantType::SmoothGrid) => {
                match SmoothGrid::decode(value.as_ref()) {
                    Ok(smooth_grid) => Ok(Cow::Owned(smooth_grid.into())),
                    Err(err) => {
                        log::warn!(
                            "Failed to parse SmoothGrid on {} because {:?}; falling back to BinaryString.

rbx-dom may require changes to fully support this property. Please open an issue at https://github.com/rojo-rbx/rbx-dom/issues and show this warning.",
                            class_name,
                            err
                        );

                        Ok(Cow::Owned(value.clone().into()))
                    }
                }
            }
            (Variant::EnumItem(enum_item), VariantType::Enum) => {
                Ok(Cow::Owned(Enum::from_u32(enum_item.value).into()))
            }
            (Variant::Content(content), VariantType::ContentId) => match content.value() {
                ContentType::None => Ok(Cow::Owned(ContentId::new().into())),
                ContentType::Uri(uri) => Ok(Cow::Owned(ContentId::from(uri.as_str()).into())),
                ContentType::Object(_) => {
                    Err(String::from("Objects cannot be converted into a ContentId"))
                }
                _ => Err(String::from(
                    "Unknown type of Content cannot be converted into a ContentId",
                )),
            },
            (_, _) => Ok(value),
        }
    }
}
