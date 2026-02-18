use std::{sync::Once, time::Duration};

use darksouls3::app_menu::*;
use darksouls3::sprj::*;
use darksouls3::util::system::wait_for_system_init;
use darksouls3_extra::input::*;
use debug::*;
use fromsoftware_shared::Program;
use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::imgui::{sys as imgui_sys, *};
use hudhook::windows::Win32::Foundation::HINSTANCE;
use hudhook::{ImguiRenderLoop, eject};

mod display;

use display::StaticDebugger;

/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32) -> i32 {
    let blocker =
        unsafe { InputBlocker::get_instance() }.expect("Failed to initialize input blocker");

    debug::initialize::<ImguiDx11Hooks>(
        hmodule,
        reason,
        || {
            wait_for_system_init(&Program::current(), Duration::MAX)
                .expect("Timeout waiting for system init");
        },
        DarkSouls3DebugGui::new(blocker),
    )
}

struct DarkSouls3DebugGui {
    input_blocker: &'static InputBlocker,
    size: [f32; 2],
    scale: f32,

    // World
    world: StaticDebugger<WorldChrMan>,
    events: StaticDebugger<SprjEventFlagMan>,
    field_area: StaticDebugger<FieldArea>,

    // Menu
    menu_man: StaticDebugger<MenuMan>,
    new_menu_system: StaticDebugger<NewMenuSystem>,
    item_get_menu_man: StaticDebugger<ItemGetMenuMan>,

    // Resource
    params: StaticDebugger<CSRegulationManager>,
    solo_params: StaticDebugger<SoloParamRepository>,
}

impl DarkSouls3DebugGui {
    fn new(input_blocker: &'static InputBlocker) -> Self {
        Self {
            input_blocker,
            size: [600., 400.],
            scale: 1.8,
            world: Default::default(),
            events: Default::default(),
            field_area: Default::default(),
            menu_man: Default::default(),
            new_menu_system: Default::default(),
            item_get_menu_man: Default::default(),
            params: Default::default(),
            solo_params: Default::default(),
        }
    }
}

impl ImguiRenderLoop for DarkSouls3DebugGui {
    fn initialize(&mut self, ctx: &mut Context, _render_context: &mut dyn hudhook::RenderContext) {
        ctx.set_clipboard_backend(WindowsClipboardBackend {});

        // TODO: Look for CSWindowImp and scale everything based on that like ER
        // does.
    }

    fn render(&mut self, ui: &mut Ui) {
        // A live reload with libhotpatch "resets" all static variables,
        // including `GImGui`, so we have to pass it to any reloaded DLLs from
        // the original one.
        //
        // SAFETY: this is threadsafe because it's a part of the imgui render
        // loop.
        unsafe {
            let ctx = imgui_sys::igGetCurrentContext();
            forward_imgui_context_on_reload(ctx);
        }

        // SAFETY: *do not* modify this function signature while the game is running.
        unsafe {
            render_live_reload(self, ui);
        }
    }
}

#[libhotpatch::hotpatch]
unsafe fn render_live_reload(gui: &mut DarkSouls3DebugGui, ui: &mut Ui) {
    let io = ui.io();
    let mut flag = InputFlags::empty();
    if io.want_capture_mouse {
        flag |= InputFlags::Mouse;
    }
    if io.want_capture_keyboard {
        flag |= InputFlags::Keyboard;
    }
    if io.want_capture_mouse && io.want_capture_keyboard {
        // Only block pad input if both the mouse and keyboard are blocked
        // (for example if a modal dialog is up).
        flag |= InputFlags::GamePad;
    }
    gui.input_blocker.block_only(flag);

    ui.window("Dark Souls III Rust Bindings Debug")
        .position([30., 30.], Condition::FirstUseEver)
        .size(gui.size, Condition::FirstUseEver)
        .build(|| {
            ui.set_window_font_scale(gui.scale);
            let tabs = ui.tab_bar("main-tabs").unwrap();
            if let Some(item) = ui.tab_item("World") {
                gui.world.render_debug(ui);
                gui.events.render_debug(ui);
                gui.field_area.render_debug(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Menu") {
                gui.menu_man.render_debug(ui);
                gui.new_menu_system.render_debug(ui);
                gui.item_get_menu_man.render_debug(ui);
                item.end();
            }

            if let Some(item) = ui.tab_item("Resource") {
                gui.params.render_debug(ui);
                gui.solo_params.render_debug(ui);
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
