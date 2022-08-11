use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

#[wasm_bindgen]
pub fn square(n1: i32) -> i32 {
    n1 * n1
}

#[wasm_bindgen]
pub fn weirdadd(n1: i32, n2: i32) -> i32 {
    square(add(n1, n2))
}

