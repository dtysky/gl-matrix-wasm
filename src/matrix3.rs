/**
 * @File   : matrix3.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : http://dtysky.moe
 * @Date   : 2019/2/7 下午9:53:03
 */
use wasm_bindgen::prelude::*;

use super::common::*;
use super::matrix2d::*;
use super::matrix4::*;
use super::quaternion::*;
use super::vector2::*;

#[wasm_bindgen]
pub struct Matrix3(
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
    pub f32,
);

#[wasm_bindgen]
impl Matrix3 {
    #[wasm_bindgen(getter)]
    pub fn elements(&self) -> Box<[f32]> {
        Box::new([
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7, self.8,
        ])
    }

    pub fn create() -> Matrix3 {
        Matrix3(1., 0., 0., 0., 1., 0., 0., 0., 1.)
    }

    pub fn fromMat4(out: &mut Matrix3, a: &Matrix4) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.4;
        out.4 = a.5;
        out.5 = a.6;
        out.6 = a.8;
        out.7 = a.9;
        out.8 = a.10;
    }

    pub fn clone(a: &Matrix3) -> Matrix3 {
        Matrix3(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7, a.8)
    }

    pub fn copy(out: &mut Matrix3, a: &Matrix3) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.3;
        out.4 = a.4;
        out.5 = a.5;
        out.6 = a.6;
        out.7 = a.7;
        out.8 = a.8;
    }

    pub fn fromValues(
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) -> Matrix3 {
        Matrix3(m00, m01, m02, m10, m11, m12, m20, m21, m22)
    }

    pub fn set(
        out: &mut Matrix3,
        m00: f32,
        m01: f32,
        m02: f32,
        m10: f32,
        m11: f32,
        m12: f32,
        m20: f32,
        m21: f32,
        m22: f32,
    ) {
        out.0 = m00;
        out.1 = m01;
        out.2 = m02;
        out.3 = m10;
        out.4 = m11;
        out.5 = m12;
        out.6 = m20;
        out.7 = m21;
        out.8 = m22;
    }

    pub fn identity(out: &mut Matrix3) {
        out.0 = 1.;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = 0.;
        out.4 = 1.;
        out.5 = 0.;
        out.6 = 0.;
        out.7 = 0.;
        out.8 = 1.;
    }

    pub fn transpose(out: &mut Matrix3, a: &Matrix3) {
        // If we are transposing ourselves we can skip a few steps but have to cache some values
        if (out as *const Matrix3) == (a as *const Matrix3) {
            let a01 = a.1;
            let a02 = a.2;
            let a12 = a.5;
            out.1 = a.3;
            out.2 = a.6;
            out.3 = a01;
            out.5 = a.7;
            out.6 = a02;
            out.7 = a12;
        } else {
            out.0 = a.0;
            out.1 = a.3;
            out.2 = a.6;
            out.3 = a.1;
            out.4 = a.4;
            out.5 = a.7;
            out.6 = a.2;
            out.7 = a.5;
            out.8 = a.8;
        }
    }

    pub fn invert(out: &mut Matrix3, a: &Matrix3) {
        let a00 = a.0;
        let a01 = a.1;
        let a02 = a.2;
        let a10 = a.3;
        let a11 = a.4;
        let a12 = a.5;
        let a20 = a.6;
        let a21 = a.7;
        let a22 = a.8;

        let b01 = a22 * a11 - a12 * a21;
        let b11 = -a22 * a10 + a12 * a20;
        let b21 = a21 * a10 - a11 * a20;

        // Calculate the determinant
        let mut det = a00 * b01 + a01 * b11 + a02 * b21;

        if (det < EPSILON) {
            return;
        }
        det = 1.0 / det;

        out.0 = b01 * det;
        out.1 = (-a22 * a01 + a02 * a21) * det;
        out.2 = (a12 * a01 - a02 * a11) * det;
        out.3 = b11 * det;
        out.4 = (a22 * a00 - a02 * a20) * det;
        out.5 = (-a12 * a00 + a02 * a10) * det;
        out.6 = b21 * det;
        out.7 = (-a21 * a00 + a01 * a20) * det;
        out.8 = (a11 * a00 - a01 * a10) * det;
    }

    pub fn adjoint(out: &mut Matrix3, a: &Matrix3) {
        let a00 = a.0;
        let a01 = a.1;
        let a02 = a.2;
        let a10 = a.3;
        let a11 = a.4;
        let a12 = a.5;
        let a20 = a.6;
        let a21 = a.7;
        let a22 = a.8;

        out.0 = (a11 * a22 - a12 * a21);
        out.1 = (a02 * a21 - a01 * a22);
        out.2 = (a01 * a12 - a02 * a11);
        out.3 = (a12 * a20 - a10 * a22);
        out.4 = (a00 * a22 - a02 * a20);
        out.5 = (a02 * a10 - a00 * a12);
        out.6 = (a10 * a21 - a11 * a20);
        out.7 = (a01 * a20 - a00 * a21);
        out.8 = (a00 * a11 - a01 * a10);
    }

    pub fn determinant(a: &Matrix3) -> f32 {
        let a00 = a.0;
        let a01 = a.1;
        let a02 = a.2;
        let a10 = a.3;
        let a11 = a.4;
        let a12 = a.5;
        let a20 = a.6;
        let a21 = a.7;
        let a22 = a.8;

        a00 * (a22 * a11 - a12 * a21)
            + a01 * (-a22 * a10 + a12 * a20)
            + a02 * (a21 * a10 - a11 * a20)
    }

    pub fn multiply(out: &mut Matrix3, a: &Matrix3, b: &Matrix3) {
        let a00 = a.0;
        let a01 = a.1;
        let a02 = a.2;
        let a10 = a.3;
        let a11 = a.4;
        let a12 = a.5;
        let a20 = a.6;
        let a21 = a.7;
        let a22 = a.8;

        let b00 = b.0;
        let b01 = b.1;
        let b02 = b.2;
        let b10 = b.3;
        let b11 = b.4;
        let b12 = b.5;
        let b20 = b.6;
        let b21 = b.7;
        let b22 = b.8;

        out.0 = b00 * a00 + b01 * a10 + b02 * a20;
        out.1 = b00 * a01 + b01 * a11 + b02 * a21;
        out.2 = b00 * a02 + b01 * a12 + b02 * a22;

        out.3 = b10 * a00 + b11 * a10 + b12 * a20;
        out.4 = b10 * a01 + b11 * a11 + b12 * a21;
        out.5 = b10 * a02 + b11 * a12 + b12 * a22;

        out.6 = b20 * a00 + b21 * a10 + b22 * a20;
        out.7 = b20 * a01 + b21 * a11 + b22 * a21;
        out.8 = b20 * a02 + b21 * a12 + b22 * a22;
    }

    pub fn translate(out: &mut Matrix3, a: &Matrix3, v: &Vector2) {
        let a00 = a.0;
        let a01 = a.1;
        let a02 = a.2;
        let a10 = a.3;
        let a11 = a.4;
        let a12 = a.5;
        let a20 = a.6;
        let a21 = a.7;
        let a22 = a.8;
        let x = v.0;
        let y = v.1;

        out.0 = a00;
        out.1 = a01;
        out.2 = a02;

        out.3 = a10;
        out.4 = a11;
        out.5 = a12;

        out.6 = x * a00 + y * a10 + a20;
        out.7 = x * a01 + y * a11 + a21;
        out.8 = x * a02 + y * a12 + a22;
    }

    pub fn rotate(out: &mut Matrix3, a: &Matrix3, rad: f32) {
        let a00 = a.0;
        let a01 = a.1;
        let a02 = a.2;
        let a10 = a.3;
        let a11 = a.4;
        let a12 = a.5;
        let a20 = a.6;
        let a21 = a.7;
        let a22 = a.8;
        let s = f32::sin(rad);
        let c = f32::cos(rad);

        out.0 = c * a00 + s * a10;
        out.1 = c * a01 + s * a11;
        out.2 = c * a02 + s * a12;

        out.3 = c * a10 - s * a00;
        out.4 = c * a11 - s * a01;
        out.5 = c * a12 - s * a02;

        out.6 = a20;
        out.7 = a21;
        out.8 = a22;
    }

    pub fn scale(out: &mut Matrix3, a: &Matrix3, v: &Vector2) {
        let x = v.0;
        let y = v.1;

        out.0 = x * a.0;
        out.1 = x * a.1;
        out.2 = x * a.2;

        out.3 = y * a.3;
        out.4 = y * a.4;
        out.5 = y * a.5;

        out.6 = a.6;
        out.7 = a.7;
        out.8 = a.8;
    }

    pub fn fromTranslation(out: &mut Matrix3, v: &Vector2) {
        out.0 = 1.;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = 0.;
        out.4 = 1.;
        out.5 = 0.;
        out.6 = v.0;
        out.7 = v.1;
        out.8 = 1.;
    }

    pub fn fromRotation(out: &mut Matrix3, rad: f32) {
        let s = f32::sin(rad);
        let c = f32::cos(rad);

        out.0 = c;
        out.1 = s;
        out.2 = 0.;

        out.3 = -s;
        out.4 = c;
        out.5 = 0.;

        out.6 = 0.;
        out.7 = 0.;
        out.8 = 1.;
    }

    pub fn fromScaling(out: &mut Matrix3, v: &Vector2) {
        out.0 = v.0;
        out.1 = 0.;
        out.2 = 0.;

        out.3 = 0.;
        out.4 = v.1;
        out.5 = 0.;

        out.6 = 0.;
        out.7 = 0.;
        out.8 = 1.;
    }

    pub fn fromMat2d(out: &mut Matrix3, a: &Matrix2d) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = 0.;

        out.3 = a.2;
        out.4 = a.3;
        out.5 = 0.;

        out.6 = a.4;
        out.7 = a.5;
        out.8 = 1.;
    }

    pub fn fromQuat(out: &mut Matrix3, q: &Quaternion) {
        let x = q.0;
        let y = q.1;
        let z = q.2;
        let w = q.3;
        let x2 = x + x;
        let y2 = y + y;
        let z2 = z + z;

        let xx = x * x2;
        let yx = y * x2;
        let yy = y * y2;
        let zx = z * x2;
        let zy = z * y2;
        let zz = z * z2;
        let wx = w * x2;
        let wy = w * y2;
        let wz = w * z2;

        out.0 = 1. - yy - zz;
        out.3 = yx - wz;
        out.6 = zx + wy;

        out.1 = yx + wz;
        out.4 = 1. - xx - zz;
        out.7 = zy - wx;

        out.2 = zx - wy;
        out.5 = zy + wx;
        out.8 = 1. - xx - yy;
    }

    pub fn normalFromMat4(out: &mut Matrix3, a: &Matrix4) {
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

        let b00 = a00 * a11 - a01 * a10;
        let b01 = a00 * a12 - a02 * a10;
        let b02 = a00 * a13 - a03 * a10;
        let b03 = a01 * a12 - a02 * a11;
        let b04 = a01 * a13 - a03 * a11;
        let b05 = a02 * a13 - a03 * a12;
        let b06 = a20 * a31 - a21 * a30;
        let b07 = a20 * a32 - a22 * a30;
        let b08 = a20 * a33 - a23 * a30;
        let b09 = a21 * a32 - a22 * a31;
        let b10 = a21 * a33 - a23 * a31;
        let b11 = a22 * a33 - a23 * a32;

        // Calculate the determinant
        let mut det = b00 * b11 - b01 * b10 + b02 * b09 + b03 * b08 - b04 * b07 + b05 * b06;

        if (det < EPSILON) {
            return;
        }
        det = 1.0 / det;

        out.0 = (a11 * b11 - a12 * b10 + a13 * b09) * det;
        out.1 = (a12 * b08 - a10 * b11 - a13 * b07) * det;
        out.2 = (a10 * b10 - a11 * b08 + a13 * b06) * det;

        out.3 = (a02 * b10 - a01 * b11 - a03 * b09) * det;
        out.4 = (a00 * b11 - a02 * b08 + a03 * b07) * det;
        out.5 = (a01 * b08 - a00 * b10 - a03 * b06) * det;

        out.6 = (a31 * b05 - a32 * b04 + a33 * b03) * det;
        out.7 = (a32 * b02 - a30 * b05 - a33 * b01) * det;
        out.8 = (a30 * b04 - a31 * b02 + a33 * b00) * det;
    }

    pub fn projection(out: &mut Matrix3, width: f32, height: f32) {
        out.0 = 2. / width;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = 0.;
        out.4 = -2. / height;
        out.5 = 0.;
        out.6 = -1.;
        out.7 = 1.;
        out.8 = 1.;
    }

    // pub fn str(a: &Matrix3) {
    //   return "mat3(" + a.0 + ", " + a.1 + ", " + a.2 + ", " +
    //           a.3 + ", " + a.4 + ", " + a.5 + ", " +
    //           a.6 + ", " + a.7 + ", " + a.8 + ")";
    // }

    pub fn frob(a: &Matrix3) -> f32 {
        (a.0.powi(2)
            + a.1.powi(2)
            + a.2.powi(2)
            + a.3.powi(2)
            + a.4.powi(2)
            + a.5.powi(2)
            + a.6.powi(2)
            + a.7.powi(2)
            + a.8.powi(2))
        .sqrt()
    }

    pub fn add(out: &mut Matrix3, a: &Matrix3, b: &Matrix3) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
        out.2 = a.2 + b.2;
        out.3 = a.3 + b.3;
        out.4 = a.4 + b.4;
        out.5 = a.5 + b.5;
        out.6 = a.6 + b.6;
        out.7 = a.7 + b.7;
        out.8 = a.8 + b.8;
    }

    pub fn subtract(out: &mut Matrix3, a: &Matrix3, b: &Matrix3) {
        out.0 = a.0 - b.0;
        out.1 = a.1 - b.1;
        out.2 = a.2 - b.2;
        out.3 = a.3 - b.3;
        out.4 = a.4 - b.4;
        out.5 = a.5 - b.5;
        out.6 = a.6 - b.6;
        out.7 = a.7 - b.7;
        out.8 = a.8 - b.8;
    }

    pub fn multiplyScalar(out: &mut Matrix3, a: &Matrix3, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
        out.2 = a.2 * b;
        out.3 = a.3 * b;
        out.4 = a.4 * b;
        out.5 = a.5 * b;
        out.6 = a.6 * b;
        out.7 = a.7 * b;
        out.8 = a.8 * b;
    }

    pub fn multiplyScalarAndAdd(out: &mut Matrix3, a: &Matrix3, b: &Matrix3, scale: f32) {
        out.0 = a.0 + (b.0 * scale);
        out.1 = a.1 + (b.1 * scale);
        out.2 = a.2 + (b.2 * scale);
        out.3 = a.3 + (b.3 * scale);
        out.4 = a.4 + (b.4 * scale);
        out.5 = a.5 + (b.5 * scale);
        out.6 = a.6 + (b.6 * scale);
        out.7 = a.7 + (b.7 * scale);
        out.8 = a.8 + (b.8 * scale);
    }

    pub fn exactEquals(a: &Matrix3, b: &Matrix3) -> bool {
        a.0 == b.0
            && a.1 == b.1
            && a.2 == b.2
            && a.3 == b.3
            && a.4 == b.4
            && a.5 == b.5
            && a.6 == b.6
            && a.7 == b.7
            && a.8 == b.8
    }

    pub fn equals(a: &Matrix3, b: &Matrix3) -> bool {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let a4 = a.4;
        let a5 = a.5;
        let a6 = a.6;
        let a7 = a.7;
        let a8 = a.8;
        let b0 = b.0;
        let b1 = b.1;
        let b2 = b.2;
        let b3 = b.3;
        let b4 = b.4;
        let b5 = b.5;
        let b6 = b.6;
        let b7 = b.7;
        let b8 = b.8;
        f32::abs(a0 - b0) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a0), f32::abs(b0)))
            && f32::abs(a1 - b1) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a1), f32::abs(b1)))
            && f32::abs(a2 - b2) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a2), f32::abs(b2)))
            && f32::abs(a3 - b3) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a3), f32::abs(b3)))
            && f32::abs(a4 - b4) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a4), f32::abs(b4)))
            && f32::abs(a5 - b5) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a5), f32::abs(b5)))
            && f32::abs(a6 - b6) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a6), f32::abs(b6)))
            && f32::abs(a7 - b7) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a7), f32::abs(b7)))
            && f32::abs(a8 - b8) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a8), f32::abs(b8)))
    }

    pub fn mul(out: &mut Matrix3, a: &Matrix3, b: &Matrix3) {
        Matrix3::multiply(out, a, b);
    }

    pub fn sub(out: &mut Matrix3, a: &Matrix3, b: &Matrix3) {
        Matrix3::subtract(out, a, b);
    }
}
