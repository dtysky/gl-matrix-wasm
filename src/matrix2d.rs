use wasm_bindgen::prelude::*;

use super::common::*;
use super::vector2::*;

#[wasm_bindgen]
pub struct Matrix2d(pub f32, pub f32, pub f32, pub f32, pub f32, pub f32);

#[wasm_bindgen]
impl Matrix2d {
    #[wasm_bindgen(getter)]
    pub fn elements(&self) -> Box<[f32]> {
        Box::new([self.0, self.1, self.2, self.3, self.4, self.5])
    }

    /**
     * 2x3 Matrix
     * @module mat2d
     *
     * @description
     * A mat2d contains six elements defined as:
     * <pre>
     * [a, b, c,
     *  d, tx, ty]
     * </pre>
     * This is a short form for the 3x3 matrix:
     * <pre>
     * [a, b, 0,
     *  c, d, 0,
     *  tx, ty, 1]
     * </pre>
     * The last column is ignored so the array is shorter and operations are faster.
     */

    /**
     * Creates a new identity mat2d
     *
     * @returns {mat2d} a new 2x3 matrix
     */
    pub fn create() -> Matrix2d {
        Matrix2d(1., 0., 0., 1., 0., 0.)
    }

    /**
     * Creates a new mat2d initialized with values from an existing matrix
     *
     * @param {mat2d} a matrix to clone
     * @returns {mat2d} a new 2x3 matrix
     */
    pub fn clone(a: &Matrix2d) -> Matrix2d {
        Matrix2d(a.0, a.1, a.2, a.3, a.4, a.5)
    }

    /**
     * Copy the values from one mat2d to another
     *
     * @param {mat2d} out the receiving matrix
     * @param {mat2d} a the source matrix
     * @returns {mat2d} out
     */
    pub fn copy(out: &mut Matrix2d, a: &Matrix2d) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.3;
        out.4 = a.4;
        out.5 = a.5;
    }

    /**
     * Set a mat2d to the identity matrix
     *
     * @param {mat2d} out the receiving matrix
     * @returns {mat2d} out
     */
    pub fn identity(out: &mut Matrix2d) {
        out.0 = 1.;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = 1.;
        out.4 = 0.;
        out.5 = 0.;
    }

    /**
     * Create a new mat2d with the given values
     *
     * @param {Number} a Component A (index 0)
     * @param {Number} b Component B (index 1)
     * @param {Number} c Component C (index 2)
     * @param {Number} d Component D (index 3)
     * @param {Number} tx Component TX (index 4)
     * @param {Number} ty Component TY (index 5)
     * @returns {mat2d} A new mat2d
     */
    pub fn fromValues(a: f32, b: f32, c: f32, d: f32, tx: f32, ty: f32) -> Matrix2d {
        Matrix2d(a, b, c, d, tx, ty)
    }

    /**
     * Set the components of a mat2d to the given values
     *
     * @param {mat2d} out the receiving matrix
     * @param {Number} a Component A (index 0)
     * @param {Number} b Component B (index 1)
     * @param {Number} c Component C (index 2)
     * @param {Number} d Component D (index 3)
     * @param {Number} tx Component TX (index 4)
     * @param {Number} ty Component TY (index 5)
     * @returns {mat2d} out
     */
    pub fn set(out: &mut Matrix2d, a: f32, b: f32, c: f32, d: f32, tx: f32, ty: f32) {
        out.0 = a;
        out.1 = b;
        out.2 = c;
        out.3 = d;
        out.4 = tx;
        out.5 = ty;
    }

    /**
     * Inverts a mat2d
     *
     * @param {mat2d} out the receiving matrix
     * @param {mat2d} a the source matrix
     * @returns {mat2d} out
     */
    pub fn invert(out: &mut Matrix2d, a: &Matrix2d) {
        let aa = a.0;
        let ab = a.1;
        let ac = a.2;
        let ad = a.3;
        let atx = a.4;
        let aty = a.5;

        let mut det = aa * ad - ab * ac;
        if (det < EPSILON) {
            return;
        }
        det = 1.0 / det;

        out.0 = ad * det;
        out.1 = -ab * det;
        out.2 = -ac * det;
        out.3 = aa * det;
        out.4 = (ac * aty - ad * atx) * det;
        out.5 = (ab * atx - aa * aty) * det;
    }

    /**
     * Calculates the determinant of a mat2d
     *
     * @param {mat2d} a the source matrix
     * @returns {Number} determinant of a
     */
    pub fn determinant(a: &Matrix2d) -> f32 {
        a.0 * a.3 - a.1 * a.2
    }

    /**
     * Multiplies two mat2d"s
     *
     * @param {mat2d} out the receiving matrix
     * @param {mat2d} a the first operand
     * @param {mat2d} b the second operand
     * @returns {mat2d} out
     */
    pub fn multiply(out: &mut Matrix2d, a: &Matrix2d, b: &Matrix2d) {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let a4 = a.4;
        let a5 = a.5;
        let b0 = b.0;
        let b1 = b.1;
        let b2 = b.2;
        let b3 = b.3;
        let b4 = b.4;
        let b5 = b.5;
        out.0 = a0 * b0 + a2 * b1;
        out.1 = a1 * b0 + a3 * b1;
        out.2 = a0 * b2 + a2 * b3;
        out.3 = a1 * b2 + a3 * b3;
        out.4 = a0 * b4 + a2 * b5 + a4;
        out.5 = a1 * b4 + a3 * b5 + a5;
    }

    /**
     * Rotates a mat2d by the given angle
     *
     * @param {mat2d} out the receiving matrix
     * @param {mat2d} a the matrix to rotate
     * @param {Number} rad the angle to rotate the matrix by
     * @returns {mat2d} out
     */
    pub fn rotate(out: &mut Matrix2d, a: &Matrix2d, rad: f32) {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let a4 = a.4;
        let a5 = a.5;
        let s = f32::sin(rad);
        let c = f32::cos(rad);
        out.0 = a0 * c + a2 * s;
        out.1 = a1 * c + a3 * s;
        out.2 = a0 * -s + a2 * c;
        out.3 = a1 * -s + a3 * c;
        out.4 = a4;
        out.5 = a5;
    }

    /**
     * Scales the mat2d by the dimensions in the given vec2
     *
     * @param {mat2d} out the receiving matrix
     * @param {mat2d} a the matrix to translate
     * @param {vec2} v the vec2 to scale the matrix by
     * @returns {mat2d} out
     **/
    pub fn scale(out: &mut Matrix2d, a: &Matrix2d, v: &Vector2) {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let a4 = a.4;
        let a5 = a.5;
        let v0 = v.0;
        let v1 = v.1;
        out.0 = a0 * v0;
        out.1 = a1 * v0;
        out.2 = a2 * v1;
        out.3 = a3 * v1;
        out.4 = a4;
        out.5 = a5;
    }

    /**
     * Translates the mat2d by the dimensions in the given vec2
     *
     * @param {mat2d} out the receiving matrix
     * @param {mat2d} a the matrix to translate
     * @param {vec2} v the vec2 to translate the matrix by
     * @returns {mat2d} out
     **/
    pub fn translate(out: &mut Matrix2d, a: &Matrix2d, v: &Vector2) {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let a4 = a.4;
        let a5 = a.5;
        let v0 = v.0;
        let v1 = v.1;
        out.0 = a0;
        out.1 = a1;
        out.2 = a2;
        out.3 = a3;
        out.4 = a0 * v0 + a2 * v1 + a4;
        out.5 = a1 * v0 + a3 * v1 + a5;
    }

    /**
     * Creates a matrix from a given angle
     * This is equivalent to (but much faster than):
     *
     *     mat2d.identity(dest);
     *     mat2d.rotate(dest, dest, rad);
     *
     * @param {mat2d} out mat2d receiving operation result
     * @param {Number} rad the angle to rotate the matrix by
     * @returns {mat2d} out
     */
    pub fn fromRotation(out: &mut Matrix2d, rad: f32) {
        let s = f32::sin(rad);
        let c = f32::cos(rad);
        out.0 = c;
        out.1 = s;
        out.2 = -s;
        out.3 = c;
        out.4 = 0.;
        out.5 = 0.;
    }

    /**
     * Creates a matrix from a vector scaling
     * This is equivalent to (but much faster than):
     *
     *     mat2d.identity(dest);
     *     mat2d.scale(dest, dest, vec);
     *
     * @param {mat2d} out mat2d receiving operation result
     * @param {vec2} v Scaling vector
     * @returns {mat2d} out
     */
    pub fn fromScaling(out: &mut Matrix2d, v: &Vector2) {
        out.0 = v.0;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = v.1;
        out.4 = 0.;
        out.5 = 0.;
    }

    /**
     * Creates a matrix from a vector translation
     * This is equivalent to (but much faster than):
     *
     *     mat2d.identity(dest);
     *     mat2d.translate(dest, dest, vec);
     *
     * @param {mat2d} out mat2d receiving operation result
     * @param {vec2} v Translation vector
     * @returns {mat2d} out
     */
    pub fn fromTranslation(out: &mut Matrix2d, v: &Vector2) {
        out.0 = 1.;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = 1.;
        out.4 = v.0;
        out.5 = v.1;
    }

    /**
     * Returns a string representation of a mat2d
     *
     * @param {mat2d} a matrix to represent as a string
     * @returns {String} string representation of the matrix
     */
    // pub fn str(a: &Matrix2d) {
    //   return "mat2d(" + a.0 + ", " + a.1 + ", " + a.2 + ", " +
    //           a.3 + ", " + a.4 + ", " + a.5 + ")";
    // }

    /**
     * Returns Frobenius norm of a mat2d
     *
     * @param {mat2d} a the matrix to calculate Frobenius norm of
     * @returns {Number} Frobenius norm
     */
    pub fn frob(a: &Matrix2d) -> f32 {
        (a.0.powi(2) + a.1.powi(2) + a.2.powi(2) + a.3.powi(2) + a.4.powi(2) + a.5.powi(2) + 1.)
            .sqrt()
    }

    /**
     * Adds two mat2d"s
     *
     * @param {mat2d} out the receiving matrix
     * @param {mat2d} a the first operand
     * @param {mat2d} b the second operand
     * @returns {mat2d} out
     */
    pub fn add(out: &mut Matrix2d, a: &Matrix2d, b: &Matrix2d) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
        out.2 = a.2 + b.2;
        out.3 = a.3 + b.3;
        out.4 = a.4 + b.4;
        out.5 = a.5 + b.5;
    }

    /**
     * Subtracts matrix b from matrix a
     *
     * @param {mat2d} out the receiving matrix
     * @param {mat2d} a the first operand
     * @param {mat2d} b the second operand
     * @returns {mat2d} out
     */
    pub fn subtract(out: &mut Matrix2d, a: &Matrix2d, b: &Matrix2d) {
        out.0 = a.0 - b.0;
        out.1 = a.1 - b.1;
        out.2 = a.2 - b.2;
        out.3 = a.3 - b.3;
        out.4 = a.4 - b.4;
        out.5 = a.5 - b.5;
    }

    /**
     * Multiply each element of the matrix by a scalar.
     *
     * @param {mat2d} out the receiving matrix
     * @param {mat2d} a the matrix to scale
     * @param {Number} b amount to scale the matrix"s elements by
     * @returns {mat2d} out
     */
    pub fn multiplyScalar(out: &mut Matrix2d, a: &Matrix2d, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
        out.2 = a.2 * b;
        out.3 = a.3 * b;
        out.4 = a.4 * b;
        out.5 = a.5 * b;
    }

    /**
     * Adds two mat2d"s after multiplying each element of the second operand by a scalar value.
     *
     * @param {mat2d} out the receiving vector
     * @param {mat2d} a the first operand
     * @param {mat2d} b the second operand
     * @param {Number} scale the amount to scale b"s elements by before adding
     * @returns {mat2d} out
     */
    pub fn multiplyScalarAndAdd(out: &mut Matrix2d, a: &Matrix2d, b: &Matrix2d, scale: f32) {
        out.0 = a.0 + (b.0 * scale);
        out.1 = a.1 + (b.1 * scale);
        out.2 = a.2 + (b.2 * scale);
        out.3 = a.3 + (b.3 * scale);
        out.4 = a.4 + (b.4 * scale);
        out.5 = a.5 + (b.5 * scale);
    }

    /**
     * Returns whether or not the matrices have exactly the same elements in the same position (when compared with ==)
     *
     * @param {mat2d} a The first matrix.
     * @param {mat2d} b The second matrix.
     * @returns {Boolean} True if the matrices are equal, false otherwise.
     */
    pub fn exactEquals(a: &Matrix2d, b: &Matrix2d) -> bool {
        a.0 == b.0 && a.1 == b.1 && a.2 == b.2 && a.3 == b.3 && a.4 == b.4 && a.5 == b.5
    }

    /**
     * Returns whether or not the matrices have approximately the same elements in the same position.
     *
     * @param {mat2d} a The first matrix.
     * @param {mat2d} b The second matrix.
     * @returns {Boolean} True if the matrices are equal, false otherwise.
     */
    pub fn equals(a: &Matrix2d, b: &Matrix2d) -> bool {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let a4 = a.4;
        let a5 = a.5;
        let b0 = b.0;
        let b1 = b.1;
        let b2 = b.2;
        let b3 = b.3;
        let b4 = b.4;
        let b5 = b.5;
        f32::abs(a0 - b0) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a0), f32::abs(b0)))
            && f32::abs(a1 - b1) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a1), f32::abs(b1)))
            && f32::abs(a2 - b2) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a2), f32::abs(b2)))
            && f32::abs(a3 - b3) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a3), f32::abs(b3)))
            && f32::abs(a4 - b4) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a4), f32::abs(b4)))
            && f32::abs(a5 - b5) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a5), f32::abs(b5)))
    }

    /**
     * Alias for {@link mat2d.multiply}
     * @function
     */
    pub fn mul(out: &mut Matrix2d, a: &Matrix2d, b: &Matrix2d) {
        Matrix2d::multiply(out, a, b);
    }

    /**
     * Alias for {@link mat2d.subtract}
     * @function
     */
    pub fn sub(out: &mut Matrix2d, a: &Matrix2d, b: &Matrix2d) {
        Matrix2d::subtract(out, a, b);
    }
}
