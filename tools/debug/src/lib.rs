use std::ffi::c_void;
use std::sync::LazyLock;
use std::sync::Once;
use std::sync::RwLock;
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
use eldenring::cs::CSWorldAiManagerImp;
use eldenring::cs::CSWorldGeomMan;
use eldenring::cs::CSWorldSceneDrawParamManager;
use eldenring::cs::FieldArea;
use eldenring::cs::WorldAreaTime;
use eldenring::cs::WorldChrMan;
use eldenring::fd4::FD4ParamRepository;
use eldenring::util::system::wait_for_system_init;

use fromsoftware_shared::FromStatic;
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

use display::render_debug_static;
use rva::RVA_GLOBAL_FIELD_AREA;
use tracing_panic::panic_hook;
use windows::Win32::System::SystemServices::DLL_PROCESS_ATTACH;

use crate::display::UiExt;

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
            render_live_reload(self.size, self.scale, ui);
        }
    }
}

#[derive(Default)]
struct InputState {
    pub group: String,
    pub state: String,
}

const INPUT_STATE: LazyLock<RwLock<InputState>> = LazyLock::new(|| RwLock::default());

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

                ui.header("Wwise", || {
                    if let Ok(mut input) = INPUT_STATE.write() {
                        ui.input_text("Custom state FNV", &mut create_hash("c7600").to_string())
                            .read_only(false)
                            .build();

                        if ui.button("Set state (custom)") {
                            let set_state = unsafe {
                                std::mem::transmute::<u64, extern "C" fn(u32, u32) -> u32>(
                                    Program::current().rva_to_va(0x223f690).unwrap(),
                                )
                            };

                            set_state(create_hash("BgmEnemyType"), create_hash("c7600"));

                            set_state(create_hash("BossBattleState"), create_hash("Battle"));
                        }

                        if ui.button("Set state (Astel)") {
                            let set_state = unsafe {
                                std::mem::transmute::<u64, extern "C" fn(u32, u32) -> u32>(
                                    Program::current().rva_to_va(0x223f690).unwrap(),
                                )
                            };

                            set_state(create_hash("BgmEnemyType"), create_hash("MidBoss_Aster"));

                            set_state(create_hash("BossBattleState"), create_hash("Battle"));
                        }
                    };
                });

                // render_debug_static::<FieldArea>(ui);
                render_debug_static::<CSEventFlagMan>(ui);
                render_debug_static::<WorldChrMan>(ui);
                render_debug_static::<CSWorldGeomMan>(ui);
                render_debug_static::<WorldAreaTime>(ui);
                render_debug_static::<CSBulletManager>(ui);
                render_debug_static::<CSEventManImp>(ui);
                render_debug_static::<CSAutoInvadePoint>(ui);
                render_debug_static::<CSWorldAiManagerImp>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Inventory") {
                render_debug_static::<CSGaitemImp>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Networking") {
                render_debug_static::<CSSessionManager>(ui);
                render_debug_static::<CSNetMan>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Resource") {
                render_debug_static::<CSTaskGroup>(ui);
                render_debug_static::<CSTaskImp>(ui);
                render_debug_static::<FD4ParamRepository>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Render") {
                render_debug_static::<CSCamera>(ui);
                render_debug_static::<CSFade>(ui);
                render_debug_static::<CSSfxImp>(ui);
                render_debug_static::<CSWorldSceneDrawParamManager>(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Front End") {
                render_debug_static::<CSFeManImp>(ui);
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

use std::num::Wrapping;

const FNV_BASE: Wrapping<u32> = Wrapping(2166136261);
const FNV_PRIME: Wrapping<u32> = Wrapping(16777619);

pub fn create_hash(input: &str) -> u32 {
    let input_lower = input.to_ascii_lowercase();
    let input_buffer = input_lower.as_bytes();

    let mut result = FNV_BASE;
    for byte in input_buffer {
        result *= FNV_PRIME;
        result ^= *byte as u32;
    }

    result.0
}
