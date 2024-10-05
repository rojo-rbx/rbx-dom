mod utils;

use utils::set_panic_hook;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn decode_file(binary_file: Vec<u8>) -> JsValue {
    set_panic_hook();

    let dom = rbx_binary::from_reader(&binary_file[..]);

    match dom {
        Ok(val) => serde_wasm_bindgen::to_value(&val).unwrap(),
        Err(_) => wasm_bindgen::throw_str("deserialization fail :("),
    }
}
