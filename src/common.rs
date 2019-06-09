/**
 * @File   : common.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : http://dtysky.moe
 * @Date   : 2019/2/7 下午9:53:03
 */
use wasm_bindgen::prelude::*;

pub static EPSILON: f32 = 0.0001;

pub static PI: f32 = 3.141592653589793;

pub static INFINITY: f32 = 1.0_f32 / 0.0_f32;

pub static NEG_INFINITY: f32 = -1.0_f32 / 0.0_f32;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = Math)]
    fn random() -> f32;
}

pub fn RANDOM() -> f32 {
    random()
}
