/**
 * @File   : matrix4.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 6/6/2019, 10:46:14 AM
 * @Description:
 */
use wasm_bindgen::prelude::*;

// use super::euler::*;
// use super::enums::*;
// use super::quaternion::*;
// use super::vector3::*;

fn clamp_f32(num: f32, min: f32, max: f32) -> f32 {
  if num < min {
    min
  } else if num > max {
    max
  } else {
    num
  }
}

#[wasm_bindgen]
pub struct Matrix4(
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32,
  pub f32
);

#[wasm_bindgen]
impl Matrix4 {
  pub fn create() -> Matrix4 {
    Matrix4(
      1., 0., 0., 0.,
      0., 1., 0., 0.,
      0., 0., 1., 0.,
      0., 0., 0., 1.
    )
  }

  #[wasm_bindgen(js_name="fromValues")]
  pub fn from_values(
    m11: f32,
    m12: f32,
    m13: f32,
    m14: f32,
    m21: f32,
    m22: f32,
    m23: f32,
    m24: f32,
    m31: f32,
    m32: f32,
    m33: f32,
    m34: f32,
    m41: f32,
    m42: f32,
    m43: f32,
    m44: f32,
  ) -> Matrix4 {
    Matrix4(
      m11, m12, m13, m14,
      m21, m22, m23, m24,
      m31, m32, m33, m34,
      m41, m42, m43, m44
    )
  }

  pub fn multiply(out: &mut Matrix4, a: &Matrix4, b: &Matrix4) {
    let a00 = a.0;
    let a01 = a.1;
    let a02 = a.2;
    let a03 = a.3;
    let a10 = a.4;
    let a11 = a.5;
    let a12 = a.6;
    let a13 = a.7;
    let a20 = a.8;
    let a21 = a.9;
    let a22 = a.10;
    let a23 = a.11;
    let a30 = a.12;
    let a31 = a.13;
    let a32 = a.14;
    let a33 = a.15;

    // Cache only the current line of the second matrix
    let b0  = b.0;
    let b1 = b.1;
    let b2 = b.2;
    let b3 = b.3;
    out.0 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
    out.1 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
    out.2 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
    out.3 = b0*a03 + b1*a13 + b2*a23 + b3*a33;

    let b0 = b.4;
    let b1 = b.5;
    let b2 = b.6;
    let b3 = b.7;
    out.4 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
    out.5 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
    out.6 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
    out.7 = b0*a03 + b1*a13 + b2*a23 + b3*a33;

    let b0 = b.8;
    let b1 = b.9;
    let b2 = b.10;
    let b3 = b.11;
    out.8 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
    out.9 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
    out.10 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
    out.11 = b0*a03 + b1*a13 + b2*a23 + b3*a33;

    let b0 = b.12;
    let b1 = b.13;
    let b2 = b.14;
    let b3 = b.15;
    out.12 = b0*a00 + b1*a10 + b2*a20 + b3*a30;
    out.13 = b0*a01 + b1*a11 + b2*a21 + b3*a31;
    out.14 = b0*a02 + b1*a12 + b2*a22 + b3*a32;
    out.15 = b0*a03 + b1*a13 + b2*a23 + b3*a33;
  }

  // #[wasm_bindgen(method, js_name = "fromTRS")]
  // pub fn from_trs(&mut self, transition: &Vector3, quat: &Quaternion, scale: &Vector3) {
  //   // Quaternion math
  //   let x = quat.x();
  //   let y = quat.y();
  //   let z = quat.z();
  //   let w = quat.w();
  //   let x2 = x + x;
  //   let y2 = y + y;
  //   let z2 = z + z;

  //   let xx = x * x2;
  //   let xy = x * y2;
  //   let xz = x * z2;
  //   let yy = y * y2;
  //   let yz = y * z2;
  //   let zz = z * z2;
  //   let wx = w * x2;
  //   let wy = w * y2;
  //   let wz = w * z2;
  //   let sx = scale.x();
  //   let sy = scale.y();
  //   let sz = scale.z();

  //   self.0 = (1. - (yy + zz)) * sx;
  //   self.1 = (xy + wz) * sx;
  //   self.2 = (xz - wy) * sx;
  //   self.3 = 0.;
  //   self.4 = (xy - wz) * sy;
  //   self.5 = (1. - (xx + zz)) * sy;
  //   self.6 = (yz + wx) * sy;
  //   self.7 = 0.;
  //   self.8 = (xz + wy) * sz;
  //   self.9 = (yz - wx) * sz;
  //   self.10 = (1. - (xx + yy)) * sz;
  //   self.11 = 0.;
  //   self.12 = transition.x();
  //   self.13 = transition.y();
  //   self.14 = transition.z();
  //   self.15 = 1.;
  // }

  // #[wasm_bindgen(method, js_name = "fromQuat")]
  // pub fn from_quat(&mut self, quat: &Quaternion) {
  //   let x = quat.x();
  //   let y = quat.y();
  //   let z = quat.z();
  //   let w = quat.w();
  //   let x2 = x + x;
  //   let y2 = y + y;
  //   let z2 = z + z;

  //   let xx = x * x2;
  //   let yx = y * x2;
  //   let yy = y * y2;
  //   let zx = z * x2;
  //   let zy = z * y2;
  //   let zz = z * z2;
  //   let wx = w * x2;
  //   let wy = w * y2;
  //   let wz = w * z2;

  //   self.0 = 1. - yy - zz;
  //   self.1 = yx + wz;
  //   self.2 = zx - wy;
  //   self.3 = 0.;

  //   self.4 = yx - wz;
  //   self.5 = 1. - xx - zz;
  //   self.6 = zy + wx;
  //   self.7 = 0.;

  //   self.8 = zx + wy;
  //   self.9 = zy - wx;
  //   self.10 = 1. - xx - yy;
  //   self.11 = 0.;

  //   self.12 = 0.;
  //   self.13 = 0.;
  //   self.14 = 0.;
  //   self.15 = 1.;
  // }

  // #[wasm_bindgen(method, js_name = "fromPerspective")]
  // pub fn from_perspective(&mut self, fov: f32, aspect: f32, near: f32, far: f32) {
  //   let f = 1.0 / f32::tan(fov / 2.);
  //   let nf = 1. / (near - far);
  //   self.0 = f / aspect;
  //   self.1 = 0.;
  //   self.2 = 0.;
  //   self.3 = 0.;
  //   self.4 = 0.;
  //   self.5 = f;
  //   self.6 = 0.;
  //   self.7 = 0.;
  //   self.8 = 0.;
  //   self.9 = 0.;
  //   self.10 = (far + near) * nf;
  //   self.11 = -1.;
  //   self.12 = 0.;
  //   self.13 = 0.;
  //   self.14 = (2. * far * near) * nf;
  //   self.15 = 0.;
  // }

  // #[wasm_bindgen(method, js_name = "buildViewMatrix")]
  // pub fn build_view_matrix(&mut self, model_matrix: &mut Matrix4) {
  //   let t = &mut Vector3::zero();
  //   model_matrix.get_transition(t);
  //   model_matrix.get_transpose_matrix(self);
  //   let rx = &mut Vector3::new(self.0, self.1, self.2);
  //   let ry = &mut Vector3::new(self.4, self.5, self.6);
  //   let rz = &mut Vector3::new(self.8, self.9, self.10);

  //   self.12 = -rx.dot(t);
  //   self.13 = -ry.dot(t);
  //   self.14 = -rz.dot(t);
  //   self.3 = 0.;
  //   self.7 = 0.;
  //   self.11 = 0.;
  //   self.15 = 1.;
  // }

  // #[wasm_bindgen(method, js_name = "transpose")]
  // pub fn get_transpose_matrix(&mut self, matrix: &mut Matrix4) {
  //   let a01 = self.1;
  //   let a02 = self.2;
  //   let a03 = self.3;
  //   let a12 = self.6;
  //   let a13 = self.7;
  //   let a23 = self.11;

  //   matrix.1 = self.4;
  //   matrix.2 = self.8;
  //   matrix.3 = self.12;
  //   matrix.4 = a01;
  //   matrix.6 = self.9;
  //   matrix.7 = self.13;
  //   matrix.8 = a02;
  //   matrix.9 = a12;
  //   matrix.11 = self.14;
  //   matrix.12 = a03;
  //   matrix.13 = a13;
  //   matrix.14 = a23;
  // }

  // #[wasm_bindgen(method, js_name = "getInverseMatrix")]
  // pub fn get_inverse_matrix(&mut self, matrix: &mut Matrix4) {
  //   let a00 = self.0;
  //   let a01 = self.1;
  //   let a02 = self.2;
  //   let a03 = self.3;
  //   let a10 = self.4;
  //   let a11 = self.5;
  //   let a12 = self.6;
  //   let a13 = self.7;
  //   let a20 = self.8;
  //   let a21 = self.9;
  //   let a22 = self.10;
  //   let a23 = self.11;
  //   let a30 = self.12;
  //   let a31 = self.13;
  //   let a32 = self.14;
  //   let a33 = self.15;

  //   let b00 = a00 * a11 - a01 * a10;
  //   let b01 = a00 * a12 - a02 * a10;
  //   let b02 = a00 * a13 - a03 * a10;
  //   let b03 = a01 * a12 - a02 * a11;
  //   let b04 = a01 * a13 - a03 * a11;
  //   let b05 = a02 * a13 - a03 * a12;
  //   let b06 = a20 * a31 - a21 * a30;
  //   let b07 = a20 * a32 - a22 * a30;
  //   let b08 = a20 * a33 - a23 * a30;
  //   let b09 = a21 * a32 - a22 * a31;
  //   let b10 = a21 * a33 - a23 * a31;
  //   let b11 = a22 * a33 - a23 * a32;

  //   let det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;
  //   let det = 1.0 / det;

  //   matrix.0 = (a11 * b11 - a12 * b10 + a13 * b09) * det;
  //   matrix.1 = (a02 * b10 - a01 * b11 - a03 * b09) * det;
  //   matrix.2 = (a31 * b05 - a32 * b04 + a33 * b03) * det;
  //   matrix.3 = (a22 * b04 - a21 * b05 - a23 * b03) * det;
  //   matrix.4 = (a12 * b08 - a10 * b11 - a13 * b07) * det;
  //   matrix.5 = (a00 * b11 - a02 * b08 + a03 * b07) * det;
  //   matrix.6 = (a32 * b02 - a30 * b05 - a33 * b01) * det;
  //   matrix.7 = (a20 * b05 - a22 * b02 + a23 * b01) * det;
  //   matrix.8 = (a10 * b10 - a11 * b08 + a13 * b06) * det;
  //   matrix.9 = (a01 * b08 - a00 * b10 - a03 * b06) * det;
  //   matrix.10 = (a30 * b04 - a31 * b02 + a33 * b00) * det;
  //   matrix.11 = (a21 * b02 - a20 * b04 - a23 * b00) * det;
  //   matrix.12 = (a11 * b07 - a10 * b09 - a12 * b06) * det;
  //   matrix.13 = (a00 * b09 - a01 * b07 + a02 * b06) * det;
  //   matrix.14 = (a31 * b01 - a30 * b03 - a32 * b00) * det;
  //   matrix.15 = (a20 * b03 - a21 * b01 + a22 * b00) * det;
  // }

  // #[wasm_bindgen(method, js_name = "getTransition")]
  // pub fn get_transition(&mut self, transition: &mut Vector3) {
  //   transition.set(self.12, self.13, self.14)
  // }

  // #[wasm_bindgen(method, js_name = "getRotation")]
  // pub fn get_rotation(&mut self, rotation: &mut Euler) {
  //   let m11 = self.0;
  //   let m21 = self.1;
  //   let m31 = self.2;
  //   let m12 = self.4;
  //   let m22 = self.5;
  //   let m32 = self.6;
  //   let m13 = self.8;
  //   let m23 = self.9;
  //   let m33 = self.10;

  //   match rotation.order() {
  //     EEulerOrder::XYZ => {
  //       rotation.set_y(f32::asin(clamp_f32(m13, -1., 1.)));
  //       if f32::abs(m13) < 0.99999 {
  //         rotation.set_x(f32::atan2(-m23, m33));
  //         rotation.set_z(f32::atan2(-m12, m11));
  //       } else {
  //         rotation.set_x(f32::atan2(m32, m22));
  //         rotation.set_z(0.);
  //       }
  //     }
  //     EEulerOrder::YXZ => {
  //       rotation.set_x(f32::asin(-clamp_f32(m23, -1., 1.)));
  //       if f32::abs(m23) < 0.99999 {
  //         rotation.set_y(f32::atan2(m13, m33));
  //         rotation.set_z(f32::atan2(m21, m22));
  //       } else {
  //         rotation.set_y(f32::atan2(-m31, m11));
  //         rotation.set_z(0.);
  //       }
  //     }
  //     EEulerOrder::ZXY => {
  //       rotation.set_x(f32::asin(clamp_f32(m32, -1., 1.)));
  //       if f32::abs(m32) < 0.99999 {
  //         rotation.set_y(f32::atan2(-m31, m33));
  //         rotation.set_z(f32::atan2(-m12, m22));
  //       } else {
  //         rotation.set_y(0.);
  //         rotation.set_z(f32::atan2(m21, m11));
  //       }
  //     }
  //     EEulerOrder::ZYX => {
  //       rotation.set_y(f32::asin(-clamp_f32(m31, -1., 1.)));
  //       if f32::abs(m31) < 0.99999 {
  //         rotation.set_x(f32::atan2(m32, m33));
  //         rotation.set_z(f32::atan2(m21, m11));
  //       } else {
  //         rotation.set_x(0.);
  //         rotation.set_z(f32::atan2(-m12, m22));
  //       }
  //     }
  //     EEulerOrder::YZX => {
  //       rotation.set_z(f32::asin(clamp_f32(m21, -1., 1.)));
  //       if f32::abs(m21) < 0.99999 {
  //         rotation.set_x(f32::atan2(-m23, m22));
  //         rotation.set_y(f32::atan2(-m31, m11));
  //       } else {
  //         rotation.set_x(0.);
  //         rotation.set_y(f32::atan2(m13, m33));
  //       }
  //     }
  //     EEulerOrder::XZY => {
  //       rotation.set_z(f32::asin(-clamp_f32(m12, -1., 1.)));
  //       if f32::abs(m12) < 0.99999 {
  //         rotation.set_x(f32::atan2(m32, m22));
  //         rotation.set_y(f32::atan2(m13, m11));
  //       } else {
  //         rotation.set_x(f32::atan2(-m23, m33));
  //         rotation.set_y(0.);
  //       }
  //     }
  //   }
  // }

  // #[wasm_bindgen(method, js_name = "getScale")]
  // pub fn get_scale(&mut self, scale: &mut Vector3) {
  //   let m11 = self.0;
  //   let m12 = self.1;
  //   let m13 = self.2;
  //   let m21 = self.4;
  //   let m22 = self.5;
  //   let m23 = self.6;
  //   let m31 = self.8;
  //   let m32 = self.9;
  //   let m33 = self.10;

  //   scale.set(
  //     (m11.powi(2) + m12.powi(2) + m13.powi(2)).sqrt(),
  //     (m21.powi(2) + m22.powi(2) + m23.powi(2)).sqrt(),
  //     (m31.powi(2) + m32.powi(2) + m33.powi(2)).sqrt(),
  //   );
  // }
}
