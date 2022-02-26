```bash
$ rustup target install wasm32-unknown-unknown
$ cargo install wasm-server-runner
$ cargo install wasm-bindgen-cli

# run locally http://127.0.0.1:1334
$ cargo run --target wasm32-unknown-unknown

# build
$ cargo build --release --target wasm32-unknown-unknown && wasm-bindgen --out-dir ./out/ --target web ./target/
```