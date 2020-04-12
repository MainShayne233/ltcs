#[macro_use] extern crate lalrpop_util;
use wasm_bindgen::prelude::*;

lalrpop_mod!(pub calculator1);

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    let result = calculator1::TermParser::new().parse("22");
    alert(&format!("Hello!!!, {}! {:?}", name, result));
}

#[wasm_bindgen]
pub fn parse(input: &str) {
    let result = calculator1::TermParser::new().parse(input);
    alert(&format!("Parsed: {:?}", result));
}
