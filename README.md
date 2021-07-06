# Kelk

Kelk is a Low-level interface for interacting with the smart contract Wasm executor

## How ro compile contracts

```
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release
```

Read this: https://os.phil-opp.com/freestanding-rust-binary/#the-1