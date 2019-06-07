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

    /**
     * 2 Dimensional Vector
     * @module vec2
     */

    /**
     * Creates a new, empty vec2
     *
     * @returns {vec2} a new 2D vector
     */
    pub fn create() -> Vector2 {
        Vector2(0., 0.)
    }

    /**
     * Creates a new vec2 initialized with values from an existing vector
     *
     * @param {vec2} a vector to clone
     * @returns {vec2} a new 2D vector
     */
    pub fn clone(a: &Vector2) -> Vector2 {
        Vector2(a.0, a.1)
    }

    /**
     * Creates a new vec2 initialized with the given values
     *
     * @param {Number} x X component
     * @param {Number} y Y component
     * @returns {vec2} a new 2D vector
     */
    pub fn fromValues(x: f32, y: f32) -> Vector2 {
        Vector2(x, y)
    }

    /**
     * Copy the values from one vec2 to another
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the source vector
     * @returns {vec2} out
     */
    pub fn copy(out: &mut Vector2, a: &Vector2) {
        out.0 = a.0;
        out.1 = a.1;
    }

    /**
     * Set the components of a vec2 to the given values
     *
     * @param {vec2} out the receiving vector
     * @param {Number} x X component
     * @param {Number} y Y component
     * @returns {vec2} out
     */
    pub fn set(out: &mut Vector2, x: f32, y: f32) {
        out.0 = x;
        out.1 = y;
    }

    /**
     * Adds two vec2"s
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {vec2} out
     */
    pub fn add(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
    }

    /**
     * Subtracts vector b from vector a
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {vec2} out
     */
    pub fn subtract(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = a.0 - b.0;
        out.1 = a.1 - b.1;
    }

    /**
     * Multiplies two vec2"s
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {vec2} out
     */
    pub fn multiply(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = a.0 * b.0;
        out.1 = a.1 * b.1;
    }

    /**
     * Divides two vec2"s
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {vec2} out
     */
    pub fn divide(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = a.0 / b.0;
        out.1 = a.1 / b.1;
    }

    /**
     * f32::ceil the components of a vec2
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a vector to ceil
     * @returns {vec2} out
     */
    pub fn ceil(out: &mut Vector2, a: &Vector2) {
        out.0 = f32::ceil(a.0);
        out.1 = f32::ceil(a.1);
    }

    /**
     * f32::floor the components of a vec2
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a vector to floor
     * @returns {vec2} out
     */
    pub fn floor(out: &mut Vector2, a: &Vector2) {
        out.0 = f32::floor(a.0);
        out.1 = f32::floor(a.1);
    }

    /**
     * Returns the minimum of two vec2"s
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {vec2} out
     */
    pub fn min(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = f32::min(a.0, b.0);
        out.1 = f32::min(a.1, b.1);
    }

    /**
     * Returns the maximum of two vec2"s
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {vec2} out
     */
    pub fn max(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        out.0 = f32::max(a.0, b.0);
        out.1 = f32::max(a.1, b.1);
    }

    /**
     * f32::round the components of a vec2
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a vector to round
     * @returns {vec2} out
     */
    pub fn round(out: &mut Vector2, a: &Vector2) {
        out.0 = f32::round(a.0);
        out.1 = f32::round(a.1);
    }

    /**
     * Scales a vec2 by a scalar number
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the vector to scale
     * @param {Number} b amount to scale the vector by
     * @returns {vec2} out
     */
    pub fn scale(out: &mut Vector2, a: &Vector2, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
    }

    /**
     * Adds two vec2"s after scaling the second operand by a scalar value
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @param {Number} scale the amount to scale b by before adding
     * @returns {vec2} out
     */
    pub fn scaleAndAdd(out: &mut Vector2, a: &Vector2, b: &Vector2, scale: f32) {
        out.0 = a.0 + (b.0 * scale);
        out.1 = a.1 + (b.1 * scale);
    }

    /**
     * Calculates the euclidian distance between two vec2"s
     *
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {Number} distance between a and b
     */
    pub fn distance(a: &Vector2, b: &Vector2) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        f32::hypot(x, y)
    }

    /**
     * Calculates the squared euclidian distance between two vec2"s
     *
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {Number} squared distance between a and b
     */
    pub fn squaredDistance(a: &Vector2, b: &Vector2) -> f32 {
        let x = b.0 - a.0;
        let y = b.1 - a.1;
        x * x + y * y
    }

    /**
     * Calculates the length of a vec2
     *
     * @param {vec2} a vector to calculate length of
     * @returns {Number} length of a
     */
    pub fn length(a: &Vector2) -> f32 {
        let x = a.0;
        let y = a.1;
        f32::hypot(x, y)
    }

    /**
     * Calculates the squared length of a vec2
     *
     * @param {vec2} a vector to calculate squared length of
     * @returns {Number} squared length of a
     */
    pub fn squaredLength(a: &Vector2) -> f32 {
        let x = a.0;
        let y = a.1;
        x * x + y * y
    }

    /**
     * Negates the components of a vec2
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a vector to negate
     * @returns {vec2} out
     */
    pub fn negate(out: &mut Vector2, a: &Vector2) {
        out.0 = -a.0;
        out.1 = -a.1;
    }

    /**
     * Returns the inverse of the components of a vec2
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a vector to invert
     * @returns {vec2} out
     */
    pub fn inverse(out: &mut Vector2, a: &Vector2) {
        out.0 = 1.0 / a.0;
        out.1 = 1.0 / a.1;
    }

    /**
     * Normalize a vec2
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a vector to normalize
     * @returns {vec2} out
     */
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

    /**
     * Calculates the dot product of two vec2"s
     *
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {Number} dot product of a and b
     */
    pub fn dot(a: &Vector2, b: &Vector2) -> f32 {
        a.0 * b.0 + a.1 * b.1
    }

    /**
     * Computes the cross product of two vec2"s
     * Note that the cross product must by definition produce a 3D vector
     *
     * @param {vec3} out the receiving vector
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @returns {vec3} out
     */
    pub fn cross(out: &mut Vector3, a: &Vector2, b: &Vector2) {
        let z = a.0 * b.1 - a.1 * b.0;
        out.0 = 0.;
        out.1 = 0.;
        out.2 = z;
    }

    /**
     * Performs a linear interpolation between two vec2"s
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the first operand
     * @param {vec2} b the second operand
     * @param {Number} t interpolation amount, in the range [0-1], between the two inputs
     * @returns {vec2} out
     */
    pub fn lerp(out: &mut Vector2, a: &Vector2, b: &Vector2, t: f32) {
        let ax = a.0;
        let ay = a.1;
        out.0 = ax + t * (b.0 - ax);
        out.1 = ay + t * (b.1 - ay);
    }

    /**
     * Generates a random vector with the given scale
     *
     * @param {vec2} out the receiving vector
     * @param {Number} [scale] Length of the resulting vector. If ommitted, a unit vector will be returned
     * @returns {vec2} out
     */
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

    /**
     * Transforms the vec2 with a mat2
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the vector to transform
     * @param {mat2} m matrix to transform with
     * @returns {vec2} out
     */
    pub fn transformMat2(out: &mut Vector2, a: &Vector2, m: &Matrix2) {
        let x = a.0;
        let y = a.1;
        out.0 = m.0 * x + m.2 * y;
        out.1 = m.1 * x + m.3 * y;
    }

    /**
     * Transforms the vec2 with a mat2d
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the vector to transform
     * @param {mat2d} m matrix to transform with
     * @returns {vec2} out
     */
    pub fn transformMat2d(out: &mut Vector2, a: &Vector2, m: &Matrix2d) {
        let x = a.0;
        let y = a.1;
        out.0 = m.0 * x + m.2 * y + m.4;
        out.1 = m.1 * x + m.3 * y + m.5;
    }

    /**
     * Transforms the vec2 with a mat3
     * 3rd vector component is implicitly "1"
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the vector to transform
     * @param {mat3} m matrix to transform with
     * @returns {vec2} out
     */
    pub fn transformMat3(out: &mut Vector2, a: &Vector2, m: &Matrix3) {
        let x = a.0;
        let y = a.1;
        out.0 = m.0 * x + m.3 * y + m.6;
        out.1 = m.1 * x + m.4 * y + m.7;
    }

    /**
     * Transforms the vec2 with a mat4
     * 3rd vector component is implicitly "0"
     * 4th vector component is implicitly "1"
     *
     * @param {vec2} out the receiving vector
     * @param {vec2} a the vector to transform
     * @param {mat4} m matrix to transform with
     * @returns {vec2} out
     */
    pub fn transformMat4(out: &mut Vector2, a: &Vector2, m: &Matrix4) {
        let x = a.0;
        let y = a.1;
        out.0 = m.0 * x + m.4 * y + m.12;
        out.1 = m.1 * x + m.5 * y + m.13;
    }

    /**
     * Rotate a 2D vector
     * @param {vec2} out The receiving vec2
     * @param {vec2} a The vec2 point to rotate
     * @param {vec2} b The origin of the rotation
     * @param {Number} c The angle of rotation
     * @returns {vec2} out
     */
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

    /**
     * Get the angle between two 2D vectors
     * @param {vec2} a The first operand
     * @param {vec2} b The second operand
     * @returns {Number} The angle in radians
     */
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

    /**
     * Set the components of a vec2 to zero
     *
     * @param {vec2} out the receiving vector
     * @returns {vec2} out
     */
    pub fn zero(out: &mut Vector2) {
        out.0 = 0.0;
        out.1 = 0.0;
    }

    /**
     * Returns a string representation of a vector
     *
     * @param {vec2} a vector to represent as a string
     * @returns {String} string representation of the vector
     */
    // pub fn str(a: &Vector2) {
    //   return "vec2(" + a.0 + ", " + a.1 + ")";
    // }

    /**
     * Returns whether or not the vectors exactly have the same elements in the same position (when compared with ==)
     *
     * @param {vec2} a The first vector.
     * @param {vec2} b The second vector.
     * @returns {Boolean} True if the vectors are equal, false otherwise.
     */
    pub fn exactEquals(a: &Vector2, b: &Vector2) -> bool {
        a.0 == b.0 && a.1 == b.1
    }

    /**
     * Returns whether or not the vectors have approximately the same elements in the same position.
     *
     * @param {vec2} a The first vector.
     * @param {vec2} b The second vector.
     * @returns {Boolean} True if the vectors are equal, false otherwise.
     */
    pub fn equals(a: &Vector2, b: &Vector2) -> bool {
        let a0 = a.0;
        let a1 = a.1;
        let b0 = b.0;
        let b1 = b.1;
        f32::abs(a0 - b0) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a0), f32::abs(b0)))
            && f32::abs(a1 - b1) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a1), f32::abs(b1)))
    }

    /**
     * Alias for {@link vec2.length}
     * @function
     */
    pub fn len(a: &Vector2) -> f32 {
        Vector2::length(a)
    }

    /**
     * Alias for {@link vec2.subtract}
     * @function
     */
    pub fn sub(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        Vector2::subtract(out, a, b);
    }

    /**
     * Alias for {@link vec2.multiply}
     * @function
     */
    pub fn mul(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        Vector2::multiply(out, a, b);
    }

    /**
     * Alias for {@link vec2.divide}
     * @function
     */
    pub fn div(out: &mut Vector2, a: &Vector2, b: &Vector2) {
        Vector2::divide(out, a, b);
    }

    /**
     * Alias for {@link vec2.distance}
     * @function
     */
    pub fn dist(a: &Vector2, b: &Vector2) -> f32 {
        Vector2::distance(a, b)
    }

    /**
     * Alias for {@link vec2.squaredDistance}
     * @function
     */
    pub fn sqrDist(a: &Vector2, b: &Vector2) -> f32 {
        Vector2::squaredDistance(a, b)
    }

    /**
     * Alias for {@link vec2.squaredLength}
     * @function
     */
    pub fn sqrLen(a: &Vector2) -> f32 {
        Vector2::squaredLength(a)
    }
}
