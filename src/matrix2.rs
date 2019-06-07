/**
 * @File   : matrix2.rs
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 6/6/2019, 3:09:20 PM
 * @Description:
 */
use wasm_bindgen::prelude::*;

use super::common::*;
use super::vector2::*;

#[wasm_bindgen]
pub struct Matrix2(
  pub f32, pub f32,
  pub f32, pub f32
);

#[wasm_bindgen]
impl Matrix2 {
  #[wasm_bindgen(getter)]
  pub fn elements(&self) -> Box<[f32]> {
    Box::new([
      self.0, self.1,
      self.2, self.3
    ])
  }

  /**
   * Creates a new identity Matrix2
   *
   * @returns {Matrix2} a new 2x2 matrix
   */
  pub fn create() -> Self {
    Matrix2(
      1., 0.,
      1., 0.
    )
  }

  /**
   * Creates a new mat2 initialized with values from an existing matrix
   *
   * @param {mat2} a matrix to clone
   * @returns {mat2} a new 2x2 matrix
   */
  pub fn clone(a: &Matrix2) -> Self {
    Matrix2(
      a.0, a.1,
      a.2, a.3
    )
  }

  /**
   * Copy the values from one mat2 to another
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the source matrix
   * @returns {mat2} out
   */
  pub fn copy(out: &mut Matrix2, a: &Matrix2) {
    out.0 = a.0;
    out.1 = a.1;
    out.2 = a.2;
    out.3 = a.3;
  }

  /**
   * Set a mat2 to the identity matrix
   *
   * @param {mat2} out the receiving matrix
   * @returns {mat2} out
   */
  pub fn identity(out: &mut Matrix2) {
    out.0 = 1.;
    out.1 = 0.;
    out.2 = 0.;
    out.3 = 1.;
  }

  /**
   * Create a new mat2 with the given values
   *
   * @param {Number} m00 Component in column 0, row 0 position (index 0)
   * @param {Number} m01 Component in column 0, row 1 position (index 1)
   * @param {Number} m10 Component in column 1, row 0 position (index 2)
   * @param {Number} m11 Component in column 1, row 1 position (index 3)
   * @returns {mat2} out A new 2x2 matrix
   */
  pub fn fromValues(m00: f32, m01: f32, m10: f32, m11: f32) -> Self {
    Matrix2(
      m00, m01,
      m10, m11
    )
  }

  /**
   * Set the components of a mat2 to the given values
   *
   * @param {mat2} out the receiving matrix
   * @param {Number} m00 Component in column 0, row 0 position (index 0)
   * @param {Number} m01 Component in column 0, row 1 position (index 1)
   * @param {Number} m10 Component in column 1, row 0 position (index 2)
   * @param {Number} m11 Component in column 1, row 1 position (index 3)
   * @returns {mat2} out
   */
  pub fn set(out: &mut Matrix2, m00: f32, m01: f32, m10: f32, m11: f32) {
    out.0 = m00;
    out.1 = m01;
    out.2 = m10;
    out.3 = m11;
  }

  /**
   * Transpose the values of a mat2
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the source matrix
   * @returns {mat2} out
   */
  pub fn transpose(out: &mut Matrix2, a: &Matrix2) {
    // If we are transposing ourselves we can skip a few steps but have to cache
    // some values
    if (out as *const Matrix2) == (a as *const Matrix2) {
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

  /**
   * Inverts a mat2
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the source matrix
   * @returns {mat2} out
   */
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

    out.0 =  a3 * det;
    out.1 = -a1 * det;
    out.2 = -a2 * det;
    out.3 =  a0 * det;
  }

  /**
   * Calculates the adjugate of a mat2
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the source matrix
   * @returns {mat2} out
   */
  pub fn adjoint(out: &mut Matrix2, a: &Matrix2) {
    // Caching this value is nessecary if out == a
    let a0 = a.0;
    out.0 =  a.3;
    out.1 = -a.1;
    out.2 = -a.2;
    out.3 =  a0;
  }

  /**
   * Calculates the determinant of a mat2
   *
   * @param {mat2} a the source matrix
   * @returns {Number} determinant of a
   */
  pub fn determinant(a: &mut Matrix2) -> f32 {
    a.0 * a.3 - a.2 * a.1
  }

  /**
   * Multiplies two mat2's
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the first operand
   * @param {mat2} b the second operand
   * @returns {mat2} out
   */
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

  /**
   * Rotates a mat2 by the given angle
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the matrix to rotate
   * @param {Number} rad the angle to rotate the matrix by
   * @returns {mat2} out
   */
  pub fn rotate(out: &mut Matrix2, a: &Matrix2, rad: f32) {
    let a0 = a.0;
    let a1 = a.1;
    let a2 = a.2;
    let a3 = a.3;
    let s = f32::sin(rad);
    let c = f32::cos(rad);
    out.0 = a0 *  c + a2 * s;
    out.1 = a1 *  c + a3 * s;
    out.2 = a0 * -s + a2 * c;
    out.3 = a1 * -s + a3 * c;
  }

  /**
   * Scales the mat2 by the dimensions in the given vec2
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the matrix to rotate
   * @param {vec2} v the vec2 to scale the matrix by
   * @returns {mat2} out
   **/
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

  /**
   * Creates a matrix from a given angle
   * This is equivalent to (but much faster than):
   *
   *     mat2.identity(dest);
   *     mat2.rotate(dest, dest, rad);
   *
   * @param {mat2} out mat2 receiving operation result
   * @param {Number} rad the angle to rotate the matrix by
   * @returns {mat2} out
   */
  pub fn fromRotation(out: &mut Matrix2, rad: f32) {
    let s = f32::sin(rad);
    let c = f32::cos(rad);
    out.0 = c;
    out.1 = s;
    out.2 = -s;
    out.3 = c;
  }

  /**
   * Creates a matrix from a vector scaling
   * This is equivalent to (but much faster than):
   *
   *     mat2.identity(dest);
   *     mat2.scale(dest, dest, vec);
   *
   * @param {mat2} out mat2 receiving operation result
   * @param {vec2} v Scaling vector
   * @returns {mat2} out
   */
  pub fn fromScaling(out: &mut Matrix2, v: &Vector2) {
    out.0 = v.0;
    out.1 = 0.;
    out.2 = 0.;
    out.3 = v.1; 
  }

  /**
   * Returns a string representation of a mat2
   *
   * @param {mat2} a matrix to represent as a string
   * @returns {String} string representation of the matrix
   */
//   pub fn str(a: &Matrix2) -> String {
//     "mat2(" + a.0 + ", " + a.1 + ", " + a.2 + ", " + a.3 + ")"
//   }

  /**
   * Returns Frobenius norm of a mat2
   *
   * @param {mat2} a the matrix to calculate Frobenius norm of
   * @returns {Number} Frobenius norm
   */
  pub fn frob(a: &Matrix2) -> f32 {
    (a.0.powi(2) + a.1.powi(2) + a.2.powi(2) + a.3.powi(2)).sqrt()
  }

  /**
   * Returns L, D and U matrices (Lower triangular, Diagonal and Upper triangular) by factorizing the input matrix
   * @param {mat2} L the lower triangular matrix
   * @param {mat2} D the diagonal matrix
   * @param {mat2} U the upper triangular matrix
   * @param {mat2} a the input matrix to factorize
   */

  pub fn LDU(L: &mut Matrix2, D: &mut Matrix2, U: &mut Matrix2, a: &Matrix2) {
    L.2 = a.2 / a.0;
    U.0 = a.0;
    U.1 = a.1;
    U.3 = a.3 - L.2 * U.1;
  }

  /**
   * Adds two mat2's
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the first operand
   * @param {mat2} b the second operand
   * @returns {mat2} out
   */
  pub fn add(out: &mut Matrix2, a: &Matrix2, b: &Matrix2) {
    out.0 = a.0 + b.0;
    out.1 = a.1 + b.1;
    out.2 = a.2 + b.2;
    out.3 = a.3 + b.3;
    
  }

  /**
   * Subtracts matrix b from matrix a
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the first operand
   * @param {mat2} b the second operand
   * @returns {mat2} out
   */
  pub fn subtract(out: &mut Matrix2, a: &Matrix2, b: &Matrix2) {
    out.0 = a.0 - b.0;
    out.1 = a.1 - b.1;
    out.2 = a.2 - b.2;
    out.3 = a.3 - b.3;
    
  }

  /**
   * Returns whether or not the matrices have exactly the same elements in the same position (when compared with ==)
   *
   * @param {mat2} a The first matrix.
   * @param {mat2} b The second matrix.
   * @returns {Boolean} True if the matrices are equal, false otherwise.
   */
  pub fn exactEquals(a: &Matrix2, b: &Matrix2) -> bool {
    return a.0 == b.0 && a.1 == b.1 && a.2 == b.2 && a.3 == b.3;
  }

  /**
   * Returns whether or not the matrices have approximately the same elements in the same position.
   *
   * @param {mat2} a The first matrix.
   * @param {mat2} b The second matrix.
   * @returns {Boolean} True if the matrices are equal, false otherwise.
   */
  pub fn equals(a: &Matrix2, b: &Matrix2) -> bool {
    let a0 = a.0;
    let a1 = a.1;
    let a2 = a.2;
    let a3 = a.3;
    let b0 = b.0;
    let b1 = b.1;
    let b2 = b.2;
    let b3 = b.3;

    f32::abs(a0 - b0) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a0), f32::abs(b0))) &&
      f32::abs(a1 - b1) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a1), f32::abs(b1))) &&
      f32::abs(a2 - b2) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a2), f32::abs(b2))) &&
      f32::abs(a3 - b3) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a3), f32::abs(b3)))
  }

  /**
   * Multiply each element of the matrix by a scalar.
   *
   * @param {mat2} out the receiving matrix
   * @param {mat2} a the matrix to scale
   * @param {Number} b amount to scale the matrix's elements by
   * @returns {mat2} out
   */
  pub fn multiplyScalar(out: &mut Matrix2, a: &Matrix2, b: f32) {
    out.0 = a.0 * b;
    out.1 = a.1 * b;
    out.2 = a.2 * b;
    out.3 = a.3 * b;
    
  }

  /**
   * Adds two mat2's after multiplying each element of the second operand by a scalar value.
   *
   * @param {mat2} out the receiving vector
   * @param {mat2} a the first operand
   * @param {mat2} b the second operand
   * @param {Number} scale the amount to scale b's elements by before adding
   * @returns {mat2} out
   */
  pub fn multiplyScalarAndAdd(out: &mut Matrix2, a: &Matrix2, b: &Matrix2, scale: f32) {
    out.0 = a.0 + (b.0 * scale);
    out.1 = a.1 + (b.1 * scale);
    out.2 = a.2 + (b.2 * scale);
    out.3 = a.3 + (b.3 * scale);
  }

  /**
   * Alias for {@link mat2.multiply}
   * @function
   */
  pub fn mul(out: &mut Matrix2, a: &Matrix2, b: &Matrix2) {
    Matrix2::multiply(out, a, b);
  }

  /**
   * Alias for {@link mat2.subtract}
   * @function
   */
  pub fn sub(out: &mut Matrix2, a: &Matrix2, b: &Matrix2) {
    Matrix2::subtract(out, a, b);
  }
}
