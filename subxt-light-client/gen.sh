#!/bin/bash

wasm-pack build --target web --out-name smoldot

# cargo build -p smoldot-wasm \
#   --target=wasm32-unknown-unknown \
#   --target-dir=alt-target
#   --release

# wasm-bindgen --target=web \
#   --out-dir=final-out-dir \
#   alt-target/wasm32-unknown-unknown/release/smoldot-wasm.wasm