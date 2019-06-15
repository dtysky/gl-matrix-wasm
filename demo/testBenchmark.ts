/**
 * @File   : testBenchmark.ts
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 6/15/2019, 11:33:09 AM
 * @Description:
 */
import * as benchmark from 'benchmark';

export default async function testBenchmark(): Promise<string> {
  const tests = (await import('./tests')).tests;

  const suite = new benchmark.Suite();

  tests.forEach(t => {
    suite.add(`${t.name}(WASM)`, t.wasm);
    suite.add(`${t.name}(JS)`, t.js);
  });

  let result = '';
  return new Promise(resolve => {
    suite
    .on('cycle', function(event) {
      const r = String(event.target) + '\n';
      result += r;
      console.log(r);
    })
    .on('complete', () => {
      resolve(result);
    })
    .run({async: true});
  });
}
