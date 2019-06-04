/**
 * @File   : utils.ts
 * @Author : dtysky (dtysky@outlook.com)
 * @Link   : dtysky.moe
 * @Date   : 2019/6/4 下午11:20:24
 */
import * as math from '../pkg/gl_matrix_wasm';

export function printVector3(vec: math.Vector3) {
  console.log(vec[0], vec[1], vec[2]);
  const ptr = (vec as any).ptr;
  const ele = new Float32Array(math.memory.buffer).slice(ptr / 4 + 1, ptr / 4 + 4);
  ele[0] = 10;
  console.log(ptr / 4, new Float32Array(math.memory.buffer).slice(ptr / 4 + 1, ptr / 4 + 4));
}
