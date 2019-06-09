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
binaryen.setOptimizeLevel(0);
binaryen.setShrinkLevel(0);
wasm.optimize();

const wast = wasm.emitText()
  // .replace(/\(br_if \$label\$1[\s\n]+?\(i32.eq\n[\s\S\n]+?i32.const -1\)[\s\n]+\)[\s\n]+\)/g, '');
  .replace(/\(br_if \$label\$\d\n\s+\(i32\.eq\n\s+\(tee_local \$\d+?\n\s+\(i32\.load\n\s+\(get_local \$\d\)\n\s+\)\n\s+\)\n\s+\(i32\.const -1\)\n\s+\)\n\s+\)/g, '');
fs.writeFileSync(fp.replace('.wasm', '.wast'), wast);

const distBuffer = binaryen.parseText(wast).emitBinary();
fs.writeFileSync(fp, distBuffer);

fp = path.resolve(__dirname, './pkg/gl_matrix_wasm.d.ts');
fs.writeFileSync(fp, header + fs.readFileSync(fp, {encoding: 'utf8'}).replace(/get elements\(\)/g, 'readonly elements'));
fp = path.resolve(__dirname, './pkg/gl_matrix_wasm.js');
fs.writeFileSync(fp, header + fs.readFileSync(fp, {encoding: 'utf8'}));
fp = path.resolve(__dirname, './pkg/gl_matrix_wasm_bg.d.ts');
fs.writeFileSync(fp, header + fs.readFileSync(fp, {encoding: 'utf8'}));
