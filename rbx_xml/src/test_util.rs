//! Contains macros `deserialize_test`, `serialize_test`, and `roundtrip_test` for testing
//! implementations of data types. More detailed documentation is available
//! on each macro specifically on how they are used.

#[macro_export]
/// Tests a deserializer function by attempting to deserialize a provided
/// value and then comparing it to the expected result.
///
/// Takes three arguments. In order:
/// - The path to a deserializer function
/// - The value expected from deserializing
/// - The input to be passed to the deserializer
///
/// The third argument should be the contents of the property element
/// not including the boilerplate `<ElementName name = "Example">` or
/// `</ElementName>`. This reflects how deserializers are implemented.
macro_rules! deserialize_test {
    ($deserializer:path, $expected:expr, $input:expr) => {{
        let _ = env_logger::try_init();
        // We do this to have a nicer panic message :-)
        let value = match $deserializer(&mut $crate::deserializer::XmlReader::from_reader(
            $input.as_bytes(),
        )) {
            Ok(v) => v,
            Err(err) => panic!("{}", err),
        };
        assert_eq!(
            $expected, value,
            concat!(
                "deserializer ",
                stringify!($deserializer),
                " failed to produce '",
                stringify!($expected),
                "' from ",
                stringify!($input)
            )
        )
    }};
}

/// Tests a serializer function by attempting to serialize a provided
/// value and then comparing it to the provided expected result.
///
/// Takes three arguments. In order:
/// - The path to a serializer function
/// - The value to be serialized
/// - The expected output of the serializer
///
/// The third argument should be the contents of the property element
/// not including the boilerplate `<ElementName name = "Example">` or
/// `</ElementName>`. Whitespace will be included, so indentation should be
/// considered. The serializer uses two spaces for indents.
#[macro_export]
macro_rules! serialize_test {
    ($serializer:path, $value:expr, $expected:expr) => {{
        let _ = env_logger::try_init();
        let mut vec = Vec::new();
        let mut writer = $crate::serializer::XmlWriter::new(&mut vec, Some((b' ', 2)));
        $serializer(&mut writer, &$value).unwrap();
        assert_eq!(
            $expected,
            String::from_utf8(vec).unwrap(),
            concat!(
                "serializer '",
                stringify!($serializer),
                "' failed to serialize {:?} as expected"
            ),
            $value
        )
    }};
}

/// Tests a deserializer and serializer function by running the result of the
/// serializer through the deserializer. To be specific, the result of the
/// serializer is ran directly through the deserializer and then the result
/// of the deserializer is compared using `==` to the original value.
///
/// Takes three arguments. In order:
/// - The path to a serializer function
/// - The path to a deserializer function
/// - The value to test the roundtrip of
///
/// This test is not useful on its own because it does not confirm the
/// values produced by the serializer and deserializer are valid. It simply
/// confirms that they together result in the original value. Tests should also
/// be ran on the functions seperately.
#[macro_export]
macro_rules! roundtrip_test {
    ($serializer:path, $deserializer:path, $value:expr) => {{
        let _ = env_logger::try_init();
        let mut buff = Vec::new();
        let mut writer = $crate::serializer::XmlWriter::new(&mut buff, Some((b' ', 2)));

        log::debug!(
            concat!(
                "Attempting to serialize {:?} using ",
                stringify!($serializer)
            ),
            $value
        );
        $serializer(&mut writer, &$value).unwrap();

        log::debug!(concat!(
            "Attempting to deserialize using ",
            stringify!($deserializer)
        ));

        let new_value = $deserializer(&mut $crate::deserializer::XmlReader::from_reader(
            buff.as_slice(),
        ))
        .unwrap();

        assert_eq!(
            $value, new_value,
            concat!(
                "round trip with ",
                stringify!($serializer),
                " and ",
                stringify!($deserializer),
                " did not produce the same value"
            )
        )
    }};
}
