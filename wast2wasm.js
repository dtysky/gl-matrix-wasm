#!/usr/bin/env node
/**
 * @File   : wast2wasm.js
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 6/8/2019, 7:37:27 PM
 * @Description:
 */
const binaryen = require("binaryen");
const path = require('path');
const fs = require('fs');

let fp = path.resolve(__dirname, './pkg/gl_matrix_wasm_bg.wast');
const wast = binaryen.parseText(fs.readFileSync(fp, {encoding: 'utf8'}));

const distBuffer = wast.emitBinary();
fs.writeFileSync(fp.replace('wast', 'wasm'), distBuffer);
