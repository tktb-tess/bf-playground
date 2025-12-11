# bf-playground

Rust と WebAssembly を利用しています。Rustの環境の他、Wasm ビルドターゲット、 `wasm-bindgen-cli` と `wasm-opt` が必要です。

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli --lock
cargo install wasm-opt --lock
```

```bash
git clone https://github.com/tktb-tess/bf-playground.git
cd bf-playground
pnpm i
./bash/build_wasm.sh
pnpm run dev
```


