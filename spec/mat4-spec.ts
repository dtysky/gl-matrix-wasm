import { expect } from './spec-helper';
import {
    init,
    Vector2 as vec2,
    Vector3 as vec3,
    Vector4 as vec4,
    Matrix2 as mat2,
    Matrix2d as mat2d,
    Matrix3 as mat3,
    Matrix4 as mat4,
    Quaternion as quat,
    Quaternion2 as quat2,
} from '../pkg/gl_matrix_wasm';

describe("mat4", function () {
    let out, matA, matB, identity, result;

    before(done => {
        init().then(() => done());
    });

    beforeEach(function () {
        // Attempting to portray a semi-realistic transform matrix
        matA = mat4.fromValues(1, 0, 0, 0,
            0, 1, 0, 0,
            0, 0, 1, 0,
            1, 2, 3, 1);

        matB = mat4.fromValues(1, 0, 0, 0,
            0, 1, 0, 0,
            0, 0, 1, 0,
            4, 5, 6, 1);

        out = mat4.fromValues(0, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0,
            0, 0, 0, 0);

        identity = mat4.fromValues(1, 0, 0, 0,
            0, 1, 0, 0,
            0, 0, 1, 0,
            0, 0, 0, 1);
    });

    describe("create", function () {
        beforeEach(function () { result = mat4.create(); });
        it("should return a 16 element array initialized to a 4x4 identity matrix", function () { expect(result).toBeEqualish(identity); });
    });

    describe("clone", function () {
        beforeEach(function () { result = mat4.clone(matA); });
        it("should return a 16 element array initialized to the values in matA", function () { expect(result).toBeEqualish(matA); });
    });

    describe("copy", function () {
        beforeEach(function () { result = mat4.copy(out, matA); });
        it("should place values into out", function () { expect(out).toBeEqualish(matA); });
        
    });

    describe("identity", function () {
        beforeEach(function () { result = mat4.identity(out); });
        it("should place values into out", function () { expect(out).toBeEqualish(identity); });
        
    });

    describe("transpose", function () {
        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.transpose(out, matA); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    1, 0, 0, 1,
                    0, 1, 0, 2,
                    0, 0, 1, 3,
                    0, 0, 0, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.transpose(matA, matA); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 1,
                    0, 1, 0, 2,
                    0, 0, 1, 3,
                    0, 0, 0, 1
                ]);
            });
            
        });
    });

    describe("invert", function () {
        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.invert(out, matA); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    -1, -2, -3, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.invert(matA, matA); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    -1, -2, -3, 1
                ]);
            });
            
        });
    });

    describe("adjoint", function () {
        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.adjoint(out, matA); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    -1, -2, -3, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.adjoint(matA, matA); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    -1, -2, -3, 1
                ]);
            });
            
        });
    });

    describe("determinant", function () {
        beforeEach(function () { result = mat4.determinant(matA); });

        it("should return the determinant", function () { expect(result).toEqual(1); });
    });

    describe("multiply", function () {
        

        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.multiply(out, matA, matB); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    5, 7, 9, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
            it("should not modify matB", function () {
                expect(matB).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    4, 5, 6, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.multiply(matA, matA, matB); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    5, 7, 9, 1
                ]);
            });
            
            it("should not modify matB", function () {
                expect(matB).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    4, 5, 6, 1
                ]);
            });
        });

        describe("when matB is the output matrix", function () {
            beforeEach(function () { result = mat4.multiply(matB, matA, matB); });

            it("should place values into matB", function () {
                expect(matB).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    5, 7, 9, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });
    });

    describe("translate", function () {
        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.translate(out, matA, vec3.fromValues(4, 5, 6)); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    5, 7, 9, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.translate(matA, matA, vec3.fromValues(4, 5, 6)); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    5, 7, 9, 1
                ]);
            });
            
        });
    });

    describe("scale", function () {
        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.scale(out, matA, vec3.fromValues(4, 5, 6)); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    4, 0, 0, 0,
                    0, 5, 0, 0,
                    0, 0, 6, 0,
                    1, 2, 3, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.scale(matA, matA, vec3.fromValues(4, 5, 6)); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    4, 0, 0, 0,
                    0, 5, 0, 0,
                    0, 0, 6, 0,
                    1, 2, 3, 1
                ]);
            });
            
        });
    });

    describe("rotate", function () {
        let rad = Math.PI * 0.5;
        let axis;
        beforeEach(() => {
            axis = vec3.fromValues(1, 0, 0);
        })

        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.rotate(out, matA, rad, axis); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    1, 0, 0, 0,
                    0, Math.cos(rad), Math.sin(rad), 0,
                    0, -Math.sin(rad), Math.cos(rad), 0,
                    1, 2, 3, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.rotate(matA, matA, rad, axis); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, Math.cos(rad), Math.sin(rad), 0,
                    0, -Math.sin(rad), Math.cos(rad), 0,
                    1, 2, 3, 1
                ]);
            });
            
        });
    });

    describe("rotateX", function () {
        let rad = Math.PI * 0.5;

        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.rotateX(out, matA, rad); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    1, 0, 0, 0,
                    0, Math.cos(rad), Math.sin(rad), 0,
                    0, -Math.sin(rad), Math.cos(rad), 0,
                    1, 2, 3, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.rotateX(matA, matA, rad); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, Math.cos(rad), Math.sin(rad), 0,
                    0, -Math.sin(rad), Math.cos(rad), 0,
                    1, 2, 3, 1
                ]);
            });
            
        });
    });

    describe("rotateY", function () {
        let rad = Math.PI * 0.5;

        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.rotateY(out, matA, rad); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    Math.cos(rad), 0, -Math.sin(rad), 0,
                    0, 1, 0, 0,
                    Math.sin(rad), 0, Math.cos(rad), 0,
                    1, 2, 3, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.rotateY(matA, matA, rad); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    Math.cos(rad), 0, -Math.sin(rad), 0,
                    0, 1, 0, 0,
                    Math.sin(rad), 0, Math.cos(rad), 0,
                    1, 2, 3, 1
                ]);
            });
            
        });
    });

    describe("rotateZ", function () {
        let rad = Math.PI * 0.5;

        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.rotateZ(out, matA, rad); });

            it("should place values into out", function () {
                expect(out).toBeEqualish([
                    Math.cos(rad), Math.sin(rad), 0, 0,
                    -Math.sin(rad), Math.cos(rad), 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
            
            it("should not modify matA", function () {
                expect(matA).toBeEqualish([
                    1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.rotateZ(matA, matA, rad); });

            it("should place values into matA", function () {
                expect(matA).toBeEqualish([
                    Math.cos(rad), Math.sin(rad), 0, 0,
                    -Math.sin(rad), Math.cos(rad), 0, 0,
                    0, 0, 1, 0,
                    1, 2, 3, 1
                ]);
            });
            
        });
    });

    // TODO: fromRotationTranslation

    describe("getTranslation", function () {
        describe("from the identity matrix", function () {
            beforeEach(function () {
                result = vec3.fromValues(1, 2, 3);
                out = vec3.fromValues(1, 2, 3);
                result = mat4.getTranslation(out, identity);
            });
            it("should return the zero vector", function () { expect(out).toBeEqualish([0, 0, 0]); });
        });

        describe("from a translation-only matrix", function () {
            beforeEach(function () {
                result = vec3.fromValues(1, 2, 3);
                out = vec3.fromValues(1, 2, 3);
                result = mat4.getTranslation(out, matB);
            });
            it("should return translation vector", function () { expect(out).toBeEqualish([4, 5, 6]); });
        });

        describe("from a translation and rotation matrix", function () {
            beforeEach(function () {
                let q = quat.create();
                let v = vec3.fromValues(5, 6, 7);
                quat.setAxisAngle(q, vec3.fromValues(0.26726124, 0.534522474, 0.8017837), 0.55);
                mat4.fromRotationTranslation(out, q, v);

                result = vec3.create();
                mat4.getTranslation(result, out);
            });
            it("should keep the same translation vector, regardless of rotation", function () {
                expect(result).toBeEqualish([5, 6, 7]);
            });
        });
    });

    describe("getScaling", function () {
        describe("from the identity matrix", function () {
            beforeEach(function () {
                result = vec3.fromValues(1, 2, 3);
                out = vec3.fromValues(1, 2, 3);
                result = mat4.getScaling(out, identity);
            });

            it("should return the identity vector", function () {
                expect(out).toBeEqualish([1, 1, 1]);
            });
        });

        describe("from a scale-only matrix", function () {
            beforeEach(function () {
                let v = vec3.fromValues(4, 5, 6);
                result = vec3.fromValues(1, 2, 3)
                out = vec3.fromValues(1, 2, 3);
                mat4.fromScaling(matA, v);
                result = mat4.getScaling(out, matA);
            });
            it("should return translation vector", function () { expect(out).toBeEqualish([4, 5, 6]); });
        });

        describe("from a translation and rotation matrix", function () {
            beforeEach(function () {
                let q = quat.create();
                let v = vec3.fromValues(5, 6, 7);
                quat.setAxisAngle(q, vec3.fromValues(1, 0, 0), 0.5);
                mat4.fromRotationTranslation(out, q, v);

                result = vec3.fromValues(1, 2, 3);
                mat4.getScaling(result, out);
            })
            it("should return the identity vector", function () { expect(result).toBeEqualish([1, 1, 1]); });
        });

        describe("from a translation, rotation and scale matrix", function () {
            beforeEach(function () {
                let q = quat.create();
                let t = vec3.fromValues(1, 2, 3);
                let s = vec3.fromValues(5, 6, 7);
                quat.setAxisAngle(q, vec3.fromValues(0, 1, 0), 0.7);
                mat4.fromRotationTranslationScale(out, q, t, s);
                result = vec3.fromValues(5, 6, 7);
                mat4.getScaling(result, out);
            })
            it("should return the same scaling factor when created", function () { expect(result).toBeEqualish([5, 6, 7]); });
        });

    });

    describe("getRotation", function () {
        describe("from the identity matrix", function () {
            beforeEach(function () {
                result = quat.fromValues(1, 2, 3, 4);
                out = quat.fromValues(1, 2, 3, 4);
                result = mat4.getRotation(out, identity);
            });
            it("should return the unit quaternion", function () {
                let unitQuat = quat.create();
                quat.identity(unitQuat);
                expect(out).toBeEqualish(unitQuat);
            });
        });

        describe("from a translation-only matrix", function () {
            beforeEach(function () {
                result = quat.fromValues(1, 2, 3, 4);
                out = quat.fromValues(1, 2, 3, 4);
                result = mat4.getRotation(out, matB);
            });
            it("should return the unit quaternion", function () {
                let unitQuat = quat.create();
                quat.identity(unitQuat);
                expect(out).toBeEqualish(unitQuat);
            });
        });

        describe("from a translation and rotation matrix", function () {
            it("should keep the same rotation as when created", function () {
                let q = quat.create();
                let outVec = vec3.fromValues(5, 6, 7);
                let testVec = vec3.fromValues(1, 5, 2);
                let ang = 0.78972;

                vec3.normalize(testVec, testVec);
                quat.setAxisAngle(q, testVec, ang);
                mat4.fromRotationTranslation(out, q, outVec);

                result = quat.fromValues(2, 3, 4, 6);
                mat4.getRotation(result, out);
                let outaxis = vec3.create();
                let outangle = quat.getAxisAngle(outaxis, result);

                expect(outaxis).toBeEqualish(testVec);
                expect(outangle).toBeEqualish(ang);
            });
        });
    });

    describe("frustum", function () {
        beforeEach(function () { result = mat4.frustum(out, -1, 1, -1, 1, -1, 1); });
        it("should place values into out", function () {
            expect(out).toBeEqualish([
                -1, 0, 0, 0,
                0, -1, 0, 0,
                0, 0, 0, -1,
                0, 0, 1, 0
            ]);
        });
        
    });

    describe("perspective", function () {
        let fovy = Math.PI * 0.5;
        beforeEach(function () { result = mat4.perspective(out, fovy, 1, 0, 1); });
        it("should place values into out", function () {
            expect(out).toBeEqualish([
                1, 0, 0, 0,
                0, 1, 0, 0,
                0, 0, -1, -1,
                0, 0, 0, 0
            ]);
        });
        

        describe("with nonzero near, 45deg fovy, and realistic aspect ratio", function () {
            beforeEach(function () { result = mat4.perspective(out, 45 * Math.PI / 180.0, 640 / 480, 0.1, 200); });
            it("should calculate correct matrix", function () {
                expect(out).toBeEqualish([
                    1.81066, 0, 0, 0,
                    0, 2.414213, 0, 0,
                    0, 0, -1.001, -1,
                    0, 0, -0.2001, 0
                ]);
            });
        });

        describe("with no far plane, 45deg fovy, and realistic aspect ratio", function () {
            beforeEach(function () { result = mat4.perspective(out, 45 * Math.PI / 180.0, 640 / 480, 0.1); });
            it("should calculate correct matrix", function () {
                expect(out).toBeEqualish([
                    1.81066, 0, 0, 0,
                    0, 2.414213, 0, 0,
                    0, 0, -1, -1,
                    0, 0, -0.2, 0
                ]);
            });
        });

        describe("with infinite far plane, 45deg fovy, and realistic aspect ratio", function () {
            beforeEach(function () { result = mat4.perspective(out, 45 * Math.PI / 180.0, 640 / 480, 0.1, Infinity); });
            it("should calculate correct matrix", function () {
                expect(out).toBeEqualish([
                    1.81066, 0, 0, 0,
                    0, 2.414213, 0, 0,
                    0, 0, -1, -1,
                    0, 0, -0.2, 0
                ]);
            });
        });
    });

    describe("ortho", function () {
        beforeEach(function () { result = mat4.ortho(out, -1, 1, -1, 1, -1, 1); });
        it("should place values into out", function () {
            expect(out).toBeEqualish([
                1, 0, 0, 0,
                0, 1, 0, 0,
                0, 0, -1, 0,
                0, 0, 0, 1
            ]);
        });
        
    });

    describe("lookAt", function () {
        let eye, center, up;
        let view, right;
        beforeEach(() => {
            eye = vec3.fromValues(0, 0, 1);
        center = vec3.fromValues(0, 0, -1);
        up = vec3.fromValues(0, 1, 0);
        });

        describe("looking down", function () {
            beforeEach(function () {
                view = vec3.fromValues(0, -1, 0);
                up = vec3.fromValues(0, 0, -1);
                right = vec3.fromValues(1, 0, 0);
                result = mat4.lookAt(out, vec3.fromValues(0, 0, 0), view, up);
            });

            it("should transform view into local -Z", function () {
                let v = vec3.create();
                result = vec3.transformMat4(v, view, out);
                expect(v).toBeEqualish([0, 0, -1]);
            });

            it("should transform up into local +Y", function () {
                let v = vec3.create();
                result = vec3.transformMat4(v, up, out);
                expect(v).toBeEqualish([0, 1, 0]);
            });

            it("should transform right into local +X", function () {
                let v = vec3.create();
                result = vec3.transformMat4(v, right, out);
                expect(v).toBeEqualish([1, 0, 0]);
            });

            
        });

        describe("#74", function () {
            let v;
            beforeEach(function () {
                v = vec3.create();
                mat4.lookAt(out,
                    vec3.fromValues(0, 2, 0),
                    vec3.fromValues(0, 0.6, 0),
                    vec3.fromValues(0, 0, -1));
            });

            it("should transform a point 'above' into local +Y", function () {
                result = vec3.transformMat4(v, vec3.fromValues(0, 2, -1), out);
                expect(v).toBeEqualish([0, 1, 0]);
            });

            it("should transform a point 'right of' into local +X", function () {
                result = vec3.transformMat4(v, vec3.fromValues(1, 2, 0), out);
                expect(v).toBeEqualish([1, 0, 0]);
            });

            it("should transform a point 'in front of' into local -Z", function () {
                result = vec3.transformMat4(v, vec3.fromValues(0, 1, 0), out);
                expect(v).toBeEqualish([0, 0, -1]);
            });
        });

        beforeEach(function () {
            eye = vec3.fromValues(0, 0, 1);
            center = vec3.fromValues(0, 0, -1);
            up = vec3.fromValues(0, 1, 0);
            result = mat4.lookAt(out, eye, center, up);
        });
        it("should place values into out", function () {
            expect(out).toBeEqualish([
                1, 0, 0, 0,
                0, 1, 0, 0,
                0, 0, 1, 0,
                0, 0, -1, 1
            ]);
        });
        
    });

    describe("targetTo", function () {
        let eye, center;
        var view, up, right;
        let v;
        beforeEach(function () {
            v = vec3.create();
            eye = vec3.fromValues(0, 0, 1);
        center = vec3.fromValues(0, 0, -1);
        up = vec3.fromValues(0, 1, 0);
        });

        describe("looking down", function () {
            beforeEach(function () {
                view = vec3.fromValues(0, -1, 0);
                up = vec3.fromValues(0, 0, -1);
                right = vec3.fromValues(1, 0, 0);
                result = mat4.targetTo(out, vec3.fromValues(0, 0, 0), view, up);
            });

            it("should transform view into local Z", function () {
                result = vec3.transformMat4(v, view, out);
                expect(v).toBeEqualish([0, 0, 1]);
            });

            it("should transform up into local -Y", function () {
                result = vec3.transformMat4(v, up, out);
                expect(v).toBeEqualish([0, -1, 0]);
            });

            it("should transform right into local +X", function () {
                result = vec3.transformMat4(v, right, out);
                expect(v).toBeEqualish([1, 0, 0]);
            });

            

            it("scaling should be [1, 1, 1]", function () {
                var scaling = vec3.create();
                mat4.getScaling(scaling, out);
                expect(scaling).toBeEqualish([1, 1, 1]);
            });
        });

        describe("#74", function () {
            let v;
            beforeEach(function () {
                v = vec3.create();
                mat4.targetTo(out,
                    vec3.fromValues(0, 2, 0),
                    vec3.fromValues(0, 0.6, 0),
                    vec3.fromValues(0, 0, -1));
            });

            it("should transform a point 'above' into local +Y", function () {
                result = vec3.transformMat4(v, vec3.fromValues(0, 2, -1), out);
                expect(v).toBeEqualish([0, 1, -2]);
            });

            it("should transform a point 'right of' into local +X", function () {
                result = vec3.transformMat4(v, vec3.fromValues(1, 2, 0), out);
                expect(v).toBeEqualish([1, 2, -2]);
            });

            it("should transform a point 'in front of' into local -Z", function () {
                result = vec3.transformMat4(v, vec3.fromValues(0, 1, 0), out);
                expect(v).toBeEqualish([0, 2, -1]);
            });

            it("scaling should be [1, 1, 1]", function () {
                var scaling = vec3.create();
                mat4.getScaling(scaling, out);
                expect(scaling).toBeEqualish([1, 1, 1]);
            });
        });

        describe("scaling test", function () {
            beforeEach(function () {
                mat4.targetTo(out,
                    vec3.fromValues(0, 1, 0),
                    vec3.fromValues(0, 0, 1),
                    vec3.fromValues(0, 0, -1));
            });

            it("scaling should be [1, 1, 1]", function () {
                var scaling = vec3.create();
                mat4.getScaling(scaling, out);
                expect(scaling).toBeEqualish([1, 1, 1]);
            });
        });

        beforeEach(function () {
            eye = vec3.fromValues(0, 0, 1);
            center = vec3.fromValues(0, 0, -1);
            up = vec3.fromValues(0, 1, 0);
            result = mat4.targetTo(out, eye, center, up);
        });
        it("should place values into out", function () {
            expect(out).toBeEqualish([
                1, 0, 0, 0,
                0, 1, 0, 0,
                0, 0, 1, 0,
                0, 0, 1, 1
            ]);
        });
        
        it("scaling should be [1, 1, 1]", function () {
            var scaling = vec3.create();
                mat4.getScaling(scaling, out);
            expect(scaling).toBeEqualish([1, 1, 1]);
        });
    });

    describe("frob", function () {
        beforeEach(function () { result = mat4.frob(matA); });
        it("should return the Frobenius Norm of the matrix", function () { expect(result).toBeEqualish(Math.sqrt(Math.pow(1, 2) + Math.pow(1, 2) + Math.pow(1, 2) + Math.pow(1, 2) + Math.pow(1, 2) + Math.pow(2, 2) + Math.pow(3, 2))); });
    });

    describe("add", function () {
        beforeEach(function () {
            matA = mat4.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
            matB = mat4.fromValues(17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
        });
        describe("with a separate output matrix", function () {
            beforeEach(function () {
                result = mat4.add(out, matA, matB);
            });

            it("should place values into out", function () { expect(out).toBeEqualish([18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48]); });
            
            it("should not modify matA", function () { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); });
            it("should not modify matB", function () { expect(matB).toBeEqualish([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]); });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.add(matA, matA, matB); });

            it("should place values into matA", function () { expect(matA).toBeEqualish([18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48]); });
            
            it("should not modify matB", function () { expect(matB).toBeEqualish([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]); });
        });

        describe("when matB is the output matrix", function () {
            beforeEach(function () { result = mat4.add(matB, matA, matB); });

            it("should place values into matB", function () { expect(matB).toBeEqualish([18, 20, 22, 24, 26, 28, 30, 32, 34, 36, 38, 40, 42, 44, 46, 48]); });
            
            it("should not modify matA", function () { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); });
        });
    });

    describe("subtract", function () {
        beforeEach(function () {
            matA = mat4.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
            matB = mat4.fromValues(17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
        });
        

        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.subtract(out, matA, matB); });

            it("should place values into out", function () { expect(out).toBeEqualish([-16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16]); });
            
            it("should not modify matA", function () { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); });
            it("should not modify matB", function () { expect(matB).toBeEqualish([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]); });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.subtract(matA, matA, matB); });

            it("should place values into matA", function () { expect(matA).toBeEqualish([-16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16]); });
            
            it("should not modify matB", function () { expect(matB).toBeEqualish([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]); });
        });

        describe("when matB is the output matrix", function () {
            beforeEach(function () { result = mat4.subtract(matB, matA, matB); });

            it("should place values into matB", function () { expect(matB).toBeEqualish([-16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16, -16]); });
            
            it("should not modify matA", function () { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); });
        });
    });

    describe("fromValues", function () {
        beforeEach(function () { result = mat4.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16); });
        it("should return a 16 element array initialized to the values passed", function () { expect(result).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); });
    });

    describe("set", function () {
        beforeEach(function () { result = mat4.set(out, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16); });
        it("should place values into out", function () { expect(out).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); });
        
    });

    describe("multiplyScalar", function () {
        beforeEach(function () {
            matA = mat4.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
        });
        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.multiplyScalar(out, matA, 2); });

            it("should place values into out", function () { expect(out).toBeEqualish([2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32]); });
            
            it("should not modify matA", function () { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.multiplyScalar(matA, matA, 2); });

            it("should place values into matA", function () { expect(matA).toBeEqualish([2, 4, 6, 8, 10, 12, 14, 16, 18, 20, 22, 24, 26, 28, 30, 32]); });
            
        });
    });

    describe("multiplyScalarAndAdd", function () {
        beforeEach(function () {
            matA = mat4.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
            matB = mat4.fromValues(17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32);
        });
        describe("with a separate output matrix", function () {
            beforeEach(function () { result = mat4.multiplyScalarAndAdd(out, matA, matB, 0.5); });

            it("should place values into out", function () { expect(out).toBeEqualish([9.5, 11, 12.5, 14, 15.5, 17, 18.5, 20, 21.5, 23, 24.5, 26, 27.5, 29, 30.5, 32]); });
            
            it("should not modify matA", function () { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); });
            it("should not modify matB", function () { expect(matB).toBeEqualish([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]); });
        });

        describe("when matA is the output matrix", function () {
            beforeEach(function () { result = mat4.multiplyScalarAndAdd(matA, matA, matB, 0.5); });

            it("should place values into matA", function () { expect(matA).toBeEqualish([9.5, 11, 12.5, 14, 15.5, 17, 18.5, 20, 21.5, 23, 24.5, 26, 27.5, 29, 30.5, 32]); });
            
            it("should not modify matB", function () { expect(matB).toBeEqualish([17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32]); });
        });

        describe("when matB is the output matrix", function () {
            beforeEach(function () { result = mat4.multiplyScalarAndAdd(matB, matA, matB, 0.5); });

            it("should place values into matB", function () { expect(matB).toBeEqualish([9.5, 11, 12.5, 14, 15.5, 17, 18.5, 20, 21.5, 23, 24.5, 26, 27.5, 29, 30.5, 32]); });
            
            it("should not modify matA", function () { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]); });
        });
    });

    describe("exactEquals", function () {
        let matC, r0, r1;
        beforeEach(function () {
            matA = mat4.fromValues(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
            matB = mat4.fromValues(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
            matC = mat4.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
            r0 = mat4.exactEquals(matA, matB);
            r1 = mat4.exactEquals(matA, matC);
        });

        it("should return true for identical matrices", function () { expect(r0).toBe(true); });
        it("should return false for different matrices", function () { expect(r1).toBe(false); });
        it("should not modify matA", function () { expect(matA).toBeEqualish([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]); });
        it("should not modify matB", function () { expect(matB).toBeEqualish([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]); });
    });

    describe("equals", function () {
        let matC, matD, r0, r1, r2;
        beforeEach(function () {
            matA = mat4.fromValues(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
            matB = mat4.fromValues(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
            matC = mat4.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16);
            matD = mat4.fromValues(1e-16, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
            r0 = mat4.equals(matA, matB);
            r1 = mat4.equals(matA, matC);
            r2 = mat4.equals(matA, matD);
        });
        it("should return true for identical matrices", function () { expect(r0).toBe(true); });
        it("should return false for different matrices", function () { expect(r1).toBe(false); });
        it("should return true for close but not identical matrices", function () { expect(r2).toBe(true); });
        it("should not modify matA", function () { expect(matA).toBeEqualish([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]); });
        it("should not modify matB", function () { expect(matB).toBeEqualish([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]); });
    });
});
