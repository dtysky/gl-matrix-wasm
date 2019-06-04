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

const fp = path.resolve(__dirname, './pkg/gl_matrix_wasm_bg.wasm');
const originBuffer = fs.readFileSync(fp);

const wasm = binaryen.readBinary(originBuffer);
binaryen.setOptimizeLevel(0);
binaryen.setShrinkLevel(0);
wasm.optimize();

fs.writeFileSync(fp.replace('.wasm', '.wast'), wasm.emitText());

const distBuffer = wasm.emitBinary();
fs.writeFileSync(fp, distBuffer);
