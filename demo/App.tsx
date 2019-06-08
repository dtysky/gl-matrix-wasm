/**
 * @File   : App.tsx
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 2018/12/11 下午10:29:56
 * @Description:
 */
import * as React from 'react';
import * as mathJS from 'gl-matrix';

export default class App extends React.PureComponent {
  private canvas: React.RefObject<HTMLCanvasElement> = React.createRef();

  public async componentDidMount() {
    const math = await import('../pkg/gl_matrix_wasm');
    const utils = await import('./utils');

    const v1 = math.Vector4.fromValues(1, 0, 0, 0);
    const v2 = math.Vector4.fromValues(0, 1, 0, 0);
    const v3 = math.Vector4.fromValues(0, 0, 1, 0);

    math.Vector4.cross(v3, v1, v2, v3);
    console.log(v1.elements);
    console.log(v2.elements);
    console.log(v3.elements);

    let matr = mathJS.mat3.create();
    let out = mathJS.quat.create();
    mathJS.mat3.transpose(matr, mathJS.mat3.invert(matr, mathJS.mat3.fromMat4(matr, mathJS.mat4.lookAt(mathJS.mat4.create(), [0, 0, 0], [-1, 0, 0], [0, -1, 0]))));
    mathJS.quat.fromMat3(out, matr);
    const result = mathJS.vec3.transformQuat(mathJS.vec3.create(), [3,2,-1], mathJS.quat.normalize(out, out));
    console.log(result);
    // const m1 = math.Matrix4.create();
    // const m2 = math.Matrix4.create();
    // const m3 = math.Matrix4.create();

    // const m1j = mathJS.mat4.create();
    // const m2j = mathJS.mat4.create();
    // const m3j = mathJS.mat4.create();

    // let res = this.test(() => math.Matrix4.multiply(m3, m1, m2), 'WASM', 1000);
    // res += '\n';
    // res += this.test(() => mathJS.mat4.multiply(m1j, m2j, m3j), 'JS', 1000);

    // alert(res);
  }

  private test(func: Function, name: string, times: number): string {
    let tsTotal = 0;

    for (let index = 0; index < times; index += 1) {
      const ts = performance.now();
      func();
      tsTotal += performance.now() - ts;
    }

    const result = `${name}:` + tsTotal / times + 'ms';
    console.log(result);

    return result;
  }

  public componentWillUnmount() {

  }

  public render() {
    return (
      <canvas id={'canvas'} ref={this.canvas} width={640} height={640} />
    );
  }
}
