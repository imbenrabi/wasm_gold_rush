use wasm_bindgen::prelude::*;
mod matrix;
mod gold_rush;

#[wasm_bindgen]
extern {
    pub fn alert(phrase: &str);
}   

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}


pub fn get_new_board(cols:usize, rows: usize) ->  matrix::matrix::Matrix {
    let matrix = matrix::matrix::create_new_matrix(cols, rows);
    return matrix;
}
