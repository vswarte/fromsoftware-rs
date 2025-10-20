/*
Licensed under the MIT and ASL2 License.

*/

use std::{
    iter,
    mem::transmute,
    time::{Duration, Instant},
};

use eldenring::{
	cs::{CSTaskImp, CSTaskGroupIndex, WorldChrMan},
	fd4::{FD4TaskData},
};

use shared::{
    singleton::get_instance,
    program::Program,
};

use eldenring_util::{
	system::wait_for_system_init, 
	task::CSTaskImpExt,
};

use pelite::pe64::Pe;

// Plays an animation on the main player character.
// w_event: The name of the animation event to play.
// The RVA was obtained from the shared Ghidra repo. 
// It may change after an update.
const PLAY_ANIMATION_RVA: u32 = 0xC14460;
#[no_mangle]
fn play_animation(w_event: &str) {
    if let Some(main_player) = unsafe { get_instance::<WorldChrMan>() }
        .and_then(|w| w.main_player.as_ref())
    {
        if let Ok(va) = Program::current()
            .rva_to_va(PLAY_ANIMATION_RVA)
        {
			// you see this shit? don't do this.
			let behavior_unk10 = main_player.chr_ins.module_container.behavior.unk10;
			if behavior_unk10 != 0usize {
				let hkb_character = behavior_unk10 + 0x30;

				// Convert the animation string to a wide string (UTF-16) with a null terminator.
				let animation: Vec<u16> = w_event.encode_utf16().chain(iter::once(0)).collect();

				let play_animation_call = unsafe { transmute::<u64, extern "C" fn(usize, *const u16) -> u64>(va) };

				// Call the function with the raw address of hkb_character and a pointer to the wide string.
				play_animation_call(hkb_character, animation.as_ptr());
			}
		}
    }
}

// Time from last_instant till the window to jump opens.
const JUMP_DELAY: std::time::Duration = Duration::from_millis(500);

// Time allotted to perform a wall jump after touching a climbable wall
const WALL_JUMP_WINDOW: std::time::Duration = Duration::from_millis(1000);

fn init_wall_climb_task() {

	let mut wall_climb_done = false; // implement a single trigger behavior.
	let mut last_instant = Instant::now(); // track time since last wall contact.

    let cs_task = unsafe{ get_instance::<CSTaskImp>() }.unwrap();
	cs_task.run_recurring(
		// We use move to capture variables defined outside of the closure. These are wall_climb_done and last_instant.
		move |_: &FD4TaskData| {
			let Some(main_player) = unsafe { get_instance::<WorldChrMan>() }
				.and_then(|w| w.main_player.as_mut())
			else {
				return;
			};

			let physics = &main_player.chr_ins.module_container.physics;
			let instant = Instant::now();

			// TODO: use the vector direction & add a raycast for getting lower height bounds
			// (small pebbles allow you to wallclimb)
			let scaleable_wall = physics.is_touching_ground && physics.slide_info.is_sliding;

			if scaleable_wall {
				if !wall_climb_done {
					let actions = &main_player.chr_ins.module_container.action_request;
					let jump_request = actions.action_requests.jump();
					let jump_window = last_instant + WALL_JUMP_WINDOW;
					let jump_delay = last_instant + JUMP_DELAY;
					if jump_request && instant > jump_delay && instant < jump_window {
						play_animation("W_Jump_D");
						wall_climb_done = true;
					}
				}
			} else {
				if physics.standing_on_solid_ground && !physics.is_jumping {
					wall_climb_done = false;
					last_instant = instant
				}
			}

		}, 
		CSTaskGroupIndex::ChrIns_PostPhysics
	);
}

#[no_mangle]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }
    std::thread::spawn(|| {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Could not await system init.");
		init_wall_climb_task(); // queue the wall climb task
    });
    true
}

