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

describe("quat", function() {
    let out, quatA, quatB, result;
    let vec, id, deg90;

    before(done => {
        init().then(() => done());
    });

    beforeEach(function() {
        quatA = quat.fromValues(1, 2, 3, 4);
        quatB = quat.fromValues(5, 6, 7, 8);
        out = quat.fromValues(0, 0, 0, 0);
        vec = vec3.fromValues(1, 1, -1);
        id = quat.fromValues(0, 0, 0, 1);
        deg90 = Math.PI / 2;
    });

    describe("slerp", function() {
        describe("the normal case", function() {
            beforeEach(function() {
                result = quat.slerp(out, quat.fromValues(0, 0, 0, 1), quat.fromValues(0, 1, 0, 0), 0.5);
            });

            
            it("should calculate proper quat", function() {
                expect(out).toBeEqualish([0, 0.707106, 0, 0.707106]);
            });
        });

        describe("where a == b", function() {
            beforeEach(function() {
                result = quat.slerp(out, quat.fromValues(0, 0, 0, 1), quat.fromValues(0, 0, 0, 1), 0.5);
            });

            
            it("should calculate proper quat", function() {
                expect(out).toBeEqualish([0, 0, 0, 1]);
            });
        });

        describe("where theta == 180deg", function() {
            beforeEach(function() {
                quat.rotateX(quatA, quat.fromValues(1,0,0,0), Math.PI); // 180 deg
                result = quat.slerp(out, quat.fromValues(1,0,0,0), quatA, 1);
            });

            it("should calculate proper quat", function() {
                expect(out).toBeEqualish([0,0,0,-1]);
            });
        });

        describe("where a == -b", function() {
            beforeEach(function() {
                result = quat.slerp(out, quat.fromValues(1, 0, 0, 0), quat.fromValues(-1, 0, 0, 0), 0.5);
            });

            
            it("should calculate proper quat", function() {
                expect(out).toBeEqualish([1, 0, 0, 0]);
            });
        });
    });

    describe("rotateX", function() {
        beforeEach(function() {
            result = quat.rotateX(out, id, deg90);
        });

        
        it("should transform vec accordingly", function() {
            vec3.transformQuat(vec, vec3.fromValues(0,0,-1), out);
            expect(vec).toBeEqualish([0, 1, 0]);
        });
    });

    describe("rotateY", function() {
        beforeEach(function() {
            result = quat.rotateY(out, id, deg90);
        });

        
        it("should transform vec accordingly", function() {
            vec3.transformQuat(vec, vec3.fromValues(0,0,-1), out);
            expect(vec).toBeEqualish([-1, 0, 0]);
        });
    });

    describe("rotateZ", function() {
        beforeEach(function() {
            result = quat.rotateZ(out, id, deg90);
        });

        
        it("should transform vec accordingly", function() {
            vec3.transformQuat(vec, vec3.fromValues(0,1,0), out);
            expect(vec).toBeEqualish([-1, 0, 0]);
        });
    });

    describe("fromMat3", function() {
        let matr;

        describe("legacy", function() {
            beforeEach(function() {
                matr = mat3.fromValues( 1, 0,  0,
                         0, 0, -1,
                         0, 1,  0 );
                result = quat.fromMat3(out, matr);
            });

            it("should set dest to the correct value", function() {
                expect(out).toBeEqualish([-0.707106, 0, 0, 0.707106]);
            });
        });

        describe("where trace > 0", function() {
            beforeEach(function() {
                matr = mat3.fromValues( 1, 0,  0,
                         0, 0, -1,
                         0, 1,  0 );
                result = quat.fromMat3(out, matr);
            });

            

            it("should produce the correct transformation", function() {
                let v = vec3.create();
                vec3.transformQuat(v, vec3.fromValues(0,1,0), out);
                expect(v).toBeEqualish([0,0,-1]);
            });
        });

        describe("from a normal matrix looking 'backward'", function() {
            beforeEach(function() {
                matr = mat3.create();
                let m = mat4.create();
                mat4.lookAt(m, vec3.fromValues(0, 0, 0), vec3.fromValues(0, 0, 1), vec3.fromValues(0, 1, 0));
                mat3.fromMat4(matr, m);
                mat3.invert(matr, matr)
                mat3.transpose(matr, matr);
                result = quat.fromMat3(out, matr);
            });

            

            it("should produce the same transformation as the given matrix", function() {
                quat.normalize(out, out);
                let v = vec3.create();
                vec3.transformQuat(v, vec3.fromValues(3,2,-1), out);
                expect(v).toBeEqualish([-3, 2, 1]);
            });
        });

        describe("from a normal matrix looking 'left' and 'upside down'", function() {
            beforeEach(function() {
                matr = mat3.create();
                let m = mat4.create();
                mat4.lookAt(m, vec3.fromValues(0, 0, 0), vec3.fromValues(-1, 0, 0), vec3.fromValues(0, -1, 0));
                mat3.fromMat4(matr, m);
                mat3.invert(matr, matr)
                mat3.transpose(matr, matr);
                result = quat.fromMat3(out, matr);
            });

            

            it("should produce the same transformation as the given matrix", function() {
                quat.normalize(out, out);
                let v = vec3.create();
                vec3.transformQuat(v, vec3.fromValues(3,2,-1), out);
                expect(v).toBeEqualish([-1, -2, 3]);
            });
        });

        describe("from a normal matrix looking 'upside down'", function() {
            beforeEach(function() {
                matr = mat3.create();
                let m = mat4.create();
                mat4.lookAt(m, vec3.fromValues(0, 0, 0), vec3.fromValues(0, 0, -1), vec3.fromValues(0, -1, 0));
                mat3.fromMat4(matr, m);
                mat3.invert(matr, matr)
                mat3.transpose(matr, matr);
                result = quat.fromMat3(out, matr);
            });

            

            it("should produce the same transformation as the given matrix", function() {
                quat.normalize(out, out);
                let v = vec3.create();
                vec3.transformQuat(v, vec3.fromValues(3,2,-1), out)
                let v2 = vec3.create();
                vec3.transformMat3(v2, vec3.fromValues(3,2,-1), matr)
                expect(v).toBeEqualish(v2);
            });
        });
    });

    describe("fromEuler", function() {
        describe("legacy", function() {
            beforeEach(function() {
                result = quat.fromEuler(out, -90, 0, 0);
            });

            it("should set dest to the correct value", function() {
                expect(out).toBeEqualish([-0.707106, 0, 0, 0.707106]);
            });
        });

        describe("where trace > 0", function() {
            beforeEach(function() {
                result = quat.fromEuler(out, -90, 0, 0);
            });

            

            it("should produce the correct transformation", function() {
                let v = vec3.create();
                vec3.transformQuat(v, vec3.fromValues(0,1,0), out);
                expect(v).toBeEqualish([0,0,-1]);
            });
        });
    });

    describe("setAxes", function() {
        let r;
        beforeEach(function() { r = vec3.create(); });

        describe("looking left", function() {
            let view, up, right;
            beforeEach(function() {
                view = vec3.fromValues(-1, 0, 0);
                up   = vec3.fromValues(0, 1, 0);
                right= vec3.fromValues(0, 0,-1);
                quat.setAxes(out, view, right, up);
            });

            it("should transform local view into world left", function() {
                vec3.transformQuat(r, vec3.fromValues(0,0,-1), out);
                expect(r).toBeEqualish([1, 0, 0]);
            });

            it("should transform local right into world front", function() {
                vec3.transformQuat(r, vec3.fromValues(1,0,0), out);
                expect(r).toBeEqualish([0, 0, 1]);
            });
        });

        describe("given opengl defaults", function() {
            let view, up, right;
            beforeEach(function() {
                view = vec3.fromValues(0, 0, -1);
                up   = vec3.fromValues(0, 1,  0);
                right= vec3.fromValues(1, 0,  0);
                quat.setAxes(out, view, right, up);
            });

            

            it("should produce identity", function() {
                expect(out).toBeEqualish([0, 0, 0, 1]);
            });
        });

        describe("legacy example", function() {
            let view, up, right;
            beforeEach(function() {
                right= vec3.fromValues(1,  0, 0);
                up   = vec3.fromValues(0,  0, 1);
                view = vec3.fromValues(0, -1, 0);
                result = quat.setAxes(out, view, right, up);
            });

            xit("should set correct quat4 values", function() {
                expect(out).toBeEqualish([0.707106, 0, 0, 0.707106]);
            });
        });
    });

    describe("rotationTo", function() {
        let r;
        beforeEach(function() { r = vec3.create(); });

        describe("at right angle", function() {
            beforeEach(function() {
                result = quat.rotationTo(out, vec3.fromValues(0, 1, 0), vec3.fromValues(1, 0, 0));
            });

            

            it("should calculate proper quaternion", function() {
                expect(out).toBeEqualish([0, 0, -0.707106, 0.707106]);
            });
        });

        describe("when vectors are parallel", function() {
            beforeEach(function() {
                result = quat.rotationTo(out, vec3.fromValues(0, 1, 0), vec3.fromValues(0, 1, 0));
            });

            

            it("multiplying A should produce B", function() {
                vec3.transformQuat(r, vec3.fromValues(0, 1, 0), out)
                expect(r).toBeEqualish([0, 1, 0]);
            });
        });

        describe("when vectors are opposed X", function() {
            beforeEach(function() {
                result = quat.rotationTo(out, vec3.fromValues(1, 0, 0), vec3.fromValues(-1, 0, 0));
            });

            

            it("multiplying A should produce B", function() {
                vec3.transformQuat(r, vec3.fromValues(1, 0, 0), out)
                expect(r).toBeEqualish([-1, 0, 0]);
            });
        });

        describe("when vectors are opposed Y", function() {
            beforeEach(function() {
                result = quat.rotationTo(out, vec3.fromValues(0, 1, 0), vec3.fromValues(0, -1, 0));
            });

            

            it("multiplying A should produce B", function() {
                vec3.transformQuat(r, vec3.fromValues(0, 1, 0), out);
                expect(r).toBeEqualish([0, -1, 0]);
            });
        });

        describe("when vectors are opposed Z", function() {
            beforeEach(function() {
                result = quat.rotationTo(out, vec3.fromValues(0, 0, 1), vec3.fromValues(0, 0, -1));
            });

            

            it("multiplying A should produce B", function() {
                vec3.transformQuat(r, vec3.fromValues(0, 0, 1), out)
                expect(r).toBeEqualish([0, 0, -1]);
            });
        });
    });

    describe("create", function() {
        beforeEach(function() { result = quat.create(); });
        it("should return a 4 element array initialized to an identity quaternion", function() { expect(result).toBeEqualish([0, 0, 0, 1]); });
    });

    describe("clone", function() {
        beforeEach(function() { result = quat.clone(quatA); });
        it("should return a 4 element array initialized to the values in quatA", function() { expect(result).toBeEqualish(quatA); });
    });

    describe("fromValues", function() {
        beforeEach(function() { result = quat.fromValues(1, 2, 3, 4); });
        it("should return a 4 element array initialized to the values passed", function() { expect(result).toBeEqualish([1, 2, 3, 4]); });
    });

    describe("copy", function() {
        beforeEach(function() { result = quat.copy(out, quatA); });
        it("should place values into out", function() { expect(out).toBeEqualish([1, 2, 3, 4]); });
        
    });

    describe("set", function() {
        beforeEach(function() { result = quat.set(out, 1, 2, 3, 4); });
        it("should place values into out", function() { expect(out).toBeEqualish([1, 2, 3, 4]); });
        
    });

    describe("identity", function() {
        beforeEach(function() { result = quat.identity(out); });
        it("should place values into out", function() { expect(out).toBeEqualish([0, 0, 0, 1]); });
        
    });

    describe("setAxisAngle", function() {
        beforeEach(function() { result = quat.setAxisAngle(out, vec3.fromValues(1, 0, 0), Math.PI * 0.5); });
        it("should place values into out", function() { expect(out).toBeEqualish([0.707106, 0, 0, 0.707106]); });
        
    });

    describe("getAxisAngle", function() {
        describe("for a quaternion representing no rotation", function() {
            beforeEach(function() { result = quat.setAxisAngle(out, vec3.fromValues(0, 1, 0), 0.0); deg90 = quat.getAxisAngle(vec, out); });
            it("should return a multiple of 2*PI as the angle component", function() { expect(deg90 % (Math.PI * 2.0)).toBeEqualish(0.0); });
        });

        describe("for a simple rotation about X axis", function() {
            beforeEach(function() { result = quat.setAxisAngle(out, vec3.fromValues(1, 0, 0), 0.7778); deg90 = quat.getAxisAngle(vec, out); });
            it("should return the same provided angle", function() { expect(deg90).toBeEqualish(0.7778); });
            it("should return the X axis as the angle", function() { expect(vec).toBeEqualish([1, 0, 0]); });
        });

        describe("for a simple rotation about Y axis", function() {
            beforeEach(function() { result = quat.setAxisAngle(out, vec3.fromValues(0, 1, 0), 0.879546); deg90 = quat.getAxisAngle(vec, out); });
            it("should return the same provided angle", function() { expect(deg90).toBeEqualish(0.879546); });
            it("should return the X axis as the angle", function() { expect(vec).toBeEqualish([0, 1, 0]); });
        });

        describe("for a simple rotation about Z axis", function() {
            beforeEach(function() { result = quat.setAxisAngle(out, vec3.fromValues(0, 0, 1), 0.123456); deg90 = quat.getAxisAngle(vec, out); });
            it("should return the same provided angle", function() { expect(deg90).toBeEqualish(0.123456); });
            it("should return the X axis as the angle", function() { expect(vec).toBeEqualish([0, 0, 1]); });
        });

        describe("for a slightly irregular axis and right angle", function() {
            beforeEach(function() { result = quat.setAxisAngle(out, vec3.fromValues(0.707106, 0, 0.707106), Math.PI * 0.5); deg90 = quat.getAxisAngle(vec, out); });
            it("should place values into vec", function() { expect(vec).toBeEqualish([0.707106, 0, 0.707106]); });
            it("should return a numeric angle", function() { expect(deg90).toBeEqualish(Math.PI * 0.5); });
        });

        describe("for a very irregular axis and negative input angle", function() {
            beforeEach(function() {
                quat.setAxisAngle(quatA, vec3.fromValues(0.65538555, 0.49153915, 0.57346237), 8.8888);
                deg90 = quat.getAxisAngle(vec, quatA);
                quat.setAxisAngle(quatB, vec, deg90);
            });
            it("should return an angle between 0 and 2*PI", function() { expect(deg90).toBeGreaterThan(0.0); expect(deg90).toBeLessThan(Math.PI * 2.0); });
            it("should create the same quaternion from axis and angle extracted", function() { expect(quatA).toBeEqualish(quatB); });
        });
    });

    describe("add", function() {
        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat.add(out, quatA, quatB); });

            it("should place values into out", function() { expect(out).toBeEqualish([6, 8, 10, 12]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
            it("should not modify quatB", function() { expect(quatB).toBeEqualish([5, 6, 7, 8]); });
        });

        describe("when quatA is the output quaternion", function() {
            beforeEach(function() { result = quat.add(quatA, quatA, quatB); });

            it("should place values into quatA", function() { expect(quatA).toBeEqualish([6, 8, 10, 12]); });
            
            it("should not modify quatB", function() { expect(quatB).toBeEqualish([5, 6, 7, 8]); });
        });

        describe("when quatB is the output quaternion", function() {
            beforeEach(function() { result = quat.add(quatB, quatA, quatB); });

            it("should place values into quatB", function() { expect(quatB).toBeEqualish([6, 8, 10, 12]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
        });
    });

    describe("multiply", function() {
        

        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat.multiply(out, quatA, quatB); });

            it("should place values into out", function() { expect(out).toBeEqualish([24, 48, 48, -6]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
            it("should not modify quatB", function() { expect(quatB).toBeEqualish([5, 6, 7, 8]); });
        });

        describe("when quatA is the output quaternion", function() {
            beforeEach(function() { result = quat.multiply(quatA, quatA, quatB); });

            it("should place values into quatA", function() { expect(quatA).toBeEqualish([24, 48, 48, -6]); });
            
            it("should not modify quatB", function() { expect(quatB).toBeEqualish([5, 6, 7, 8]); });
        });

        describe("when quatB is the output quaternion", function() {
            beforeEach(function() { result = quat.multiply(quatB, quatA, quatB); });

            it("should place values into quatB", function() { expect(quatB).toBeEqualish([24, 48, 48, -6]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
        });
    });

    describe("scale", function() {
        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat.scale(out, quatA, 2); });

            it("should place values into out", function() { expect(out).toBeEqualish([2, 4, 6, 8]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
        });

        describe("when quatA is the output quaternion", function() {
            beforeEach(function() { result = quat.scale(quatA, quatA, 2); });

            it("should place values into quatA", function() { expect(quatA).toBeEqualish([2, 4, 6, 8]); });
            
        });
    });

    describe("length", function() {
        

        beforeEach(function() { result = quat.len(quatA); });

        it("should return the length", function() { expect(result).toBeEqualish(5.477225); });
    });

    describe("squaredLength", function() {
        

        beforeEach(function() { result = quat.squaredLength(quatA); });

        it("should return the squared length", function() { expect(result).toEqual(30); });
    });

    describe("normalize", function() {
        beforeEach(function() { quatA = quat.fromValues(5, 0, 0, 0); });

        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat.normalize(out, quatA); });

            it("should place values into out", function() { expect(out).toBeEqualish([1, 0, 0, 0]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([5, 0, 0, 0]); });
        });

        describe("when quatA is the output quaternion", function() {
            beforeEach(function() { result = quat.normalize(quatA, quatA); });

            it("should place values into quatA", function() { expect(quatA).toBeEqualish([1, 0, 0, 0]); });
            
        });
    });

    describe("lerp", function() {
        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat.lerp(out, quatA, quatB, 0.5); });

            it("should place values into out", function() { expect(out).toBeEqualish([3, 4, 5, 6]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
            it("should not modify quatB", function() { expect(quatB).toBeEqualish([5, 6, 7, 8]); });
        });

        describe("when quatA is the output quaternion", function() {
            beforeEach(function() { result = quat.lerp(quatA, quatA, quatB, 0.5); });

            it("should place values into quatA", function() { expect(quatA).toBeEqualish([3, 4, 5, 6]); });
            
            it("should not modify quatB", function() { expect(quatB).toBeEqualish([5, 6, 7, 8]); });
        });

        describe("when quatB is the output quaternion", function() {
            beforeEach(function() { result = quat.lerp(quatB, quatA, quatB, 0.5); });

            it("should place values into quatB", function() { expect(quatB).toBeEqualish([3, 4, 5, 6]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
        });
    });

    describe("slerp", function() {
        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat.slerp(out, quatA, quatB, 0.5); });

            it("should place values into out", function() { expect(out).toBeEqualish([3, 4, 5, 6]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
            it("should not modify quatB", function() { expect(quatB).toBeEqualish([5, 6, 7, 8]); });
        });

        describe("when quatA is the output quaternion", function() {
            beforeEach(function() { result = quat.slerp(quatA, quatA, quatB, 0.5); });

            it("should place values into quatA", function() { expect(quatA).toBeEqualish([3, 4, 5, 6]); });
            
            it("should not modify quatB", function() { expect(quatB).toBeEqualish([5, 6, 7, 8]); });
        });

        describe("when quatB is the output quaternion", function() {
            beforeEach(function() { result = quat.slerp(quatB, quatA, quatB, 0.5); });

            it("should place values into quatB", function() { expect(quatB).toBeEqualish([3, 4, 5, 6]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
        });
    });

    describe("random", function() {
        beforeEach(function() { result = quat.random(out); });

        it("should result in a normalized quaternion", function() {
            let copy = quat.clone(out);
            quat.normalize(out, out)
            expect(out).toBeEqualish(copy);
        });
        
    });

    describe("invert", function() {
        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat.invert(out, quatA); });

            it("should place values into out", function() { expect(out).toBeEqualish([-0.033333, -0.066666, -0.1, 0.133333]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
        });

        describe("when quatA is the output quaternion", function() {
            beforeEach(function() { result = quat.invert(quatA, quatA); });

            it("should place values into quatA", function() { expect(quatA).toBeEqualish([-0.033333, -0.066666, -0.1, 0.133333]); });
            
        });
    });

    describe("conjugate", function() {
        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat.conjugate(out, quatA); });

            it("should place values into out", function() { expect(out).toBeEqualish([-1, -2, -3, 4]); });
            
            it("should not modify quatA", function() { expect(quatA).toBeEqualish([1, 2, 3, 4]); });
        });

        describe("when quatA is the output quaternion", function() {
            beforeEach(function() { result = quat.conjugate(quatA, quatA); });

            it("should place values into quatA", function() { expect(quatA).toBeEqualish([-1, -2, -3, 4]); });
            
        });
    });

    describe("exactEquals", function() {
        let quatC, r0, r1;
        beforeEach(function() {
            quatA = quat.fromValues(0, 1, 2, 3);
            quatB = quat.fromValues(0, 1, 2, 3);
            quatC = quat.fromValues(1, 2, 3, 4);
            r0 = quat.exactEquals(quatA, quatB);
            r1 = quat.exactEquals(quatA, quatC);
        });

        it("should return true for identical quaternions", function() { expect(r0).toBe(true); });
        it("should return false for different quaternions", function() { expect(r1).toBe(false); });
        it("should not modify quatA", function() { expect(quatA).toBeEqualish([0, 1, 2, 3]); });
        it("should not modify quatB", function() { expect(quatB).toBeEqualish([0, 1, 2, 3]); });
    });

    describe("equals", function() {
        let quatC, quatD, r0, r1, r2;
        beforeEach(function() {
            quatA = quat.fromValues(0, 1, 2, 3);
            quatB = quat.fromValues(0, 1, 2, 3);
            quatC = quat.fromValues(1, 2, 3, 4);
            quatD = quat.fromValues(1e-16, 1, 2, 3);
            r0 = quat.equals(quatA, quatB);
            r1 = quat.equals(quatA, quatC);
            r2 = quat.equals(quatA, quatD);
        });
        it("should return true for identical quaternions", function() { expect(r0).toBe(true); });
        it("should return false for different quaternions", function() { expect(r1).toBe(false); });
        it("should return true for close but not identical quaternions", function() { expect(r2).toBe(true); });
        it("should not modify quatA", function() { expect(quatA).toBeEqualish([0, 1, 2, 3]); });
        it("should not modify quatB", function() { expect(quatB).toBeEqualish([0, 1, 2, 3]); });
    });
});
