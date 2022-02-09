use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    pub fn alert(phrase: &str);
}   

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}   

#[cfg(test)]
mod tests {
    #[test]
    fn testing() {
        assert_eq!(2 + 2, 4);
    }
}