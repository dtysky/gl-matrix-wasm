use wasm_bindgen::prelude::*;
  
use super::common::*;
use super::matrix3::*;
use super::vector3::*;
use super::vector4::*;

#[wasm_bindgen]
pub struct Quaternion(
  pub f32,
pub f32,
pub f32,
pub f32
);

#[wasm_bindgen]
impl Matrix2 {
  #[wasm_bindgen(getter)]
  pub fn elements(&self) -> Box<[f32]> {
    Box::new([
      self.0,
self.1,
self.2,
self.3
    ])
  }

/**
 * Quaternion
 * @module quat
 */

/**
 * Creates a new identity quat
 *
 * @returns {quat} a new quaternion
 */
pub fn create()  -> Quaternion {
  Quaternion(0., 0., 0., 1.)
}

/**
 * Set a quat to the identity quaternion
 *
 * @param {quat} out the receiving quaternion
 * @returns {quat} out
 */
pub fn identity(out: &mut Quaternion) {
  out.0 = 0;
  out.1 = 0;
  out.2 = 0;
  out.3 = 1;
  }

/**
 * Sets a quat from the given angle and rotation axis,
 * then returns it.
 *
 * @param {quat} out the receiving quaternion
 * @param {vec3} axis the axis around which to rotate
 * @param {Number} rad the angle in radians
 * @returns {quat} out
 **/
pub fn setAxisAngle(out: &mut Quaternion, axis: &Vector3, rad: f32) {
  rad = rad * 0.5;
  let s = f32::sin(rad);
  out.0 = s * axis.0;
  out.1 = s * axis.1;
  out.2 = s * axis.2;
  out.3 = f32::cos(rad);
  }

/**
 * Gets the rotation axis and angle for a given
 *  quaternion. If a quaternion is created with
 *  setAxisAngle, this method will return the same
 *  values as providied in the original parameter list
 *  OR functionally equivalent values.
 * Example: The quaternion formed by axis [0, 0, 1] and
 *  angle -90 is the same as the quaternion formed by
 *  [0, 0, 1] and 270. This method favors the latter.
 * @param  {vec3} out_axis  Vector receiving the axis of rotation
 * @param  {quat} q     Quaternion to be decomposed
 * @return {Number}     Angle, in radians, of the rotation
 */
pub fn getAxisAngle(out_axis: &Vector3, q: &Quaternion) {
  let rad = f32::acos(q.3) * 2.0;
  let s = f32::sin(rad / 2.0);
  if (s > EPSILON) {
    out_axis.0 = q.0 / s;
    out_axis.1 = q.1 / s;
    out_axis.2 = q.2 / s;
  } else {
    // If s is zero, return any axis (no rotation - axis does not matter)
    out_axis.0 = 1;
    out_axis.1 = 0;
    out_axis.2 = 0;
  }
  return rad;
}

/**
 * Multiplies two quat"s
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a the first operand
 * @param {quat} b the second operand
 * @returns {quat} out
 */
pub fn multiply(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  let ax=a.0;
let ay=a.1;
let az=a.2;
let aw=a.3;
  let bx=b.0;
let by=b.1;
let bz=b.2;
let bw=b.3;

  out.0 = ax * bw + aw * bx + ay * bz - az * by;
  out.1 = ay * bw + aw * by + az * bx - ax * bz;
  out.2 = az * bw + aw * bz + ax * by - ay * bx;
  out.3 = aw * bw - ax * bx - ay * by - az * bz;
  }

/**
 * Rotates a quaternion by the given angle about the X axis
 *
 * @param {quat} out quat receiving operation result
 * @param {quat} a quat to rotate
 * @param {number} rad angle (in radians) to rotate
 * @returns {quat} out
 */
pub fn rotateX(out: &mut Quaternion, a: &Quaternion, rad: &undefined) {
  rad *= 0.5;

  let ax=a.0;
let ay=a.1;
let az=a.2;
let aw=a.3;
  let bx=f32::sin(rad);
let bw=f32::cos(rad);

  out.0 = ax * bw + aw * bx;
  out.1 = ay * bw + az * bx;
  out.2 = az * bw - ay * bx;
  out.3 = aw * bw - ax * bx;
  }

/**
 * Rotates a quaternion by the given angle about the Y axis
 *
 * @param {quat} out quat receiving operation result
 * @param {quat} a quat to rotate
 * @param {number} rad angle (in radians) to rotate
 * @returns {quat} out
 */
pub fn rotateY(out: &mut Quaternion, a: &Quaternion, rad: &undefined) {
  rad *= 0.5;

  let ax=a.0;
let ay=a.1;
let az=a.2;
let aw=a.3;
  let by=f32::sin(rad);
let bw=f32::cos(rad);

  out.0 = ax * bw - az * by;
  out.1 = ay * bw + aw * by;
  out.2 = az * bw + ax * by;
  out.3 = aw * bw - ay * by;
  }

/**
 * Rotates a quaternion by the given angle about the Z axis
 *
 * @param {quat} out quat receiving operation result
 * @param {quat} a quat to rotate
 * @param {number} rad angle (in radians) to rotate
 * @returns {quat} out
 */
pub fn rotateZ(out: &mut Quaternion, a: &Quaternion, rad: &undefined) {
  rad *= 0.5;

  let ax=a.0;
let ay=a.1;
let az=a.2;
let aw=a.3;
  let bz=f32::sin(rad);
let bw=f32::cos(rad);

  out.0 = ax * bw + ay * bz;
  out.1 = ay * bw - ax * bz;
  out.2 = az * bw + aw * bz;
  out.3 = aw * bw - az * bz;
  }

/**
 * Calculates the W component of a quat from the X, Y, and Z components.
 * Assumes that quaternion is 1 unit in length.
 * Any existing W component will be ignored.
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a quat to calculate W component of
 * @returns {quat} out
 */
pub fn calculateW(out: &mut Quaternion, a: &Quaternion) {
  let x=a.0;
let y=a.1;
let z=a.2;

  out.0 = x;
  out.1 = y;
  out.2 = z;
  out.3 = f32::sqrt(f32::abs(1.0 - x * x - y * y - z * z));
  }

/**
 * Performs a spherical linear interpolation between two quat
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a the first operand
 * @param {quat} b the second operand
 * @param {Number} t interpolation amount, in the range [0-1], between the two inputs
 * @returns {quat} out
 */
pub fn slerp(out: &mut Quaternion, a: &Quaternion, b: &Quaternion, t: f32) {
  // benchmarks:
  //    http://jsperf.com/quaternion-slerp-implementations
  let ax=a.0;
let ay=a.1;
let az=a.2;
let aw=a.3;
  let bx=b.0;
let by=b.1;
let bz=b.2;
let bw=b.3;

  let mut omega = 0.;
  let mut cosom = 0.;
  let mut sinom = 0.;
  let mut scale0 = 0.;
  let mut scale1 = 0.;

  // calc cosine
  cosom = ax * bx + ay * by + az * bz + aw * bw;
  // adjust signs (if necessary)
  if ( cosom < 0.0 ) {
    cosom = -cosom;
    bx = - bx;
    by = - by;
    bz = - bz;
    bw = - bw;
  }
  // calculate coefficients
  if ( (1.0 - cosom) > EPSILON ) {
    // standard case (slerp)
    omega  = f32::acos(cosom);
    sinom  = f32::sin(omega);
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

/**
 * Generates a random quaternion
 *
 * @param {quat} out the receiving quaternion
 * @returns {quat} out
 */
pub fn random(out: &mut Quaternion) {
  // Implementation of http://planning.cs.uiuc.edu/node198.html
  // TODO: Calling random 3 times is probably not the fastest solution
  let u1 = RANDOM();
  let u2 = RANDOM();
  let u3 = RANDOM();

  let sqrt1MinusU1 = f32::sqrt(1 - u1);
  let sqrtU1 = f32::sqrt(u1);

  out.0 = sqrt1MinusU1 * f32::sin(2.0 * f32::PI * u2);
  out.1 = sqrt1MinusU1 * f32::cos(2.0 * f32::PI * u2);
  out.2 = sqrtU1 * f32::sin(2.0 * f32::PI * u3);
  out.3 = sqrtU1 * f32::cos(2.0 * f32::PI * u3);
  }

/**
 * Calculates the inverse of a quat
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a quat to calculate inverse of
 * @returns {quat} out
 */
pub fn invert(out: &mut Quaternion, a: &Quaternion) {
  let a0=a.0;
let a1=a.1;
let a2=a.2;
let a3=a.3;
  let dot = a0*a0 + a1*a1 + a2*a2 + a3*a3;
  let invDot = if dot == 0. { 0. } else { 1. / dot } ;

  // TODO: Would be faster to return [0,0,0,0] immediately if dot == 0

  out.0 = -a0*invDot;
  out.1 = -a1*invDot;
  out.2 = -a2*invDot;
  out.3 = a3*invDot;
  }

/**
 * Calculates the conjugate of a quat
 * If the quaternion is normalized, this function is faster than quat.inverse and produces the same result.
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a quat to calculate conjugate of
 * @returns {quat} out
 */
pub fn conjugate(out: &mut Quaternion, a: &Quaternion) {
  out.0 = -a.0;
  out.1 = -a.1;
  out.2 = -a.2;
  out.3 = a.3;
  }

/**
 * Creates a quaternion from the given 3x3 rotation matrix.
 *
 * NOTE: The resultant quaternion is not normalized, so you should be sure
 * to renormalize the quaternion yourself where necessary.
 *
 * @param {quat} out the receiving quaternion
 * @param {mat3} m rotation matrix
 * @returns {quat} out
 * @function
 */
pub fn fromMat3(out: &mut Quaternion, m: &Matrix3) {
  // Algorithm in Ken Shoemake"s article in 1987 SIGGRAPH course notes
  // article "Quaternion Calculus and Fast Animation".
   let fTrace = m.0 + m.4 + m.8;
  let fRoot;

if ( fTrace > 0.0 ) {
  // |w| > 1/2, may as well choose w > 1/2
  fRoot=f32::sqrt(fTrace+1.0);  // 2w
    out.3 = 0.5 * fRoot;
    fRoot = 0.5/fRoot;  // 1/(4w)
    out.0 = (m[5]-m.7)*fRoot;
    out.1 = (m[6]-m.2)*fRoot;
    out.2 = (m[1]-m.3)*fRoot;
  } else {
    // |w| <= 1/2
    let i = 0;
    if ( m.4 > m.0 ) {
      i = 1;
    }
    if ( m.8 > m[i*3+i] ) {

      i = 2;
    }
    let j = (i+1)%3;
    let k = (i+2)%3;

    fRoot = f32::sqrt(m[i*3+i]-m[j*3+j]-m[k*3+k] + 1.0);
    out[i] = 0.5 * fRoot;
    fRoot = 0.5 / fRoot;
    out.3 = (m[j*3+k] - m[k*3+j]) * fRoot;
    out[j] = (m[j*3+i] + m[i*3+j]) * fRoot;
    out[k] = (m[k*3+i] + m[i*3+k]) * fRoot;
  }

  }

/**
 * Creates a quaternion from the given euler angle x, y, z.
 *
 * @param {quat} out the receiving quaternion
 * @param {x} Angle to rotate around X axis in degrees.
 * @param {y} Angle to rotate around Y axis in degrees.
 * @param {z} Angle to rotate around Z axis in degrees.
 * @returns {quat} out
 * @function
 */
pub fn fromEuler(out: &mut Quaternion, Angle: &undefined, Angle: &undefined, Angle: &undefined) {
    let halfToRad = 0.5 * f32::PI / 180.0;
    x *= halfToRad;
    y *= halfToRad;
    z *= halfToRad;

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

/**
 * Returns a string representation of a quatenion
 *
 * @param {quat} a vector to represent as a string
 * @returns {String} string representation of the vector
 */
pub fn str(a: &Quaternion) {
  return "quat(" + a.0 + ", " + a.1 + ", " + a.2 + ", " + a.3 + ")";
}

/**
 * Creates a new quat initialized with values from an existing quaternion
 *
 * @param {quat} a quaternion to clone
 * @returns {quat} a new quaternion
 * @function
 */
pub fn clone(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.clone(out, a, b);
}

/**
 * Creates a new quat initialized with the given values
 *
 * @param {Number} x X component
 * @param {Number} y Y component
 * @param {Number} z Z component
 * @param {Number} w W component
 * @returns {quat} a new quaternion
 * @function
 */
pub fn fromValues(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.fromValues(out, a, b);
}

/**
 * Copy the values from one quat to another
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a the source quaternion
 * @returns {quat} out
 * @function
 */
pub fn copy(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.copy(out, a, b);
}

/**
 * Set the components of a quat to the given values
 *
 * @param {quat} out the receiving quaternion
 * @param {Number} x X component
 * @param {Number} y Y component
 * @param {Number} z Z component
 * @param {Number} w W component
 * @returns {quat} out
 * @function
 */
pub fn set(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.set(out, a, b);
}

/**
 * Adds two quat"s
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a the first operand
 * @param {quat} b the second operand
 * @returns {quat} out
 * @function
 */
pub fn add(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.add(out, a, b);
}

/**
 * Alias for {@link quat.multiply}
 * @function
 */
pub fn mul(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::multiply(out, a, b);
}

/**
 * Scales a quat by a scalar number
 *
 * @param {quat} out the receiving vector
 * @param {quat} a the vector to scale
 * @param {Number} b amount to scale the vector by
 * @returns {quat} out
 * @function
 */
pub fn scale(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.scale(out, a, b);
}

/**
 * Calculates the dot product of two quat"s
 *
 * @param {quat} a the first operand
 * @param {quat} b the second operand
 * @returns {Number} dot product of a and b
 * @function
 */
pub fn dot(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.dot(out, a, b);
}

/**
 * Performs a linear interpolation between two quat"s
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a the first operand
 * @param {quat} b the second operand
 * @param {Number} t interpolation amount, in the range [0-1], between the two inputs
 * @returns {quat} out
 * @function
 */
pub fn lerp(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.lerp(out, a, b);
}

/**
 * Calculates the length of a quat
 *
 * @param {quat} a vector to calculate length of
 * @returns {Number} length of a
 */
pub fn length(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.length(out, a, b);
}

/**
 * Alias for {@link quat.length}
 * @function
 */
pub fn len(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::length(out, a, b);
}

/**
 * Calculates the squared length of a quat
 *
 * @param {quat} a vector to calculate squared length of
 * @returns {Number} squared length of a
 * @function
 */
pub fn squaredLength(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.squaredLength(out, a, b);
}

/**
 * Alias for {@link quat.squaredLength}
 * @function
 */
pub fn sqrLen(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::squaredLength(out, a, b);
}

/**
 * Normalize a quat
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a quaternion to normalize
 * @returns {quat} out
 * @function
 */
pub fn normalize(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.normalize(out, a, b);
}

/**
 * Returns whether or not the quaternions have exactly the same elements in the same position (when compared with ==)
 *
 * @param {quat} a The first quaternion.
 * @param {quat} b The second quaternion.
 * @returns {Boolean} True if the vectors are equal, false otherwise.
 */
pub fn exactEquals(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.exactEquals(out, a, b);
}

/**
 * Returns whether or not the quaternions have approximately the same elements in the same position.
 *
 * @param {quat} a The first vector.
 * @param {quat} b The second vector.
 * @returns {Boolean} True if the vectors are equal, false otherwise.
 */
pub fn equals(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  Quaternion::vec4.equals(out, a, b);
}

/**
 * Sets a quaternion to represent the shortest rotation from one
 * vector to another.
 *
 * Both vectors are assumed to be unit length.
 *
 * @param {quat} out the receiving quaternion.
 * @param {vec3} a the initial vector
 * @param {vec3} b the destination vector
 * @returns {quat} out
 */
pub fn rotationTo(out: &mut Quaternion, a: &Quaternion, b: &Quaternion) {
  let tmpvec3 = Vector3::create();
  let xUnitVec3 = Vector3::fromValues(1,0,0);
  let yUnitVec3 = Vector3::fromValues(0,1,0);

  let dot = Vector3::dot(a, b);
  if (dot < -0.999999) {
    Vector3::cross(tmpvec3, xUnitVec3, a);
    if (Vector3::len(tmpvec3) < 0.000001) {
      Vector3::cross(tmpvec3, yUnitVec3, a);
    }
    Vector3::normalize(tmpvec3, tmpvec3);
    setAxisAngle(out, tmpvec3, f32::PI);
        } else if (dot > 0.999999) {
    out.0 = 0;
    out.1 = 0;
    out.2 = 0;
    out.3 = 1;
        } else {
    Vector3::cross(tmpvec3, a, b);
    out.0 = tmpvec3.0;
    out.1 = tmpvec3.1;
    out.2 = tmpvec3.2;
    out.3 = 1 + dot;
    return normalize(out, out);
  }
}

/**
 * Performs a spherical linear interpolation with two control points
 *
 * @param {quat} out the receiving quaternion
 * @param {quat} a the first operand
 * @param {quat} b the second operand
 * @param {quat} c the third operand
 * @param {quat} d the fourth operand
 * @param {Number} t interpolation amount, in the range [0-1], between the two inputs
 * @returns {quat} out
 */
pub fn sqlerp(out: &mut Quaternion, a: &Quaternion, b: &Quaternion, c: &Quaternion, d: &Quaternion, t: f32) {
  let temp1 = Quaternion::create();
  let temp2 = Quaternion::create();

  Quaternion::slerp(temp1, a, d, t);
  Quaternion::slerp(temp2, b, c, t);
  Quaternion::slerp(out, temp1, temp2, 2 * t * (1 - t));
}

/**
 * Sets the specified quaternion with values corresponding to the given
 * axes. Each axis is a vec3 and is expected to be unit length and
 * perpendicular to all other specified axes.
 *
 * @param {vec3} view  the vector representing the viewing direction
 * @param {vec3} right the vector representing the local "right" direction
 * @param {vec3} up    the vector representing the local "up" direction
 * @returns {quat} out
 */
pub fn setAxes(out: &mut Quaternion, view: &Vector3, right: &Vector3, up: &Vector3) {
  let matr = Matrix3::create();

    matr.0 = right.0;
    matr.3 = right.1;
    matr.6 = right.2;

    matr.1 = up.0;
    matr.4 = up.1;
    matr.7 = up.2;

    matr.2 = -view.0;
    matr.5 = -view.1;
    matr.8 = -view.2;

    Quaternion::normalize(out, Quaternion::fromMat3(out, matr));
}
}
