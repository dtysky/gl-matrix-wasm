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

describe("mat3", function() {
    let out, matA, matB, identity, result;

    before(done => {
        init().then(() => done());
    });

    beforeEach(function() {
        matA = mat3.fromValues(1, 0, 0,
                0, 1, 0,
                1, 2, 1);

        matB = mat3.fromValues(1, 0, 0,
                0, 1, 0,
                3, 4, 1);

        out = mat3.fromValues(0, 0, 0,
                0, 0, 0,
                0, 0, 0);

        identity = mat3.fromValues(1, 0, 0,
                    0, 1, 0,
                    0, 0, 1);
    });

    describe("normalFromMat4", function() {
        beforeEach(function() {
            matA = mat4.fromValues(1, 0, 0, 0,
                    0, 1, 0, 0,
                    0, 0, 1, 0,
                    0, 0, 0, 1);
            result = mat3.normalFromMat4(out, matA);
        });

        

        describe("with translation and rotation", function() {
            beforeEach(function() {
                mat4.translate(matA, matA, vec3.fromValues(2, 4, 6));
                mat4.rotateX(matA, matA, Math.PI / 2);

                result = mat3.normalFromMat4(out, matA);
            });

            it("should give rotated matrix", function() {
                expect(out).toBeEqualish([1, 0, 0,
                                             0, 0, 1,
                                             0,-1, 0]);
            });

            describe("and scale", function() {
                beforeEach(function() {
                    mat4.scale(matA, matA, vec3.fromValues(2, 3, 4));

                    result = mat3.normalFromMat4(out, matA);
                });

                it("should give rotated matrix", function() {
                    expect(out).toBeEqualish([0.5, 0,    0,
                                                 0,   0,    0.333333,
                                                 0,  -0.25, 0]);
                });
            });
        });
    });

    describe("fromQuat", function() {
        let q;

        beforeEach(function() {
            q = quat.fromValues( 0, -0.7071067811865475, 0, 0.7071067811865475 );
            result = mat3.fromQuat(out, q);
        });

        

        it("should rotate a vector the same as the original quat", function() {
            let v = vec3.create();
            vec3.transformMat3(v, vec3.fromValues(0,0,-1), out);
            let v2 = vec3.create();
            vec3.transformQuat(v2, vec3.fromValues(0,0,-1), q)
            expect(v).toBeEqualish(v2);
        });

        it("should rotate a vector by PI/2 radians", function() {
            let v = vec3.create();
            vec3.transformMat3(v, vec3.fromValues(0,0,-1), out);
            expect(v).toBeEqualish([1,0,0]);
        });
    });

    describe("fromMat4", function() {
        beforeEach(function() {
            result = mat3.fromMat4(out, mat4.fromValues(1, 2, 3, 4,
                                          5, 6, 7, 8,
                                          9,10,11,12,
                                         13,14,15,16)); });

        

        it("should calculate proper mat3", function() {
            expect(out).toBeEqualish([ 1, 2, 3,
                                       5, 6, 7,
                                       9,10,11]);
        });
    });

    describe("scale", function() {
        beforeEach(function() { result = mat3.scale(out, matA, vec2.fromValues(2,2)); });
        
        it('should place proper values in out', function() {
            expect(out).toBeEqualish([ 2, 0, 0,
                                       0, 2, 0,
                                       1, 2, 1 ]);
        });
    });

    describe("create", function() {
        beforeEach(function() { result = mat3.create(); });
        it("should return a 9 element array initialized to a 3x3 identity matrix", function() { expect(result).toBeEqualish(identity); });
    });

    describe("clone", function() {
        beforeEach(function() { result = mat3.clone(matA); });
        it("should return a 9 element array initialized to the values in matA", function() { expect(result).toBeEqualish(matA); });
    });

    describe("copy", function() {
        beforeEach(function() { result = mat3.copy(out, matA); });
        it("should place values into out", function() { expect(out).toBeEqualish(matA); });
        
    });

    describe("identity", function() {
        beforeEach(function() { result = mat3.identity(out); });
        it("should place values into out", function() { expect(out).toBeEqualish(identity); });
        
    });

    describe("transpose", function() {
        describe("with a separate output matrix", function() {
            beforeEach(function() { result = mat3.transpose(out, matA); });

            it("should place values into out", function() {
                expect(out).toBeEqualish([
                    1, 0, 1,
                    0, 1, 2,
                    0, 0, 1
                ]);
            });
            
            it("should not modify matA", function() {
                expect(matA).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    1, 2, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function() {
            beforeEach(function() { result = mat3.transpose(matA, matA); });

            it("should place values into matA", function() {
                expect(matA).toBeEqualish([
                    1, 0, 1,
                    0, 1, 2,
                    0, 0, 1
                ]);
            });
            
        });
    });

    describe("invert", function() {
        describe("with a separate output matrix", function() {
            beforeEach(function() { result = mat3.invert(out, matA); });

            it("should place values into out", function() {
                expect(out).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    -1, -2, 1
                ]);
            });
            
            it("should not modify matA", function() {
                expect(matA).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    1, 2, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function() {
            beforeEach(function() { result = mat3.invert(matA, matA); });

            it("should place values into matA", function() {
                expect(matA).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    -1, -2, 1
                ]);
            });
            
        });
    });

    describe("adjoint", function() {
        describe("with a separate output matrix", function() {
            beforeEach(function() { result = mat3.adjoint(out, matA); });

            it("should place values into out", function() {
                expect(out).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    -1, -2, 1
                ]);
            });
            
            it("should not modify matA", function() {
                expect(matA).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    1, 2, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function() {
            beforeEach(function() { result = mat3.adjoint(matA, matA); });

            it("should place values into matA", function() {
                expect(matA).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    -1, -2, 1
                ]);
            });
            
        });
    });

    describe("determinant", function() {
        beforeEach(function() { result = mat3.determinant(matA); });

        it("should return the determinant", function() { expect(result).toEqual(1); });
    });

    describe("multiply", function() {
        

        describe("with a separate output matrix", function() {
            beforeEach(function() { result = mat3.multiply(out, matA, matB); });

            it("should place values into out", function() {
                expect(out).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    4, 6, 1
                ]);
            });
            
            it("should not modify matA", function() {
                expect(matA).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    1, 2, 1
                ]);
            });
            it("should not modify matB", function() {
                expect(matB).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    3, 4, 1
                ]);
            });
        });

        describe("when matA is the output matrix", function() {
            beforeEach(function() { result = mat3.multiply(matA, matA, matB); });

            it("should place values into matA", function() {
                expect(matA).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    4, 6, 1
                ]);
            });
            
            it("should not modify matB", function() {
                expect(matB).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    3, 4, 1
                ]);
            });
        });

        describe("when matB is the output matrix", function() {
            beforeEach(function() { result = mat3.multiply(matB, matA, matB); });

            it("should place values into matB", function() {
                expect(matB).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    4, 6, 1
                ]);
            });
            
            it("should not modify matA", function() {
                expect(matA).toBeEqualish([
                    1, 0, 0,
                    0, 1, 0,
                    1, 2, 1
                ]);
            });
        });
    });

   describe("frob", function() {
        beforeEach(function() { result = mat3.frob(matA); });
        it("should return the Frobenius Norm of the matrix", function() { expect(result).toEqual( Math.sqrt(Math.pow(1, 2) + Math.pow(0, 2) + Math.pow(0, 2) + Math.pow(0, 2) + Math.pow(1, 2) + Math.pow(0, 2) + Math.pow(1, 2) + Math.pow(2, 2) + Math.pow(1, 2))); });
   });

    describe("add", function() {
        beforeEach(function() {
            matA = mat3.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9);
            matB = mat3.fromValues(10, 11, 12, 13, 14, 15, 16, 17, 18);
        });
        describe("with a separate output matrix", function() {
            beforeEach(function() {
                result = mat3.add(out, matA, matB);
            });

            it("should place values into out", function() { expect(out).toBeEqualish([11, 13, 15, 17, 19, 21, 23, 25, 27]); });
            
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9]); });
            it("should not modify matB", function() { expect(matB).toBeEqualish([10, 11, 12, 13, 14, 15, 16, 17, 18]); });
        });

        describe("when matA is the output matrix", function() {
            beforeEach(function() { result = mat3.add(matA, matA, matB); });

            it("should place values into matA", function() { expect(matA).toBeEqualish([11, 13, 15, 17, 19, 21, 23, 25, 27]); });
            
            it("should not modify matB", function() { expect(matB).toBeEqualish([10, 11, 12, 13, 14, 15, 16, 17, 18]); });
        });

        describe("when matB is the output matrix", function() {
            beforeEach(function() { result = mat3.add(matB, matA, matB); });

            it("should place values into matB", function() { expect(matB).toBeEqualish([11, 13, 15, 17, 19, 21, 23, 25, 27]); });
            
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9]); });
        });
    });

    describe("subtract", function() {
        beforeEach(function() {
            matA = mat3.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9);
            matB = mat3.fromValues(10, 11, 12, 13, 14, 15, 16, 17, 18);
        });
        

        describe("with a separate output matrix", function() {
            beforeEach(function() { result = mat3.subtract(out, matA, matB); });

            it("should place values into out", function() { expect(out).toBeEqualish([-9, -9, -9, -9, -9, -9, -9, -9, -9]); });
            
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9]); });
            it("should not modify matB", function() { expect(matB).toBeEqualish([10, 11, 12, 13, 14, 15, 16, 17, 18]); });
        });

        describe("when matA is the output matrix", function() {
            beforeEach(function() { result = mat3.subtract(matA, matA, matB); });

            it("should place values into matA", function() { expect(matA).toBeEqualish([-9, -9, -9, -9, -9, -9, -9, -9, -9]); });
            
            it("should not modify matB", function() { expect(matB).toBeEqualish([10, 11, 12, 13, 14, 15, 16, 17, 18]); });
        });

        describe("when matB is the output matrix", function() {
            beforeEach(function() { result = mat3.subtract(matB, matA, matB); });

            it("should place values into matB", function() { expect(matB).toBeEqualish([-9, -9, -9, -9, -9, -9, -9, -9, -9]); });
            
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9]); });
        });
    });

    describe("fromValues", function() {
        beforeEach(function() { result = mat3.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9); });
        it("should return a 9 element array initialized to the values passed", function() { expect(result).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9]); });
    });

    describe("set", function() {
        beforeEach(function() { result = mat3.set(out, 1, 2, 3, 4, 5, 6, 7, 8, 9); });
        it("should place values into out", function() { expect(out).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9]); });
        
    });

    describe("multiplyScalar", function() {
        beforeEach(function() {
            matA = mat3.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9);
        });
        describe("with a separate output matrix", function() {
            beforeEach(function() { result = mat3.multiplyScalar(out, matA, 2); });

            it("should place values into out", function() { expect(out).toBeEqualish([2, 4, 6, 8, 10, 12, 14, 16, 18]); });
            
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9]); });
        });

        describe("when matA is the output matrix", function() {
            beforeEach(function() { result = mat3.multiplyScalar(matA, matA, 2); });

            it("should place values into matA", function() { expect(matA).toBeEqualish([2, 4, 6, 8, 10, 12, 14, 16, 18]); });
            
        });
    });

    describe("multiplyScalarAndAdd", function() {
        beforeEach(function() {
            matA = mat3.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9);
            matB = mat3.fromValues(10, 11, 12, 13, 14, 15, 16, 17, 18);
        });
        describe("with a separate output matrix", function() {
            beforeEach(function() { result = mat3.multiplyScalarAndAdd(out, matA, matB, 0.5); });

            it("should place values into out", function() { expect(out).toBeEqualish([6, 7.5, 9, 10.5, 12, 13.5, 15, 16.5, 18]); });
            
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9]); });
            it("should not modify matB", function() { expect(matB).toBeEqualish([10, 11, 12, 13, 14, 15, 16, 17, 18]); });
        });

        describe("when matA is the output matrix", function() {
            beforeEach(function() { result = mat3.multiplyScalarAndAdd(matA, matA, matB, 0.5); });

            it("should place values into matA", function() { expect(matA).toBeEqualish([6, 7.5, 9, 10.5, 12, 13.5, 15, 16.5, 18]); });
            
            it("should not modify matB", function() { expect(matB).toBeEqualish([10, 11, 12, 13, 14, 15, 16, 17, 18]); });
        });

        describe("when matB is the output matrix", function() {
            beforeEach(function() { result = mat3.multiplyScalarAndAdd(matB, matA, matB, 0.5); });

            it("should place values into matB", function() { expect(matB).toBeEqualish([6, 7.5, 9, 10.5, 12, 13.5, 15, 16.5, 18]); });
            
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6, 7, 8, 9]); });
        });
    });

    describe("projection", function() {
        beforeEach(function() {
            result = mat3.projection(out, 100.0, 200.0);
        });

        

        it("should give projection matrix", function() {
            expect(out).toBeEqualish([0.02,     0, 0,
                                            0, -0.01, 0,
                                           -1,     1, 1]);
        });
    });

    describe("exactEquals", function() {
        let matC, r0, r1;
        beforeEach(function() {
            matA = mat3.fromValues(0, 1, 2, 3, 4, 5, 6, 7, 8);
            matB = mat3.fromValues(0, 1, 2, 3, 4, 5, 6, 7, 8);
            matC = mat3.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9);
            r0 = mat3.exactEquals(matA, matB);
            r1 = mat3.exactEquals(matA, matC);
        });

        it("should return true for identical matrices", function() { expect(r0).toBe(true); });
        it("should return false for different matrices", function() { expect(r1).toBe(false); });
        it("should not modify matA", function() { expect(matA).toBeEqualish([0, 1, 2, 3, 4, 5, 6, 7, 8]); });
        it("should not modify matB", function() { expect(matB).toBeEqualish([0, 1, 2, 3, 4, 5, 6, 7, 8]); });
    });

    describe("equals", function() {
        let matC, matD, r0, r1, r2;
        beforeEach(function() {
            matA = mat3.fromValues(0, 1, 2, 3, 4, 5, 6, 7, 8);
            matB = mat3.fromValues(0, 1, 2, 3, 4, 5, 6, 7, 8);
            matC = mat3.fromValues(1, 2, 3, 4, 5, 6, 7, 8, 9);
            matD = mat3.fromValues(1e-16, 1, 2, 3, 4, 5, 6, 7, 8);
            r0 = mat3.equals(matA, matB);
            r1 = mat3.equals(matA, matC);
            r2 = mat3.equals(matA, matD);
        });
        it("should return true for identical matrices", function() { expect(r0).toBe(true); });
        it("should return false for different matrices", function() { expect(r1).toBe(false); });
        it("should return true for close but not identical matrices", function() { expect(r2).toBe(true); });
        it("should not modify matA", function() { expect(matA).toBeEqualish([0, 1, 2, 3, 4, 5, 6, 7, 8]); });
        it("should not modify matB", function() { expect(matB).toBeEqualish([0, 1, 2, 3, 4, 5, 6, 7, 8]); });
    });

});
