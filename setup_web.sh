#!/bin/bash
mkdir -p dist
cp target/wasm32-unknown-emscripten/release/rust_sdl2_wasm.wasm dist
cp target/wasm32-unknown-emscripten/release/rust-sdl2-wasm.js dist
cp index.html dist
