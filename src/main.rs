use egui_backend::egui::Window;
use egui_backend::{BackendConfig, EguiUserApp, GfxBackend, WindowBackend};
use egui_render_glow::GlowBackend;
use egui_window_sdl2::Sdl2Backend;
use mlua::Lua;

/// This is our userdata.
struct UserAppData {
    /// lua code that we can edit live and execute.
    code: String,
    /// well.. lua vm to execute the above code.
    lua_vm: mlua::Lua,
    /// glow (rusty opengl wrapper) renderer to draw egui.. and other stuff with opengl. use three-d backend for high level features like gltf/meshes/materials/lighting etc..
    glow_backend: GlowBackend,
    /// egui context
    egui_context: egui_backend::egui::Context,
}
/// just some default lua code to show in text editor
pub const LUA_CODE: &str = r#"
print('hello from lua');
"#;

// we care generic over the window backend. so, we can just decide at runtime which backend to use. eg: winit, glfw3, sdl2 are provided by `etk`
impl<WB: egui_backend::WindowBackend> EguiUserApp<WB> for UserAppData {
    // these are used by some default trait method implementations to abstract out the common parts like providing egui input or drawing egui output etc..
    type UserGfxBackend = GlowBackend;
    fn get_gfx_backend(&mut self) -> &mut Self::UserGfxBackend {
        &mut self.glow_backend
    }
    fn get_egui_context(&mut self) -> egui_backend::egui::Context {
        self.egui_context.clone()
    }
    // the only function we care about. add whatever gui code you want.
    fn gui_run(&mut self, egui_context: &egui_backend::egui::Context, _window_backend: &mut WB) {
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
}
fn main() {
    // init logging
    tracing_subscriber::fmt().init();
    // just create a new backend. ask sdl for an opengl window because we chose glow backend. on vulkan/dx/metal(desktop), we would choose non-gl window.
    let mut sdl2_backend = Sdl2Backend::new(
        Default::default(),
        BackendConfig {
            gfx_api_type: egui_backend::GfxApiType::GL,
        },
    );
    // create a opengl backend.
    let glow_backend = GlowBackend::new(&mut sdl2_backend, Default::default());
    // create our app data
    let app = UserAppData {
        code: LUA_CODE.to_string(),
        lua_vm: Lua::new(),
        glow_backend,
        egui_context: Default::default(),
    };
    // enter event loop and run forever :)
    sdl2_backend.run_event_loop(app);
}
