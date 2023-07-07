use std::io;

use rbx_dom_weak::types::{BinaryString, Content, Enum};

use super::{EncodeError, XmlWriter};

pub fn string_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &str,
) -> Result<(), EncodeError> {
    if value.starts_with(char::is_whitespace) || value.ends_with(char::is_whitespace) {
        writer.write_raw_text(value)
    } else {
        writer.write_text(value)
    }
}

pub fn binary_string_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &BinaryString,
) -> Result<(), EncodeError> {
    writer.write_base64(value.as_ref())
}

pub fn bool_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &bool,
) -> Result<(), EncodeError> {
    writer.write_text(match value {
        true => "true",
        false => "false",
    })
}

pub fn i32_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &i32,
) -> Result<(), EncodeError> {
    writer.write_text(&value.to_string())
}

pub fn i64_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &i64,
) -> Result<(), EncodeError> {
    writer.write_text(&value.to_string())
}

pub fn f32_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &f32,
) -> Result<(), EncodeError> {
    if value.is_nan() {
        writer.write_text("NAN")
    } else if *value == f32::INFINITY {
        writer.write_text("INF")
    } else if *value == f32::NEG_INFINITY {
        writer.write_text("-INF")
    } else if *value == 0.0 {
        // Without this, -0.0 serializers as -0
        writer.write_text("0")
    } else {
        writer.write_text(&value.to_string())
    }
}

pub fn f64_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &f64,
) -> Result<(), EncodeError> {
    if value.is_nan() {
        writer.write_text("NAN")
    } else if *value == f64::INFINITY {
        writer.write_text("INF")
    } else if *value == f64::NEG_INFINITY {
        writer.write_text("-INF")
    } else if *value == 0.0 {
        // Without this, -0.0 serializers as -0
        writer.write_text("0")
    } else {
        writer.write_text(&value.to_string())
    }
}

pub fn enum_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Enum,
) -> Result<(), EncodeError> {
    writer.write_text(&value.to_u32().to_string())
}

pub fn content_serializer<W: io::Write>(
    writer: &mut XmlWriter<W>,
    value: &Content,
) -> Result<(), EncodeError> {
    // FIXME: Content should have method for taking it as a &str
    let str: &str = value.as_ref();
    if str.is_empty() {
        // This is necessary to mimic the formatting of the old
        // version of rbx_xml. Without it, `<null></null>` will get written
        // onto one line which breaks diffs.
        writer.start_element("null").finalize()?;
        writer.end_element("null")
    } else {
        writer.write_element("url", str)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serialize_test;

    #[test]
    fn string_normal() {
        serialize_test!(string_serializer, "", "");

        serialize_test!(string_serializer, "Hello, world!", "Hello, world!");
        serialize_test!(string_serializer, "Hello,\nworld!", "Hello,\nworld!");

        serialize_test!(string_serializer, "🥔🤳", "🥔🤳");
        serialize_test!(string_serializer, "你好", "你好");
        serialize_test!(string_serializer, "مرحبًا", "مرحبًا");

        serialize_test!(string_serializer, ">Test<", "&gt;Test&lt;");
    }

    #[test]
    fn string_with_cdata() {
        serialize_test!(string_serializer, "   ", "<![CDATA[   ]]>");

        serialize_test!(
            string_serializer,
            " Hello, world!",
            "<![CDATA[ Hello, world!]]>"
        );
        serialize_test!(
            string_serializer,
            "Hello, world! ",
            "<![CDATA[Hello, world! ]]>"
        );
        serialize_test!(
            string_serializer,
            " Hello, world! ",
            "<![CDATA[ Hello, world! ]]>"
        );

        serialize_test!(string_serializer, " 🥔🤳", "<![CDATA[ 🥔🤳]]>");
        serialize_test!(string_serializer, "你好 ", "<![CDATA[你好 ]]>");
        serialize_test!(string_serializer, " مرحبًا ", "<![CDATA[ مرحبًا ]]>");

        serialize_test!(string_serializer, " >Test< ", "<![CDATA[ >Test< ]]>");
    }

    #[test]
    fn string_with_quotes() {
        serialize_test!(string_serializer, "\"Hello, world!\"", "\"Hello, world!\"");
        serialize_test!(string_serializer, "'Hello, world!'", "'Hello, world!'");
    }

    #[test]
    #[should_panic]
    fn string_with_bad_sequence() {
        // This will serialize incorrectly because of the library we're using.
        // It's relatively simple to fix, but it has a significant performance
        // cost so I'm willing to write it off.
        serialize_test!(string_serializer, " ]]> ", "<!CDATA[ ]]]]><!CDATA[> ]]>");
    }

    #[test]
    fn binary_string_valid() {
        serialize_test!(binary_string_serializer, BinaryString::new(), "");

        // This is the only testing done here since the bulk of the work
        // is being carried by the base64 library, which is unlikely to break.
        serialize_test!(
            binary_string_serializer,
            BinaryString::from("Hello, world!".as_bytes()),
            "SGVsbG8sIHdvcmxkIQ=="
        );
    }

    #[test]
    fn bool_valid() {
        serialize_test!(bool_serializer, true, "true");
        serialize_test!(bool_serializer, false, "false");
    }

    #[test]
    fn f32_valid() {
        serialize_test!(f32_serializer, 0.0, "0");
        serialize_test!(f32_serializer, -0.0, "0");
        serialize_test!(f32_serializer, 1.0, "1");
        serialize_test!(f32_serializer, -1.0, "-1");
        serialize_test!(f32_serializer, 0.15625, "0.15625");
        serialize_test!(f32_serializer, -0.15625, "-0.15625");
        serialize_test!(f32_serializer, f32::INFINITY, "INF");
        serialize_test!(f32_serializer, f32::NEG_INFINITY, "-INF");
        serialize_test!(f32_serializer, f32::NAN, "NAN");
    }

    #[test]
    fn f64_valid() {
        serialize_test!(f64_serializer, 0.0, "0");
        serialize_test!(f64_serializer, -0.0, "0");
        serialize_test!(f64_serializer, 1.0, "1");
        serialize_test!(f64_serializer, -1.0, "-1");
        serialize_test!(f64_serializer, 0.15625, "0.15625");
        serialize_test!(f64_serializer, -0.15625, "-0.15625");
        serialize_test!(f64_serializer, f64::INFINITY, "INF");
        serialize_test!(f64_serializer, f64::NEG_INFINITY, "-INF");
        serialize_test!(f64_serializer, f64::NAN, "NAN");
    }

    #[test]
    fn i32_valid() {
        serialize_test!(i32_serializer, i32::MIN, "-2147483648");
        serialize_test!(i32_serializer, i32::MAX, "2147483647");
        serialize_test!(i32_serializer, 0, "0");
        serialize_test!(i32_serializer, 1234, "1234");
        serialize_test!(i32_serializer, -1234, "-1234");
    }

    #[test]
    fn i64_valid() {
        serialize_test!(i64_serializer, i64::MIN, "-9223372036854775808");
        serialize_test!(i64_serializer, i64::MAX, "9223372036854775807");
        serialize_test!(i64_serializer, 0, "0");
        serialize_test!(i64_serializer, 1234, "1234");
        serialize_test!(i64_serializer, -1234, "-1234");
    }

    #[test]
    fn enum_valid() {
        serialize_test!(enum_serializer, Enum::from_u32(u32::MIN), "0");
        serialize_test!(enum_serializer, Enum::from_u32(u32::MAX), "4294967295");
        serialize_test!(enum_serializer, Enum::from_u32(1234), "1234");
    }

    #[test]
    fn content_url() {
        serialize_test!(content_serializer, Content::from("TEST"), "<url>TEST</url>");
        serialize_test!(
            content_serializer,
            Content::from("http://www.example.com"),
            "<url>http://www.example.com</url>"
        );
    }

    #[test]
    fn content_null() {
        // Compatibility-wise, we have to preserve `<null>` being on a
        // different line from </null> so this is expected behavior.
        serialize_test!(content_serializer, Content::from(""), "<null>\n</null>");
    }
}
