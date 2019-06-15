/**
 * @File   : tests.ts
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 6/15/2019, 2:40:30 PM
 * @Description:
 */
import * as math from 'gl-matrix-wasm';
import * as mathJS from 'gl-matrix';

const ms = [
  math.Matrix4.fromValues(7, 3, 6, 9, 2, 3, 2, 5, 1, 9, 5, 8, 3, 7, 2, 2),
  math.Matrix4.fromValues(2, 5, 7, 8, 4, 8, 3, 9, 2, 5, 4, 9, 5, 6, 3, 1),
  math.Matrix4.fromValues(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
];
const v = math.Vector3.fromValues(1, 1, 1);
const q = math.Quaternion.fromValues(0, 0, 0, 1);

const mjs = [
  mathJS.mat4.fromValues(7, 3, 6, 9, 2, 3, 2, 5, 1, 9, 5, 8, 3, 7, 2, 2),
  mathJS.mat4.fromValues(2, 5, 7, 8, 4, 8, 3, 9, 2, 5, 4, 9, 5, 6, 3, 1),
  mathJS.mat4.fromValues(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1)
];
const vj = mathJS.vec3.fromValues(1, 1, 1);
const qj = mathJS.quat.fromValues(0, 0, 0, 1);

export const multiplyTest = {
  name: 'multiply',
  wasm: () => math.Matrix4.multiply(ms[2], ms[0], ms[1]),
  js: () => mathJS.mat4.multiply(mjs[2], mjs[0], mjs[1])
};

export const fromRotationTranslationScaleTest = {
  name: 'fromRotationTranslationScale',
  wasm: () => math.Matrix4.fromRotationTranslationScale(ms[1], q, v, v),
  js: () => mathJS.mat4.fromRotationTranslationScale(mjs[1], qj, vj, vj)
};

export const getElementsTest = {
  name: 'getElements',
  wasm: () => ms[1].elements,
  js: () => mjs[1]
};

export const tests = [
  multiplyTest,
  fromRotationTranslationScaleTest,
  {
    name: 'create',
    wasm: () => math.Matrix4.create(),
    js: () => mathJS.mat4.create()
  },
  {
    name: 'clone',
    wasm: () => math.Matrix4.clone(ms[0]),
    js: () => mathJS.mat4.clone(mjs[0])
  },
  {
    name: 'copy',
    wasm: () => math.Matrix4.copy(ms[1], ms[0]),
    js: () => mathJS.mat4.copy(mjs[1], mjs[0])
  },
  {
    name: 'fromValues',
    wasm: () => math.Matrix4.fromValues(2, 5, 7, 8, 4, 8, 3, 9, 2, 5, 4, 9, 5, 6, 3, 1),
    js: () => mathJS.mat4.fromValues(2, 5, 7, 8, 4, 8, 3, 9, 2, 5, 4, 9, 5, 6, 3, 1)
  },
  {
    name: 'set',
    wasm: () => math.Matrix4.set(ms[1], 2, 5, 7, 8, 4, 8, 3, 9, 2, 5, 4, 9, 5, 6, 3, 1),
    js: () => mathJS.mat4.set(mjs[1], 2, 5, 7, 8, 4, 8, 3, 9, 2, 5, 4, 9, 5, 6, 3, 1)
  },
  getElementsTest,
  {
    name: 'add',
    wasm: () => math.Matrix4.add(ms[2], ms[0], ms[1]),
    js: () => mathJS.mat4.add(mjs[2], mjs[0], mjs[1])
  },
  {
    name: 'sub',
    wasm: () => math.Matrix4.sub(ms[2], ms[0], ms[1]),
    js: () => mathJS.mat4.sub(mjs[2], mjs[0], mjs[1])
  },
  {
    name: 'identity',
    wasm: () => math.Matrix4.identity(ms[1]),
    js: () => mathJS.mat4.identity(mjs[1])
  },
  {
    name: 'transpose',
    wasm: () => math.Matrix4.transpose(ms[1], ms[0]),
    js: () => mathJS.mat4.transpose(mjs[1], mjs[0])
  },
  {
    name: 'invert',
    wasm: () => math.Matrix4.invert(ms[1], ms[0]),
    js: () => mathJS.mat4.invert(mjs[1], mjs[0])
  },
  {
    name: 'adjoint',
    wasm: () => math.Matrix4.adjoint(ms[1], ms[0]),
    js: () => mathJS.mat4.adjoint(mjs[1], mjs[0])
  },
  {
    name: 'determinant',
    wasm: () => math.Matrix4.determinant(ms[1]),
    js: () => mathJS.mat4.determinant(mjs[1])
  },
  {
    name: 'translate',
    wasm: () => math.Matrix4.translate(ms[1], ms[0], v),
    js: () => mathJS.mat4.translate(mjs[1], mjs[0], vj)
  },
  {
    name: 'scale',
    wasm: () => math.Matrix4.scale(ms[1], ms[0], v),
    js: () => mathJS.mat4.scale(mjs[1], mjs[0], vj)
  },
  {
    name: 'rotate',
    wasm: () => math.Matrix4.rotate(ms[1], ms[0], Math.PI, v),
    js: () => mathJS.mat4.rotate(mjs[1], mjs[0], Math.PI, vj)
  },
  {
    name: 'fromRotationTranslationScaleOrigin',
    wasm: () => math.Matrix4.fromRotationTranslationScaleOrigin(ms[1], q, v, v, v),
    js: () => mathJS.mat4.fromRotationTranslationScaleOrigin(mjs[1], qj, vj, vj, vj)
  },
  {
    name: 'lookAt',
    wasm: () => math.Matrix4.lookAt(ms[1], v, v, v),
    js: () => mathJS.mat4.lookAt(mjs[1], vj, vj, vj)
  }
];
