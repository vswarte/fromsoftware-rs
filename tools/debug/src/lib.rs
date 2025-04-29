use std::time::Duration;

use display::DebugDisplay;

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

use eldenring_util::program::Program;
use eldenring_util::system::wait_for_system_init;

use hudhook::eject;
use hudhook::hooks::dx12::ImguiDx12Hooks;
use hudhook::imgui::Condition;
use hudhook::imgui::*;
use hudhook::windows::Win32::Foundation::HINSTANCE;
use hudhook::Hudhook;
use hudhook::ImguiRenderLoop;

use pelite::pe64::Pe;

use display::render_debug_singleton;
use rva::RVA_GLOBAL_FIELD_AREA;
use tracing_panic::panic_hook;

mod display;
mod rva;

/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
#[no_mangle]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32) -> bool {
    if reason == 1 {
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
    }

    true
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
        if let Ok(Some(window)) =
            unsafe { eldenring_util::singleton::get_instance::<CSWindowImp>() }
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
        let program = Program::current();

        ui.window("Elden Ring Rust Bindings Debug")
            .position([0., 0.], Condition::FirstUseEver)
            .size(self.size, Condition::FirstUseEver)
            .build(|| {
                ui.set_window_font_scale(self.scale);
                let tabs = ui.tab_bar("main-tabs").unwrap();
                if let Some(item) = ui.tab_item("World") {
                    if ui.collapsing_header("FieldArea", TreeNodeFlags::empty()) {
                        ui.indent();

                        if let Some(field_area) = unsafe {
                            (*(program.rva_to_va(RVA_GLOBAL_FIELD_AREA).unwrap()
                                as *const *const FieldArea))
                                .as_ref()
                        } {
                            field_area.render_debug(&ui);
                        }

                        ui.unindent();
                    }

                    // render_debug_singleton::<FieldArea>(&ui);
                    render_debug_singleton::<CSEventFlagMan>(&ui);
                    render_debug_singleton::<WorldChrMan>(&ui);
                    render_debug_singleton::<CSWorldGeomMan>(&ui);
                    render_debug_singleton::<WorldAreaTime>(&ui);
                    render_debug_singleton::<CSBulletManager>(&ui);
                    render_debug_singleton::<CSEventManImp>(&ui);
                    item.end();
                }

                if let Some(item) = ui.tab_item("Inventory") {
                    render_debug_singleton::<CSGaitemImp>(&ui);
                    item.end();
                }

                if let Some(item) = ui.tab_item("Networking") {
                    render_debug_singleton::<CSSessionManager>(&ui);
                    render_debug_singleton::<CSNetMan>(&ui);
                    item.end();
                }

                if let Some(item) = ui.tab_item("Resource") {
                    render_debug_singleton::<CSTaskGroup>(&ui);
                    render_debug_singleton::<CSTaskImp>(&ui);
                    render_debug_singleton::<FD4ParamRepository>(&ui);
                    item.end();
                }

                if let Some(item) = ui.tab_item("Render") {
                    render_debug_singleton::<CSCamera>(&ui);
                    render_debug_singleton::<CSFade>(&ui);
                    render_debug_singleton::<CSSfxImp>(&ui);
                    render_debug_singleton::<CSWorldSceneDrawParamManager>(&ui);
                    item.end();
                }

                if let Some(item) = ui.tab_item("Front End") {
                    render_debug_singleton::<CSFeManImp>(&ui);
                    item.end();
                }
                tabs.end();
                // if let Some(item) = ui.tab_item("Eject") {
                //     if ui.button("Eject") {
                //         eject();
                //     }
                //     item.end();
                // }
            });
    }
}
