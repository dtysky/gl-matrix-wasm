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

    /**
     * 4 Dimensional Vector
     * @module vec4
     */

    /**
     * Creates a new, empty vec4
     *
     * @returns {vec4} a new 4D vector
     */
    pub fn create() -> Vector4 {
        Vector4(0., 0., 0., 0.)
    }

    /**
     * Creates a new vec4 initialized with values from an existing vector
     *
     * @param {vec4} a vector to clone
     * @returns {vec4} a new 4D vector
     */
    pub fn clone(a: &Vector4) -> Vector4 {
        Vector4(a.0, a.1, a.2, a.3)
    }

    /**
     * Creates a new vec4 initialized with the given values
     *
     * @param {Number} x X component
     * @param {Number} y Y component
     * @param {Number} z Z component
     * @param {Number} w W component
     * @returns {vec4} a new 4D vector
     */
    pub fn fromValues(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4(x, y, z, w)
    }

    /**
     * Copy the values from one vec4 to another
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the source vector
     * @returns {vec4} out
     */
    pub fn copy(out: &mut Vector4, a: &Vector4) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.3;
    }

    /**
     * Set the components of a vec4 to the given values
     *
     * @param {vec4} out the receiving vector
     * @param {Number} x X component
     * @param {Number} y Y component
     * @param {Number} z Z component
     * @param {Number} w W component
     * @returns {vec4} out
     */
    pub fn set(out: &mut Vector4, x: f32, y: f32, z: f32, w: f32) {
        out.0 = x;
        out.1 = y;
        out.2 = z;
        out.3 = w;
    }

    /**
     * Adds two vec4"s
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @returns {vec4} out
     */
    pub fn add(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
        out.2 = a.2 + b.2;
        out.3 = a.3 + b.3;
    }

    /**
     * Subtracts vector b from vector a
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @returns {vec4} out
     */
    pub fn subtract(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = a.0 - b.0;
        out.1 = a.1 - b.1;
        out.2 = a.2 - b.2;
        out.3 = a.3 - b.3;
    }

    /**
     * Multiplies two vec4"s
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @returns {vec4} out
     */
    pub fn multiply(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = a.0 * b.0;
        out.1 = a.1 * b.1;
        out.2 = a.2 * b.2;
        out.3 = a.3 * b.3;
    }

    /**
     * Divides two vec4"s
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @returns {vec4} out
     */
    pub fn divide(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = a.0 / b.0;
        out.1 = a.1 / b.1;
        out.2 = a.2 / b.2;
        out.3 = a.3 / b.3;
    }

    /**
     * f32::ceil the components of a vec4
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a vector to ceil
     * @returns {vec4} out
     */
    pub fn ceil(out: &mut Vector4, a: &Vector4) {
        out.0 = f32::ceil(a.0);
        out.1 = f32::ceil(a.1);
        out.2 = f32::ceil(a.2);
        out.3 = f32::ceil(a.3);
    }

    /**
     * f32::floor the components of a vec4
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a vector to floor
     * @returns {vec4} out
     */
    pub fn floor(out: &mut Vector4, a: &Vector4) {
        out.0 = f32::floor(a.0);
        out.1 = f32::floor(a.1);
        out.2 = f32::floor(a.2);
        out.3 = f32::floor(a.3);
    }

    /**
     * Returns the minimum of two vec4"s
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @returns {vec4} out
     */
    pub fn min(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = f32::min(a.0, b.0);
        out.1 = f32::min(a.1, b.1);
        out.2 = f32::min(a.2, b.2);
        out.3 = f32::min(a.3, b.3);
    }

    /**
     * Returns the maximum of two vec4"s
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @returns {vec4} out
     */
    pub fn max(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        out.0 = f32::max(a.0, b.0);
        out.1 = f32::max(a.1, b.1);
        out.2 = f32::max(a.2, b.2);
        out.3 = f32::max(a.3, b.3);
    }

    /**
     * f32::round the components of a vec4
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a vector to round
     * @returns {vec4} out
     */
    pub fn round(out: &mut Vector4, a: &Vector4) {
        out.0 = f32::round(a.0);
        out.1 = f32::round(a.1);
        out.2 = f32::round(a.2);
        out.3 = f32::round(a.3);
    }

    /**
     * Scales a vec4 by a scalar number
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the vector to scale
     * @param {Number} b amount to scale the vector by
     * @returns {vec4} out
     */
    pub fn scale(out: &mut Vector4, a: &Vector4, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
        out.2 = a.2 * b;
        out.3 = a.3 * b;
    }

    /**
     * Adds two vec4"s after scaling the second operand by a scalar value
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @param {Number} scale the amount to scale b by before adding
     * @returns {vec4} out
     */
    pub fn scaleAndAdd(out: &mut Vector4, a: &Vector4, b: &Vector4, scale: f32) {
        out.0 = a.0 + (b.0 * scale);
        out.1 = a.1 + (b.1 * scale);
        out.2 = a.2 + (b.2 * scale);
        out.3 = a.3 + (b.3 * scale);
    }

    /**
     * Calculates the euclidian distance between two vec4"s
     *
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @returns {Number} distance between a and b
     */
    pub fn distance(a: &Vector4, b: &Vector4) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        let z = b.2 - a.2;
        let w = b.3 - a.3;
        (x.powi(2) + y.powi(2) + z.powi(2) + z.powi(3)).sqrt()
    }

    /**
     * Calculates the squared euclidian distance between two vec4"s
     *
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @returns {Number} squared distance between a and b
     */
    pub fn squaredDistance(a: &Vector4, b: &Vector4) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        let z = b.2 - a.2;
        let w = b.3 - a.3;
        x * x + y * y + z * z + w * w
    }

    /**
     * Calculates the length of a vec4
     *
     * @param {vec4} a vector to calculate length of
     * @returns {Number} length of a
     */
    pub fn length(a: &Vector4) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        (x.powi(2) + y.powi(2) + z.powi(2) + z.powi(3)).sqrt()
    }

    /**
     * Calculates the squared length of a vec4
     *
     * @param {vec4} a vector to calculate squared length of
     * @returns {Number} squared length of a
     */
    pub fn squaredLength(a: &Vector4) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        x * x + y * y + z * z + w * w
    }

    /**
     * Negates the components of a vec4
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a vector to negate
     * @returns {vec4} out
     */
    pub fn negate(out: &mut Vector4, a: &Vector4) {
        out.0 = -a.0;
        out.1 = -a.1;
        out.2 = -a.2;
        out.3 = -a.3;
    }

    /**
     * Returns the inverse of the components of a vec4
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a vector to invert
     * @returns {vec4} out
     */
    pub fn inverse(out: &mut Vector4, a: &Vector4) {
        out.0 = 1.0 / a.0;
        out.1 = 1.0 / a.1;
        out.2 = 1.0 / a.2;
        out.3 = 1.0 / a.3;
    }

    /**
     * Normalize a vec4
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a vector to normalize
     * @returns {vec4} out
     */
    pub fn normalize(out: &mut Vector4, a: &Vector4) {
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

    /**
     * Calculates the dot product of two vec4"s
     *
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @returns {Number} dot product of a and b
     */
    pub fn dot(a: &Vector4, b: &Vector4) -> f32 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2 + a.3 * b.3
    }

    /**
     * Returns the cross-product of three vectors in a 4-dimensional space
     *
     * @param {vec4} result the receiving vector
     * @param {vec4} U the first vector
     * @param {vec4} V the second vector
     * @param {vec4} W the third vector
     * @returns {vec4} result
     */
    pub fn cross(out: &mut Vector4, u: &Vector4, v: &Vector4, w: &Vector4) {
        let A = (v.0 * w.1) - (v.1 * w.0);
        let B = (v.0 * w.2) - (v.2 * w.0);
        let C = (v.0 * w.3) - (v.3 * w.0);
        let D = (v.1 * w.2) - (v.2 * w.1);
        let E = (v.1 * w.3) - (v.3 * w.1);
        let F = (v.2 * w.3) - (v.3 * w.2);
        let G = u.0;
        let H = u.1;
        let I = u.2;
        let J = u.3;

        out.0 = (H * F) - (I * E) + (J * D);
        out.1 = -(G * F) + (I * C) - (J * B);
        out.2 = (G * E) - (H * C) + (J * A);
        out.3 = -(G * D) + (H * B) - (I * A);
    }

    /**
     * Performs a linear interpolation between two vec4"s
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the first operand
     * @param {vec4} b the second operand
     * @param {Number} t interpolation amount, in the range [0-1], between the two inputs
     * @returns {vec4} out
     */
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

    /**
     * Generates a random vector with the given scale
     *
     * @param {vec4} out the receiving vector
     * @param {Number} [scale] Length of the resulting vector. If ommitted, a unit vector will be returned
     * @returns {vec4} out
     */
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
        let mut s1 = 0.;
        let mut s2 = 0.;

        while (s1 > 1.) {
            v1 = RANDOM() * 2. - 1.;
            v2 = RANDOM() * 2. - 1.;
            s1 = v1 * v1 + v2 * v2;
        }

        while (s2 > 1.) {
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

    /**
     * Transforms the vec4 with a mat4.
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the vector to transform
     * @param {mat4} m matrix to transform with
     * @returns {vec4} out
     */
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

    /**
     * Transforms the vec4 with a quat
     *
     * @param {vec4} out the receiving vector
     * @param {vec4} a the vector to transform
     * @param {quat} q quaternion to transform with
     * @returns {vec4} out
     */
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

    /**
     * Set the components of a vec4 to zero
     *
     * @param {vec4} out the receiving vector
     * @returns {vec4} out
     */
    pub fn zero(out: &mut Vector4) {
        out.0 = 0.0;
        out.1 = 0.0;
        out.2 = 0.0;
        out.3 = 0.0;
    }

    /**
     * Returns a string representation of a vector
     *
     * @param {vec4} a vector to represent as a string
     * @returns {String} string representation of the vector
     */
    // pub fn str(a: &Vector4) {
    //   return "vec4(" + a.0 + ", " + a.1 + ", " + a.2 + ", " + a.3 + ")";
    // }

    /**
     * Returns whether or not the vectors have exactly the same elements in the same position (when compared with ==)
     *
     * @param {vec4} a The first vector.
     * @param {vec4} b The second vector.
     * @returns {Boolean} True if the vectors are equal, false otherwise.
     */
    pub fn exactEquals(a: &Vector4, b: &Vector4) -> bool {
        a.0 == b.0 && a.1 == b.1 && a.2 == b.2 && a.3 == b.3
    }

    /**
     * Returns whether or not the vectors have approximately the same elements in the same position.
     *
     * @param {vec4} a The first vector.
     * @param {vec4} b The second vector.
     * @returns {Boolean} True if the vectors are equal, false otherwise.
     */
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

    /**
     * Alias for {@link vec4.subtract}
     * @function
     */
    pub fn sub(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        Vector4::subtract(out, a, b);
    }

    /**
     * Alias for {@link vec4.multiply}
     * @function
     */
    pub fn mul(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        Vector4::multiply(out, a, b);
    }

    /**
     * Alias for {@link vec4.divide}
     * @function
     */
    pub fn div(out: &mut Vector4, a: &Vector4, b: &Vector4) {
        Vector4::divide(out, a, b);
    }

    /**
     * Alias for {@link vec4.distance}
     * @function
     */
    pub fn dist(a: &Vector4, b: &Vector4) -> f32 {
        Vector4::distance(a, b)
    }

    /**
     * Alias for {@link vec4.squaredDistance}
     * @function
     */
    pub fn sqrDist(a: &Vector4, b: &Vector4) -> f32 {
        Vector4::squaredDistance(a, b)
    }

    /**
     * Alias for {@link vec4.length}
     * @function
     */
    pub fn len(a: &Vector4) -> f32 {
        Vector4::length(a)
    }

    /**
     * Alias for {@link vec4.squaredLength}
     * @function
     */
    pub fn sqrLen(a: &Vector4) -> f32 {
        Vector4::squaredLength(a)
    }
}
