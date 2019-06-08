import { expect } from './spec-helper';

describe("vec2", function() {
    let vec2, vec3, mat2, mat2d;
    let out, vecA, vecB, result;

    before(done => {
        import('../pkg/gl_matrix_wasm').then(({Vector2, Vector3, Matrix2, Matrix2d}) => {
            vec2 = Vector2;
            vec3 = Vector3;
            mat2 = Matrix2;
            mat2d = Matrix2d;
            done();
        });
    });

    beforeEach(function() { vecA = vec2.fromValues(1, 2); vecB = vec2.fromValues(3, 4); out = vec2.fromValues(0, 0); });

    describe("create", function() {
        beforeEach(function() { result = vec2.create(); });
        it("should return a 2 element array initialized to 0s", function() { expect(result).toBeEqualish([0, 0]); });
    });

    describe("clone", function() {
        beforeEach(function() { result = vec2.clone(vecA); });
        it("should return a 2 element array initialized to the values in vecA", function() { expect(result).toBeEqualish(vecA); });
    });

    describe("fromValues", function() {
        beforeEach(function() { result = vec2.fromValues(1, 2); });
        it("should return a 2 element array initialized to the values passed", function() { expect(result).toBeEqualish([1, 2]); });
    });

    describe("copy", function() {
        beforeEach(function() { result = vec2.copy(out, vecA); });
        it("should place values into out", function() { expect(out).toBeEqualish([1, 2]); });
        
    });

    describe("set", function() {
        beforeEach(function() { result = vec2.set(out, 1, 2); });
        it("should place values into out", function() { expect(out).toBeEqualish([1, 2]); });
        
    });

    describe("add", function() {
        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.add(out, vecA, vecB); });

            it("should place values into out", function() { expect(out).toBeEqualish([4, 6]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.add(vecA, vecA, vecB); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([4, 6]); });
            
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecB is the output vector", function() {
            beforeEach(function() { result = vec2.add(vecB, vecA, vecB); });

            it("should place values into vecB", function() { expect(vecB).toBeEqualish([4, 6]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        });
    });

    describe("subtract", function() {
        

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.subtract(out, vecA, vecB); });

            it("should place values into out", function() { expect(out).toBeEqualish([-2, -2]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.subtract(vecA, vecA, vecB); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([-2, -2]); });
            
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecB is the output vector", function() {
            beforeEach(function() { result = vec2.subtract(vecB, vecA, vecB); });

            it("should place values into vecB", function() { expect(vecB).toBeEqualish([-2, -2]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        });
    });

    describe("multiply", function() {
        

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.multiply(out, vecA, vecB); });

            it("should place values into out", function() { expect(out).toBeEqualish([3, 8]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.multiply(vecA, vecA, vecB); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([3, 8]); });
            
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecB is the output vector", function() {
            beforeEach(function() { result = vec2.multiply(vecB, vecA, vecB); });

            it("should place values into vecB", function() { expect(vecB).toBeEqualish([3, 8]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        });
    });

    describe("divide", function() {
        

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.divide(out, vecA, vecB); });

            it("should place values into out", function() { expect(out).toBeEqualish([0.3333333, 0.5]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.divide(vecA, vecA, vecB); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([0.3333333, 0.5]); });
            
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecB is the output vector", function() {
            beforeEach(function() { result = vec2.divide(vecB, vecA, vecB); });

            it("should place values into vecB", function() { expect(vecB).toBeEqualish([0.3333333, 0.5]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        });
    });

    describe("ceil", function() {
        beforeEach(function() { vecA = vec2.fromValues(Math.E, Math.PI); });

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.ceil(out, vecA); });

            it("should place values into out", function() { expect(out).toBeEqualish([3, 4]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([Math.E, Math.PI]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.ceil(vecA, vecA); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([3, 4]); });
            
        });
    });

    describe("floor", function() {
        beforeEach(function() { vecA = vec2.fromValues(Math.E, Math.PI); });

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.floor(out, vecA); });

            it("should place values into out", function() { expect(out).toBeEqualish([2, 3]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([Math.E, Math.PI]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.floor(vecA, vecA); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([2, 3]); });
            
        });
    });

    describe("min", function() {
        beforeEach(function() { vecA = vec2.fromValues(1, 4); vecB = vec2.fromValues(3, 2); });

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.min(out, vecA, vecB); });

            it("should place values into out", function() { expect(out).toBeEqualish([1, 2]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 4]); });
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 2]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.min(vecA, vecA, vecB); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
            
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 2]); });
        });

        describe("when vecB is the output vector", function() {
            beforeEach(function() { result = vec2.min(vecB, vecA, vecB); });

            it("should place values into vecB", function() { expect(vecB).toBeEqualish([1, 2]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 4]); });
        });
    });

    describe("max", function() {
        beforeEach(function() { vecA = vec2.fromValues(1, 4); vecB = vec2.fromValues(3, 2); });

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.max(out, vecA, vecB); });

            it("should place values into out", function() { expect(out).toBeEqualish([3, 4]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 4]); });
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 2]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.max(vecA, vecA, vecB); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([3, 4]); });
            
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 2]); });
        });

        describe("when vecB is the output vector", function() {
            beforeEach(function() { result = vec2.max(vecB, vecA, vecB); });

            it("should place values into vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 4]); });
        });
    });

    describe("round", function() {
        beforeEach(function() { vecA = vec2.fromValues(Math.E, Math.PI); });

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.round(out, vecA); });

            it("should place values into out", function() { expect(out).toBeEqualish([3, 3]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([Math.E, Math.PI]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.round(vecA, vecA); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([3, 3]); });
            
        });
    });

    describe("scale", function() {
        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.scale(out, vecA, 2); });

            it("should place values into out", function() { expect(out).toBeEqualish([2, 4]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.scale(vecA, vecA, 2); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([2, 4]); });
            
        });
    });

    describe("scaleAndAdd", function() {
        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.scaleAndAdd(out, vecA, vecB, 0.5); });

            it("should place values into out", function() { expect(out).toBeEqualish([2.5, 4]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.scaleAndAdd(vecA, vecA, vecB, 0.5); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([2.5, 4]); });
            
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecB is the output vector", function() {
            beforeEach(function() { result = vec2.scaleAndAdd(vecB, vecA, vecB, 0.5); });

            it("should place values into vecB", function() { expect(vecB).toBeEqualish([2.5, 4]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        });
    });

    describe("distance", function() {
        

        beforeEach(function() { result = vec2.distance(vecA, vecB); });

        it("should return the distance", function() { expect(result).toBeEqualish(2.828427); });
    });

    describe("squaredDistance", function() {
        

        beforeEach(function() { result = vec2.squaredDistance(vecA, vecB); });

        it("should return the squared distance", function() { expect(result).toEqual(8); });
    });

    describe("length", function() {
        

        beforeEach(function() { result = vec2.len(vecA); });

        it("should return the length", function() { expect(result).toBeEqualish(2.236067); });
    });

    describe("squaredLength", function() {
        

        beforeEach(function() { result = vec2.squaredLength(vecA); });

        it("should return the squared length", function() { expect(result).toEqual(5); });
    });

    describe("negate", function() {
        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.negate(out, vecA); });

            it("should place values into out", function() { expect(out).toBeEqualish([-1, -2]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.negate(vecA, vecA); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([-1, -2]); });
            
        });
    });

    describe("normalize", function() {
        beforeEach(function() { vecA = vec2.fromValues(5, 0); });

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.normalize(out, vecA); });

            it("should place values into out", function() { expect(out).toBeEqualish([1, 0]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([5, 0]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.normalize(vecA, vecA); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([1, 0]); });
            
        });
    });

    describe("dot", function() {
        beforeEach(function() { result = vec2.dot(vecA, vecB); });

        it("should return the dot product", function() { expect(result).toEqual(11); });
        it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
    });

    describe("cross", function() {
        let out3;

        beforeEach(function() {
            out3 = vec3.fromValues(0, 0, 0);
            result = vec2.cross(out3, vecA, vecB);
        });

        it("should place values into out", function() { expect(out3).toBeEqualish([0, 0, -2]); });
        
        it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
    });

    describe("lerp", function() {
        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.lerp(out, vecA, vecB, 0.5); });

            it("should place values into out", function() { expect(out).toBeEqualish([2, 3]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.lerp(vecA, vecA, vecB, 0.5); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([2, 3]); });
            
            it("should not modify vecB", function() { expect(vecB).toBeEqualish([3, 4]); });
        });

        describe("when vecB is the output vector", function() {
            beforeEach(function() { result = vec2.lerp(vecB, vecA, vecB, 0.5); });

            it("should place values into vecB", function() { expect(vecB).toBeEqualish([2, 3]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
        });
    });

    describe("random", function() {
        describe("with no scale", function() {
            beforeEach(function() { result = vec2.random(out); });

            it("should result in a unit length vector", function() { expect(vec2.len(out)).toBeEqualish(1.0); });
            
        });

        describe("with a scale", function() {
            beforeEach(function() { result = vec2.random(out, 5.0); });

            it("should result in a unit length vector", function() { expect(vec2.len(out)).toBeEqualish(5.0); });
            
        });
    });

    describe("transformMat2", function() {
        let matA;
        beforeEach(function() { matA = mat2.fromValues(1, 2, 3, 4); });

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.transformMat2(out, vecA, matA); });

            it("should place values into out", function() { expect(out).toBeEqualish([7, 10]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.transformMat2(vecA, vecA, matA); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([7, 10]); });
            
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4]); });
        });
    });

    describe("transformMat2d", function() {
        let matA;
        beforeEach(function() { matA = mat2d.fromValues(1, 2, 3, 4, 5, 6); });

        describe("with a separate output vector", function() {
            beforeEach(function() { result = vec2.transformMat2d(out, vecA, matA); });

            it("should place values into out", function() { expect(out).toBeEqualish([12, 16]); });
            
            it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 2]); });
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6]); });
        });

        describe("when vecA is the output vector", function() {
            beforeEach(function() { result = vec2.transformMat2d(vecA, vecA, matA); });

            it("should place values into vecA", function() { expect(vecA).toBeEqualish([12, 16]); });
            
            it("should not modify matA", function() { expect(matA).toBeEqualish([1, 2, 3, 4, 5, 6]); });
        });
    });

    describe('rotate', function(){
		describe('rotation around world origin [0, 0, 0]', function(){
			  beforeEach(function(){ vecA = vec2.fromValues(0, 1); vecB = vec2.fromValues(0, 0); result = vec2.rotate(out, vecA, vecB, Math.PI); });
			  it("should return the rotated vector", function(){ expect(out).toBeEqualish([0, -1]); });
		});
		describe('rotation around an arbitrary origin', function(){
			  beforeEach(function(){ vecA = vec2.fromValues(6, -5); vecB = vec2.fromValues(0, -5); result = vec2.rotate(out, vecA, vecB, Math.PI); });
			  it("should return the rotated vector", function(){ expect(out).toBeEqualish([-6, -5]); });
		});
	});
    
    describe("angle", function() {
        beforeEach(function() {
            vecA = vec2.fromValues(1,0);
            vecB = vec2.fromValues(1,2);
            result = vec2.angle(vecA, vecB);
        });

        it("should return the angle", function() { expect(result).toBeEqualish(1.10714); });
        it("should not modify vecA", function() { expect(vecA).toBeEqualish([1, 0]); });
        it("should not modify vecB", function() { expect(vecB).toBeEqualish([1, 2]); });
    });

    describe("exactEquals", function() {
        let vecC, r0, r1;
        beforeEach(function() {
            vecA = vec2.fromValues(0, 1);
            vecB = vec2.fromValues(0, 1);
            vecC = vec2.fromValues(1, 2);
            r0 = vec2.exactEquals(vecA, vecB);
            r1 = vec2.exactEquals(vecA, vecC);
        });

        it("should return true for identical vectors", function() { expect(r0).toBe(true); });
        it("should return false for different vectors", function() { expect(r1).toBe(false); });
        it("should not modify vecA", function() { expect(vecA).toBeEqualish([0, 1]); });
        it("should not modify vecB", function() { expect(vecB).toBeEqualish([0, 1]); });
    });

    describe("equals", function() {
        let vecC, vecD, r0, r1, r2;
        beforeEach(function() {
            vecA = vec2.fromValues(0, 1);
            vecB = vec2.fromValues(0, 1);
            vecC = vec2.fromValues(1, 2);
            vecD = vec2.fromValues(1e-16, 1);
            r0 = vec2.equals(vecA, vecB);
            r1 = vec2.equals(vecA, vecC);
            r2 = vec2.equals(vecA, vecD);
        });
        it("should return true for identical vectors", function() { expect(r0).toBe(true); });
        it("should return false for different vectors", function() { expect(r1).toBe(false); });
        it("should return true for close but not identical vectors", function() { expect(r2).toBe(true); });
        it("should not modify vecA", function() { expect(vecA).toBeEqualish([0, 1]); });
        it("should not modify vecB", function() { expect(vecB).toBeEqualish([0, 1]); });
    });

    describe("zero", function() {
        beforeEach(function() {
            vecA = vec2.fromValues(1, 2);
            result = vec2.zero(vecA);
        });
        it("should result in a 2 element vector with zeros", function() { expect(vecA).toBeEqualish([0, 0]); });
    });
});
