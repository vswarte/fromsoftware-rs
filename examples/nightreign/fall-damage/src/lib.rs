use std::time::Duration;

use fromsoftware_shared::{FromStatic, SharedTaskImpExt};
use nightreign::{
    cs::{CSTaskGroupIndex, CSTaskImp, WorldChrMan},
    fd4::FD4TaskData,
};

const HEIGHT_DAMAGE_LOWER_BOUND: f32 = 8.0;
const HEIGHT_DAMAGE_UPPER_BOUND: f32 = 20.0;

#[unsafe(no_mangle)]
/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    std::thread::spawn(|| {
        // Track height at which we initiated a fall.
        let mut fall_origin = None;

        std::thread::sleep(Duration::from_secs(10));

        let cs_task = unsafe { CSTaskImp::instance_mut() }.unwrap();

        cs_task.run_recurring(
            move |_: &FD4TaskData| {
                let Ok(Some(player)) =
                    unsafe { WorldChrMan::instance_mut() }.map(|w| w.main_player.as_mut())
                else {
                    return;
                };

                let falling_anim = player
                    .modules
                    .time_act
                    .anim_queue
                    .iter()
                    .any(|a| (4000..4200).contains(&(a.anim_id % 10000)));
                let falling_timer = player.modules.fall.fall_timer > 0.0;

                let current_y = player.modules.physics.position.1;
                if falling_anim || falling_timer {
                    if fall_origin.is_none() {
                        fall_origin = Some(current_y);
                    }
                } else if let Some(origin_y) = fall_origin.take() {
                    let height = origin_y - current_y;

                    if height > 0.0 {
                        let damage = calc_fall_damage(player.modules.data.hp_max, height);
                        player.modules.data.hp -= damage;
                    }
                }
            },
            CSTaskGroupIndex::ChrIns_PostPhysics,
        );
    });

    true
}

fn calc_fall_damage(hp_max: u32, height: f32) -> u32 {
    if height < HEIGHT_DAMAGE_LOWER_BOUND {
        0
    } else if height > HEIGHT_DAMAGE_UPPER_BOUND {
        hp_max
    } else {
        let alpha = (height - HEIGHT_DAMAGE_LOWER_BOUND)
            / (HEIGHT_DAMAGE_UPPER_BOUND - HEIGHT_DAMAGE_LOWER_BOUND);

        (hp_max as f32 * alpha.clamp(0.0, 1.0)) as u32
    }
}
