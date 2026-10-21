use behave::prelude::*;
use rbx_types::{
    AttributeError, Attributes, ErrorKind, MaterialColors, MaterialColorsError, Matrix3Error,
    UniqueId, UniqueIdError,
};

behave! {
    "Inspecting errors from native value decoders" {
        "invalid values expose their original typed causes without replacing the error" {
            // @behavior native-value-error-cause-inspection
            // @boundary before=invalid-attribute-and-material-bytes at=public-value-decoders after=borrowed-original-structured-causes
            // @red The public error has no kind accessor or exported cause enums.
            // @green Original full-width color, rotation, length and I/O causes are inspectable without parsing error text.
            let mut color = vec![1, 0, 0, 0, 1, 0, 0, 0, b'A', 0x0e];
            color.extend(65_537u32.to_le_bytes());
            let error = Attributes::from_reader(color.as_slice()).err().ok_or("the color must fail")?;
            expect_match!(error.kind(), ErrorKind::Attribute(AttributeError::InvalidBrickColor(65_537)))?;

            let mut rotation = vec![1, 0, 0, 0, 1, 0, 0, 0, b'C', 0x14];
            rotation.extend([0; 12]);
            rotation.push(255);
            let error = Attributes::from_reader(rotation.as_slice()).err().ok_or("the rotation must fail")?;
            expect_match!(error.kind(), ErrorKind::Attribute(AttributeError::BadAttributeValue(nested))
                if matches!(nested.kind(), ErrorKind::Matrix3(Matrix3Error::BadRotationId { id: 255 })))?;

            let error = MaterialColors::decode(&[0; 3]).err().ok_or("the material length must fail")?;
            expect_match!(error.kind(), ErrorKind::MaterialColors(MaterialColorsError::WrongLength(3)))?;

            let font = [1, 0, 0, 0, 1, 0, 0, 0, b'F', 0x21];
            let error = Attributes::from_reader(font.as_slice()).err().ok_or("the font must fail")?;
            expect_match!(error.kind(), ErrorKind::Attribute(AttributeError::Io(cause))
                if cause.kind() == std::io::ErrorKind::UnexpectedEof)?;

            let key = [1, 0, 0, 0, 1, 0, 0, 0, 255];
            let error = Attributes::from_reader(key.as_slice()).err().ok_or("the key must fail")?;
            let source = std::error::Error::source(&error).ok_or("the original UTF-8 source remains")?;
            let source = source.downcast_ref::<std::string::FromUtf8Error>().ok_or("the source remains a UTF-8 error")?;
            expect_match!(error.kind(), ErrorKind::Attribute(AttributeError::KeyBadUnicode(cause))
                if cause.as_bytes() == [255] && std::ptr::eq(source, cause))?;

            let error = "gggggggggggggggggggggggggggggggg".parse::<UniqueId>().err().ok_or("the ID must fail")?;
            let source = std::error::Error::source(&error).ok_or("the integer parser source remains")?;
            let source = source.downcast_ref::<std::num::ParseIntError>().ok_or("the source remains an integer parser error")?;
            expect_match!(error.kind(), ErrorKind::UniqueId(UniqueIdError::FromStrParseError { err })
                if err.kind() == &std::num::IntErrorKind::InvalidDigit && std::ptr::eq(source, err))?;
        }
    }
}
