/**
 * @File   : App.tsx
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 2018/12/11 下午10:29:56
 * @Description:
 */
import * as React from 'react';
import * as mathJS from 'gl-matrix';
import * as benchmark from 'benchmark';

export default class App extends React.PureComponent {
  private canvas: React.RefObject<HTMLCanvasElement> = React.createRef();

  public async componentDidMount() {
    const math = await import('../pkg/gl_matrix_wasm');
    const utils = await import('./utils');

    const v1 = math.Vector4.fromValues(1, 0, 0, 0);
    const v2 = math.Vector4.fromValues(0, 1, 0, 0);
    const v3 = math.Vector4.fromValues(0, 0, 1, 0);

    // math.Vector4.cross(v3, v1, v2, v3);
    // console.log(v1.elements);
    // console.log(v2.elements);
    // console.log(v3.elements);

    let matr = mathJS.mat3.create();
    let m = mathJS.mat4.create();
    const out = mathJS.quat.create();
    mathJS.mat4.lookAt(m, mathJS.vec3.fromValues(0, 0, 0), mathJS.vec3.fromValues(-1, 0, 0), mathJS.vec3.fromValues(0, -1, 0));
    mathJS.mat3.fromMat4(matr, m);
    mathJS.mat3.invert(matr, matr)
    mathJS.mat3.transpose(matr, matr);
    mathJS.quat.fromMat3(out, matr);
    mathJS.quat.normalize(out, out);
    let v = mathJS.vec3.create();
    mathJS.vec3.transformQuat(v, mathJS.vec3.fromValues(3,2,-1), out);
    console.log(v);

    const m1 = math.Matrix4.create();
    const m2 = math.Matrix4.create();
    const m3 = math.Matrix4.create();

    const m1j = mathJS.mat4.create();
    const m2j = mathJS.mat4.create();
    const m3j = mathJS.mat4.create();

    const ptr1 = ((m1 as any).ptr + 4) / 4;
    const ptr2 = ((m2 as any).ptr + 4) / 4;
    const ptr3 = ((m3 as any).ptr + 4) / 4;
    const m5 = math.Matrix4.fromValues(1, 2, 3, 4, 5 ,6 , 7,8 ,9, 10, 11, 12, 13, 14, 15, 16);
    const ptr5 = ((m5 as any).ptr + 4) / 4;
    // const e = new Float32Array((math as any).memory.buffer).slice(ptr5, ptr5  + 16);
    const buf = new Float32Array((math as any).memory.buffer);

    // console.log(e);
    // const res = await this.test([
    //   {
    //     func: () => {
    //       math.Matrix4.multiply(m3, m1, m2);
    //       math.Matrix4.multiply(m3, m1, m2);
    //       math.Matrix4.multiply(m3, m1, m2);
    //       math.Matrix4.multiply(m3, m1, m2);
    //       math.Matrix4.multiply(m3, m1, m2);
    //       math.Matrix4.multiply(m3, m1, m2);
    //       math.Matrix4.multiply(m3, m1, m2);
    //       math.Matrix4.multiply(m3, m1, m2);
    //       math.Matrix4.multiply(m3, m1, m2);
    //       math.Matrix4.multiply(m3, m1, m2);
    //       // const e = buf.slice(ptr3 / 4 + 1, ptr3 / 4 + 4);
    //       // const e = m3.elements;
    //     },
    //     name: 'WASM'
    //   },
    //   {
    //     func: () => {
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //       mathJS.mat4.multiply(m1j, m2j, m3j);
    //     },
    //     name: 'JS'
    //   }
    // ]);

    const res = await this.test([
      {
        func: () => math.Matrix4.multiply(m3, m1, m2),
        name: 'WASM'
      },
      {
        func: () => mathJS.mat4.multiply(m1j, m2j, m3j),
        name: 'JS'
      }
    ]);

    // alert(res);
  }

  private async test(tests: {func: Function, name: string}[]): Promise<string> {
    const suite = new benchmark.Suite();

    let result = '';
    tests.forEach(t => suite.add(t.name, t.func, {async: true}));

    return new Promise(resolve => {
      suite
      .on('cycle', function(event) {
        result += String(event.target) + '\n';
      })
      .on('complete', () => {
        console.log(result);
        resolve(result);
      })
      .run({async: true});
    });
  }

  public componentWillUnmount() {

  }

  public render() {
    return (
      <canvas id={'canvas'} ref={this.canvas} width={640} height={640} />
    );
  }
}
