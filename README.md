## Archived 
There's already a similar (and simpler) project at https://github.com/coderedart/luaegui where i show glfw + egui + mlua (Luau) combination.


# Rust + SDL2 + WASM + mlua example
proof of concept that you can use rust + sdl2 + mlua on browser wasm together to enable scripting for rust based apps/games on browser.

live version deployed at https://coderedart.github.io/rust-sdl2-wasm/

### Instructions
1. run these in shell (linux only)
```sh
./serve.sh
```
2. go to browser `http://127.0.0.1:8000/`

For native, just use `cargo run --target=x86_64-unknown-linux-gnu` as usual for linux.
