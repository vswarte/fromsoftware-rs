use std::time::Duration;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan},
    fd4::FD4TaskData,
    util::{input, system::wait_for_system_init},
};
use shared::{program::Program, singleton::FromStatic, task::*};

const SP_EFFECT: i32 = 4330;

/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
#[no_mangle]
pub unsafe extern "C" fn DllMain(_hmodule: u64, reason: u32) -> bool {
    // Exit early if we're not attaching a DLL
    if reason != 1 {
        return true;
    }

    std::thread::spawn(move || {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        // Retrieve games task runner and register a task at frame begin.
        let cs_task = CSTaskImp::instance().unwrap();
        cs_task.run_recurring(
            |_: &FD4TaskData| {
                // Retrieve WorldChrMan
                let Ok(world_chr_man) = (unsafe { WorldChrMan::instance() }) else {
                    return;
                };

                // Retrieve main player
                let Some(ref mut main_player) = world_chr_man.main_player else {
                    return;
                };

                // Check if "o" is pressed
                if input::is_key_pressed(0x4F) {
                    main_player.chr_ins.apply_speffect(SP_EFFECT, true);
                }

                // Check if "p" is pressed
                if input::is_key_pressed(0x50) {
                    main_player.chr_ins.remove_speffect(SP_EFFECT);
                }
            },
            CSTaskGroupIndex::FrameBegin,
        );
    });

    true
}
