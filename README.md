# Rust + SDL2 + WASM + mlua example
proof of concept that you can use rust + sdl2 + mlua on browser wasm together to enable scripting for rust based apps/games on browser.

live version deployed at https://coderedart.github.io/rust-sdl2-wasm/
see warning below.

### Instructions
1. run these in shell (linux only)
```sh
./build_and_run_web.sh
```
2. go to browser `http://127.0.0.1:8000/`

## WARNING
I am using sdl2 ttf feature to render the "lua code". so, text editing is non-existent.
just type to add text to lua code and press backspace to delete the last character.
it doesn't have text shaping, you can only type one line of statement at a time.
when the last remaining char is deleted, it will still be rendered because sdl2 can't render empty text.
just type a char, and that now it should show you the freshly typed char as the only remaining char.
click on the canvas anywhere to run the lua code.
you can use print statements in lua and the output will be in browser console.
