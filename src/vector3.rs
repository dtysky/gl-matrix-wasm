/**
 * @File   : vector3.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : dtysky.moe
 * @Date   : 2019/6/4 下午10:47:35
 */
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Vector3(pub f32, pub f32, pub f32);

#[wasm_bindgen]
impl Vector3 {
  pub fn create() -> Vector3 {
    Vector3(0., 0., 0.)
  }

  #[wasm_bindgen(js_name="fromValues")]
  pub fn from_values(x: f32, y: f32, z: f32) -> Vector3 {
    Vector3(x, y, z)
  }

  pub fn add(out: &mut Vector3, a: &Vector3, b: &Vector3) {
    out.0 = a.0 + b.0;
    out.1 = a.1 + b.1;
    out.2 = a.2 + b.2;
  }
}
