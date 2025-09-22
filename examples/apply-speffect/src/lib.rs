use std::time::Duration;

use shared::program::Program;
use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan},
    fd4::FD4TaskData,
};
use eldenring_util::{
    chr_ins::ChrInsExt, input, singleton::get_instance,
    system::wait_for_system_init, task::CSTaskImpExt,
};

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
        let cs_task = get_instance::<CSTaskImp>().unwrap().unwrap();
        cs_task.run_recurring(
            |_: &FD4TaskData| {
                // Retrieve WorldChrMan
                let Some(world_chr_man) = unsafe { get_instance::<WorldChrMan>() }.unwrap() else {
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
