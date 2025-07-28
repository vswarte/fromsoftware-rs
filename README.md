# FromSoftware-rs 🔩  From Software runtime rust bindings

Rust bindings to facilitate mod creation for From Software games.

[![Build Status](https://github.com/vswarte/eldenring-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/vswarte/eldenring-rs/actions)
![Crates.io License](https://img.shields.io/crates/l/eldenring)

<details>

<summary>Example Elden Ring mod code: render debug line</summary>

Your project's Cargo.toml should contain the following lines:

```toml
[lib]
# Tell Cargo that this is a dynamic library.
crate-type = ["cdylib"]

[dependencies]
eldenring = "0.8.0"
eldenring-util = "0.8.0"
fromsoftware-shared = "0.8.0"
nalgebra-glm = "0.19.0"
```

```rust
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

use fromsoftware_shared::FSVector4;

use nalgebra_glm as glm;

#[link(name = "kernel32")]
unsafe extern "C" {
    // Import the DisableThreadLibraryCalls function from kernel32.dll.
    unsafe fn DisableThreadLibraryCalls(hmodule: usize) -> bool;
}

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

    // Not important, but generally a good idea to disable DLL_THREAD_ATTACH and
    // DLL_THREAD_DETACH calls to this DLL.
    // Game creates quite a few threads, so this can help reduce overhead.
    DisableThreadLibraryCalls(hmodule);

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
```

Result:
![Debug line rendered by example mode code](img/example-mod-debug-line.png)

</details>

## Project structure (crates)

- `crates/eldenring` Contains the definitions for the elden ring structures. [![Crates.io](https://img.shields.io/crates/v/eldenring.svg?label=eldenring)](https://crates.io/crates/eldenring) [![Documentation](https://docs.rs/eldenring/badge.svg)](https://docs.rs/eldenring)
- `crates/nightreign` Contains the definitions for the nightreign structures. [![Crates.io](https://img.shields.io/crates/v/nightreign.svg?label=nightreign)](https://crates.io/crates/nightreign) [![Documentation](https://docs.rs/nightreign/badge.svg)](https://docs.rs/nightreign)
- `crates/util` Provides helper methods for common stuff. [![Crates.io](https://img.shields.io/crates/v/eldenring-util.svg?label=eldenring-util)](https://crates.io/crates/eldenring-util) [![Documentation](https://docs.rs/eldenring-util/badge.svg)](https://docs.rs/eldenring-util)
- `crates/dlrf` Defines a trait and exports a macro for interacting with the games reflection system. [![Crates.io](https://img.shields.io/crates/v/eldenring-dlrf.svg?label=eldenring-dlrf)](https://crates.io/crates/eldenring-dlrf)  [![Documentation](https://docs.rs/eldenring-dlrf/badge.svg)](https://docs.rs/eldenring-dlrf)
- `crates/dlrf/derive` Defines the derive macro for implementing the DLRF trait on types. **Do not depend on this directly since the macro is reexported through `eldenring-dlrf`**. [![Crates.io](https://img.shields.io/crates/v/eldenring-dlrf-derive.svg?label=eldenring-dlrf-derive)](https://crates.io/crates/eldenring-dlrf-derive)  [![Documentation](https://docs.rs/eldenring-dlrf-derive/badge.svg)](https://docs.rs/eldenring-dlrf-derive)

## Credits (aside listed contributors to this repository)

- Tremwil (for the arxan code restoration disabler, vtable-rs and a few other boilerplate-y things as well as implementing the initial FD4 singleton finder for TGA that I appropriated).
- Dasaav (for [libER](https://github.com/Dasaav-dsv/libER) and heaps of engine-related structures).
- Sfix (for coming up with the FD4 singleton finder approach at all).
- Yui (for some structures as well as AOBs and hinting at some logic existing in the binary).
- Vawser (and probably many more) (for hosting the param defs used with the param struct generator).

(Have you contributed to TGA in some manner and does this repository have your work in it? Reach out to @chainfailure on Discord for proper credit disclosure).
