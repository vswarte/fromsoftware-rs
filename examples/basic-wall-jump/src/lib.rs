use eldenring::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan},
    fd4::FD4TaskData,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{FromStatic, Program, SharedTaskImpExt};
use std::time::{Duration, Instant};

mod play_animation;
use play_animation::ChrInsPlayAnim;

mod wall_climb;
use wall_climb::*;

fn init_wall_jump_task() {
    let mut wj_man = WallJumpManager::new();
    let cs_task = (unsafe { CSTaskImp::instance() }).expect("Could not obtain CSTaskImp instance.");
    cs_task.run_recurring(
        move |_: &FD4TaskData| {
            let Some(main_player) = unsafe { WorldChrMan::instance() }
                .ok()
                .and_then(|wrld_chr_man| wrld_chr_man.main_player.as_ref())
            else {
                return;
            };

            let physics = &main_player.chr_ins.module_container.physics;
            if wj_man.has_jumped {
                wj_man.has_jumped = physics.is_jumping;
                return;
            }

            let now = Instant::now();

            // Are we on a sliding cliff/surface?
            let slide_info = &physics.slide_info;
            let scaleable_slope = !physics.touching_solid_ground && slide_info.is_sliding;

            if scaleable_slope {
                if !wj_man.has_entered_window {
                    wj_man.has_entered_window = true;
                    wj_man.window_enter_time = now;
                }
            } else {
                wj_man.has_entered_window = false;
            }

            let in_jump_window = now.duration_since(wj_man.window_enter_time) <= JUMP_TIME_FRAME;

            let jump_requested = main_player
                .chr_ins
                .module_container
                .action_request
                .action_requests
                .jump();

            if scaleable_slope && in_jump_window && jump_requested {
                wj_man.has_jumped =  main_player.chr_ins.play_animation_by_name("W_Jump_D");
            }
        },
        CSTaskGroupIndex::ChrIns_PostPhysicsSafe,
    );
}

// Exposed for dll loaders, a.e ModEngine 3.
#[unsafe(no_mangle)]
unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    std::thread::spawn(move || {
        // Wait for the game to initialize. Panic if it doesn't.
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");

        init_wall_jump_task();
    });
    true
}
