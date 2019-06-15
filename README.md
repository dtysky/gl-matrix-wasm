# gl-matrix-wasm

Port [gl-matrix](https://github.com/toji/gl-matrix) to WebAssembly by rust, wasm-bindgen and wasm-pack.

## Goals

1. **Complete**: Support all functions in gl-matrix.
2. **Pure**: Write by pure rust without any third-part dependencies.
3. **Small**: 25K(gzip, separate files) / 32K (gzip, wasm buffer will be embedded js file).
4. **Reliable**: Full unit tests as same as gl-matrix.
5. **Fast**: Some tricks to speed up the performance in production version.
6. **Low cost**: Lower CPU and memory cost than js version.

## Difference

Although this library supports all functions in gl-matrix, but there are a little difference:

1. Namespace: this library use `Vector2`, `Matrix4`... as namespace, it is not as same as gl-matrix's `vec2`, `mat4`.
2. Async: You must initialize this library asynchronous, it is painful, but was wasm required.
3. Data storage: When you use some ways such as `const vec2 = Vector2.create();` to create vectors and matrixes, you will get a **Object contains pointer** but not **TypedArray**. This is the largest difference between wasm and js version. In wasm version, all data are stored in wasm memory, and in js only store those pointers. If you want to get the real value of wasm object, please use `object.elements`, it will return a `Float32Array` that could be pass to GPU, or you can use `object.elements[0]`, `object.elements[1]`... to get each element by index.


## Usage

First, install it:

```shell
npm i gl-matrix-wasm --save

// or yarn add gl-matrix-wasm
```

Then you can use two ways to import and use it:

### One JS file

In current time, this way is suggested. It combine wasm file and js wrapper file to one js file(as umd module) which embed a wasm buffer. That means you don't need any external tools, you can use it just as you use **gl-matrix** before.

```ts
import * as math from 'gl-matrix-wasm';

async function test() {
  // initialize
  await math.init();

  const vec3 = math.Vector3.create();
  console.log(vec3.elements);

  // don't want to free
  v1.free();
}
```

### Separate files

This way faces the future, it provides a wasm file and a js(es6) file, in js file, it simply import the wasm file. It means you need some tools to compile it to es5 and split wasm file to your final results.

A common toolchain that support wasm is webpack(4.0+), your can put these config to your webpack.config.js file:

```js
module: {
  rules: [
    ......
    {
      test: /\.wasm$/,
      type: 'webassembly/experimental'
    }
    ......
  ]
}
```

And you must not exclude **node_modules/gl-matrix** in your js loader.

Now, you can use this library is separate-files mode:

```ts
async function test() {
  const math = await import('gl-matrix-wasm/pkg/gl_matrix_wasm.split');

  const v1 = math.Vector4.fromValues(1, 0, 0, 0);
  console.log(vec3.elements);

  // don't want to free
  v1.free();
}
```

## Performance

I did many tests to show how wasm version faster than js. But unfortunately, wasm does not run faster for all scene.

So, for evaluating performance reliably, I made two kinds of tests: benchmark and real-world. 

>PS: In current time, rust & wasm-bindgen version is slower than c++ & emscripten, see [#1585](https://github.com/rustwasm/wasm-bindgen/issues/1585).

### Benchmark

See [Benchmark(Matrix4, 2015 RMBP, Chrome)](./Benchmark.md)

### Real World

Real world is different from benchmark, I made some tests for each matrix4's method with 10000 calling, and a "really real world" test is also provided, it supposes we will execute operations "Mul x 4 -> fromRTS x 1 -> getElements" 1000 times per frame and run it in 60fps.

You can run these tests yourself:  

[Matrix4 Performance Tests](http://gl-matrix-wasm.dtysky.moe)

### CPU & Memory

Waiting...

## Development

Welcome to contribute to this project, you can run this project in development environment follow this steps:

### Install RUST

```sh
$ curl https://sh.rustup.rs -sSf | sh
$ rustup default nightly
$ rustup target add wasm32-unknown-unknown --toolchain nightly
$ cargo +nightly install wasm-pack
$ cargo update
```

### Install node packages

```sh
$ npm i
```

### Run

#### Develop rust

Low performance, but could be used to debug rust sources.

```sh
npm run dev
```

#### Develop demo

High performance, could be used to test the production.

```sh
npm run dev-build
```

### Unit test

I use karma for testing.

```sh
npm run test
```

## Next

SIMD on WebAssembly.

## License

Copyright © 2019, 戴天宇, Tianyu Dai (dtysky < [dtysky@outlook.com](mailto:dtysky@outlook.com) >). All Rights Reserved. This project is free software and released under the [MIT License](https://opensource.org/licenses/MIT).
