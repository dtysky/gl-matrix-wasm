use wasm_bindgen::prelude::*;

use super::common::*;
use super::matrix3::*;
use super::matrix4::*;
use super::quaternion::*;
use super::vector3::*;

#[wasm_bindgen]
pub struct Quaternion2(
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
impl Quaternion2 {
    #[wasm_bindgen(getter)]
    pub fn elements(&self) -> Box<[f32]> {
        Box::new([
            self.0, self.1, self.2, self.3, self.4, self.5, self.6, self.7,
        ])
    }

    /**
     * Dual Quaternion<br>
     * Format: [real, dual]<br>
     * Quaternion format: XYZW<br>
     * Make sure to have normalized dual quaternions, otherwise the functions may not work as intended.<br>
     * @module quat2
     */

    /**
     * Creates a new identity dual quat
     *
     * @returns {quat2} a new dual quaternion [real -> rotation, dual -> translation]
     */
    pub fn create() -> Quaternion2 {
        Quaternion2(0., 0., 0., 1., 0., 0., 0., 0.)
    }

    /**
     * Creates a new quat initialized with values from an existing quaternion
     *
     * @param {quat2} a dual quaternion to clone
     * @returns {quat2} new dual quaternion
     * @function
     */
    pub fn clone(a: &Quaternion2) -> Quaternion2 {
        Quaternion2(a.0, a.1, a.2, a.3, a.4, a.5, a.6, a.7)
    }

    /**
     * Creates a new dual quat initialized with the given values
     *
     * @param {Number} x1 X component
     * @param {Number} y1 Y component
     * @param {Number} z1 Z component
     * @param {Number} w1 W component
     * @param {Number} x2 X component
     * @param {Number} y2 Y component
     * @param {Number} z2 Z component
     * @param {Number} w2 W component
     * @returns {quat2} new dual quaternion
     * @function
     */
    pub fn fromValues(
        x1: f32,
        y1: f32,
        z1: f32,
        w1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        w2: f32,
    ) -> Quaternion2 {
        Quaternion2(x1, y1, z1, w1, x2, y2, z2, w2)
    }

    /**
     * Creates a new dual quat from the given values (quat and translation)
     *
     * @param {Number} x1 X component
     * @param {Number} y1 Y component
     * @param {Number} z1 Z component
     * @param {Number} w1 W component
     * @param {Number} x2 X component (translation)
     * @param {Number} y2 Y component (translation)
     * @param {Number} z2 Z component (translation)
     * @returns {quat2} new dual quaternion
     * @function
     */
    pub fn fromRotationTranslationValues(
        x1: f32,
        y1: f32,
        z1: f32,
        w1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
    ) -> Quaternion2 {
        let ax = x2 * 0.5;
        let ay = y2 * 0.5;
        let az = z2 * 0.5;

        Quaternion2(
            x1,
            y1,
            z1,
            w1,
            ax * w1 + ay * z1 - az * y1,
            ay * w1 + az * x1 - ax * z1,
            az * w1 + ax * y1 - ay * x1,
            -ax * x1 - ay * y1 - az * z1,
        )
    }

    /**
     * Creates a dual quat from a quaternion and a translation
     *
     * @param {quat2} dual quaternion receiving operation result
     * @param {quat} q a normalized quaternion
     * @param {vec3} t tranlation vector
     * @returns {quat2} dual quaternion receiving operation result
     * @function
     */
    pub fn fromRotationTranslation(out: &mut Quaternion2, q: &Quaternion, t: &Vector3) {
        let ax = t.0 * 0.5;
        let ay = t.1 * 0.5;
        let az = t.2 * 0.5;
        let bx = q.0;
        let by = q.1;
        let bz = q.2;
        let bw = q.3;
        out.0 = bx;
        out.1 = by;
        out.2 = bz;
        out.3 = bw;
        out.4 = ax * bw + ay * bz - az * by;
        out.5 = ay * bw + az * bx - ax * bz;
        out.6 = az * bw + ax * by - ay * bx;
        out.7 = -ax * bx - ay * by - az * bz;
    }

    /**
     * Creates a dual quat from a translation
     *
     * @param {quat2} dual quaternion receiving operation result
     * @param {vec3} t translation vector
     * @returns {quat2} dual quaternion receiving operation result
     * @function
     */
    pub fn fromTranslation(out: &mut Quaternion2, t: &Vector3) {
        out.0 = 0.;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = 1.;
        out.4 = t.0 * 0.5;
        out.5 = t.1 * 0.5;
        out.6 = t.2 * 0.5;
        out.7 = 0.;
    }

    /**
     * Creates a dual quat from a quaternion
     *
     * @param {quat2} dual quaternion receiving operation result
     * @param {quat} q the quaternion
     * @returns {quat2} dual quaternion receiving operation result
     * @function
     */
    pub fn fromRotation(out: &mut Quaternion2, q: &Quaternion) {
        out.0 = q.0;
        out.1 = q.1;
        out.2 = q.2;
        out.3 = q.3;
        out.4 = 0.;
        out.5 = 0.;
        out.6 = 0.;
        out.7 = 0.;
    }

    /**
     * Creates a new dual quat from a matrix (4x4)
     *
     * @param {quat2} out the dual quaternion
     * @param {mat4} a the matrix
     * @returns {quat2} dual quat receiving operation result
     * @function
     */
    pub fn fromMat4(out: &mut Quaternion2, a: &Matrix4) {
        //TODO Optimize this
        let outer = &mut Quaternion::create();
        Matrix4::getRotation(outer, a);
        let t = &mut Vector3::create();
        Matrix4::getTranslation(t, a);
        Quaternion2::fromRotationTranslation(out, outer, t);
    }

    /**
     * Copy the values from one dual quat to another
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a the source dual quaternion
     * @returns {quat2} out
     * @function
     */
    pub fn copy(out: &mut Quaternion2, a: &Quaternion2) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.3;
        out.4 = a.4;
        out.5 = a.5;
        out.6 = a.6;
        out.7 = a.7;
    }

    /**
     * Set a dual quat to the identity dual quaternion
     *
     * @param {quat2} out the receiving quaternion
     * @returns {quat2} out
     */
    pub fn identity(out: &mut Quaternion2) {
        out.0 = 0.;
        out.1 = 0.;
        out.2 = 0.;
        out.3 = 1.;
        out.4 = 0.;
        out.5 = 0.;
        out.6 = 0.;
        out.7 = 0.;
    }

    /**
     * Set the components of a dual quat to the given values
     *
     * @param {quat2} out the receiving quaternion
     * @param {Number} x1 X component
     * @param {Number} y1 Y component
     * @param {Number} z1 Z component
     * @param {Number} w1 W component
     * @param {Number} x2 X component
     * @param {Number} y2 Y component
     * @param {Number} z2 Z component
     * @param {Number} w2 W component
     * @returns {quat2} out
     * @function
     */
    pub fn set(
        out: &mut Quaternion2,
        x1: f32,
        y1: f32,
        z1: f32,
        w1: f32,
        x2: f32,
        y2: f32,
        z2: f32,
        w2: f32,
    ) {
        out.0 = x1;
        out.1 = y1;
        out.2 = z1;
        out.3 = w1;

        out.4 = x2;
        out.5 = y2;
        out.6 = z2;
        out.7 = w2;
    }

    /**
     * Gets the real part of a dual quat
     * @param  {quat} out real part
     * @param  {quat2} a Dual Quaternion
     * @return {quat} real part
     */
    pub fn getReal(out: &mut Quaternion, a: &Quaternion2) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.3;
    }

    /**
     * Gets the dual part of a dual quat
     * @param  {quat} out dual part
     * @param  {quat2} a Dual Quaternion
     * @return {quat} dual part
     */
    pub fn getDual(out: &mut Quaternion, a: &Quaternion2) {
        out.0 = a.4;
        out.1 = a.5;
        out.2 = a.6;
        out.3 = a.7;
    }

    /**
     * Set the real component of a dual quat to the given quaternion
     *
     * @param {quat2} out the receiving quaternion
     * @param {quat} q a quaternion representing the real part
     * @returns {quat2} out
     * @function
     */
    pub fn setReal(out: &mut Quaternion2, a: &Quaternion) {
        out.0 = a.0;
        out.1 = a.1;
        out.2 = a.2;
        out.3 = a.3;
    }

    /**
     * Set the dual component of a dual quat to the given quaternion
     *
     * @param {quat2} out the receiving quaternion
     * @param {quat} q a quaternion representing the dual part
     * @returns {quat2} out
     * @function
     */
    pub fn setDual(out: &mut Quaternion2, q: &Quaternion) {
        out.4 = q.0;
        out.5 = q.1;
        out.6 = q.2;
        out.7 = q.3;
    }

    /**
     * Gets the translation of a normalized dual quat
     * @param  {vec3} out translation
     * @param  {quat2} a Dual Quaternion to be decomposed
     * @return {vec3} translation
     */
    pub fn getTranslation(out: &mut Vector3, a: &Quaternion2) {
        let ax = a.4;
        let ay = a.5;
        let az = a.6;
        let aw = a.7;
        let bx = -a.0;
        let by = -a.1;
        let bz = -a.2;
        let bw = a.3;
        out.0 = (ax * bw + aw * bx + ay * bz - az * by) * 2.;
        out.1 = (ay * bw + aw * by + az * bx - ax * bz) * 2.;
        out.2 = (az * bw + aw * bz + ax * by - ay * bx) * 2.;
    }

    /**
     * Translates a dual quat by the given vector
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a the dual quaternion to translate
     * @param {vec3} v vector to translate by
     * @returns {quat2} out
     */
    pub fn translate(out: &mut Quaternion2, a: &Quaternion2, v: &Vector3) {
        let ax1 = a.0;
        let ay1 = a.1;
        let az1 = a.2;
        let aw1 = a.3;
        let bx1 = v.0 * 0.5;
        let by1 = v.1 * 0.5;
        let bz1 = v.2 * 0.5;
        let ax2 = a.4;
        let ay2 = a.5;
        let az2 = a.6;
        let aw2 = a.7;
        out.0 = ax1;
        out.1 = ay1;
        out.2 = az1;
        out.3 = aw1;
        out.4 = aw1 * bx1 + ay1 * bz1 - az1 * by1 + ax2;
        out.5 = aw1 * by1 + az1 * bx1 - ax1 * bz1 + ay2;
        out.6 = aw1 * bz1 + ax1 * by1 - ay1 * bx1 + az2;
        out.7 = -ax1 * bx1 - ay1 * by1 - az1 * bz1 + aw2;
    }

    /**
     * Rotates a dual quat around the X axis
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a the dual quaternion to rotate
     * @param {number} rad how far should the rotation be
     * @returns {quat2} out
     */
    pub fn rotateX(out: &mut Quaternion2, a: &Quaternion2, rad: f32) {
        let mut bx = -a.0;
        let mut by = -a.1;
        let mut bz = -a.2;
        let mut bw = a.3;
        let ax = a.4;
        let ay = a.5;
        let az = a.6;
        let aw = a.7;
        let ax1 = ax * bw + aw * bx + ay * bz - az * by;
        let ay1 = ay * bw + aw * by + az * bx - ax * bz;
        let az1 = az * bw + aw * bz + ax * by - ay * bx;
        let aw1 = aw * bw - ax * bx - ay * by - az * bz;
        let o = &mut Quaternion::fromValues(out.0, out.1, out.2, out.3);
        Quaternion::rotateX(o, &Quaternion::fromValues(a.0, a.1, a.2, a.3), rad);
        bx = o.0;
        by = o.1;
        bz = o.2;
        bw = o.3;
        out.4 = ax1 * bw + aw1 * bx + ay1 * bz - az1 * by;
        out.5 = ay1 * bw + aw1 * by + az1 * bx - ax1 * bz;
        out.6 = az1 * bw + aw1 * bz + ax1 * by - ay1 * bx;
        out.7 = aw1 * bw - ax1 * bx - ay1 * by - az1 * bz;
    }

    /**
     * Rotates a dual quat around the Y axis
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a the dual quaternion to rotate
     * @param {number} rad how far should the rotation be
     * @returns {quat2} out
     */
    pub fn rotateY(out: &mut Quaternion2, a: &Quaternion2, rad: f32) {
        let mut bx = -a.0;
        let mut by = -a.1;
        let mut bz = -a.2;
        let mut bw = a.3;
        let ax = a.4;
        let ay = a.5;
        let az = a.6;
        let aw = a.7;
        let ax1 = ax * bw + aw * bx + ay * bz - az * by;
        let ay1 = ay * bw + aw * by + az * bx - ax * bz;
        let az1 = az * bw + aw * bz + ax * by - ay * bx;
        let aw1 = aw * bw - ax * bx - ay * by - az * bz;
        let o = &mut Quaternion::fromValues(out.0, out.1, out.2, out.3);
        Quaternion::rotateY(o, &Quaternion::fromValues(a.0, a.1, a.2, a.3), rad);
        bx = o.0;
        by = o.1;
        bz = o.2;
        bw = o.3;
        out.4 = ax1 * bw + aw1 * bx + ay1 * bz - az1 * by;
        out.5 = ay1 * bw + aw1 * by + az1 * bx - ax1 * bz;
        out.6 = az1 * bw + aw1 * bz + ax1 * by - ay1 * bx;
        out.7 = aw1 * bw - ax1 * bx - ay1 * by - az1 * bz;
    }

    /**
     * Rotates a dual quat around the Z axis
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a the dual quaternion to rotate
     * @param {number} rad how far should the rotation be
     * @returns {quat2} out
     */
    pub fn rotateZ(out: &mut Quaternion2, a: &Quaternion2, rad: f32) {
        let mut bx = -a.0;
        let mut by = -a.1;
        let mut bz = -a.2;
        let mut bw = a.3;
        let ax = a.4;
        let ay = a.5;
        let az = a.6;
        let aw = a.7;
        let ax1 = ax * bw + aw * bx + ay * bz - az * by;
        let ay1 = ay * bw + aw * by + az * bx - ax * bz;
        let az1 = az * bw + aw * bz + ax * by - ay * bx;
        let aw1 = aw * bw - ax * bx - ay * by - az * bz;
        let o = &mut Quaternion::fromValues(out.0, out.1, out.2, out.3);
        Quaternion::rotateZ(o, &Quaternion::fromValues(a.0, a.1, a.2, a.3), rad);
        bx = o.0;
        by = o.1;
        bz = o.2;
        bw = o.3;
        out.4 = ax1 * bw + aw1 * bx + ay1 * bz - az1 * by;
        out.5 = ay1 * bw + aw1 * by + az1 * bx - ax1 * bz;
        out.6 = az1 * bw + aw1 * bz + ax1 * by - ay1 * bx;
        out.7 = aw1 * bw - ax1 * bx - ay1 * by - az1 * bz;
    }

    /**
     * Rotates a dual quat by a given quaternion (a * q)
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a the dual quaternion to rotate
     * @param {quat} q quaternion to rotate by
     * @returns {quat2} out
     */
    pub fn rotateByQuatAppend(out: &mut Quaternion2, a: &Quaternion2, q: &Quaternion) {
        let qx = q.0;
        let qy = q.1;
        let qz = q.2;
        let qw = q.3;
        let mut ax = a.0;
        let mut ay = a.1;
        let mut az = a.2;
        let mut aw = a.3;

        out.0 = ax * qw + aw * qx + ay * qz - az * qy;
        out.1 = ay * qw + aw * qy + az * qx - ax * qz;
        out.2 = az * qw + aw * qz + ax * qy - ay * qx;
        out.3 = aw * qw - ax * qx - ay * qy - az * qz;
        ax = a.4;
        ay = a.5;
        az = a.6;
        aw = a.7;
        out.4 = ax * qw + aw * qx + ay * qz - az * qy;
        out.5 = ay * qw + aw * qy + az * qx - ax * qz;
        out.6 = az * qw + aw * qz + ax * qy - ay * qx;
        out.7 = aw * qw - ax * qx - ay * qy - az * qz;
    }

    /**
     * Rotates a dual quat by a given quaternion (q * a)
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat} q quaternion to rotate by
     * @param {quat2} a the dual quaternion to rotate
     * @returns {quat2} out
     */
    pub fn rotateByQuatPrepend(out: &mut Quaternion2, q: &Quaternion, a: &Quaternion2) {
        let qx = q.0;
        let qy = q.1;
        let qz = q.2;
        let qw = q.3;
        let mut bx = a.0;
        let mut by = a.1;
        let mut bz = a.2;
        let mut bw = a.3;

        out.0 = qx * bw + qw * bx + qy * bz - qz * by;
        out.1 = qy * bw + qw * by + qz * bx - qx * bz;
        out.2 = qz * bw + qw * bz + qx * by - qy * bx;
        out.3 = qw * bw - qx * bx - qy * by - qz * bz;
        bx = a.4;
        by = a.5;
        bz = a.6;
        bw = a.7;
        out.4 = qx * bw + qw * bx + qy * bz - qz * by;
        out.5 = qy * bw + qw * by + qz * bx - qx * bz;
        out.6 = qz * bw + qw * bz + qx * by - qy * bx;
        out.7 = qw * bw - qx * bx - qy * by - qz * bz;
    }

    /**
     * Rotates a dual quat around a given axis. Does the normalisation automatically
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a the dual quaternion to rotate
     * @param {vec3} axis the axis to rotate around
     * @param {Number} rad how far the rotation should be
     * @returns {quat2} out
     */
    pub fn rotateAroundAxis(out: &mut Quaternion2, a: &Quaternion2, axis: &Vector3, rad: f32) {
        //Special case for rad = 0
        if (f32::abs(rad) < EPSILON) {
            Quaternion2::copy(out, a);
            return;
        }
        let axisLength = (axis.0.powi(2) + axis.1.powi(2) + axis.2.powi(2)).sqrt();

        let rad = rad * 0.5;
        let s = f32::sin(rad);
        let bx = s * axis.0 / axisLength;
        let by = s * axis.1 / axisLength;
        let bz = s * axis.2 / axisLength;
        let bw = f32::cos(rad);

        let ax1 = a.0;
        let ay1 = a.1;
        let az1 = a.2;
        let aw1 = a.3;
        out.0 = ax1 * bw + aw1 * bx + ay1 * bz - az1 * by;
        out.1 = ay1 * bw + aw1 * by + az1 * bx - ax1 * bz;
        out.2 = az1 * bw + aw1 * bz + ax1 * by - ay1 * bx;
        out.3 = aw1 * bw - ax1 * bx - ay1 * by - az1 * bz;

        let ax = a.4;
        let ay = a.5;
        let az = a.6;
        let aw = a.7;
        out.4 = ax * bw + aw * bx + ay * bz - az * by;
        out.5 = ay * bw + aw * by + az * bx - ax * bz;
        out.6 = az * bw + aw * bz + ax * by - ay * bx;
        out.7 = aw * bw - ax * bx - ay * by - az * bz;
    }

    /**
     * Adds two dual quat"s
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a the first operand
     * @param {quat2} b the second operand
     * @returns {quat2} out
     * @function
     */
    pub fn add(out: &mut Quaternion2, a: &Quaternion2, b: &Quaternion2) {
        out.0 = a.0 + b.0;
        out.1 = a.1 + b.1;
        out.2 = a.2 + b.2;
        out.3 = a.3 + b.3;
        out.4 = a.4 + b.4;
        out.5 = a.5 + b.5;
        out.6 = a.6 + b.6;
        out.7 = a.7 + b.7;
    }

    /**
     * Multiplies two dual quat"s
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a the first operand
     * @param {quat2} b the second operand
     * @returns {quat2} out
     */
    pub fn multiply(out: &mut Quaternion2, a: &Quaternion2, b: &Quaternion2) {
        let ax0 = a.0;
        let ay0 = a.1;
        let az0 = a.2;
        let aw0 = a.3;
        let bx1 = b.4;
        let by1 = b.5;
        let bz1 = b.6;
        let bw1 = b.7;
        let ax1 = a.4;
        let ay1 = a.5;
        let az1 = a.6;
        let aw1 = a.7;
        let bx0 = b.0;
        let by0 = b.1;
        let bz0 = b.2;
        let bw0 = b.3;
        out.0 = ax0 * bw0 + aw0 * bx0 + ay0 * bz0 - az0 * by0;
        out.1 = ay0 * bw0 + aw0 * by0 + az0 * bx0 - ax0 * bz0;
        out.2 = az0 * bw0 + aw0 * bz0 + ax0 * by0 - ay0 * bx0;
        out.3 = aw0 * bw0 - ax0 * bx0 - ay0 * by0 - az0 * bz0;
        out.4 = ax0 * bw1 + aw0 * bx1 + ay0 * bz1 - az0 * by1 + ax1 * bw0 + aw1 * bx0 + ay1 * bz0
            - az1 * by0;
        out.5 = ay0 * bw1 + aw0 * by1 + az0 * bx1 - ax0 * bz1 + ay1 * bw0 + aw1 * by0 + az1 * bx0
            - ax1 * bz0;
        out.6 = az0 * bw1 + aw0 * bz1 + ax0 * by1 - ay0 * bx1 + az1 * bw0 + aw1 * bz0 + ax1 * by0
            - ay1 * bx0;
        out.7 = aw0 * bw1 - ax0 * bx1 - ay0 * by1 - az0 * bz1 + aw1 * bw0
            - ax1 * bx0
            - ay1 * by0
            - az1 * bz0;
    }

    /**
     * Alias for {@link quat2.multiply}
     * @function
     */
    pub fn mul(out: &mut Quaternion2, a: &Quaternion2, b: &Quaternion2) {
        Quaternion2::multiply(out, a, b);
    }

    /**
     * Scales a dual quat by a scalar number
     *
     * @param {quat2} out the receiving dual quat
     * @param {quat2} a the dual quat to scale
     * @param {Number} b amount to scale the dual quat by
     * @returns {quat2} out
     * @function
     */
    pub fn scale(out: &mut Quaternion2, a: &Quaternion2, b: f32) {
        out.0 = a.0 * b;
        out.1 = a.1 * b;
        out.2 = a.2 * b;
        out.3 = a.3 * b;
        out.4 = a.4 * b;
        out.5 = a.5 * b;
        out.6 = a.6 * b;
        out.7 = a.7 * b;
    }

    /**
     * Calculates the dot product of two dual quat"s (The dot product of the real parts)
     *
     * @param {quat2} a the first operand
     * @param {quat2} b the second operand
     * @returns {Number} dot product of a and b
     * @function
     */
    pub fn dot(a: &Quaternion2, b: &Quaternion2) -> f32 {
        a.0 * b.0 + a.1 * b.1 + a.2 * b.2 + a.3 * b.3
    }

    /**
     * Performs a linear interpolation between two dual quats"s
     * NOTE: The resulting dual quaternions won"t always be normalized (The error is most noticeable when t = 0.5)
     *
     * @param {quat2} out the receiving dual quat
     * @param {quat2} a the first operand
     * @param {quat2} b the second operand
     * @param {Number} t interpolation amount, in the range [0-1], between the two inputs
     * @returns {quat2} out
     */
    pub fn lerp(out: &mut Quaternion2, a: &Quaternion2, b: &Quaternion2, t: f32) {
        let mt = 1. - t;
        let mut t = t;
        if (Quaternion2::dot(a, b) < EPSILON) {
            t = -t;
        }

        out.0 = a.0 * mt + b.0 * t;
        out.1 = a.1 * mt + b.1 * t;
        out.2 = a.2 * mt + b.2 * t;
        out.3 = a.3 * mt + b.3 * t;
        out.4 = a.4 * mt + b.4 * t;
        out.5 = a.5 * mt + b.5 * t;
        out.6 = a.6 * mt + b.6 * t;
        out.7 = a.7 * mt + b.7 * t;
    }

    /**
     * Calculates the inverse of a dual quat. If they are normalized, conjugate is cheaper
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a dual quat to calculate inverse of
     * @returns {quat2} out
     */
    pub fn invert(out: &mut Quaternion2, a: &Quaternion2) {
        let sqlen = Quaternion2::squaredLength(a);
        out.0 = -a.0 / sqlen;
        out.1 = -a.1 / sqlen;
        out.2 = -a.2 / sqlen;
        out.3 = a.3 / sqlen;
        out.4 = -a.4 / sqlen;
        out.5 = -a.5 / sqlen;
        out.6 = -a.6 / sqlen;
        out.7 = a.7 / sqlen;
    }

    /**
     * Calculates the conjugate of a dual quat
     * If the dual quaternion is normalized, this function is faster than quat2.inverse and produces the same result.
     *
     * @param {quat2} out the receiving quaternion
     * @param {quat2} a quat to calculate conjugate of
     * @returns {quat2} out
     */
    pub fn conjugate(out: &mut Quaternion2, a: &Quaternion2) {
        out.0 = -a.0;
        out.1 = -a.1;
        out.2 = -a.2;
        out.3 = a.3;
        out.4 = -a.4;
        out.5 = -a.5;
        out.6 = -a.6;
        out.7 = a.7;
    }

    /**
     * Calculates the length of a dual quat
     *
     * @param {quat2} a dual quat to calculate length of
     * @returns {Number} length of a
     * @function
     */
    pub fn length(a: &Quaternion2) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        (x.powi(2) + y.powi(2) + z.powi(2) + w.powi(2)).sqrt()
    }

    /**
     * Alias for {@link quat2.length}
     * @function
     */
    pub fn len(a: &Quaternion2) -> f32 {
        Quaternion2::length(a)
    }

    /**
     * Calculates the squared length of a dual quat
     *
     * @param {quat2} a dual quat to calculate squared length of
     * @returns {Number} squared length of a
     * @function
     */
    pub fn squaredLength(a: &Quaternion2) -> f32 {
        let x = a.0;
        let y = a.1;
        let z = a.2;
        let w = a.3;
        x * x + y * y + z * z + w * w
    }

    /**
     * Alias for {@link quat2.squaredLength}
     * @function
     */
    pub fn sqrLen(a: &Quaternion2) -> f32 {
        Quaternion2::squaredLength(a)
    }

    /**
     * Normalize a dual quat
     *
     * @param {quat2} out the receiving dual quaternion
     * @param {quat2} a dual quaternion to normalize
     * @returns {quat2} out
     * @function
     */
    pub fn normalize(out: &mut Quaternion2, a: &Quaternion2) {
        let mut magnitude = Quaternion2::squaredLength(a);
        if (magnitude > EPSILON) {
            magnitude = f32::sqrt(magnitude);

            let a0 = a.0 / magnitude;
            let a1 = a.1 / magnitude;
            let a2 = a.2 / magnitude;
            let a3 = a.3 / magnitude;

            let b0 = a.4;
            let b1 = a.5;
            let b2 = a.6;
            let b3 = a.7;

            let a_dot_b = (a0 * b0) + (a1 * b1) + (a2 * b2) + (a3 * b3);

            out.0 = a0;
            out.1 = a1;
            out.2 = a2;
            out.3 = a3;

            out.4 = (b0 - (a0 * a_dot_b)) / magnitude;
            out.5 = (b1 - (a1 * a_dot_b)) / magnitude;
            out.6 = (b2 - (a2 * a_dot_b)) / magnitude;
            out.7 = (b3 - (a3 * a_dot_b)) / magnitude;
        }
    }

    /**
     * Returns a string representation of a dual quatenion
     *
     * @param {quat2} a dual quaternion to represent as a string
     * @returns {String} string representation of the dual quat
     */
    // pub fn str(a: &Quaternion2) {
    //   return "quat2(" + a.0 + ", " + a.1 + ", " + a.2 + ", " + a.3 + ", " +
    //     a.4 + ", " + a.5 + ", " + a.6 + ", " + a.7 + ")";
    // }

    /**
     * Returns whether or not the dual quaternions have exactly the same elements in the same position (when compared with ==)
     *
     * @param {quat2} a the first dual quaternion.
     * @param {quat2} b the second dual quaternion.
     * @returns {Boolean} true if the dual quaternions are equal, false otherwise.
     */
    pub fn exactEquals(a: &Quaternion2, b: &Quaternion2) -> bool {
        a.0 == b.0
            && a.1 == b.1
            && a.2 == b.2
            && a.3 == b.3
            && a.4 == b.4
            && a.5 == b.5
            && a.6 == b.6
            && a.7 == b.7
    }

    /**
     * Returns whether or not the dual quaternions have approximately the same elements in the same position.
     *
     * @param {quat2} a the first dual quat.
     * @param {quat2} b the second dual quat.
     * @returns {Boolean} true if the dual quats are equal, false otherwise.
     */
    pub fn equals(a: &Quaternion2, b: &Quaternion2) -> bool {
        let a0 = a.0;
        let a1 = a.1;
        let a2 = a.2;
        let a3 = a.3;
        let a4 = a.4;
        let a5 = a.5;
        let a6 = a.6;
        let a7 = a.7;
        let b0 = b.0;
        let b1 = b.1;
        let b2 = b.2;
        let b3 = b.3;
        let b4 = b.4;
        let b5 = b.5;
        let b6 = b.6;
        let b7 = b.7;
        f32::abs(a0 - b0) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a0), f32::abs(b0)))
            && f32::abs(a1 - b1) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a1), f32::abs(b1)))
            && f32::abs(a2 - b2) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a2), f32::abs(b2)))
            && f32::abs(a3 - b3) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a3), f32::abs(b3)))
            && f32::abs(a4 - b4) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a4), f32::abs(b4)))
            && f32::abs(a5 - b5) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a5), f32::abs(b5)))
            && f32::abs(a6 - b6) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a6), f32::abs(b6)))
            && f32::abs(a7 - b7) <= EPSILON * f32::max(1.0, f32::max(f32::abs(a7), f32::abs(b7)))
    }
}
