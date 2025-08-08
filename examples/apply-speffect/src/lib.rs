use std::time::Duration;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan},
    fd4::FD4TaskData,
};
use eldenring_util::{
    input, program::Program, singleton::get_instance, system::wait_for_system_init,
    task::CSTaskImpExt, chr_ins::ChrInsExt,
};

const SP_EFFECT: i32 = 4330; 

/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
#[no_mangle]
pub unsafe extern "C" fn DllMain(hmodule: u64, reason: u32) -> bool {
    // Check if DLL is being attached
    if reason == 1 {
        std::thread::spawn(move || {
            wait_for_system_init(&Program::current(), Duration::MAX)
                .expect("Timeout waiting for system init");

            // Retrieve games task runner.
            let cs_task = get_instance::<CSTaskImp>().unwrap().unwrap();
            cs_task.run_recurring(
                // The registered task will be our closure.
                |_: &FD4TaskData| {
                    let Some(world_chr_man) = unsafe { get_instance::<WorldChrMan>() }.unwrap()
                    else {
                        return;
                    };

                    let Some(ref mut main_player) = world_chr_man.main_player else {
                        return;
                    };

                    if input::is_key_pressed(0x4F) {
                        main_player.chr_ins.apply_speffect(SP_EFFECT, true);
                    }

                    if input::is_key_pressed(0x50) {
                        main_player.chr_ins.remove_speffect(SP_EFFECT);
                    }

                },
                CSTaskGroupIndex::FrameBegin,
            );
        });
    }

    true
}
