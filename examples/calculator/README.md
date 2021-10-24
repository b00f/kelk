## Calculator contract

A simple calculator contractor to do basic calculation with memory storage.


## How to compile

```
rustup target add wasm32-unknown-unknown
cargo +nightly build --target wasm32-unknown-unknown --release -Z unstable-options --out-dir ./wasm
```


## How to test

```
cargo +nightly test --target wasm32-unknown-unknown -Z unstable-options
```

## WASM optimization

Download and install the latest version of [binaryen](https://github.com/WebAssembly/binaryen) first.

```
wasm-opt -Os -o wasm/calculator.wasm wasm/calculator.wasm
```
