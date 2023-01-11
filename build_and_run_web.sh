#!/bin/sh
echo "building for emscripten target"
cargo build --target=wasm32-unknown-emscripten --release
echo "copying files to dist directory"
mkdir -p dist
cp target/wasm32-unknown-emscripten/release/rust_sdl2_wasm.wasm dist
cp target/wasm32-unknown-emscripten/release/rust-sdl2-wasm.js dist
cp index.html dist
echo "launching server using python http.server on http://127.0.0.1:8000/"
(cd dist && python -m http.server --bind 127.0.0.1)
