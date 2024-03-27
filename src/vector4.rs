/**
 * @File   : vector4.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : http://dtysky.moe
 * @Date   : 2019/2/7 下午9:53:03
 */
use wasm_bindgen::prelude::*;

use super::common::*;
use super::matrix4::*;
use super::quaternion::*;

#[wasm_bindgen]
pub struct Vector4(pub f32, pub f32, pub f32, pub f32);

#[wasm_bindgen]
impl Vector4 {
    #[wasm_bindgen(getter)]
    pub fn elements(&self) -> Box<[f32]> {
        Box::new([self.0, self.1, self.2, self.3])
    }

    pub fn create() -> Vector4 {
        Vector4(0., 0., 0., 0.)
    }

    pub fn clone(a: &Vector4) -> Vector4 {
        Vector4(a.0, a.1, a.2, a.3)
    }

    pub fn fromValues(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4(x, y, z, w)
    }

    pub fn copy(out: &mut Vector4, a: &Vector4) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.3;
    }

    pub fn set(out: &mut Vector4, x: f32, y: f32, z: f32, w: f32) {
        out.0 = x;
        out.1 = y;
        out.2 = z;
        out.3 = w;
    }

    pub fn add(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
        out.2 = a.2 + b.2;
        out.3 = a.3 + b.3;
    }

    pub fn subtract(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = a.0 - b.0;
        out.1 = a.1 - b.1;
        out.2 = a.2 - b.2;
        out.3 = a.3 - b.3;
    }

    pub fn multiply(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = a.0 * b.0;
        out.1 = a.1 * b.1;
        out.2 = a.2 * b.2;
        out.3 = a.3 * b.3;
    }

    pub fn divide(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = a.0 / b.0;
        out.1 = a.1 / b.1;
        out.2 = a.2 / b.2;
        out.3 = a.3 / b.3;
    }

    pub fn ceil(out: &mut Vector4, a: &Vector4) {
        out.0 = f32::ceil(a.0);
        out.1 = f32::ceil(a.1);
        out.2 = f32::ceil(a.2);
        out.3 = f32::ceil(a.3);
    }

    pub fn floor(out: &mut Vector4, a: &Vector4) {
        out.0 = f32::floor(a.0);
        out.1 = f32::floor(a.1);
        out.2 = f32::floor(a.2);
        out.3 = f32::floor(a.3);
    }

    pub fn min(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = f32::min(a.0, b.0);
        out.1 = f32::min(a.1, b.1);
        out.2 = f32::min(a.2, b.2);
        out.3 = f32::min(a.3, b.3);
    }

    pub fn max(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = f32::max(a.0, b.0);
        out.1 = f32::max(a.1, b.1);
        out.2 = f32::max(a.2, b.2);
        out.3 = f32::max(a.3, b.3);
    }

    pub fn round(out: &mut Vector4, a: &Vector4) {
        out.0 = f32::round(a.0);
        out.1 = f32::round(a.1);
        out.2 = f32::round(a.2);
        out.3 = f32::round(a.3);
    }

    pub fn scale(out: &mut Vector4, a: &Vector4, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
        out.2 = a.2 * b;
        out.3 = a.3 * b;
    }

    pub fn scaleAndAdd(out: &mut Vector4, a: &Vector4, b: &Vector4, scale: f32) {
        out.0 = a.0 + (b.0 * scale);
        out.1 = a.1 + (b.1 * scale);
        out.2 = a.2 + (b.2 * scale);
        out.3 = a.3 + (b.3 * scale);
    }

    pub fn distance(a: &Vector4, b: &Vector4) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        let z = b.2 - a.2;
        let w = b.3 - a.3;
        (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt()
    }

    pub fn squaredDistance(a: &Vector4, b: &Vector4) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        let z = b.2 - a.2;
        let w = b.3 - a.3;
        x * x + y * y + z * z + w * w
    }

    pub fn len(a: &Vector4) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt()
    }

    pub fn squaredLength(a: &Vector4) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        x * x + y * y + z * z + w * w
    }

    pub fn negate(out: &mut Vector4, a: &Vector4) {
        out.0 = -a.0;
        out.1 = -a.1;
        out.2 = -a.2;
        out.3 = -a.3;
    }

    pub fn inverse(out: &mut Vector4, a: &Vector4) {
        out.0 = 1.0 / a.0;
        out.1 = 1.0 / a.1;
        out.2 = 1.0 / a.2;
        out.3 = 1.0 / a.3;
    }

    pub fn normalize(out: &mut Vector4, a: &Vector4) {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        let mut len = x * x + y * y + z * z + w * w;
        if len > EPSILON {
            len = 1. / f32::sqrt(len);
        }
        out.0 = x * len;
        out.1 = y * len;
        out.2 = z * len;
        out.3 = w * len;
    }

    pub fn dot(a: &Vector4, b: &Vector4) -> f32 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2 + a.3 * b.3
    }

    pub fn cross(out: &mut Vector4, u: &Vector4, v: &Vector4, w: &Vector4) {
        if std::ptr::eq(out, u) {
            let v0 = v.0;
            let v1 = v.1;
            let v2 = v.2;
            let v3 = v.3;
            let w0 = w.0;
            let w1 = w.1;
            let w2 = w.2;
            let w3 = w.3;
            let A = (v0 * w1) - (v1 * w0);
            let B = (v0 * w2) - (v2 * w0);
            let C = (v0 * w3) - (v3 * w0);
            let D = (v1 * w2) - (v2 * w1);
            let E = (v1 * w3) - (v3 * w1);
            let F = (v2 * w3) - (v3 * w2);
            let G = out.0;
            let H = out.1;
            let I = out.2;
            let J = out.3;

            out.0 = (H * F) - (I * E) + (J * D);
            out.1 = -(G * F) + (I * C) - (J * B);
            out.2 = (G * E) - (H * C) + (J * A);
            out.3 = -(G * D) + (H * B) - (I * A);
            return;
        }

        let v0 = v.0;
        let v1 = v.1;
        let v2 = v.2;
        let v3 = v.3;
        let w0 = w.0;
        let w1 = w.1;
        let w2 = w.2;
        let w3 = w.3;
        let A = (v0 * w1) - (v1 * w0);
        let B = (v0 * w2) - (v2 * w0);
        let C = (v0 * w3) - (v3 * w0);
        let D = (v1 * w2) - (v2 * w1);
        let E = (v1 * w3) - (v3 * w1);
        let F = (v2 * w3) - (v3 * w2);
        let G = u.0;
        let H = u.1;
        let I = u.2;
        let J = u.3;

        out.0 = (H * F) - (I * E) + (J * D);
        out.1 = -(G * F) + (I * C) - (J * B);
        out.2 = (G * E) - (H * C) + (J * A);
        out.3 = -(G * D) + (H * B) - (I * A);
    }

    pub fn lerp(out: &mut Vector4, a: &Vector4, b: &Vector4, t: f32) {
        let ax = a.0;
        let ay = a.1;
        let az = a.2;
        let aw = a.3;
        out.0 = ax + t * (b.0 - ax);
        out.1 = ay + t * (b.1 - ay);
        out.2 = az + t * (b.2 - az);
        out.3 = aw + t * (b.3 - aw);
    }

    pub fn random(out: &mut Vector4, scale: Option<f32>) {
        let mut s = 1.;
        match scale {
            Some(value) => s = value,
            None => {}
        };
        let scale = s;

        // Marsaglia, George. Choosing a Point from the Surface of a
        // Sphere. Ann. f32:: Statist. 43 (1972), no. 2, 645--646.
        // http://projecteuclid.org/euclid.aoms/1177692644;
        let mut v1 = 0.;
        let mut v2 = 0.;
        let mut v3 = 0.;
        let mut v4 = 0.;
        let mut s1 = 2.;
        let mut s2 = 2.;

        while s1 > 1. {
            v1 = RANDOM() * 2. - 1.;
            v2 = RANDOM() * 2. - 1.;
            s1 = v1 * v1 + v2 * v2;
        }

        while s2 > 1. {
            v3 = RANDOM() * 2. - 1.;
            v4 = RANDOM() * 2. - 1.;
            s2 = v3 * v3 + v4 * v4;
        }

        let d = f32::sqrt((1. - s1) / s2);
        out.0 = scale * v1;
        out.1 = scale * v2;
        out.2 = scale * v3 * d;
        out.3 = scale * v4 * d;
    }

    pub fn transformMat4(out: &mut Vector4, a: &Vector4, m: &Matrix4) {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        out.0 = m.0 * x + m.4 * y + m.8 * z + m.12 * w;
        out.1 = m.1 * x + m.5 * y + m.9 * z + m.13 * w;
        out.2 = m.2 * x + m.6 * y + m.10 * z + m.14 * w;
        out.3 = m.3 * x + m.7 * y + m.11 * z + m.15 * w;
    }

    pub fn transformQuat(out: &mut Vector4, a: &Vector4, q: &Quaternion) {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let qx = q.0;
        let qy = q.1;
        let qz = q.2;
        let qw = q.3;

        // calculate quat * vec
        let ix = qw * x + qy * z - qz * y;
        let iy = qw * y + qz * x - qx * z;
        let iz = qw * z + qx * y - qy * x;
        let iw = -qx * x - qy * y - qz * z;

        // calculate result * inverse quat
        out.0 = ix * qw + iw * -qx + iy * -qz - iz * -qy;
        out.1 = iy * qw + iw * -qy + iz * -qx - ix * -qz;
        out.2 = iz * qw + iw * -qz + ix * -qy - iy * -qx;
        out.3 = a.3;
    }

    pub fn zero(out: &mut Vector4) {
        out.0 = 0.0;
        out.1 = 0.0;
        out.2 = 0.0;
        out.3 = 0.0;
    }

    // pub fn str(a: &Vector4) {
    //   return "vec4(" + a.0 + ", " + a.1 + ", " + a.2 + ", " + a.3 + ")";
    // }

    pub fn exactEquals(a: &Vector4, b: &Vector4) -> bool {
        a.0 == b.0 && a.1 == b.1 && a.2 == b.2 && a.3 == b.3
    }

    pub fn equals(a: &Vector4, b: &Vector4) -> bool {
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

    pub fn sub(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        Vector4::subtract(out, a, b);
    }

    pub fn mul(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        Vector4::multiply(out, a, b);
    }

    pub fn div(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        Vector4::divide(out, a, b);
    }

    pub fn dist(a: &Vector4, b: &Vector4) -> f32 {
        Vector4::distance(a, b)
    }

    pub fn sqrDist(a: &Vector4, b: &Vector4) -> f32 {
        Vector4::squaredDistance(a, b)
    }

    pub fn sqrLen(a: &Vector4) -> f32 {
        Vector4::squaredLength(a)
    }
}
