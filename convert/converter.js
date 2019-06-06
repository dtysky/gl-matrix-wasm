#!/usr/bin/env node
/**
 * @File   : converter.js
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 6/6/2019, 5:21:14 PM
 * @Description:
 */
// use super::common::*;

// out
// out, a
// out, a, b
// a
// b
// a, b
// m(dd)
// x, y, z
// v vec3
// q quat


// ===
// return out;

// let x = a.0, a1 = a.1, ...;

// export function

// new glMatrix.ARRAY_TYPE(d)

// Math

// export const sqrLen = squaredLength;

const fs = require('fs');
const path = require('path');

const glMatrixPath = path.resolve(__dirname, '../../gl-matrix/src');

function walk(dir, callback) {
  fs.readdirSync(dir).forEach(file => {
    const filePath = path.join(dir, file);

    if (/index/.test(file) || /common/.test(file)) {
      return;
    }

    callback(filePath);
  });
}

function addHeader(content, name) {
  return `use wasm_bindgen::prelude::*;
  
use super::common::*;

#[wasm_bindgen]
pub struct ${name}(
  pub f32, pub f32,
  pub f32, pub f32
);

#[wasm_bindgen]
impl Matrix2 {
  #[wasm_bindgen(getter)]
  pub fn elements(&self) -> Box<[f32]> {
    Box::new([
      self.0, self.1,
      self.2, self.3
    ])
  }

${content.replace('import * as glMatrix from "./common.js"', '')}
}`;
}

function replaceParameters(content, name) {

}

function replaceReturn(content, name) {

}

function replaceBody(content, name) {

}

function replaceConstants(content, name) {

}

function replaceAlias(content, name) {

}

function replaceAssignments(content, name) {

}

function replaceMath(content, name) {

}

walk(glMatrixPath, fp => {
  console.log(fp);
});
