use std::{fs, path::Path};

use rbx_dom_weak::DomViewer;

macro_rules! test_models {
    ( $( $test_name: ident : $file_name: expr,)* ) => {
        $(
            #[test]
            fn $test_name() {
                let _ = env_logger::try_init();

                let mut path = Path::new("../test-files/models").join($file_name);
                path.push("xml.rbxmx");

                let contents = fs::read_to_string(path).unwrap();
                let dom = rbx_xml::from_str_default(&contents).unwrap();

                let mut viewer = DomViewer::new();

                insta::assert_yaml_snapshot!(viewer.view_children(&dom));
            }
        )*
    };
}

test_models! {
    ball_socket_constraint: "ball-socket-constraint",
    default_inserted_folder: "default-inserted-folder",
    default_inserted_part: "default-inserted-part",
    // faces: "faces",
    // axes: "axes",
    ref_adjacent: "ref-adjacent",
    ref_child: "ref-child",
    ref_parent: "ref-parent",
    body_movers: "body-movers",
    // default_inserted_modulescript: "default-inserted-modulescript",
}
