mod utils;

use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     fn alert(s: &str);
// }

// #[wasm_bindgen]
// pub fn greet() {
//     alert("Hello, rbx_binary_wasm!");
// }

#[wasm_bindgen]
pub fn decode_file(binary_file: Vec<u8>) -> JsValue {
    let dom = rbx_binary::from_reader(&binary_file[..]);

    match dom {
        Ok(val) => serde_wasm_bindgen::to_value(&val).unwrap(),
        Err(_) => wasm_bindgen::throw_str("deserialization fail :("),
    }
}
