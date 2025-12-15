use std::ffi::c_void;
use std::sync::Once;
use std::time::Duration;

use display::DebugDisplay;

use eldenring::cs::CSAutoInvadePoint;
use eldenring::cs::CSBulletManager;
use eldenring::cs::CSCamera;
use eldenring::cs::CSEventFlagMan;
use eldenring::cs::CSEventManImp;
use eldenring::cs::CSFade;
use eldenring::cs::CSFeManImp;
use eldenring::cs::CSGaitemImp;
use eldenring::cs::CSNetMan;
use eldenring::cs::CSSessionManager;
use eldenring::cs::CSSfxImp;
use eldenring::cs::CSTaskGroup;
use eldenring::cs::CSTaskImp;
use eldenring::cs::CSWindowImp;
use eldenring::cs::CSWorldGeomMan;
use eldenring::cs::CSWorldSceneDrawParamManager;
use eldenring::cs::FieldArea;
use eldenring::cs::WorldAreaTime;
use eldenring::cs::WorldChrMan;
use eldenring::fd4::FD4ParamRepository;
use eldenring::util::system::wait_for_system_init;

use fromsoftware_shared::program::Program;

use hudhook::Hudhook;
use hudhook::ImguiRenderLoop;
use hudhook::eject;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::imgui::Condition;
use hudhook::imgui::Context;
use hudhook::imgui::TreeNodeFlags;
use hudhook::imgui::Ui;
use hudhook::imgui::sys as imgui_sys;
use hudhook::windows::Win32::Foundation::HINSTANCE;

use pelite::pe64::Pe;

use display::render_debug_singleton;
use rva::RVA_GLOBAL_FIELD_AREA;
use tracing_panic::panic_hook;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

mod display;
mod rva;

/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32, _: *mut c_void) -> i32 {
    if reason != DLL_PROCESS_ATTACH {
        return 1;
    }

    // Check if this DLL is being loaded by `libhotpatch` and skip initialization.
    if libhotpatch::is_hotpatched() {
        return 1;
    }

    std::panic::set_hook(Box::new(panic_hook));

    let appender = tracing_appender::rolling::never("./", "chains-debug.log");
    tracing_subscriber::fmt().with_writer(appender).init();

    std::thread::spawn(move || {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        if let Err(e) = Hudhook::builder()
            .with::<ImguiDx12Hooks>(EldenRingDebugGui::new())
            .with_hmodule(hmodule)
            .build()
            .apply()
        {
            tracing::error!("Couldn't apply hooks: {e:?}");
            eject();
        }
    });

    1
}

struct EldenRingDebugGui {
    size: [f32; 2],
    scale: f32,
}

impl EldenRingDebugGui {
    fn new() -> Self {
        Self {
            size: [600., 400.],
            scale: 1.0,
        }
    }
}

impl ImguiRenderLoop for EldenRingDebugGui {
    fn initialize(&mut self, ctx: &mut Context, _render_context: &mut dyn hudhook::RenderContext) {
        if let Ok(window) = unsafe { <CSWindowImp as fromsoftware_shared::FromStatic>::instance() }
        {
            if window.screen_width > 1920 {
                self.scale = window.screen_width as f32 / 1920.0;
                self.size[0] *= self.scale;
                self.size[1] *= self.scale;
            }
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

        // SAFETY: *do not* modify this function signature while the game is running.
        unsafe {
            render_live_reload(self.size, self.scale, ui);
        }
    }
}

#[libhotpatch::hotpatch]
unsafe fn render_live_reload(gui_size: [f32; 2], gui_scale: f32, ui: &mut Ui) {
    let program = Program::current();

    ui.window("Elden Ring Rust Bindings Debug")
        .position([0., 0.], Condition::FirstUseEver)
        .size(gui_size, Condition::FirstUseEver)
        .build(|| {
            ui.set_window_font_scale(gui_scale);
            let tabs = ui.tab_bar("main-tabs").unwrap();
            if let Some(item) = ui.tab_item("World") {
                if ui.collapsing_header("FieldArea", TreeNodeFlags::empty()) {
                    ui.indent();

                    if let Some(field_area) = unsafe {
                        (*(program.rva_to_va(RVA_GLOBAL_FIELD_AREA).unwrap()
                            as *const *const FieldArea))
                            .as_ref()
                    } {
                        field_area.render_debug(ui);
                    }

                    ui.unindent();
                }

                // render_debug_singleton::<FieldArea>(ui);
                render_debug_singleton::<CSEventFlagMan>(ui);
                render_debug_singleton::<WorldChrMan>(ui);
                render_debug_singleton::<CSWorldGeomMan>(ui);
                render_debug_singleton::<WorldAreaTime>(ui);
                render_debug_singleton::<CSBulletManager>(ui);
                render_debug_singleton::<CSEventManImp>(ui);
                render_debug_singleton::<CSAutoInvadePoint>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Inventory") {
                render_debug_singleton::<CSGaitemImp>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Networking") {
                render_debug_singleton::<CSSessionManager>(ui);
                render_debug_singleton::<CSNetMan>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Resource") {
                render_debug_singleton::<CSTaskGroup>(ui);
                render_debug_singleton::<CSTaskImp>(ui);
                render_debug_singleton::<FD4ParamRepository>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Render") {
                render_debug_singleton::<CSCamera>(ui);
                render_debug_singleton::<CSFade>(ui);
                render_debug_singleton::<CSSfxImp>(ui);
                render_debug_singleton::<CSWorldSceneDrawParamManager>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Front End") {
                render_debug_singleton::<CSFeManImp>(ui);
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
