use std::{sync::Once, time::Duration};

use debug::*;
use fromsoftware_shared::Program;
use hudhook::ImguiRenderLoop;
use hudhook::hooks::dx11::ImguiDx11Hooks;
use hudhook::imgui::{sys as imgui_sys, *};
use hudhook::windows::Win32::Foundation::HINSTANCE;
use sekiro::util::system::wait_for_system_init;

/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(hmodule: HINSTANCE, reason: u32) -> i32 {
    debug::initialize::<ImguiDx11Hooks>(
        hmodule,
        reason,
        || {
            wait_for_system_init(&Program::current(), Duration::MAX)
                .expect("Timeout waiting for system init");
        },
        SekiroDebugGui::new(),
    )
}

#[derive(Default)]
struct SekiroDebugGui {
    size: [f32; 2],
    scale: f32,
}

impl SekiroDebugGui {
    fn new() -> Self {
        Self {
            size: [600., 400.],
            scale: 1.8,
        }
    }
}

impl ImguiRenderLoop for SekiroDebugGui {
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
unsafe fn render_live_reload(gui: &mut SekiroDebugGui, ui: &mut Ui) {
    ui.window("Sekiro Rust Bindings Debug")
        .position([30., 30.], Condition::FirstUseEver)
        .size(gui.size, Condition::FirstUseEver)
        .build(|| {
            ui.set_window_font_scale(gui.scale);

            ui.text("Nothing here yet...");
        });
}

#[libhotpatch::hotpatch]
unsafe fn forward_imgui_context_on_reload(ctx: *mut imgui_sys::ImGuiContext) {
    static ONCE: Once = Once::new();
    ONCE.call_once(|| unsafe { imgui_sys::igSetCurrentContext(ctx) });
}
