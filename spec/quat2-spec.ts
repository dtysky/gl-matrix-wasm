import { expect } from './spec-helper';

describe("quat2", function() {
    let vec3, mat4, quat, quat2: typeof import('../pkg/gl_matrix_wasm').Quaternion2;
    let out, outVec, quat2A, quat2B, result, resultVec, outQuat;
    let matrixA, matOut, quatOut;
    let vec, ax;

    before(done => {
        import('../pkg/gl_matrix_wasm').then(({Vector3, Matrix4, Quaternion, Quaternion2}) => {
            vec3 = Vector3;
            mat4 = Matrix4;
            quat = Quaternion;
            quat2 = Quaternion2;
            done();
        });
    });

    beforeEach(function() {
        quat2A = quat2.fromValues(1, 2, 3, 4, 2, 5, 6, -2);
        quat2B = quat2.fromValues(5, 6, 7, 8, 9, 8, 6, -4);
        out= quat2.fromValues(0, 0, 0, 0, 0, 0, 0, 0);
        outVec= vec3.fromValues(0, 0, 0);
        outQuat= quat.fromValues(0, 0, 0, 0);
        vec = vec3.fromValues(1, 1, -1);
    });

    describe("translate", function() {
        beforeEach(function() {
            matrixA = mat4.create(), matOut = mat4.create(), quatOut = quat2.create();
              //quat2A only seems to work when created using this function?
              quat2.fromRotationTranslation(quat2A, quat.fromValues(1,2,3,4), vec3.fromValues(-5, 4, 10));
              quat2.normalize(quat2A, quat2A);
              quat2.copy(quat2B, quat2A);
              mat4.fromQuat2(matrixA, quat2A);
        });

        describe("with a separate output quaternion", function() {
            beforeEach(function() {
                    result = quat2.translate(out, quat2A, vec);
                    //Same thing with a matrix
                    mat4.translate(matOut, matrixA, vec);
                    quat2.fromMat4(quatOut, matOut);
                });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2(quatOut); });
            
            it("should not modify quat2A", function() {
                expect(quat2A).toBeEqualishQuat2(quat2B);
            });
            it("should not modify vec", function() { expect(vec).toBeEqualish([1,1,-1]); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() {
                    result = quat2.translate(quat2A, quat2A, vec);
                    //Same thing with a matrix
                    mat4.translate(matOut, matrixA, vec);
                    quat2.fromMat4(quatOut, matOut);
                });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2(quatOut); });
            
        });
    });

    describe("rotateAroundAxis", function() {
        beforeEach(function() {
            matrixA = mat4.create(), matOut = mat4.create(), ax = vec3.fromValues(1,4,2);
              //quat2A only seems to work when created using this function?
              quat2.fromRotationTranslation(quat2A, quat.fromValues(1,2,3,4), vec3.fromValues(-5, 4, 10));
              quat2.normalize(quat2A, quat2A);
              mat4.fromQuat2(matrixA, quat2A);
            });


        describe("with a separate output quaternion", function() {
            beforeEach(function() {
                    result = quat2.rotateAroundAxis(out, quat2A, ax, 5);

                    //Same thing with a matrix
                    mat4.rotate(matOut, matrixA, 5, ax);
                    quat2.fromMat4(quat2B, matOut);
                });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2(quat2B); });
            
            it("should not modify quat2A", function() { 
                expect(quat2A).toBeEqualishQuat2(
                [0.18257418583505536, 0.3651483716701107, 0.5477225575051661, 0.7302967433402214,
                -2.556038601690775, 3.742770809618635, 2.37346441585572, -3.0124740662784135]
            ); });
            it("should not modify ax", function() { expect(ax).toBeEqualish([1, 4, 2]); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() {
                    result = quat2.rotateAroundAxis(quat2A, quat2A, ax, 5);
                    //Same thing with a matrix

                    mat4.rotate(matOut, matrixA, 5, ax);
                    quat2.fromMat4(quat2B, matOut);
                });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2(quat2B); });
            
            it("should not modify ax", function() { expect(ax).toBeEqualish([1, 4, 2]); });
        });
    });

    describe("rotateByQuatAppend", function() {
        let correctResult;
        let rotationQuat;
        beforeEach(function() {
            correctResult = quat2.create();
            rotationQuat = quat2.create();
            rotationQuat[0] = 2;
            rotationQuat[1] = 5;
            rotationQuat[2] = 2;
            rotationQuat[3] = -10;
            quat2.multiply(correctResult, quat2A, rotationQuat);
        })
        describe("with a separate output quaternion", function() {
            beforeEach(function() {
                result = quat2.rotateByQuatAppend(out, quat2A, quat.fromValues(2, 5, 2, -10));
            });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2(correctResult); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
            it("should not modify the rotation quaternion", function() { expect(rotationQuat).toBeEqualishQuat2([2,5,2,-10,0,0,0,0]); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() { result = quat2.rotateByQuatAppend(quat2A, quat2A, quat.fromValues(2, 5, 2, -10)); });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2(correctResult); });
            
            it("should not modify the rotation quaternion", function() { expect(rotationQuat).toBeEqualishQuat2([2,5,2,-10,0,0,0,0]); });
        });
    });

    describe("rotateByQuatPrepend", function() {
        let correctResult;
        let rotationQuat;
        beforeEach(function() {
            correctResult = quat2.create();
        rotationQuat = quat2.create();
            rotationQuat[0] = 2;
            rotationQuat[1] = 5;
            rotationQuat[2] = 2;
            rotationQuat[3] = -10;
            quat2.multiply(correctResult, rotationQuat, quat2A);
        })
        describe("with a separate output quaternion", function() {
            beforeEach(function() {
                quat2.getReal(outQuat, rotationQuat);
                result = quat2.rotateByQuatPrepend(out, outQuat, quat2A);
            });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2(correctResult); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
            it("should not modify the rotation quaternion", function() { expect(rotationQuat).toBeEqualishQuat2([2,5,2,-10,0,0,0,0]); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() {
                quat2.getReal(outQuat, rotationQuat);
                result = quat2.rotateByQuatPrepend(quat2A, outQuat, quat2A);
            });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2(correctResult); });
            
            it("should not modify the rotation quaternion", function() { expect(rotationQuat).toBeEqualishQuat2([2,5,2,-10,0,0,0,0]); });
        });
    });

    describe("rotateX", function() {
        beforeEach(function() {
            matrixA = mat4.create(), matOut = mat4.create(), quatOut = quat2.create();
              //quat2A only seems to work when created using this function?
              quat2.fromRotationTranslation(quat2A, quat.fromValues(1,2,3,4), vec3.fromValues(-5, 4, 10));
              quat2.normalize(quat2A, quat2A);
              quat2.copy(quat2B, quat2A);
              mat4.fromQuat2(matrixA, quat2A);
            });
        describe("with a separate output quaternion", function() {
            beforeEach(function() {
                    result = quat2.rotateX(out, quat2A, 5);
                    //Same thing with a matrix
                    mat4.rotateX(matOut, matrixA, 5);
                    quat2.fromMat4(quatOut, matOut);
                });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2(quatOut); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2(quat2B); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() {
                    result = quat2.rotateX(quat2A, quat2A, 5);
                    //Same thing with a matrix
                    mat4.rotateX(matOut, matrixA, 5);
                    quat2.fromMat4(quatOut, matOut);
                });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2(quatOut); });
            
        });
    });

    describe("rotateY", function() {
        beforeEach(function() {
            matrixA = mat4.create(), matOut = mat4.create(), quatOut = quat2.create();
              //quat2A only seems to work when created using this function?
              quat2.fromRotationTranslation(quat2A, quat.fromValues(1,2,3,4), vec3.fromValues(5, 4, -10));
              quat2.normalize(quat2A, quat2A);
              quat2.copy(quat2B, quat2A);
              mat4.fromQuat2(matrixA, quat2A);
            });

        describe("with a separate output quaternion", function() {
            beforeEach(function() {
                    result = quat2.rotateY(out, quat2A, -2);
                    //Same thing with a matrix
                    mat4.rotateY(matOut, matrixA, -2);
                    quat2.fromMat4(quatOut, matOut);
                });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2(quatOut); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2(quat2B); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() {
                    result = quat2.rotateY(quat2A, quat2A, -2);
                    //Same thing with a matrix
                    mat4.rotateY(matOut, matrixA, -2);
                    quat2.fromMat4(quatOut, matOut);
                });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2(quatOut); });
            
        });
    });

    describe("rotateZ", function() {
        beforeEach(function() {
            matrixA = mat4.create(), matOut = mat4.create(), quatOut = quat2.create();
              //quat2A only seems to work when created using this function?
              quat2.fromRotationTranslation(quat2A, quat.fromValues(1,0,3,-4), vec3.fromValues(0, -4, -10));
              quat2.normalize(quat2A, quat2A);
              quat2.copy(quat2B, quat2A);
              mat4.fromQuat2(matrixA, quat2A);
            });
        describe("with a separate output quaternion", function() {
            beforeEach(function() {
                    result = quat2.rotateZ(out, quat2A, 1);
                    //Same thing with a matrix
                    mat4.rotateZ(matOut, matrixA, 1);
                    quat2.fromMat4(quatOut, matOut);
                });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2(quatOut); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2(quat2B); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() {
                    result = quat2.rotateZ(quat2A, quat2A, 1);
                    //Same thing with a matrix
                    mat4.rotateZ(matOut, matrixA, 1);
                    quat2.fromMat4(quatOut, matOut);
                });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2(quatOut); });
            
        });
    });

    describe("from/toMat4", function() {
        let matRes, matOut;
        let rotationQuat;
        describe("quat to matrix and back", function() {
            beforeEach(function() {
                matRes = mat4.create(), matOut = mat4.create();
                rotationQuat = quat.create();
                quat.normalize(rotationQuat, quat.fromValues(1,2,3,4));

                quat2.fromRotationTranslation(quat2A, rotationQuat, vec3.fromValues(1,-5,3));
                mat4.fromQuat2(matOut, quat2A);

                result = quat2.fromMat4(out, matOut);
            });

            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([0.18257418, 0.36514836, 0.54772257, 0.73029673,  -1.5518806, -1.82574184, 1.73445473, 0 ]); });

            it("should be equal to the starting dual quat", function() {
                expect(quat2A).toBeEqualishQuat2(out);
            });

        });
    });

    describe("create", function() {
        beforeEach(function() { result = quat2.create(); });
        it("should return 2 4 element arrays initialized to an identity dual quaternion", function() { expect(result).toBeEqualishQuat2([0, 0, 0, 1, 0, 0, 0, 0]); });
    });

    describe("clone", function() {
        beforeEach(function() { result = quat2.clone(quat2A); });
        it("should return 2 4 element arrays initialized to the values in quat2A", function() { expect(result).toBeEqualishQuat2(quat2A); });
    });

    describe("fromValues", function() {
        beforeEach(function() { result = quat2.fromValues(1, 2, 3, 4, 5, 7, 8, -2); });
        it("should return 2 4 element arrays initialized to the values passedd to the values passed", function() {
            expect(result).toBeEqualishQuat2([1, 2, 3, 4, 5, 7, 8, -2]);
        });
    });

    describe("copy", function() {
        beforeEach(function() { result = quat2.copy(out, quat2A); });
        it("should place values into out", function() { expect(out).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
        
    });

    describe("set", function() {
        beforeEach(function() { result = quat2.set(out, 1, 2, 3, 4, 2, 5, 6, -2); });
        it("should place values into out", function() { expect(out).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
        
    });

    describe("identity", function() {
        beforeEach(function() { result = quat2.identity(out); });
        it("should place values into out", function() { expect(out).toBeEqualishQuat2([0, 0, 0, 1, 0, 0, 0, 0]); });
        
    });

    describe("add", function() {
        describe("with a separate output dual quaternion", function() {
            beforeEach(function() { result = quat2.add(out, quat2A, quat2B); });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2([6, 8, 10, 12, 11, 13, 12, -6]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
            it("should not modify quat2B", function() { expect(quat2B).toBeEqualishQuat2([5, 6, 7, 8, 9, 8, 6, -4]); });
        });

        describe("when quat2A is the output dual quaternion", function() {
            beforeEach(function() { result = quat2.add(quat2A, quat2A, quat2B); });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2([6, 8, 10, 12, 11, 13, 12, -6]); });
            
            it("should not modify quat2B", function() { expect(quat2B).toBeEqualishQuat2([5, 6, 7, 8, 9, 8, 6, -4])});
        });

        describe("when quat2B is the output dual quaternion", function() {
            beforeEach(function() { result = quat2.add(quat2B, quat2A, quat2B); });

            it("should place values into quat2B", function() { expect(quat2B).toBeEqualishQuat2([6, 8, 10, 12, 11, 13, 12, -6]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
        });
    });

    describe("multiply", function() {
        

        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat2.multiply(out, quat2A, quat2B); });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2([24, 48, 48, -6,  25, 89, 23, -157 ]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
            it("should not modify quat2B", function() { expect(quat2B).toBeEqualishQuat2([5, 6, 7, 8, 9, 8, 6, -4]); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() { result = quat2.multiply(quat2A, quat2A, quat2B); });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2([24, 48, 48, -6,  25, 89, 23, -157 ]); });
            
            it("should not modify quat2B", function() { expect(quat2B).toBeEqualishQuat2([5, 6, 7, 8, 9, 8, 6, -4]); });
        });

        describe("when quat2B is the output quaternion", function() {
            beforeEach(function() { result = quat2.multiply(quat2B, quat2A, quat2B); });

            it("should place values into quat2B", function() { expect(quat2B).toBeEqualishQuat2([24, 48, 48, -6,  25, 89, 23, -157 ]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
        });

        describe("same as matrix multiplication", function() {
            let matrixA, matrixB;
            let matOut, quatOut;
           beforeEach(function() {
            matrixA = mat4.create(), matrixB = mat4.create();
            matOut = mat4.create(), quatOut = quat2.create();
              //quat2A and quat2B only seem to work when created using this function?
              quat2.fromRotationTranslation(quat2A, quat.fromValues(1,2,3,4), vec3.fromValues(-5, 4, 10));
              quat2.normalize(quat2A, quat2A);
              mat4.fromQuat2(matrixA, quat2A);

              quat2.fromRotationTranslation(quat2B, quat.fromValues(5, 6, 7, 8), vec3.fromValues(9, 8, 6));
              quat2.normalize(quat2B, quat2B);
              mat4.fromQuat2(matrixB, quat2B);

            });
            it("the matrices should be equal to the dual quaternions", function() {
                let testQuat = quat2.create();
                quat2.fromMat4(testQuat, matrixA);
                expect(testQuat).toBeEqualishQuat2(quat2A);

                quat2.fromMat4(testQuat, matrixB);
                expect(testQuat).toBeEqualishQuat2(quat2B);
            });

            it("should be equal to the matrix multiplication", function() {
                quat2.multiply(out, quat2A, quat2B);
                mat4.mul(matOut, matrixA, matrixB);
                quat2.fromMat4(quatOut, matOut);
                expect(out).toBeEqualishQuat2(quatOut);
            });

        });
    });

    describe("scale", function() {
        describe("with a separate output dual quaternion", function() {
            beforeEach(function() { result = quat2.scale(out, quat2A, 2); });
            it("should place values into out", function() { expect(out).toBeEqualishQuat2([2, 4, 6, 8, 4, 10, 12, -4]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
        });

        describe("when quat2A is the output dual quaternion", function() {
            beforeEach(function() { result = quat2.scale(quat2A, quat2A, 2); });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2([2, 4, 6, 8, 4, 10, 12, -4]); });
            
        });
    });

    describe("length", function() {
        

        beforeEach(function() { result = quat2.len(quat2A); });

        it("should return the length", function() { expect(result).toBeEqualish(5.477225); });
    });

    describe("squaredLength", function() {
        

        beforeEach(function() { result = quat2.squaredLength(quat2A); });

        it("should return the squared length", function() { expect(result).toEqual(30); });
    });

    describe("fromRotation", function() {
        beforeEach(function() { result = quat2.fromRotation(out, quat.fromValues(1, 2, 3, 4)); });
        it("should place values into out", function() { expect(out).toBeEqualishQuat2([1, 2, 3, 4, 0, 0, 0, 0]); });
        
        it("should not modify the quaternion", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
    });

    describe("fromTranslation", function(){
        beforeEach(function() { vec = vec3.fromValues(1, 2, 3); result = quat2.fromTranslation(out, vec); });
        it("should place values into out", function() { expect(out).toBeEqualishQuat2([0, 0, 0, 1, 0.5, 1, 1.5, 0]); });
        
        it("should not modify the vector", function() { expect(vec).toBeEqualish([1, 2, 3]); });
    });

    describe("fromRotationTranslation", function() {
        beforeEach(function() {
            vec = vec3.fromValues(1, 2, 3);
            result = quat2.fromRotationTranslation(out, quat.fromValues(1, 2, 3, 4), vec);
        });
        it("should place values into out", function() { expect(out).toBeEqualishQuat2([1, 2, 3, 4,  2, 4, 6, -7]); });
        
        it("should not modify the quaternion", function() {quat2.getReal(outQuat, quat2A); expect(outQuat).toBeEqualish([1, 2, 3, 4]); });
        it("should not modify the vector", function() { expect(vec).toBeEqualish([1, 2, 3]); });
        it("should have a translation that can be retrieved with getTranslation", function() {
            let t = vec3.fromValues(0, 0, 0);
            quat2.normalize(out, out);
            quat2.getTranslation(t, out);

            expect(t).toBeEqualish([1, 2, 3]);
        });
    });

    describe("fromRotationTranslationValues", function() {
        beforeEach(function() { result = quat2.fromRotationTranslationValues(1,2,3,4, 1,2,3); });
        it("should return the correct result", function() { expect(result).toBeEqualishQuat2([1, 2, 3, 4,  2, 4, 6, -7]); });
        it("should have a translation that can be retrieved with getTranslation", function() {
            let t = vec3.fromValues(0, 0, 0);
            quat2.normalize(result, result);
            quat2.getTranslation(t, result);
            expect(t).toBeEqualish([1, 2, 3]);
        });
    });

    describe("getTranslation", function() {
        describe("without a real part", function() {
           beforeEach(function() {
               quat2.fromTranslation(out, vec3.fromValues(1,2,3));
               resultVec = quat2.getTranslation(outVec, out);
           });
           describe("not normalized", function() {
                    it("should return the same translation value", function() {
                        expect(outVec).toBeEqualish([1, 2, 3]);
                    });
                    
               });
           describe("normalized", function() {
                it("should return the same translation value", function() {
                    quat2.normalize(out, out);
                    quat2.getTranslation(outVec, out);
                    expect(outVec).toBeEqualish([1, 2, 3]);
                });
           });
        });
        describe("with a real part", function() {
            beforeEach(function() {
                quat2.fromRotationTranslation(out, quat.fromValues(2, 4, 6, 2), vec3.fromValues(1, 2, 3));
                resultVec = quat2.getTranslation(outVec, out);
            });
            describe("not normalized", function() {
                    it("should not return the same translation value", function() {
                        expect(outVec).not.toBeEqualish([1, 2, 3]);
                    });
                    
               });
            describe("normalized", function() {
                it("should return the same translation value", function() {
                    quat2.normalize(out, out);
                    quat2.getTranslation(outVec, out);
                    expect(outVec).toBeEqualish([1, 2, 3]);
                });
           });
        });
    });

    describe("normalize", function() {
        describe("when it is normalizing quat2A", function() {
            beforeEach(function() {
                quat2A = quat2.fromValues(1, 2, 3, 4, 2, 5, 6, -2);
                quat2.normalize(out, quat2A);
            });
            it("both parts should have been normalized", function() { expect(out).toBeEqualishQuat2([1/5.4772255, 2/5.4772255, 3/5.4772255, 4/5.4772255, 0.231260, 0.6450954, 0.693781,-0.9006993]); });
        });

        beforeEach(function() { quat2A = quat2.fromValues(5, 0, 0, 0, 0, 0, 0, 0); });

        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat2.normalize(out, quat2A); });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2([1, 0, 0, 0, 0, 0, 0, 0]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([5, 0, 0, 0, 0, 0, 0, 0]); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() { result = quat2.normalize(quat2A, quat2A); });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 0, 0, 0, 0, 0, 0, 0]); });
            
        });

        describe("when it contains a translation", function() {
            beforeEach(function() {
                quat2.set(out, 5, 0, 0, 0, 1, 2, 3, 5);
                quat2.normalize(out, out);
            });
            it("both parts should have been normalized", function() { expect(out).toBeEqualishQuat2([1, 0, 0, 0, 0, 0.4, 0.6, 1]); });
        });
    });

    describe("lerp", function() {
        describe("with a separate output quaternion", function() {
            beforeEach(function() { result = quat2.lerp(out, quat2A, quat2B, 0.7); });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2([3.8, 4.8, 5.8, 6.8, 6.9, 7.1, 6.0, -3.4]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
            it("should not modify quat2B", function() { expect(quat2B).toBeEqualishQuat2([5, 6, 7, 8, 9, 8, 6, -4]); });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() { result = quat2.lerp(quat2A, quat2A, quat2B, 0.5); });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2([3, 4, 5, 6,5.5, 6.5, 6, -3]); });
            
            it("should not modify quat2B", function() { expect(quat2B).toBeEqualishQuat2([5, 6, 7, 8, 9, 8, 6, -4]); });
        });

        describe("when quat2B is the output quaternion", function() {
            beforeEach(function() { result = quat2.lerp(quat2B, quat2A, quat2B, 0.5); });

            it("should place values into quat2B", function() { expect(quat2B).toBeEqualishQuat2([3, 4, 5, 6,5.5, 6.5, 6, -3]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
        });

        describe("shortest path", function() {
            beforeEach(function() { result = quat2.lerp(out, quat2.fromValues(1, 2, 3, -4, 2, 5, 6, -2), quat2.fromValues(5, -6, 7, 8, 9, 8, 6, -4), 0.4); });
            it("should pick the shorter path", function() { expect(out).toBeEqualishQuat2([ -1.4, 3.6, -1, -5.6, -2.4, -0.2, 1.2, 0.4 ]); });
        });
    });

    describe("dot", function() {
        describe("with a separate output dual quaternion", function() {
            beforeEach(function() { result = quat2.dot(quat2A, quat2B); });
            it("should return the dot product", function() { expect(result).toBeEqualish(70); });
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
            it("should not modify quat2B", function() { expect(quat2B).toBeEqualishQuat2([5, 6, 7, 8, 9, 8, 6, -4]); });
        });
    });

    describe("invert", function() {
        describe("with a separate output dual quaternion", function() {
            beforeEach(function() { result = quat2.invert(out, quat2A); });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2([-0.0333333333, -0.06666666666, -0.1, 0.13333333333, -2/30, -5/30, -6/30, -2/30]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
            it("the real part should be equal to a inverted quaternion", function() {
                quat.invert(outQuat, quat.fromValues(1, 2, 3, 4));
                quat2.getReal(outQuat, out);

                expect(outQuat).toBeEqualish(outQuat);
            });
        });

        describe("when quat2A is the output quaternion", function() {
            beforeEach(function() { result = quat2.invert(quat2A, quat2A); });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualish([-0.0333333333, -0.06666666666, -0.1, 0.13333333333, -2/30, -5/30, -6/30, -2/30]); });
            

        });
    });

    describe("get real/dual", function() {
        describe("get real", function() {
            beforeEach(function() { result = quat2.getReal(outQuat, quat2A); });

            it("should place values into out", function() { expect(outQuat).toBeEqualish([1, 2, 3, 4]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
        });

        describe("get dual", function() {
            beforeEach(function() { result = quat2.getDual(outQuat, quat2A); });

            it("should place values into out", function() { expect(outQuat).toBeEqualish([2, 5, 6, -2]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
        });
    });

    describe("set real/dual", function() {
        describe("set real", function() {
            beforeEach(function() {
                outQuat= quat.fromValues(4, 6, 8, -100);
                result = quat2.setReal(quat2A, outQuat);
            });

            it("should place values into out", function() { expect(quat2A).toBeEqualishQuat2([4, 6, 8, -100, 2, 5, 6, -2]); });
            
            it("should not modify outQuat", function() { expect(outQuat).toBeEqualish([4, 6, 8, -100]); });
        });

        describe("set dual", function() {
            beforeEach(function() {
                outQuat= quat.fromValues(4.3, 6, 8, -100);
                result = quat2.setDual(quat2A, outQuat);
            });

            it("should place values into out", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 4.3, 6, 8, -100]); });
            
            it("should not modify outQuat", function() { expect(outQuat).toBeEqualish([4.3, 6, 8, -100]); });
        });
    });

    describe("conjugate", function() {
        describe("with a separate output dual quaternion", function() {
            beforeEach(function() { result = quat2.conjugate(out, quat2A); });

            it("should place values into out", function() { expect(out).toBeEqualishQuat2([-1, -2, -3, 4, -2, -5, -6, -2]); });
            
            it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([1, 2, 3, 4, 2, 5, 6, -2]); });
        });

        describe("when quat2A is the output dual quaternion", function() {
            beforeEach(function() { result = quat2.conjugate(quat2A, quat2A); });

            it("should place values into quat2A", function() { expect(quat2A).toBeEqualishQuat2([-1, -2, -3, 4, -2, -5, -6, -2]); });
            
        });
    });

    describe("exactEquals", function() {
        let quat2C, r0, r1;
        beforeEach(function() {
            quat2A = quat2.fromValues(0, 1, 2, 3, 4, 5, 6, 7);
            quat2B = quat2.fromValues(0, 1, 2, 3, 4, 5, 6, 7);
            quat2C = quat2.fromValues(1, 2, 3, 4, 5, 6, 7, 8);
            r0 = quat2.exactEquals(quat2A, quat2B);
            r1 = quat2.exactEquals(quat2A, quat2C);
        });

        it("should return true for identical quaternions", function() { expect(r0).toBe(true); });
        it("should return false for different quaternions", function() { expect(r1).toBe(false); });
        it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([0, 1, 2, 3, 4, 5, 6, 7]); });
        it("should not modify quat2B", function() { expect(quat2B).toBeEqualishQuat2([0, 1, 2, 3, 4, 5, 6, 7]); });
    });

    describe("equals", function() {
        let quat2C, quat2D, r0, r1, r2;
        beforeEach(function() {
            quat2A = quat2.fromValues(0, 1, 2, 3, 4, 5, 6, 7);
            quat2B = quat2.fromValues(0, 1, 2, 3, 4, 5, 6, 7);
            quat2C = quat2.fromValues(1, 2, 3, 4, 5, 6, 7, 8);
            quat2D = quat2.fromValues(1e-16, 1, 2, 3, 4, 5, 6, 7);
            r0 = quat2.equals(quat2A, quat2B);
            r1 = quat2.equals(quat2A, quat2C);
            r2 = quat2.equals(quat2A, quat2D);
        });
        it("should return true for identical dual quaternions", function() { expect(r0).toBe(true); });
        it("should return false for different dual quaternions", function() { expect(r1).toBe(false); });
        it("should return true for close but not identical quaternions", function() { expect(r2).toBe(true); });
        it("should not modify quat2A", function() { expect(quat2A).toBeEqualishQuat2([0, 1, 2, 3, 4, 5, 6, 7]); });
        it("should not modify quat2B", function() { expect(quat2B).toBeEqualishQuat2([0, 1, 2, 3, 4, 5, 6, 7]); });
    });

});
