/**
 * @File   : testRealworld.ts
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 6/15/2019, 11:33:20 AM
 * @Description:
 */
export async function test(t: {name: string, js: Function, wasm: Function}, times: number) {
  let ts = performance.now();
  const result = {name: t.name, js: 0, wasm: 0};
  for (let index = 0; index < times; index += 1) {
    t.wasm();
  }
  result.wasm = performance.now() - ts;

  for (let index = 0; index < times; index += 1) {
    t.js();
  }
  result.js = performance.now() - ts;

  return result;
}

export default async function testRealworld(times: number): Promise<{name: string, js: number, wasm: number}[]> {
  const tests = (await import('./tests')).tests;

  const result = [];

  return new Promise(resolve => {
    tests.forEach(async (t, index) => {
      const r = await test(t, times);
      result.push(r);
  
      if (index === tests.length - 1) {
        resolve(result);
      }
    });
  });
}
