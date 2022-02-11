use wasm_bindgen::prelude::*;
pub mod matrix;

#[wasm_bindgen]
extern {
    pub fn alert(phrase: &str);
}   

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}   
