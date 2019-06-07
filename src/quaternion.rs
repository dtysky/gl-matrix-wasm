/**
 * @File   : quaternion.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : http://dtysky.moe
 * @Date   : 2019/2/7 下午9:53:03
 */
use wasm_bindgen::prelude::*;

use super::common::*;
use super::matrix3::*;
use super::vector3::*;

#[wasm_bindgen]
pub struct Quaternion(pub f32, pub f32, pub f32, pub f32);

#[wasm_bindgen]
impl Quaternion {
    #[wasm_bindgen(getter)]
    pub fn elements(&self) -> Box<[f32]> {
        Box::new([self.0, self.1, self.2, self.3])
    }

    pub fn create() -> Quaternion {
        Quaternion(0., 0., 0., 1.)
    }

    pub fn identity(out: &mut Quaternion) {
        out.0 = 0.;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = 1.;
    }

    pub fn setAxisAngle(out: &mut Quaternion, axis: &Vector3, rad: f32) {
        let rad = rad * 0.5;
        let s = f32::sin(rad);
        out.0 = s * axis.0;
        out.1 = s * axis.1;
        out.2 = s * axis.2;
        out.3 = f32::cos(rad);
    }

    pub fn getAxisAngle(out_axis: &mut Vector3, q: &Quaternion) -> f32 {
        let rad = f32::acos(q.3) * 2.0;
        let s = f32::sin(rad / 2.0);
        if (s > EPSILON) {
            out_axis.0 = q.0 / s;
            out_axis.1 = q.1 / s;
            out_axis.2 = q.2 / s;
        } else {
            // If s is zero, return any axis (no rotation - axis does not matter)
            out_axis.0 = 1.;
            out_axis.1 = 0.;
            out_axis.2 = 0.;
        }
        rad
    }

    pub fn multiply(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
        let ax = a.0;
        let ay = a.1;
        let az = a.2;
        let aw = a.3;
        let bx = b.0;
        let by = b.1;
        let bz = b.2;
        let bw = b.3;

        out.0 = ax * bw + aw * bx + ay * bz - az * by;
        out.1 = ay * bw + aw * by + az * bx - ax * bz;
        out.2 = az * bw + aw * bz + ax * by - ay * bx;
        out.3 = aw * bw - ax * bx - ay * by - az * bz;
    }

    pub fn rotateX(out: &mut Quaternion, a: &Quaternion, rad: f32) {
        let rad = rad * 0.5;

        let ax = a.0;
        let ay = a.1;
        let az = a.2;
        let aw = a.3;
        let bx = f32::sin(rad);
        let bw = f32::cos(rad);

        out.0 = ax * bw + aw * bx;
        out.1 = ay * bw + az * bx;
        out.2 = az * bw - ay * bx;
        out.3 = aw * bw - ax * bx;
    }

    pub fn rotateY(out: &mut Quaternion, a: &Quaternion, rad: f32) {
        let rad = rad * 0.5;

        let ax = a.0;
        let ay = a.1;
        let az = a.2;
        let aw = a.3;
        let by = f32::sin(rad);
        let bw = f32::cos(rad);

        out.0 = ax * bw - az * by;
        out.1 = ay * bw + aw * by;
        out.2 = az * bw + ax * by;
        out.3 = aw * bw - ay * by;
    }

    pub fn rotateZ(out: &mut Quaternion, a: &Quaternion, rad: f32) {
        let rad = rad * 0.5;

        let ax = a.0;
        let ay = a.1;
        let az = a.2;
        let aw = a.3;
        let bz = f32::sin(rad);
        let bw = f32::cos(rad);

        out.0 = ax * bw + ay * bz;
        out.1 = ay * bw - ax * bz;
        out.2 = az * bw + aw * bz;
        out.3 = aw * bw - az * bz;
    }

    pub fn calculateW(out: &mut Quaternion, a: &Quaternion) {
        let x = a.0;
        let y = a.1;
        let z = a.2;

        out.0 = x;
        out.1 = y;
        out.2 = z;
        out.3 = f32::sqrt(f32::abs(1.0 - x * x - y * y - z * z));
    }

    pub fn slerp(out: &mut Quaternion, a: &Quaternion, b: &Quaternion, t: f32) {
        // benchmarks:
        //    http://jsperf.com/quaternion-slerp-implementations
        let ax = a.0;
        let ay = a.1;
        let az = a.2;
        let aw = a.3;
        let mut bx = b.0;
        let mut by = b.1;
        let mut bz = b.2;
        let mut bw = b.3;

        let mut omega = 0.;
        let mut cosom = 0.;
        let mut sinom = 0.;
        let mut scale0 = 0.;
        let mut scale1 = 0.;

        // calc cosine
        let mut cosom = ax * bx + ay * by + az * bz + aw * bw;
        // adjust signs (if necessary)
        if (cosom < 0.0) {
            cosom = -cosom;
            bx = -bx;
            by = -by;
            bz = -bz;
            bw = -bw;
        }
        // calculate coefficients
        if ((1.0 - cosom) > EPSILON) {
            // standard case (slerp)
            omega = f32::acos(cosom);
            sinom = f32::sin(omega);
            scale0 = f32::sin((1.0 - t) * omega) / sinom;
            scale1 = f32::sin(t * omega) / sinom;
        } else {
            // "from" and "to" quaternions are very close
            //  ... so we can do a linear interpolation
            scale0 = 1.0 - t;
            scale1 = t;
        }
        // calculate final values
        out.0 = scale0 * ax + scale1 * bx;
        out.1 = scale0 * ay + scale1 * by;
        out.2 = scale0 * az + scale1 * bz;
        out.3 = scale0 * aw + scale1 * bw;
    }

    pub fn random(out: &mut Quaternion) {
        // Implementation of http://planning.cs.uiuc.edu/node198.html
        // TODO: Calling random 3 times is probably not the fastest solution
        let u1 = RANDOM();
        let u2 = RANDOM();
        let u3 = RANDOM();

        let sqrt1MinusU1 = f32::sqrt(1. - u1);
        let sqrtU1 = f32::sqrt(u1);

        out.0 = sqrt1MinusU1 * f32::sin(2.0 * PI * u2);
        out.1 = sqrt1MinusU1 * f32::cos(2.0 * PI * u2);
        out.2 = sqrtU1 * f32::sin(2.0 * PI * u3);
        out.3 = sqrtU1 * f32::cos(2.0 * PI * u3);
    }

    pub fn invert(out: &mut Quaternion, a: &Quaternion) {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let dot = a0 * a0 + a1 * a1 + a2 * a2 + a3 * a3;
        let invDot = if dot < EPSILON { 0. } else { 1. / dot };

        // TODO: Would be faster to return [0,0,0,0] immediately if dot == 0

        out.0 = -a0 * invDot;
        out.1 = -a1 * invDot;
        out.2 = -a2 * invDot;
        out.3 = a3 * invDot;
    }

    pub fn conjugate(out: &mut Quaternion, a: &Quaternion) {
        out.0 = -a.0;
        out.1 = -a.1;
        out.2 = -a.2;
        out.3 = a.3;
    }

    pub fn fromMat3(out: &mut Quaternion, m: &Matrix3) {
        // Algorithm in Ken Shoemake"s article in 1987 SIGGRAPH course notes
        // article "Quaternion Calculus and Fast Animation".
        let fTrace = m.0 + m.4 + m.8;

        if (fTrace > 0.0) {
            // |w| > 1/2, may as well choose w > 1/2
            let mut fRoot = f32::sqrt(fTrace + 1.0); // 2w
            out.3 = 0.5 * fRoot;
            fRoot = 0.5 / fRoot; // 1/(4w)
            out.0 = (m.5 - m.7) * fRoot;
            out.1 = (m.6 - m.2) * fRoot;
            out.2 = (m.1 - m.3) * fRoot;
        } else {
            // |w| <= 1/2
            let mut i = 0;
            if (m.4 > m.0) {
                i = 1;
            }
            let tmp = if i == 0 { m.0 } else { m.4 };
            if (m.8 > tmp) {
                i = 2;
            }

            match i {
                0 => {
                    // i = 0, j = 1, k = 2
                    let mut fRoot = f32::sqrt(m.0 - m.4 - m.8 + 1.0);
                    out.0 = 0.5 * fRoot;
                    fRoot = 0.5 / fRoot;
                    out.3 = (m.5 - m.7) * fRoot;
                    out.1 = (m.3 + m.1) * fRoot;
                    out.2 = (m.6 + m.2) * fRoot;
                }
                1 => {
                    // i = 1, j = 2, k = 0
                    let mut fRoot = f32::sqrt(m.4 - m.8 - m.0 + 1.0);
                    out.1 = 0.5 * fRoot;
                    fRoot = 0.5 / fRoot;
                    out.3 = (m.6 - m.2) * fRoot;
                    out.2 = (m.7 + m.5) * fRoot;
                    out.0 = (m.1 + m.3) * fRoot;
                }
                2 => {
                    // i = 2, j = 0, k = 1
                    let mut fRoot = f32::sqrt(m.8 - m.0 - m.4 + 1.0);
                    out.2 = 0.5 * fRoot;
                    fRoot = 0.5 / fRoot;
                    out.3 = (m.1 - m.3) * fRoot;
                    out.0 = (m.2 + m.6) * fRoot;
                    out.1 = (m.5 + m.7) * fRoot;
                }
                _ => {}
            }
        }
    }

    pub fn fromEuler(out: &mut Quaternion, x: f32, y: f32, z: f32) {
        let halfToRad = 0.5 * PI / 180.0;
        let x = x * halfToRad;
        let y = y * halfToRad;
        let z = z * halfToRad;

        let sx = f32::sin(x);
        let cx = f32::cos(x);
        let sy = f32::sin(y);
        let cy = f32::cos(y);
        let sz = f32::sin(z);
        let cz = f32::cos(z);

        out.0 = sx * cy * cz - cx * sy * sz;
        out.1 = cx * sy * cz + sx * cy * sz;
        out.2 = cx * cy * sz - sx * sy * cz;
        out.3 = cx * cy * cz + sx * sy * sz;
    }

    // pub fn str(a: &Quaternion) {
    //   return "quat(" + a.0 + ", " + a.1 + ", " + a.2 + ", " + a.3 + ")";
    // }

    pub fn clone(a: &Quaternion) -> Quaternion {
        Quaternion(a.0, a.1, a.2, a.3)
    }

    pub fn fromValues(x: f32, y: f32, z: f32, w: f32) -> Quaternion {
        Quaternion(x, y, z, w)
    }

    pub fn copy(out: &mut Quaternion, a: &Quaternion) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.3;
    }

    pub fn set(out: &mut Quaternion, x: f32, y: f32, z: f32, w: f32) {
        out.0 = x;
        out.1 = y;
        out.2 = z;
        out.3 = w;
    }

    pub fn add(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
        out.2 = a.2 + b.2;
        out.3 = a.3 + b.3;
    }

    pub fn mul(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
        Quaternion::multiply(out, a, b);
    }

    pub fn scale(out: &mut Quaternion, a: &Quaternion, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
        out.2 = a.2 * b;
        out.3 = a.3 * b;
    }

    pub fn dot(a: &Quaternion, b: &Quaternion) -> f32 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2 + a.3 * b.3
    }

    pub fn lerp(out: &mut Quaternion, a: &Quaternion, b: &Quaternion, t: f32) {
        let ax = a.0;
        let ay = a.1;
        let az = a.2;
        let aw = a.3;
        out.0 = ax + t * (b.0 - ax);
        out.1 = ay + t * (b.1 - ay);
        out.2 = az + t * (b.2 - az);
        out.3 = aw + t * (b.3 - aw);
    }

    pub fn length(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt()
    }

    pub fn len(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
        Quaternion::length(out, a, b);
    }

    pub fn squaredLength(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        x * x + y * y + z * z + w * w
    }

    pub fn sqrLen(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
        Quaternion::squaredLength(out, a, b);
    }

    pub fn normalize(out: &mut Quaternion, a: &Quaternion) {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        let mut len = x * x + y * y + z * z + w * w;
        if (len > EPSILON) {
            len = 1. / f32::sqrt(len);
        }
        out.0 = x * len;
        out.1 = y * len;
        out.2 = z * len;
        out.3 = w * len;
    }

    pub fn exactEquals(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) -> bool {
        a.0 == b.0 && a.1 == b.1 && a.2 == b.2 && a.3 == b.3
    }

    pub fn equals(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) -> bool {
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

    pub fn rotationTo(out: &mut Quaternion, a: &Vector3, b: &Vector3) {
        let tmpvec3 = &mut Vector3::create();
        let xUnitVec3 = &Vector3::fromValues(1., 0., 0.);
        let yUnitVec3 = &Vector3::fromValues(0., 1., 0.);

        let dot = Vector3::dot(a, b);
        if (dot < -0.999999) {
            Vector3::cross(tmpvec3, xUnitVec3, a);
            if (Vector3::len(tmpvec3) < EPSILON) {
                Vector3::cross(tmpvec3, yUnitVec3, a);
            }
            Vector3::normalize(tmpvec3, &Vector3::clone(tmpvec3));
            Quaternion::setAxisAngle(out, tmpvec3, PI);
        } else if (dot > 0.999999) {
            out.0 = 0.;
            out.1 = 0.;
            out.2 = 0.;
            out.3 = 1.;
        } else {
            Vector3::cross(tmpvec3, a, b);
            out.0 = tmpvec3.0;
            out.1 = tmpvec3.1;
            out.2 = tmpvec3.2;
            out.3 = 1. + dot;
            Quaternion::normalize(out, &Quaternion::clone(out));
        }
    }

    pub fn sqlerp(
        out: &mut Quaternion,
        a: &Quaternion,
        b: &Quaternion,
        c: &Quaternion,
        d: &Quaternion,
        t: f32,
    ) {
        let temp1 = &mut Quaternion::create();
        let temp2 = &mut Quaternion::create();

        Quaternion::slerp(temp1, a, d, t);
        Quaternion::slerp(temp2, b, c, t);
        Quaternion::slerp(out, temp1, temp2, 2. * t * (1. - t));
    }

    pub fn setAxes(out: &mut Quaternion, view: &Vector3, right: &Vector3, up: &Vector3) {
        let matr = &mut Matrix3::create();

        matr.0 = right.0;
        matr.3 = right.1;
        matr.6 = right.2;

        matr.1 = up.0;
        matr.4 = up.1;
        matr.7 = up.2;

        matr.2 = -view.0;
        matr.5 = -view.1;
        matr.8 = -view.2;

        Quaternion::fromMat3(out, matr);
        Quaternion::normalize(out, &Quaternion::clone(out));
    }
}
