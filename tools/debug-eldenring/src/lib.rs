use std::ffi::c_void;
use std::sync::Once;
use std::time::Duration;

use hudhook::imgui::{Condition, Context, Ui, sys as imgui_sys};
use hudhook::windows::Win32::Foundation::HINSTANCE;
use hudhook::{ImguiRenderLoop, eject, hooks::dx12::ImguiDx12Hooks};
use pelite::pe64::Pe;
use rva::RVA_GLOBAL_FIELD_AREA;

use debug::*;
use eldenring::cs::*;
use eldenring::{fd4::FD4ParamRepository, util::system::wait_for_system_init};
use fromsoftware_shared::{FromStatic, program::Program};

mod display;
mod rva;

use display::{DebugDisplay, StaticDebugger};

/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut c_void) -> i32 {
    debug::initialize::<ImguiDx12Hooks>(
        hmodule,
        reason,
        || {
            wait_for_system_init(&Program::current(), Duration::MAX)
                .expect("Timeout waiting for system init");
        },
        EldenRingDebugGui::new(),
    )
}

#[derive(Default)]
struct EldenRingDebugGui {
    size: [f32; 2],
    scale: f32,

    // World
    event_flag: StaticDebugger<CSEventFlagMan>,
    world_chr: StaticDebugger<WorldChrMan>,
    world_geom: StaticDebugger<CSWorldGeomMan>,
    world_area_time: StaticDebugger<WorldAreaTime>,
    bullet: StaticDebugger<CSBulletManager>,
    event: StaticDebugger<CSEventManImp>,
    auto_invade_point: StaticDebugger<CSAutoInvadePoint>,

    // Game Data
    gaitem: StaticDebugger<CSGaitemImp>,
    game_data: StaticDebugger<GameDataMan>,

    // Networking
    session: StaticDebugger<CSSessionManager>,
    net: StaticDebugger<CSNetMan>,

    // Resource
    task_group: StaticDebugger<CSTaskGroup>,
    task: StaticDebugger<CSTaskImp>,
    param_repository: StaticDebugger<FD4ParamRepository>,

    // Render
    camera: StaticDebugger<CSCamera>,
    fade: StaticDebugger<CSFade>,
    sfx: StaticDebugger<CSSfxImp>,
    world_scene_draw_param: StaticDebugger<CSWorldSceneDrawParamManager>,

    // Front ENd
    fe: StaticDebugger<CSFeManImp>,
}

impl EldenRingDebugGui {
    fn new() -> Self {
        Self {
            size: [600., 400.],
            scale: 1.0,
            ..Default::default()
        }
    }

    fn update_scale(&mut self) -> bool {
        if let Ok(window) = unsafe { CSWindowImp::instance() } {
            self.scale = window.screen_width as f32 / 1920.0;
            self.size[0] = 600.0 * self.scale;
            self.size[1] = 400.0 * self.scale;
            return true;
        }
        false
    }
}

impl ImguiRenderLoop for EldenRingDebugGui {
    fn initialize(&mut self, ctx: &mut Context, _render_context: &mut dyn hudhook::RenderContext) {
        ctx.set_clipboard_backend(WindowsClipboardBackend {});

        if self.update_scale() {
            ctx.style_mut()
                .scale_all_sizes(f32::max(self.scale / 2.0, 1.0));
        }
    }

    fn render(&mut self, ui: &mut Ui) {
        // A live reload with libhotpatch "resets" all static variables, including `GImGui`,
        // so we have to pass it to any reloaded DLLs from the original one.
        //
        // SAFETY: this is threadsafe because it's a part of the imgui render loop.
        unsafe {
            let ctx = imgui_sys::igGetCurrentContext();
            forward_imgui_context_on_reload(ctx);
        }
        self.update_scale();

        // SAFETY: *do not* modify this function signature while the game is running.
        unsafe {
            render_live_reload(self, ui);
        }
    }
}

#[libhotpatch::hotpatch]
unsafe fn render_live_reload(gui: &mut EldenRingDebugGui, ui: &mut Ui) {
    let program = Program::current();

    ui.window("Elden Ring Rust Bindings Debug")
        .position([0., 0.], Condition::FirstUseEver)
        .size(gui.size, Condition::FirstUseEver)
        .build(|| {
            ui.set_window_font_scale(gui.scale);
            let tabs = ui.tab_bar("main-tabs").unwrap();
            if let Some(item) = ui.tab_item("World") {
                ui.header("FieldArea", || {
                    if let Some(field_area) = unsafe {
                        (*(program.rva_to_va(RVA_GLOBAL_FIELD_AREA).unwrap()
                            as *const *const FieldArea))
                            .as_ref()
                    } {
                        field_area.render_debug(ui);
                    }
                });

                gui.event_flag.render_debug(ui);
                gui.world_chr.render_debug(ui);
                gui.world_geom.render_debug(ui);
                gui.world_area_time.render_debug(ui);
                gui.bullet.render_debug(ui);
                gui.event.render_debug(ui);
                gui.auto_invade_point.render_debug(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Game Data") {
                gui.gaitem.render_debug(ui);
                gui.game_data.render_debug(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Networking") {
                gui.session.render_debug(ui);
                gui.net.render_debug(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Resource") {
                gui.task_group.render_debug(ui);
                gui.task.render_debug(ui);
                gui.param_repository.render_debug(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Render") {
                gui.camera.render_debug(ui);
                gui.fade.render_debug(ui);
                gui.sfx.render_debug(ui);
                gui.world_scene_draw_param.render_debug(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Front End") {
                gui.fe.render_debug(ui);
                item.end();
            }
            if let Some(item) = ui.tab_item("Eject") {
                if ui.button("Eject") {
                    eject();
                }
                item.end();
            }
            tabs.end();
        });
}

#[libhotpatch::hotpatch]
unsafe fn forward_imgui_context_on_reload(ctx: *mut imgui_sys::ImGuiContext) {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe { imgui_sys::igSetCurrentContext(ctx) });
}
