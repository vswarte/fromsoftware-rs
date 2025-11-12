use std::time::{Duration, Instant};

use eldenring::{
    cs::{
        CSActionButtonManImp, CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, GeometrySpawnParameters,
        WorldChrMan,
    },
    fd4::FD4TaskData,
    util::system::wait_for_system_init,
};
use shared::{program::Program, singleton::FromStatic, task::*};

const DEBOUNCE_DELAY: std::time::Duration = Duration::from_secs(2);

#[unsafe(no_mangle)]
/// # Safety
///
/// This is exposed this way such that windows LoadLibrary API can call it. Do not call this yourself.
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    // Kick off new thread.
    std::thread::spawn(|| {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");

        let mut last_pressed = Instant::now();
        let cs_task = CSTaskImp::instance().unwrap();
        cs_task.run_recurring(
            move |_: &FD4TaskData| {
                if Instant::now() - last_pressed < DEBOUNCE_DELAY {
                    return;
                }

                let Ok(action_button_man) = CSActionButtonManImp::instance() else {
                    return;
                };

                let Some(player) = WorldChrMan::instance()
                    .ok()
                    .and_then(|w| w.main_player.as_ref())
                else {
                    return;
                };

                let Some(block_geom_data) = unsafe { CSWorldGeomMan::instance() }
                    .ok()
                    .and_then(|wgm| wgm.geom_block_data_by_id_mut(&player.chr_ins.block_id_1))
                else {
                    return;
                };

                if !action_button_man.present_action_button(1000) {
                    return;
                }

                last_pressed = Instant::now();
                block_geom_data.spawn_geometry(
                    "AEG099_831",
                    &GeometrySpawnParameters {
                        position: player.block_position,
                        rot_x: 0.0,
                        rot_y: 0.0,
                        rot_z: 0.0,
                        scale_x: 2.0,
                        scale_y: 2.0,
                        scale_z: 2.0,
                    },
                );
            },
            CSTaskGroupIndex::ChrIns_PostPhysics,
        );
    });

    true
}
