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

    const v1 = math.Vector3.fromValues(1, 1, 1);
    const v2 = math.Vector3.fromValues(2, 2, 2);
    const v3 = math.Vector3.create();

    math.Vector3.add(v3, v1, v2);
    utils.printVector3(v3);


    const m1 = math.Matrix4.create();
    const m2 = math.Matrix4.create();
    const m3 = math.Matrix4.create();

    const m1j = mathJS.mat4.create();
    const m2j = mathJS.mat4.create();
    const m3j = mathJS.mat4.create();

    // let res = this.test(() => math.Matrix4.multiply(m3, m1, m2), 'WASM', 1000);
    // res += '\n';
    // res += this.test(() => mathJS.mat4.multiply(m1j, m2j, m3j), 'JS', 1000);

    const a0 = math.Matrix2.create();
    const b0 = math.Matrix2.create();
    const o0 = math.Matrix2.create();
    this.test(() => math.Matrix2.add(o0, a0, b0), '0', 1000);
    console.log(o0.elements);

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
