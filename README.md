# gl-matrix-wasm

Port gl-matrix to WebAssembly by rust, wasm-bindgen and wasm-pack.

## Goals

1. Complete: Support all functions in gl-matrix.
2. Pure: Write by pure rust without any third-part dependencies.
3. Small: 26K(gzip, spilt files) / 34K (gzip, one js file).
4. Reliable: Full unit tests as same as gl-matrix's.
5. Fast: Some tricks to speed up the performance in production version.

## Usage

First, install it

```shell
npm i gl-matrix-wasm -s
```

### One Js file

### Split files

## Performance

### Benchmark

### Real World

Every frame we execute 1000 mul

## Development

### Install RUST

### Install node packages

### Run

### Unit test

## Next

1. SIMD on WebAssembly.
