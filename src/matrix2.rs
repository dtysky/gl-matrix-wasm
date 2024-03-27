/**
 * @File   : matrix2.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : http://dtysky.moe
 * @Date   : 2019/2/7 下午9:53:03
 */
use wasm_bindgen::prelude::*;

use super::common::*;
use super::vector2::*;

#[wasm_bindgen]
pub struct Matrix2(pub f32, pub f32, pub f32, pub f32);

#[wasm_bindgen]
impl Matrix2 {
    #[wasm_bindgen(getter)]
    pub fn elements(&self) -> Box<[f32]> {
        Box::new([self.0, self.1, self.2, self.3])
    }

    pub fn create() -> Self {
        Matrix2(1., 0., 0., 1.)
    }

    pub fn clone(a: &Matrix2) -> Self {
        Matrix2(a.0, a.1, a.2, a.3)
    }

    pub fn copy(out: &mut Matrix2, a: &Matrix2) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.3;
    }

    pub fn identity(out: &mut Matrix2) {
        out.0 = 1.;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = 1.;
    }

    pub fn fromValues(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
        Matrix2(m00, m01, m10, m11)
    }

    pub fn set(out: &mut Matrix2, m00: f32, m01: f32, m10: f32, m11: f32) {
        out.0 = m00;
        out.1 = m01;
        out.2 = m10;
        out.3 = m11;
    }

    pub fn transpose(out: &mut Matrix2, a: &Matrix2) {
        // If we are transposing ourselves we can skip a few steps but have to cache
        // some values
        if std::ptr::eq(out, a) {
            let a1 = a.1;
            out.1 = a.2;
            out.2 = a1;
        } else {
            out.0 = a.0;
            out.1 = a.2;
            out.2 = a.1;
            out.3 = a.3;
        }
    }

    pub fn invert(out: &mut Matrix2, a: &Matrix2) {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;

        // Calculate the determinant
        let det = a0 * a3 - a2 * a1;

        if det == 0. {
            return;
        }

        let det = 1.0 / det;

        out.0 = a3 * det;
        out.1 = -a1 * det;
        out.2 = -a2 * det;
        out.3 = a0 * det;
    }

    pub fn adjoint(out: &mut Matrix2, a: &Matrix2) {
        // Caching this value is nessecary if out == a
        if std::ptr::eq(out, a) {
            let a = Matrix2::clone(a);
            out.0 = a.3;
            out.1 = -a.1;
            out.2 = -a.2;
            out.3 = a.0;
            return;
        }

        out.0 = a.3;
        out.1 = -a.1;
        out.2 = -a.2;
        out.3 = a.0;

        // console_log!("{} {} {} {}", out.0, out.1, out.2, out.3);
    }

    pub fn determinant(a: &mut Matrix2) -> f32 {
        a.0 * a.3 - a.2 * a.1
    }

    pub fn multiply(out: &mut Matrix2, a: &Matrix2, b: &Matrix2) {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let b0 = b.0;
        let b1 = b.1;
        let b2 = b.2;
        let b3 = b.3;
        out.0 = a0 * b0 + a2 * b1;
        out.1 = a1 * b0 + a3 * b1;
        out.2 = a0 * b2 + a2 * b3;
        out.3 = a1 * b2 + a3 * b3;
    }

    pub fn rotate(out: &mut Matrix2, a: &Matrix2, rad: f32) {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let s = f32::sin(rad);
        let c = f32::cos(rad);
        out.0 = a0 * c + a2 * s;
        out.1 = a1 * c + a3 * s;
        out.2 = a0 * -s + a2 * c;
        out.3 = a1 * -s + a3 * c;
    }

    pub fn scale(out: &mut Matrix2, a: &Matrix2, v: &Vector2) {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let v0 = v.0;
        let v1 = v.1;
        out.0 = a0 * v0;
        out.1 = a1 * v0;
        out.2 = a2 * v1;
        out.3 = a3 * v1;
    }

    pub fn fromRotation(out: &mut Matrix2, rad: f32) {
        let s = f32::sin(rad);
        let c = f32::cos(rad);
        out.0 = c;
        out.1 = s;
        out.2 = -s;
        out.3 = c;
    }

    pub fn fromScaling(out: &mut Matrix2, v: &Vector2) {
        out.0 = v.0;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = v.1;
    }

    //   pub fn str(a: &Matrix2) -> String {
    //     "mat2(" + a.0 + ", " + a.1 + ", " + a.2 + ", " + a.3 + ")"
    //   }

    pub fn frob(a: &Matrix2) -> f32 {
        (a.0.powi(2) + a.1.powi(2) + a.2.powi(2) + a.3.powi(2)).sqrt()
    }

    pub fn LDU(L: &mut Matrix2, _D: &mut Matrix2, U: &mut Matrix2, a: &Matrix2) {
        L.2 = a.2 / a.0;
        U.0 = a.0;
        U.1 = a.1;
        U.3 = a.3 - L.2 * U.1;
    }

    pub fn add(out: &mut Matrix2, a: &Matrix2, b: &Matrix2) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
        out.2 = a.2 + b.2;
        out.3 = a.3 + b.3;
    }

    pub fn subtract(out: &mut Matrix2, a: &Matrix2, b: &Matrix2) {
        out.0 = a.0 - b.0;
        out.1 = a.1 - b.1;
        out.2 = a.2 - b.2;
        out.3 = a.3 - b.3;
    }

    pub fn exactEquals(a: &Matrix2, b: &Matrix2) -> bool {
        a.0 == b.0 && a.1 == b.1 && a.2 == b.2 && a.3 == b.3
    }

    pub fn equals(a: &Matrix2, b: &Matrix2) -> bool {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let b0 = b.0;
        let b1 = b.1;
        let b2 = b.2;
        let b3 = b.3;

        f32::abs(a0 - b0) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a0), f32::abs(b0)))
            && f32::abs(a1 - b1) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a1), f32::abs(b1)))
            && f32::abs(a2 - b2) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a2), f32::abs(b2)))
            && f32::abs(a3 - b3) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a3), f32::abs(b3)))
    }

    pub fn multiplyScalar(out: &mut Matrix2, a: &Matrix2, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
        out.2 = a.2 * b;
        out.3 = a.3 * b;
    }

    pub fn multiplyScalarAndAdd(out: &mut Matrix2, a: &Matrix2, b: &Matrix2, scale: f32) {
        out.0 = a.0 + (b.0 * scale);
        out.1 = a.1 + (b.1 * scale);
        out.2 = a.2 + (b.2 * scale);
        out.3 = a.3 + (b.3 * scale);
    }

    pub fn mul(out: &mut Matrix2, a: &Matrix2, b: &Matrix2) {
        Matrix2::multiply(out, a, b);
    }

    pub fn sub(out: &mut Matrix2, a: &Matrix2, b: &Matrix2) {
        Matrix2::subtract(out, a, b);
    }
}
