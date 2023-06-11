//! Implements deserialization for simple to parse types.
//! Namely:
//! - `bool`
//! - `i32`, `i64`, `f32`, `f64`, `Enum`
//! - `String`, `ProtectedString`, `BinaryString`
//!
//! Does not handle parsing particular `BinaryString` subtypes and instead
//! provides for parsing the raw base64 into a `rbx_types::BinaryString`.
use std::io::BufRead;

use rbx_dom_weak::types::{BinaryString, Enum};

use crate::deserializer::{error::DecodeError, reader::XmlReader};

pub fn string_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<String, DecodeError> {
    reader.eat_text()
}

pub fn binary_string_deserializer<R: BufRead>(
    reader: &mut XmlReader<R>,
) -> Result<BinaryString, DecodeError> {
    Ok(BinaryString::from(reader.eat_base64()?))
}

pub fn bool_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<bool, DecodeError> {
    let content = reader.eat_text()?;
    content
        .parse()
        .map_err(|_| reader.error(format!("value '{content}' is not a valid bool")))
}

pub fn f32_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<f32, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not get 32-bit float from `{content}` because {err}"
        ))
    })
}

pub fn f64_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<f64, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not get 64-bit float from `{content}` because {err}"
        ))
    })
}

pub fn i32_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<i32, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not get 32-bit int from `{content}` because {err}"
        ))
    })
}

pub fn i64_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<i64, DecodeError> {
    let content = reader.eat_text()?;
    content.parse().map_err(|err| {
        reader.error(format!(
            "could not get 64-bit int from `{content}` because {err}"
        ))
    })
}

pub fn enum_deserializer<R: BufRead>(reader: &mut XmlReader<R>) -> Result<Enum, DecodeError> {
    let content = reader.eat_text()?;
    content
        .parse()
        .map(Enum::from_u32)
        .map_err(|err| reader.error(format!("could not get Enum from `{content}` because {err}")))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::deserialize_test;

    #[test]
    fn string_normal() {
        deserialize_test!(string_deserializer, "", "");

        deserialize_test!(string_deserializer, "Hello, world!", "Hello, world!");
        deserialize_test!(string_deserializer, "Hello,\nworld!", "Hello,\nworld!");
        // no cdata = no preserve whitespace
        deserialize_test!(string_deserializer, "Hello, world", "   Hello, world   ");

        // potato selfie with no skin tone
        deserialize_test!(string_deserializer, "🥔🤳", "🥔🤳");
        // 'Hello' in simplified Mandarin
        deserialize_test!(string_deserializer, "你好", "你好");
        // 'Hello' in Arabic
        deserialize_test!(string_deserializer, "مرحبًا", "مرحبًا");

        deserialize_test!(string_deserializer, ">Test<", "&gt;Test&lt;");
        deserialize_test!(string_deserializer, "&'\"", "&amp;&apos;&quot;");
    }

    #[test]
    fn string_with_cdata() {
        deserialize_test!(string_deserializer, "", "<![CDATA[]]>");

        deserialize_test!(
            string_deserializer,
            "Hello, world!",
            "<![CDATA[Hello, world!]]>"
        );
        deserialize_test!(
            string_deserializer,
            "   Hello, world   ",
            "<![CDATA[   Hello, world   ]]>"
        );
        deserialize_test!(string_deserializer, "&gt;", "<![CDATA[&gt;]]>");
        // This is awful to read manually so just trust me, it equals ]]>
        deserialize_test!(string_deserializer, "]]>", "<![CDATA[]]]]><![CDATA[>]]>")
    }

    #[test]
    fn binary_string_valid() {
        deserialize_test!(binary_string_deserializer, BinaryString::new(), "");
        deserialize_test!(
            binary_string_deserializer,
            BinaryString::new(),
            "<![CDATA[]]>"
        );
        deserialize_test!(
            binary_string_deserializer,
            BinaryString::from(b"Hello, world!".as_slice()),
            "SGVsbG8sIHdvcmxkIQ=="
        );
        deserialize_test!(
            binary_string_deserializer,
            BinaryString::from(b"Hello, world!".as_slice()),
            "<![CDATA[SGVsbG8sIHdvcmxkIQ==]]>"
        );
        deserialize_test!(
            binary_string_deserializer,
            BinaryString::from(b"Hello, world!".as_slice()),
            "SGV\nsbG8s\n\nIHdv\r\ncmx  kIQ=="
        );
    }

    #[test]
    #[ignore = "unclear whether we should accept this"]
    // This currently fails when it's run but it's trivially easy to fix.
    // The reason I haven't is because I'm unsure whether this is a good thing
    // Open to feedback.
    fn binary_string_no_padding() {
        deserialize_test!(
            binary_string_deserializer,
            BinaryString::from(b"Hello, world!".as_slice()),
            "SGVsbG8sIHdvcmxkIQ"
        );
    }

    #[test]
    #[should_panic = "invalid base64 string"]
    fn binary_string_invalid_chars() {
        deserialize_test!(binary_string_deserializer, BinaryString::new(), ":-)");
    }

    #[test]
    fn bool_valid() {
        deserialize_test!(bool_deserializer, true, "true");
        deserialize_test!(bool_deserializer, false, "false");
    }

    #[test]
    #[should_panic]
    fn bool_invalid() {
        deserialize_test!(bool_deserializer, false, "FALSE");
    }

    #[test]
    fn f32_valid() {
        deserialize_test!(f32_deserializer, f32::INFINITY, "INF");
        deserialize_test!(f32_deserializer, f32::NEG_INFINITY, "-INF");
        deserialize_test!(f32_deserializer, 0.0, "0");
        deserialize_test!(f32_deserializer, 0.0, "0.0");
        deserialize_test!(f32_deserializer, 1.0, "1");
        deserialize_test!(f32_deserializer, 0.15625, "0.15625");
        deserialize_test!(f32_deserializer, -0.15625, "-0.15625");

        // Can't compare Nan to itself and the macros use assert_eq! (which uses ==)
        let input = "NAN";
        let output = f32_deserializer(&mut XmlReader::from_str(input)).unwrap();
        assert!(
            output.is_nan(),
            "f32_deserializer did not produce NaN from {}",
            input
        );
    }

    #[test]
    fn f64_valid() {
        deserialize_test!(f64_deserializer, f64::INFINITY, "INF");
        deserialize_test!(f64_deserializer, f64::NEG_INFINITY, "-INF");
        deserialize_test!(f64_deserializer, 0.0, "0");
        deserialize_test!(f64_deserializer, 0.0, "0.0");
        deserialize_test!(f64_deserializer, 1.0, "1");
        deserialize_test!(f64_deserializer, 0.15625, "0.15625");
        deserialize_test!(f64_deserializer, -0.15625, "-0.15625");

        // Can't compare nan to itself and the macro uses assert_eq! (which uses ==)
        let input = "NAN";
        let output = f64_deserializer(&mut XmlReader::from_str(input)).unwrap();
        assert!(
            output.is_nan(),
            "f64_deserializer did not produce NaN from {}",
            input
        );
    }

    #[test]
    fn i32_valid() {
        deserialize_test!(i32_deserializer, i32::MIN, "-2147483648");
        deserialize_test!(i32_deserializer, i32::MAX, "2147483647");
        deserialize_test!(i32_deserializer, 0, "0");
        deserialize_test!(i32_deserializer, 1234567890, "1234567890");
        deserialize_test!(i32_deserializer, -1234567890, "-1234567890");
    }

    #[test]
    #[should_panic = "invalid digit"]
    fn i32_invalid_digit() {
        deserialize_test!(i32_deserializer, 1, "0x01");
    }

    #[test]
    #[should_panic = "number too large"]
    fn i32_too_big() {
        // 9999999999999999 is much too big for an i32
        deserialize_test!(i32_deserializer, 0, "9999999999999999");
    }

    #[test]
    #[should_panic = "number too small"]
    fn i32_too_small() {
        // -9999999999999999 is much too small for an i32
        deserialize_test!(i32_deserializer, 0, "-9999999999999999");
    }

    #[test]
    fn i64_valid() {
        deserialize_test!(i64_deserializer, i64::MIN, "-9223372036854775808");
        deserialize_test!(i64_deserializer, i64::MAX, "9223372036854775807");
        deserialize_test!(i64_deserializer, 0, "0");
        deserialize_test!(i64_deserializer, 1234567890, "1234567890");
        deserialize_test!(i64_deserializer, -1234567890, "-1234567890");
    }

    #[test]
    #[should_panic = "invalid digit"]
    fn i64_invalid_digit() {
        deserialize_test!(i64_deserializer, 1, "0x01");
    }

    #[test]
    #[should_panic = "number too large"]
    fn i64_too_big() {
        // 999999999999999999999999 is much too big for an i64
        deserialize_test!(i64_deserializer, 0, "999999999999999999999999");
    }

    #[test]
    #[should_panic = "number too small"]
    fn i64_too_small() {
        // -9999999999999999 is much too small for an i64
        deserialize_test!(i64_deserializer, 0, "-999999999999999999999999");
    }

    #[test]
    fn enum_valid() {
        deserialize_test!(enum_deserializer, Enum::from_u32(u32::MIN), "0");
        deserialize_test!(enum_deserializer, Enum::from_u32(u32::MAX), "4294967295");
        deserialize_test!(enum_deserializer, Enum::from_u32(1234567890), "1234567890");
    }

    #[test]
    #[should_panic = "invalid digit"]
    fn enum_invalid_digit() {
        deserialize_test!(enum_deserializer, Enum::from_u32(1), "0x01");
    }

    #[test]
    #[should_panic = "number too large"]
    fn enum_too_big() {
        // 9999999999999999 is much too big for an enum
        deserialize_test!(enum_deserializer, Enum::from_u32(0), "9999999999999999");
    }

    #[test]
    #[should_panic = "invalid digit"]
    fn enum_too_small() {
        // -1 is too small for an enum
        deserialize_test!(enum_deserializer, Enum::from_u32(0), "-1");
    }
}
