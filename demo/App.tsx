/**
 * @File   : App.tsx
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 2018/12/11 下午10:29:56
 * @Description:
 */
import * as React from 'react';

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
  }

  public componentWillUnmount() {

  }

  public render() {
    return (
      <canvas id={'canvas'} ref={this.canvas} width={640} height={640} />
    );
  }
}
