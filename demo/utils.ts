/**
 * @File   : utils.ts
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : dtysky.moe
 * @Date   : 2019/6/4 下午11:20:24
 */
import * as math from '../pkg/gl_matrix_wasm';

export function printVector3(vec: math.Vector3) {
  console.log(vec[0], vec[1], vec[2]);
  // const ptr = (vec as any).ptr;
  // console.log(ptr / 4, new Float32Array(math.memory.buffer).slice(ptr / 4 + 1, ptr / 4 + 4));
}

export function printMatrix4(mat: math.Matrix4) {
  console.log(
    mat[0], mat[1], mat[2], mat[3],
    mat[4], mat[5], mat[6], mat[7],
    mat[8], mat[9], mat[10], mat[11],
    mat[12], mat[13], mat[14], mat[15]
  );
  // const ptr = (vec as any).ptr;
  // console.log(ptr / 4, new Float32Array(math.memory.buffer).slice(ptr / 4 + 1, ptr / 4 + 4));
}
