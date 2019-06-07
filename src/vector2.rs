/**
 * @File   : vector2.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : http://dtysky.moe
 * @Date   : 2019/2/7 下午9:53:03
 */
use wasm_bindgen::prelude::*;

use super::common::*;
use super::matrix2::*;
use super::matrix2d::*;
use super::matrix3::*;
use super::matrix4::*;
use super::vector3::*;

#[wasm_bindgen]
pub struct Vector2(pub f32, pub f32);

#[wasm_bindgen]
impl Vector2 {
    #[wasm_bindgen(getter)]
    pub fn elements(&self) -> Box<[f32]> {
        Box::new([self.0, self.1])
    }

    pub fn create() -> Vector2 {
        Vector2(0., 0.)
    }

    pub fn clone(a: &Vector2) -> Vector2 {
        Vector2(a.0, a.1)
    }

    pub fn fromValues(x: f32, y: f32) -> Vector2 {
        Vector2(x, y)
    }

    pub fn copy(out: &mut Vector2, a: &Vector2) {
        out.0 = a.0;
        out.1 = a.1;
    }

    pub fn set(out: &mut Vector2, x: f32, y: f32) {
        out.0 = x;
        out.1 = y;
    }

    pub fn add(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
    }

    pub fn subtract(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = a.0 - b.0;
        out.1 = a.1 - b.1;
    }

    pub fn multiply(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = a.0 * b.0;
        out.1 = a.1 * b.1;
    }

    pub fn divide(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = a.0 / b.0;
        out.1 = a.1 / b.1;
    }

    pub fn ceil(out: &mut Vector2, a: &Vector2) {
        out.0 = f32::ceil(a.0);
        out.1 = f32::ceil(a.1);
    }

    pub fn floor(out: &mut Vector2, a: &Vector2) {
        out.0 = f32::floor(a.0);
        out.1 = f32::floor(a.1);
    }

    pub fn min(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = f32::min(a.0, b.0);
        out.1 = f32::min(a.1, b.1);
    }

    pub fn max(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = f32::max(a.0, b.0);
        out.1 = f32::max(a.1, b.1);
    }

    pub fn round(out: &mut Vector2, a: &Vector2) {
        out.0 = f32::round(a.0);
        out.1 = f32::round(a.1);
    }

    pub fn scale(out: &mut Vector2, a: &Vector2, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
    }

    pub fn scaleAndAdd(out: &mut Vector2, a: &Vector2, b: &Vector2, scale: f32) {
        out.0 = a.0 + (b.0 * scale);
        out.1 = a.1 + (b.1 * scale);
    }

    pub fn distance(a: &Vector2, b: &Vector2) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        f32::hypot(x, y)
    }

    pub fn squaredDistance(a: &Vector2, b: &Vector2) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        x * x + y * y
    }

    pub fn len(a: &Vector2) -> f32 {
        let x = a.0;
        let y = a.1;
        f32::hypot(x, y)
    }

    pub fn squaredLength(a: &Vector2) -> f32 {
        let x = a.0;
        let y = a.1;
        x * x + y * y
    }

    pub fn negate(out: &mut Vector2, a: &Vector2) {
        out.0 = -a.0;
        out.1 = -a.1;
    }

    pub fn inverse(out: &mut Vector2, a: &Vector2) {
        out.0 = 1.0 / a.0;
        out.1 = 1.0 / a.1;
    }

    pub fn normalize(out: &mut Vector2, a: &Vector2) {
        let x = a.0;
        let y = a.1;
        let mut len = x * x + y * y;
        if (len > EPSILON) {
            //TODO: evaluate use of glm_invsqrt here?
            len = 1. / f32::sqrt(len);
        }
        out.0 = a.0 * len;
        out.1 = a.1 * len;
    }

    pub fn dot(a: &Vector2, b: &Vector2) -> f32 {
        a.0 * b.0 + a.1 * b.1
    }

    pub fn cross(out: &mut Vector3, a: &Vector2, b: &Vector2) {
        let z = a.0 * b.1 - a.1 * b.0;
        out.0 = 0.;
        out.1 = 0.;
        out.2 = z;
    }

    pub fn lerp(out: &mut Vector2, a: &Vector2, b: &Vector2, t: f32) {
        let ax = a.0;
        let ay = a.1;
        out.0 = ax + t * (b.0 - ax);
        out.1 = ay + t * (b.1 - ay);
    }

    pub fn random(out: &mut Vector2, scale: Option<f32>) {
        let mut s = 1.;
        match scale {
            Some(value) => s = value,
            None => {}
        };
        let scale = s;
        let r = RANDOM() * 2.0 * PI;
        out.0 = f32::cos(r) * scale;
        out.1 = f32::sin(r) * scale;
    }

    pub fn transformMat2(out: &mut Vector2, a: &Vector2, m: &Matrix2) {
        let x = a.0;
        let y = a.1;
        out.0 = m.0 * x + m.2 * y;
        out.1 = m.1 * x + m.3 * y;
    }

    pub fn transformMat2d(out: &mut Vector2, a: &Vector2, m: &Matrix2d) {
        let x = a.0;
        let y = a.1;
        out.0 = m.0 * x + m.2 * y + m.4;
        out.1 = m.1 * x + m.3 * y + m.5;
    }

    pub fn transformMat3(out: &mut Vector2, a: &Vector2, m: &Matrix3) {
        let x = a.0;
        let y = a.1;
        out.0 = m.0 * x + m.3 * y + m.6;
        out.1 = m.1 * x + m.4 * y + m.7;
    }

    pub fn transformMat4(out: &mut Vector2, a: &Vector2, m: &Matrix4) {
        let x = a.0;
        let y = a.1;
        out.0 = m.0 * x + m.4 * y + m.12;
        out.1 = m.1 * x + m.5 * y + m.13;
    }

    pub fn rotate(out: &mut Vector2, a: &Vector2, b: &Vector2, c: f32) {
        //Translate point to the origin
        let p0 = a.0 - b.0;
        let p1 = a.1 - b.1;
        let sinC = f32::sin(c);
        let cosC = f32::cos(c);

        //perform rotation and translate to correct position
        out.0 = p0 * cosC - p1 * sinC + b.0;
        out.1 = p0 * sinC + p1 * cosC + b.1;
    }

    pub fn angle(a: &Vector2, b: &Vector2) -> f32 {
        let x1 = a.0;
        let y1 = a.1;
        let x2 = b.0;
        let y2 = b.1;

        let mut len1 = x1 * x1 + y1 * y1;
        if (len1 > EPSILON) {
            //TODO: evaluate use of glm_invsqrt here?
            len1 = 1. / f32::sqrt(len1);
        }

        let mut len2 = x2 * x2 + y2 * y2;
        if (len2 > EPSILON) {
            //TODO: evaluate use of glm_invsqrt here?
            len2 = 1. / f32::sqrt(len2);
        }

        let cosine = (x1 * x2 + y1 * y2) * len1 * len2;

        if (cosine > 1.0) {
            0.
        } else if (cosine < -1.0) {
            PI
        } else {
            f32::acos(cosine)
        }
    }

    pub fn zero(out: &mut Vector2) {
        out.0 = 0.0;
        out.1 = 0.0;
    }

    // pub fn str(a: &Vector2) {
    //   return "vec2(" + a.0 + ", " + a.1 + ")";
    // }

    pub fn exactEquals(a: &Vector2, b: &Vector2) -> bool {
        a.0 == b.0 && a.1 == b.1
    }

    pub fn equals(a: &Vector2, b: &Vector2) -> bool {
        let a0 = a.0;
        let a1 = a.1;
        let b0 = b.0;
        let b1 = b.1;
        f32::abs(a0 - b0) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a0), f32::abs(b0)))
            && f32::abs(a1 - b1) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a1), f32::abs(b1)))
    }

    pub fn sub(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        Vector2::subtract(out, a, b);
    }

    pub fn mul(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        Vector2::multiply(out, a, b);
    }

    pub fn div(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        Vector2::divide(out, a, b);
    }

    pub fn dist(a: &Vector2, b: &Vector2) -> f32 {
        Vector2::distance(a, b)
    }

    pub fn sqrDist(a: &Vector2, b: &Vector2) -> f32 {
        Vector2::squaredDistance(a, b)
    }

    pub fn sqrLen(a: &Vector2) -> f32 {
        Vector2::squaredLength(a)
    }
}
