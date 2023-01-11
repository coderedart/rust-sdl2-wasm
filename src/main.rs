use std::cell::RefCell;
use std::mem;
use std::os::raw::{c_int, c_uchar, c_void};
use std::ptr::null_mut;

use sdl2::gfx::primitives::DrawRenderer;

// pub mod gl {
//     include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
// }
// use self::gl::types::*;

#[allow(non_camel_case_types)]
type em_callback_func = unsafe extern "C" fn();

extern "C" {
    // This extern is built in by Emscripten.
    pub fn emscripten_run_script_int(x: *const c_uchar) -> c_int;
    pub fn emscripten_cancel_main_loop();
    pub fn emscripten_set_main_loop(
        func: em_callback_func,
        fps: c_int,
        simulate_infinite_loop: c_int,
    );
}

thread_local!(static MAIN_LOOP_CALLBACK: RefCell<Option<Box<dyn FnMut()>>> = RefCell::new(None));

pub fn set_main_loop_callback<F: 'static>(callback: F)
where
    F: FnMut(),
{
    MAIN_LOOP_CALLBACK.with(|log| {
        *log.borrow_mut() = Some(Box::new(callback));
    });

    unsafe {
        emscripten_set_main_loop(wrapper::<F>, 0, 1);
    }

    extern "C" fn wrapper<F>()
    where
        F: FnMut(),
    {
        MAIN_LOOP_CALLBACK.with(|z| {
            if let Some(ref mut callback) = *z.borrow_mut() {
                callback();
            }
        });
    }
}
pub const LUA_CODE: &str = r#"
lua_fn = function(click)
    print("current color: "..click)
end
"#;
const DANCING_TTF: &[u8] = include_bytes!("dancing.ttf");
struct Owner {
    ttx: sdl2::ttf::Sdl2TtfContext,
}
fn main() {
    println!("Startup");

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    video_subsystem
        .gl_attr()
        .set_context_profile(sdl2::video::GLProfile::GLES);
    video_subsystem.gl_attr().set_context_major_version(2);
    video_subsystem.gl_attr().set_context_minor_version(0);

    let window = video_subsystem
        .window("rust-sdl2-wasm", 1920, 1080)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    let mut canvas = window
        .into_canvas()
        .build()
        .expect("failed to create canvas from window");
    let ttx = Box::new(sdl2::ttf::init().expect("failed to init ttf"));
    let ttx = Box::leak(ttx);
    let dancing = ttx
        .load_font_from_rwops(sdl2::rwops::RWops::from_bytes(DANCING_TTF).unwrap(), 16)
        .expect("failed to load font hack");

    let lua_vm = mlua::Lua::new();
    let mut lua_code = "print('hello')".to_string();
    lua_vm
        .load(LUA_CODE)
        .exec()
        .expect("failed to execute lua code");
    let mut font_surface = dancing
        .render(&lua_code)
        .blended(sdl2::pixels::Color::RGB(200, 15, 40))
        .expect("failed to render hack font");

    println!("setting loop callback");
    let mut timer = sdl_context.timer().unwrap();
    let mut event_pump = sdl_context.event_pump().unwrap();
    event_pump.enable_event(sdl2::event::EventType::MouseButtonDown);
    set_main_loop_callback(move || {
        // use glow::HasContext;
        let milliseconds = timer.ticks();
        // lets chagne color every 3 seconds
        // let elapsed = milliseconds % 3000;
        // let elapsed = elapsed as f32;
        // clamp to 0.0-1.0
        let color = milliseconds as f32 / 1000.0;
        let r = (color.sin() * 255.0) as u8;
        let g = (color.cos() * 255.0) as u8;
        let b = (color.tan() * 255.0) as u8;
        // unsafe {
        //     gtx.clear_color(1.0, 1.0, 1.0, 1.0);
        //     gtx.clear(glow::COLOR_BUFFER_BIT);
        // }
        canvas.set_draw_color(sdl2::pixels::Color::RGB(255, 255, 255));
        canvas.clear();
        let color = sdl2::pixels::Color::RGB(r, g, b);
        canvas.set_draw_color(color);
        canvas
            .fill_rect(sdl2::rect::Rect::new(100, 100, 300, 150))
            .expect("failed to draw rect");
        let texture_creator = canvas.texture_creator();
        let font_texture = font_surface
            .as_texture(&texture_creator)
            .expect("failed to ccreate texture of font surface");
        canvas
            .copy(
                &font_texture,
                None,
                Some(sdl2::rect::Rect::new(
                    40,
                    40,
                    font_surface.width(),
                    font_surface.height(),
                )),
            )
            .expect("failed to copy font texture to canvas");
        // canvas.string(300, 300, "hello from sdl2", color).unwrap();
        while let Some(ev) = event_pump.poll_event() {
            match ev {
                sdl2::event::Event::MouseButtonDown { .. } => match lua_vm.load(&lua_code).exec() {
                    Ok(_) => {
                        println!("lua_code executed successfully");
                    }
                    Err(e) => {
                        println!("lua_code failed to run. error: {e:?}");
                    }
                },
                sdl2::event::Event::KeyDown {
                    keycode,
                    keymod,
                    repeat,
                    ..
                } => {
                    if keymod == sdl2::keyboard::Mod::NOMOD {
                        if let Some(keycode) = keycode {
                            if match keycode {
                                sdl2::keyboard::Keycode::Backspace => {
                                    lua_code.pop();
                                    true
                                }
                                sdl2::keyboard::Keycode::Return => {
                                    lua_code.push('\n');
                                    true
                                }
                                _ => false,
                            } {
                                if lua_code.len() > 0 {
                                    font_surface = dancing
                                        .render(&lua_code)
                                        .blended(sdl2::pixels::Color::RGB(200, 15, 40))
                                        .expect("failed to render hack font");
                                }
                            }
                        }
                    }
                }
                sdl2::event::Event::TextInput { text, .. } => {
                    lua_code.push_str(&text);
                    font_surface = dancing
                        .render(&lua_code)
                        .blended(sdl2::pixels::Color::RGB(200, 15, 40))
                        .expect("failed to render hack font");
                }
                _ => {}
            }
        }
        canvas.present();
    });
}
