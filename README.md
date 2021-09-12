# Kelk

Kelk is a Low-level interface for interacting with the smart contract Wasm executor

## How to compile contracts

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release

nightly:

cargo +nightly build --target wasm32-unknown-unknown --release -Z unstable-options --out-dir ./wasm
wasm-opt -Os -o wasm/calculator.wasm wasm/calculator.wasm

testing:
cargo +nightly test --target wasm32-unknown-unknown -Z unstable-options
```

Read this: https://os.phil-opp.com/freestanding-rust-binary/#the-1
https://learnxinyminutes.com/docs/wasm/
