/**
 * @File   : common.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 6/6/2019, 5:41:09 PM
 * @Description:
 */
use wasm_bindgen::prelude::*;

pub static EPSILON: f32 = 0.0001;

pub static PI: f32 = 3.141592653589793;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

pub fn RANDOM() -> f32 {
    random()
}
