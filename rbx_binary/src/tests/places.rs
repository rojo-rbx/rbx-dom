use std::path::PathBuf;

use heck::ToKebabCase;

use super::util::run_model_base_suite;

macro_rules! place_tests {
    ($($test_name: ident,)*) => {
        $(
            #[test]
            fn $test_name() {
                let _ = env_logger::try_init();

                let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                assert!(test_path.pop());
                test_path.push("test-files");
                test_path.push("places");
                test_path.push(stringify!($test_name).to_kebab_case());
                test_path.push("binary.rbxl");

                run_model_base_suite(test_path);
            }
        )*
    };
}

place_tests! {
    baseplate_566,
}
