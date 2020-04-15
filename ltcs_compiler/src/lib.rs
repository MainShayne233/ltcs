mod langlang;
use wasm_bindgen::prelude::*;
#[macro_use] extern crate lalrpop_util;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello!!!, {}!", name));
}

#[wasm_bindgen]
pub fn parse(input: &str) -> String {
    match langlang::parse(input) {
        Ok(program) => format!("{:?}", program),
        Err(error) => error,
    }
}
