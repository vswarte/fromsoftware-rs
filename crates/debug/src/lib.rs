use hudhook::{Hooks, Hudhook, ImguiRenderLoop, eject};
use tracing_panic::panic_hook;
use windows::Win32::{Foundation::HINSTANCE, System::SystemServices::DLL_PROCESS_ATTACH};

mod display;
mod ext;

pub use ext::*;

/// Initializes the debug tool as a DLL plugin, setting up a panic handler,
/// tracing, and the basic hooks into the underlying rendering system.
///
/// The `wait_for_system_init` callback should block until the core of the
/// underlying game systems are initialized.
///
/// This doesn't automatically set up any hotpatches; however, it is
/// hotpatch-safe, and won't re-run the core initialization logic when run from
/// a hotpatched DLL.
pub fn initialize<T>(
    hmodule: HINSTANCE,
    reason: u32,
    wait_for_system_init: impl FnOnce() + Send + 'static,
    render_loop: impl ImguiRenderLoop + Send + Sync + 'static,
) -> i32
where
    T: Hooks + 'static,
{
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
        wait_for_system_init();

        if let Err(e) = Hudhook::builder()
            .with::<T>(render_loop)
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
