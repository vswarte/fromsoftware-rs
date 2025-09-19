use std::time::Duration;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, WorldChrMan},
    fd4::FD4TaskData,
};
use eldenring_util::{
    geometry::{CSWorldGeomManBlockDataExt, GeometrySpawnParameters}, input, program::Program, singleton::get_instance, system::wait_for_system_init, task::CSTaskImpExt
};

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

        let cs_task = get_instance::<CSTaskImp>().unwrap().unwrap();
        cs_task.run_recurring(
            |_: &FD4TaskData| {
                if !input::is_key_pressed(0x48) {
                    return;
                }

                let Some(player) = get_instance::<WorldChrMan>()
                    .expect("No reflection data for WorldChrMan")
                    .and_then(|w| w.main_player.as_ref())
                else {
                    return;
                };

                let Some(block_geom_data) = unsafe { get_instance::<CSWorldGeomMan>() }
                    .unwrap()
                    .and_then(|wgm| wgm.geom_block_data_by_id_mut(&player.chr_ins.block_id_1))
                else {
                    return;
                };

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
