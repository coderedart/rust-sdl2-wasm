#!/bin/bash
mkdir -p temp_web
cp target/wasm32-unknown-emscripten/release/rust_sdl2_wasm.wasm temp_web
cp target/wasm32-unknown-emscripten/release/rust-sdl2-wasm.js temp_web
cp index.html temp_web
