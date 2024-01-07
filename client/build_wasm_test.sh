#!/bin/bash

# Create the directory if it doesn't exist
mkdir -p web_test

# Remove the existing content in the directory
rm -fr web_test/*

# Copy the necessary files to the web_test directory
cp index.html web_test
cp -r ./assets ./web_test/

# Build the project using cargo for the wasm32 target
cargo build --release --target wasm32-unknown-unknown

# Use wasm-bindgen to generate the WASM bindings
wasm-bindgen --out-dir web_test --target web ../target/wasm32-unknown-unknown/release/knotter.wasm

# Optimize the WASM binary using wasm-opt
wasm-opt -O4 -o web_test/knotter_optimized.wasm web_test/knotter_bg.wasm

# Optionally, you can replace the original WASM file with the optimized version
mv web_test/knotter_optimized.wasm web_test/knotter_bg.wasm
