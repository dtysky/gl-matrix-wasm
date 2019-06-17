#!/usr/bin/env node
/**
 * @File   : wasm-opt.js
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 2018/12/11 下午11:55:43
 * @Description:
 */
const binaryen = require("binaryen");
const path = require('path');
const fs = require('fs');
const typescript = require('typescript');

const package = require('./package.json');
const header = `/** 
 * @license gl-matrix-wasm v${package.version}
 * Copyright (c) 2018-present Tianyu Dai (dtysky)<dtysky@Outlook.com>.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */
`;

let fp = path.resolve(__dirname, './pkg/gl_matrix_wasm_bg.wasm');
const originBuffer = fs.readFileSync(fp);

const wasm = binaryen.readBinary(originBuffer);
binaryen.setOptimizeLevel(2);
binaryen.setShrinkLevel(2);
wasm.optimize();

const wast = wasm.emitText()
  // remove borrow checking
  .replace(/\(br_if \$label\$\d\n\s+\(i32\.eq\n\s+\(tee_local \$\d+?\n\s+\(i32\.load\n\s+\(get_local \$\d\)\n\s+\)\n\s+\)\n\s+\(i32\.const -1\)\n\s+\)\n\s+\)/g, '')
  // remove null checking
  // .replace(/\(br_if \$label\$\d\n\s+\(i32\.eqz[\s\S\n]+?get_local \$\d+?\)\n\s+\)\n\s+\)/g, '')
  // remove getters
  .replace(/\(export "__wbg_(get|set)_\S+\d+"[\S\s]+?\n/g, '')
  .replace(/\(func \$__wbg_(set|get)_\S+?_\d+?[ \S]+type \$(6|7)\) \(param \$0 i32\) \((result|param \$1) f32\)\n[\s\S\n]+?\(unreachable\)[\s\S\n]+?\(unreachable\)\n\s+\)/g, '')
  .replace(/\(func \$\S+?_elements.+?\(type \$1\) \(param \$0 i32\) \(param \$1 i32\)[\s\S\n]+?\n  \(unreachable\)\n\s*\)/g, '')
  .replace(/\(export "\S+_elements" \(func \S+\)\)/g, '')
// fs.writeFileSync(fp.replace('.wasm', '.wast'), wast);

const distBuffer = binaryen.parseText(wast).emitBinary();
fs.writeFileSync(fp, distBuffer);

fp = path.resolve(__dirname, './pkg/gl_matrix_wasm.d.ts');
fs.writeFileSync(fp, header + fs.readFileSync(fp, {encoding: 'utf8'})
  .replace(/get elements\(\)/g, 'readonly elements')
  .replace(/\d+?: number;\n/g, '')
  .replace(
    'export default function init (module_or_path: RequestInfo | BufferSource | WebAssembly.Module): Promise<any>;',
    'export function init (): Promise<any>;'
  )
  .replace(/(@param {\S+?} out[\s\S\n]+?@returns {void} \n\*\/)\n\s+static (\S+?out[\s\S]+?): void;/g, (_, comm, funh) => {
    const type = /@param {(\S+)} out/.exec(comm)[1];
    
    return `${comm.replace('void', type)}
    static ${funh}: ${type};
    `
  })
);
fp = path.resolve(__dirname, './pkg/gl_matrix_wasm.js');

const offsets = {
  Matrix2: 4,
  Matrix2d: 6,
  Matrix3: 9,
  Matrix4: 16,
  Vector2: 2,
  Vector3: 3,
  Vector4: 4,
  Quaternion: 4,
  Quaternion2: 8
};
const content = fs.readFileSync(fp, {encoding: 'utf8'})
  .replace(/static __wrap\(ptr\) {\n[\s\S]+?\.create\((\S+?)\.prototype\);\n\s+obj\.ptr = ptr;\n\n\s+return obj;\n(\s+?)}\n/g, (_, type, indent) => `static __wrap(ptr) {
${indent}  const obj = Object.create(Matrix4.prototype);
${indent}  obj.ptr = ptr;
${indent}  const realPtr = ptr / 4 + 1;
${indent}  this._elements = new Float32Array(wasm.memory.buffer).subarray(realPtr, realPtr + ${offsets[type]});

${indent}  return obj;
${indent}}`)
  .replace(/get elements\(\) {\n[\s\S]+?\n[\s\S]+?\n(\s+)}/g, `get elements() {
$1    return this._elements;
$1}`)
  .replace(/\/\*\*\n\s+?\* @returns {number}\n\s+?\*\/\n\s+?get \d+?\(\) {\n[\s\S]+?\n\s+}\n\s+set \d+?\(arg0\) {\n[\s\S]+?\n\s+?}\n/g, '')
  .replace('function init(module) {', 'function initModule(module) {')
  .replace('export default init;', '')
  .replace(/(@param {\S+?} out[\s\S\n]+?@returns {void}\n\s+\*\/)\n\s+static (\S+?\(out[\s\S]+?){\n\s+([\s\S\n]+?)}/g, (_, comm, funh, funb) => {
    const type = /@param {(\S+)} out/.exec(comm)[1];
    
    return `${comm.replace('void', type)}
    static ${funh} {
      ${funb.replace('return ', '')}  return out;
    }
    `
  });

const tsconfig = {
  compilerOptions: {
    allowJs: true,
    sourceMap: true,
    noImplicitAny: false,
    module: "umd",
    target: "es5",
    lib: ["es2017", "dom"],
    skipLibCheck: true
  },
  include: [
    'pkg/**/*.ts'
  ]
};

fs.writeFileSync(fp, typescript.transpileModule(
  header + content + `
export async function init() {
  return initModule(new Uint8Array([${distBuffer.join(',')}]));
}
  `,
  tsconfig
).outputText);

fs.writeFileSync(
  fp.replace('.js', '.split.js'),
  header + `import * as wasm from './gl_matrix_wasm_bg';` + content.replace('let wasm;', '')
);

fp = path.resolve(__dirname, './pkg/gl_matrix_wasm_bg.d.ts');
fs.writeFileSync(fp, header + fs.readFileSync(fp, {encoding: 'utf8'}));

fs.unlinkSync(path.resolve(__dirname, './pkg/package.json'));
fs.unlinkSync(path.resolve(__dirname, './pkg/.gitignore'));
fs.unlinkSync(path.resolve(__dirname, './pkg/README.md'));
