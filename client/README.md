Bygge wasm
~/koding/knotter/client   cargo build --release --target wasm32-unknown-unknown

Bygge katalog med javascript og rigging
~/koding/knotter$ wasm-bindgen --out-dir ./out/ --target web ./target/wasm32-unknown-unknown/release/knotter.wasm 

Optimalisere wasm (gjøre mindre)

~/koding/knotter/target/wasm32-unknown-unknown/release$ wasm-opt -Oz -o output2.wasm knotter.wasm

RUST_LOG=off cargo run --release