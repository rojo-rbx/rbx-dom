mod utils;

use rbx_dom_weak::WeakDom;
use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decode_file(binary_file: Vec<u8>) -> WeakDom {
    set_panic_hook();

    let dom = rbx_binary::from_reader(&binary_file[..]).expect("Failed to deserialize binary file.");

    dom
}

#[wasm_bindgen]
pub fn encode_file(dom: WeakDom) -> Vec<u8> {
    let mut binary_file = Vec::new();

    rbx_binary::to_writer(&mut binary_file, &dom, &[dom.root_ref()])
        .expect("Failed to serialize from DOM");

    binary_file
}
