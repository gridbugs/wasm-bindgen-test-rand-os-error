extern crate rand;
extern crate wasm_bindgen;
extern crate wasm_bindgen_test;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

use rand::RngCore;

#[wasm_bindgen]
pub fn generate_with_os_rng() -> u32 {
    let mut rng = rand::rngs::OsRng::new().unwrap();
    rng.next_u32()
}

#[wasm_bindgen_test]
fn test_with_os_rng() {
    let _ = generate_with_os_rng();
}
