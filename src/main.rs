use std::cell::RefCell;
use std::mem;
use std::os::raw::{c_int, c_uchar, c_void};
use std::ptr::null_mut;

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
        .window("rust-sdl2-wasm", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .unwrap();

    let gl_context = window.gl_create_context().unwrap();
    window.gl_make_current(&gl_context).unwrap();
    // gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const _);

    let gtx = unsafe {
        glow::Context::from_loader_function(|symbol| video_subsystem.gl_get_proc_address(symbol) as _)
    };

    set_main_loop_callback(move || {
        use glow::HasContext;
        unsafe {
            gtx.clear_color(0.3, 0.4, 0.9, 1.0);
            gtx.clear(glow::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();
    });
}
