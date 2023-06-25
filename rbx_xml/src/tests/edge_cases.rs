//! Tests to cover edge cases encountered over time.

use super::test_suite;

use std::path::PathBuf;

use heck::ToKebabCase;

macro_rules! edge_cases {
    ($($test_name: ident,)*) => {
        $(
            #[test]
            fn $test_name() {
                let _ = env_logger::try_init();
                let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                assert!(test_path.pop());

                test_path.push("test-files");
                test_path.push("edge-cases");
                test_path.push(stringify!($test_name).to_kebab_case());
                test_path.push("xml.rbxmx");

                test_suite(test_path).unwrap()
            }
        )*
    };
}

edge_cases! {
    empty_font,
    xml_unknown_type,
}
