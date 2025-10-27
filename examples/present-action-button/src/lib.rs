use std::time::{Duration, Instant};

use eldenring::{
    cs::{CSActionButtonManImp, CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, WorldChrMan},
    fd4::FD4TaskData,
};
use eldenring_util::{
    action_button::CSActionButtonManImpExt,
    geometry::{CSWorldGeomManBlockDataExt, GeometrySpawnParameters},
    system::wait_for_system_init,
};
use shared::{program::Program, singleton::get_instance, task::*};

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
        let cs_task = get_instance::<CSTaskImp>().unwrap();
        cs_task.run_recurring(
            move |_: &FD4TaskData| {
                if Instant::now() - last_pressed < DEBOUNCE_DELAY {
                    return;
                }

                let Some(action_button_man) = get_instance::<CSActionButtonManImp>() else {
                    return;
                };

                let Some(player) =
                    get_instance::<WorldChrMan>().and_then(|w| w.main_player.as_ref())
                else {
                    return;
                };

                let Some(block_geom_data) = unsafe { get_instance::<CSWorldGeomMan>() }
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
