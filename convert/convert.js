#!/usr/bin/env node
/**
 * @File   : converter.js
 * @Author : dtysky (dtysky@outlook.com)
 * @Date   : 6/6/2019, 5:21:14 PM
 * @Description:
 */
const fs = require('fs');
const path = require('path');

const glMatrixPath = path.resolve(__dirname, '../../gl-matrix/src');

const LUT = {
  'vec2': 'Vector2',
  'vec3': 'Vector3',
  'vec4': 'Vector4',
  'quat': 'Quaternion',
  'quat2': 'Quaternion2',
  'mat2': 'Matrix2',
  'mat2d': 'Matrix2d',
  'mat3': 'Matrix3',
  'mat4': 'Matrix4',
  'Number': 'f32'
};

function walk(dir, callback) {
  fs.readdirSync(dir).forEach(file => {
    const filePath = path.join(dir, file);

    if (/index/.test(file) || /common/.test(file)) {
      return;
    }

    callback(filePath, file.replace(path.extname(file), ''));
  });
}

function addHeader(content, name) {
  const isMat = /Matrix/.test(name);
  const isQuat = /Quat/.test(name);
  let size = 0;

  if (isQuat) {
    size = 4;
  } else {
    size = parseInt(/(\d+)/g.exec(name)[1], 10);
    if (isMat) {
      size *= size;
    }
  }
  const values = new Array(size).fill(0).map((_, i) => i);

  return `use wasm_bindgen::prelude::*;
  
use super::common::*;

#[wasm_bindgen]
pub struct ${name}(
  ${values.map(i => 'pub f32').join(',\n')}
);

#[wasm_bindgen]
impl ${name} {
  #[wasm_bindgen(getter)]
  pub fn elements(&self) -> Box<[f32]> {
    Box::new([
      ${values.map(i => 'self.' + i).join(',\n')}
    ])
  }
${content.replace('import * as glMatrix from "./common.js";', '')}
}`;
}

function replaceParameters(content, name) {
  content.match(/(\/\*\*[\s\S\n]+?export function [\s\S]+?{)/g).forEach(s => {
    const res = s.match(/\@param {(\S+)} (\S+) /g);
    
    if (!res || res.length === 0) {
      return;
    }

    let [fun, funName, funParams] = /function (\S+)\s*\(([\S\s]+)\)\s*{/.exec(s);
    funParams = funParams.split(',');

    let i = 0;
    res.forEach(p => {
      let [__, type, name] = /\@param {(\S+)} \[*(\S+?)\]* /.exec(p);

      type = LUT[type];
      funParams[i] = `${name}: ${name === 'out' ? '&mut ' : type === 'f32' ? '' : '&'}${type}`;

      i += 1;
    });

    const funRes = `function ${funName}(${funParams.join(', ')}) {`
    content = content.replace(s, s.replace(fun, funRes));
  });

  return content.replace(/export function/g, 'pub fn');
}

function replaceBody(content, name) {
  return content.replace(/(\S+)\[(\d+)\]/g, '$1.$2')
    .replace(/var /g, 'let ')
    .replace(/pub fn [\s\S]+? {[\s\S\n]+?}/g, body => {
      return body.replace(/let \S+ = [\S\s]+?,([\n\s\S])+?;/g, s => {  
        if (/return/.test(s)) {
          return s;
        }
        return s.replace(/[\s\n]|let|var|;/g, '').split(',').map(item => `let ${item};`).join('\n');
      });
    });
}

function replaceReturn(content, name) {
  return content.replace(/(pub fn (create|clone|fromValues)[\s\S]+? ){[\s\n]+?let out = [\s\S\n]+?return out;[\s\n]+?}/g, (fun, f) => {
    let [_, count] = /glMatrix\.ARRAY_TYPE\((\d+)\)/.exec(fun);

    let values = new Array(count);
    for (let i = 0; i < count; i += 1) {
      let [__, value] = (new RegExp(`out\.${i} = (\\S+);`)).exec(fun);
      if (/^\d+$/.test(value)) {
        value += '.';
      }

      values[i] = value;
    }

    return `${f} -> ${name} {
  ${name}(${values.join(', ')})
}`;
  })
    .replace(/return out;\n/g, '');
}

function replaceConstants(content, name) {
  return content.replace(/glMatrix\./g, '');
}

function replaceAssignments(content, name) {
  return content.replace(/===/g, '==')
    .replace(/'/g, '"');
}

function replaceMath(content, name) {
  return content.replace(/Math\./g, 'f32::');
}

function replaceAlias(content, name) {
  return content.replace(/export const (\S+) = (\S+);/g, (s, alias, orig) => {
    return `pub fn ${alias}(out: &mut ${name}, a: &${name}, b: &${name}) {
  ${name}::${orig}(out, a, b);
}`
  });
}

walk(glMatrixPath, (fp, filename) => {
  const name = LUT[filename];
  console.log(fp, filename, name);
  let content = fs.readFileSync(fp, {encoding: 'utf8'});
  content = addHeader(content, name);
  content = replaceParameters(content, name);
  content = replaceBody(content, name);
  content = replaceReturn(content, name);
  content = replaceConstants(content, name);
  content = replaceAssignments(content, name);
  content = replaceMath(content, name);
  content = replaceAlias(content, name);

  fs.writeFileSync(path.resolve(__dirname, `./rust/${name.toLowerCase()}.rs`), content);
});
