#!/bin/bash
echo "Removing previous WebAssembly files ..."
rm -rf ./src/wasm

cd ./rust

echo "Compiling Rust sources to WebAssembly ..."
cargo build \
    --release \
    --lib

echo "Executing wasm-bindgen ..."
wasm-bindgen \
    --out-dir ../src/wasm \
    --typescript \
    --target web \
    ./target/wasm32-unknown-unknown/release/wasm_part.wasm

cd ../src/wasm

echo "Optimizing WebAssembly using wasm-opt ..."
wasm-opt ./wasm_part_bg.wasm \
    -o ./opted.wasm \
    -O

mv ./opted.wasm ./wasm_part_bg.wasm

echo "Finished!"
