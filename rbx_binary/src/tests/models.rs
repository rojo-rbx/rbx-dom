use std::path::PathBuf;

use heck::KebabCase;

use super::util::run_model_base_suite;

macro_rules! binary_tests {
    ($($test_name: ident,)*) => {
        $(
            #[test]
            fn $test_name() {
                let _ = env_logger::try_init();

                let mut test_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
                assert!(test_path.pop());
                test_path.push("test-files");
                test_path.push("models");
                test_path.push(stringify!($test_name).to_kebab_case());
                test_path.push("binary.rbxm");

                run_model_base_suite(test_path);
            }
        )*
    };
}

binary_tests! {
    default_inserted_folder,
    default_inserted_modulescript,
    three_screengui,
    bloomeffect,
    funny_numbervalue,
    funny_uipadding,
    three_uigridlayouts,
}
