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
    
    const v2j = mathJS.vec4.fromValues(0, 1, 0, 0);
    const v1j = mathJS.vec4.fromValues(1, 0, 0, 0);
    const v3j = mathJS.vec4.fromValues(0, 0, 1, 0);

    console.log(math.Vector4.add(v1, v2, v3).elements);

    // const res = await this.test([
    //   {
    //     func: () => math.Vector4.add(v1, v2, v3),
    //     name: 'WASM'
    //   },
    //   {
    //     func: () => mathJS.vec4.add(v1j, v2j, v3j),
    //     name: 'JS'
    //   }
    // ]);

    const m1 = math.Matrix4.fromValues(7, 3, 6, 9, 2, 3, 2, 5, 1, 9, 5, 8, 3, 7, 2, 2);
    const m2 = math.Matrix4.fromValues(2, 5, 7, 8, 4, 8, 3, 9, 2, 5, 4, 9, 5, 6, 3, 1);
    const m3 = math.Matrix4.fromValues(1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1);

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

    // const res = await this.test([
    //   {
    //     func: () => math.Matrix4.multiply(m3, m1, m2),
    //     name: 'WASM'
    //   },
    //   {
    //     func: () => mathJS.mat4.multiply(m1j, m2j, m3j),
    //     name: 'JS'
    //   }
    // ]);

    const memory = new WebAssembly.Memory({ initial: 256 });
    const imports = {
        global: {
            NaN: 5,
            Infinity: 6
        },
        env: {
            memoryBase: 0,
            memory: memory,
        }
    };
    let r: any = await fetch('/assets/matrix.wasm');
    r = await r.arrayBuffer();
    console.log(imports);
    r = await WebAssembly.instantiate(r, imports);
    const ins = r.instance;
    console.log(r);
    const multiply = ins.exports.__Z8multiplyPfS_S_;

    const a = new Float32Array(memory.buffer, 1048576, 16);
    const b = new Float32Array(memory.buffer, 1048576 + 18 * 4, 16);
    const c = new Float32Array(memory.buffer, 1048576 + 36 * 4, 16);

    a.set([7, 3, 6, 9, 2, 3, 2, 5, 1, 9, 5, 8, 3, 7, 2, 2]);
    b.set([2, 5, 7, 8, 4, 8, 3, 9, 2, 5, 4, 9, 5, 6, 3, 1]);
    c.set([1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 1]);

    const runTimes = 10000;
    let ts = performance.now();
    for (let index = 0; index < runTimes; index++) {
      math.Matrix4.multiply(m3, m1, m2);
    }
    console.log('RUST', performance.now() - ts);
    ts = performance.now();
    for (let i = 0; i < runTimes; i++) {
      multiply(1048576 + 36 * 4, 1048576, 1048576 + 18 * 4);
    }
    console.log('CPP', performance.now() - ts);
    ts = performance.now();
    for (let index = 0; index < runTimes; index++) {
      mathJS.mat4.multiply(m1j, m2j, m3j);
    }
    console.log('JS', performance.now() - ts);

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
