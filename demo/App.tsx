/**
 * @File   : App.tsx
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 2018/12/11 下午10:29:56
 * @Description:
 */
import * as React from 'react';

import * as math from 'gl-matrix-wasm';
import testBenchmark from './testBenchmark';
import testRealworld, {test} from './testRealworld';

interface IStateTypes {
  [key: string]: {js: number, wasm: number};
}

const rrName = 'Real world: (mul x 4 -> fromRTS x 1 -> getElements) x 1000 x 60 / s';

export default class App extends React.Component<{}, IStateTypes> {
  public state: IStateTypes = {};
  private names: string[] = [];

  public async componentDidMount() {
    await math.init();
    const {tests} = (await import('./tests'));
    this.names = tests.map(t => t.name);
    this.names.splice(0, 0, rrName);
    // const math = await import('gl-matrix-wasm/pkg/gl_matrix_wasm.split');

    // const v1 = math.Vector4.fromValues(1, 0, 0, 0);
    // const v2 = math.Vector4.fromValues(0, 1, 0, 0);
    // const v3 = math.Vector4.fromValues(0, 0, 1, 0);
    // const v2j = mathJS.vec4.fromValues(0, 1, 0, 0);
    // const v1j = mathJS.vec4.fromValues(1, 0, 0, 0);
    // const v3j = mathJS.vec4.fromValues(0, 0, 1, 0);
    // console.log(math.Vector4.add(v1, v2, v3).elements);

    
    const state = {};
    const result = await testRealworld(10000);
    result.forEach(r => {
      state[r.name] = {js: r.js, wasm: r.wasm};
    });
    this.setState(state);

    this.testEverySec();
  }

  public testEverySec = async () => {
    const {multiplyTest, fromRotationTranslationScaleTest, getElementsTest} = (await import('./tests'));

    const result = await test({
      name: rrName,
      js: () => {
        for (let index = 0; index < 4; index += 1) {
          multiplyTest.js();
        }
        fromRotationTranslationScaleTest.js();
        getElementsTest.js();
      },
      wasm: () => {
        for (let index = 0; index < 4; index += 1) {
          multiplyTest.wasm();
        }
        fromRotationTranslationScaleTest.wasm();
        getElementsTest.wasm();
      }
    }, 1000 * 60);

    this.setState({[rrName]: result});
    setTimeout(this.testEverySec, 1000);
  }

  public render() {
    return (
      <React.Fragment>
        <h1>Matrix4(10000x)</h1>
        <div className={'content'}>
          {
            this.names.map(name => this.state[name] && (
              <div key={name} className={'test'}>
                <h3>{name}</h3>
                <p className={'test-wasm'}>Wasm: {this.state[name].wasm}ms</p>
                <p className={'test-js'}>JS: {this.state[name].js}ms</p>
              </div>
            ))
          }
        </div>
      </React.Fragment>
    );
  }
}
