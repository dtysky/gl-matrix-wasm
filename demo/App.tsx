/**
 * @File   : App.tsx
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 2018/12/11 下午10:29:56
 * @Description:
 */
import * as React from 'react';
import * as mathJS from 'gl-matrix';
import * as benchmark from 'benchmark';

import * as math from 'gl-matrix-wasm';

export default class App extends React.PureComponent {
  private canvas: React.RefObject<HTMLCanvasElement> = React.createRef();

  public async componentDidMount() {
    await math.init();
    // const math = await import('gl-matrix-wasm/pkg/gl_matrix_wasm.split');

    const v1 = math.Vector4.fromValues(1, 0, 0, 0);
    const v2 = math.Vector4.fromValues(0, 1, 0, 0);
    const v3 = math.Vector4.fromValues(0, 0, 1, 0);
    // math.Vector4.cross(v3, v1, v2, v3);
    // console.log(v1.elements);
    // console.log(v2.elements);
    // console.log(v3.elements);

    const m1 = math.Matrix4.create();
    const m2 = math.Matrix4.create();
    const m3 = math.Matrix4.create();

    const m1j = mathJS.mat4.create();
    const m2j = mathJS.mat4.create();
    const m3j = mathJS.mat4.create();

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
    //       const e = m3.elements;
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

    alert(res);
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
