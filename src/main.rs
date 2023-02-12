use egui_backend::egui::Window;
use egui_backend::{BackendConfig, EguiUserApp, GfxBackend, WindowBackend};
use egui_render_glow::GlowBackend;
use egui_window_sdl2::Sdl2Backend;
use mlua::Lua;

/// This is our userdata.
struct UserAppData<WB: WindowBackend> {
    /// lua code that we can edit live and execute.
    code: String,
    /// well.. lua vm to execute the above code.
    lua_vm: mlua::Lua,
    /// glow (rusty opengl wrapper) renderer to draw egui.. and other stuff with opengl. use three-d backend for high level features like gltf/meshes/materials/lighting etc..
    glow_backend: GlowBackend,
    /// egui context
    egui_context: egui_backend::egui::Context,
    window_backend: WB
}
/// just some default lua code to show in text editor
pub const LUA_CODE: &str = r#"
print('hello from lua');
"#;

// we care generic over the window backend. so, we can just decide at runtime which backend to use. eg: winit, glfw3, sdl2 are provided by `etk`
impl<WB: egui_backend::WindowBackend> EguiUserApp for UserAppData<WB> {
    // these are used by some default trait method implementations to abstract out the common parts like providing egui input or drawing egui output etc..
    type UserGfxBackend = GlowBackend;
    
    // the only function we care about. add whatever gui code you want.
    fn gui_run(&mut self) {
        let egui_context = self.egui_context.clone();
        let input = egui_context.input(|i| i.clone());
        Window::new("Input window").show(&egui_context, |ui| {
            input.ui(ui);
        });
        Window::new("hello window").show(&egui_context, |ui| {
            ui.code_editor(&mut self.code);
            if ui.button("run code").clicked() {
                if let Err(e) = self.lua_vm.load(&self.code).exec() {
                    eprintln!("failed to run lua code: {e}")
                }
            }
        });
        // this tells sdl2 to immediately repaint after vsync. otherwise, it will sleep, waiting for events -> causing unresponsive browser tab.
        egui_context.request_repaint();
    }

    type UserWindowBackend = WB;

    fn get_all(
        &mut self,
    ) -> (
        &mut Self::UserWindowBackend,
        &mut Self::UserGfxBackend,
        &egui_backend::egui::Context,
    ) {
        (&mut self.window_backend, &mut self.glow_backend, &self.egui_context)
    }
}
fn main() {
    // init logging
    use tracing_subscriber::prelude::*;
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    // log_panics::init();
    // just create a new backend. ask sdl for an opengl window because we chose glow backend. on vulkan/dx/metal(desktop), we would choose non-gl window.
    let mut window_backend = Sdl2Backend::new(
        egui_window_sdl2::SDL2Config {
            window_creator_callback: Box::new(|vs| {
                #[cfg(not(target_arch = "wasm32"))]
                vs.gl_attr().set_context_major_version(3);
                #[cfg(not(target_arch = "wasm32"))]
                vs.gl_attr().set_context_minor_version(3);
                let mut window_builder = vs.window("default title", 800, 600);
                // use opengl on wasm
                // #[cfg(target_arch = "wasm32")]
                window_builder.opengl();
                // #[cfg(not(target_arch = "wasm32"))]
                // window_builder.vulkan();
                window_builder.allow_highdpi();
                window_builder.resizable();
                window_builder.build().expect("failed to create a window")
                // egui_window_sdl2::default_window_creator_callback(vs)
            }),
        },
        BackendConfig {},
    );
    dbg!(window_backend.window.subsystem().display_dpi(0).unwrap());
    // create a opengl backend.
    let gfx_backend = GlowBackend::new(&mut window_backend, Default::default());
    // create our app data
    let app = UserAppData {
        code: LUA_CODE.to_string(),
        lua_vm: Lua::new(),
        glow_backend: gfx_backend,
        egui_context: Default::default(),
        window_backend
    };
    // enter event loop and run forever :)
    Sdl2Backend::run_event_loop(app);
}
