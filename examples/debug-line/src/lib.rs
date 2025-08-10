use std::time::Duration;

use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, RendMan, WorldChrMan},
    fd4::FD4TaskData,
    position::PositionDelta,
};
use eldenring_util::{
    ez_draw::CSEzDrawExt, program::Program, singleton::get_instance, system::wait_for_system_init,
    task::CSTaskImpExt,
};

use shared::FSVector4;

use nalgebra_glm as glm;

#[unsafe(no_mangle)]
/// # Safety
///
/// This is exposed this way such that windows LoadLibrary API can call it. Do not call this yourself.
pub unsafe extern "C" fn DllMain(hmodule: usize, reason: u32) -> bool {
    // Check if the reason for the call is DLL_PROCESS_ATTACH.
    // This indicates that the DLL is being loaded into a process.
    if reason != 1 {
        return true;
    }

    // Kick off new thread.
    std::thread::spawn(|| {
        // Wait for game (current program we're injected into) to boot up.
        // This will block until the game initializes its systems (singletons, statics, etc).
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");

        // Retrieve games task runner.
        let cs_task = get_instance::<CSTaskImp>().unwrap().unwrap();

        // Register a new task with the game to happen every frame during the gameloops
        // ChrIns_PostPhysics phase because all the physics calculations have ran at this
        // point.
        cs_task.run_recurring(
            // The registered task will be our closure.
            |_: &FD4TaskData| {
                // Grab the debug ez draw from RendMan if it's available. Bail otherwise.
                let Some(ez_draw) = get_instance::<RendMan>()
                    .expect("No reflection data for RendMan")
                    .map(|r| r.debug_ez_draw.as_ref())
                else {
                    return;
                };

                // Grab the main player from WorldChrMan if it's available. Bail otherwise.
                let Some(player) = get_instance::<WorldChrMan>()
                    .expect("No reflection data for WorldChrMan")
                    .and_then(|w| w.main_player.as_ref())
                else {
                    return;
                };

                // Grab physics module from player.
                let physics = &player.chr_ins.module_container.physics;

                // Make a directional vector that points forward following the players
                // rotation.
                let directional_vector = {
                    let forward = glm::vec3(0.0, 0.0, -1.0);
                    glm::quat_rotate_vec3(&physics.orientation.into(), &forward)
                };

                // Set color for the to-be-rendered line.
                ez_draw.set_color(&FSVector4(0.0, 0.0, 1.0, 1.0));

                // Draw the line from the players position to a meter in front of the player.
                ez_draw.draw_line(
                    &physics.position,
                    &(physics.position
                        + PositionDelta(
                            directional_vector.x,
                            directional_vector.y,
                            directional_vector.z,
                        )),
                );
            },
            // Specify the task group in which physics calculations are already done.
            CSTaskGroupIndex::ChrIns_PostPhysics,
        );
    });

    // Signal that DllMain executed successfully
    true
}
