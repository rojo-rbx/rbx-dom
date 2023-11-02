//! Roundtrip testing for model files from the `test-files` submodule

use std::path::PathBuf;

use heck::ToKebabCase;

use super::test_suite;

macro_rules! model_tests {
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
                test_path.push("xml.rbxmx");

                test_suite(test_path).unwrap()
            }
        )*
    };
}

model_tests! {
    attributes,
    axes,
    bloomeffect,
    cframe_case_mixture,
    default_inserted_folder,
    default_inserted_modulescript,
    faces,
    funny_numbervalue,
    funny_uipadding,
    optionalcoordinateframe_models,
    three_beams,
    ref_adjacent,
    ref_child,
    ref_parent,
    sharedstring,
    tags,
    three_brickcolorvalues,
    three_color3values,
    three_intvalues,
    three_nested_folders,
    three_screengui,
    three_uigradients,
    three_uigridlayouts,
    three_unique_frames,
    three_unique_parts,
    three_vector3values,
    two_cframevalues,
    unions,
    two_imagebuttons,
    two_particleemitters,
    two_ray_values,
    two_terrainregions,
    weldconstraint,
    package_link,
    text_label_with_font,
    gui_inset_and_font_migration,
    folder_with_cframe_attributes,
    folder_with_font_attribute,
    number_values_with_security_capabilities,
}
