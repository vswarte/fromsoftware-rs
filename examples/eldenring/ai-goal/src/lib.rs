use std::{
    ptr::NonNull,
    time::{Duration, Instant},
};

use eldenring::{
    cs::{
        BlockId, CSAiFunc, CSGoalBaseVmt, CSGoalFunc, CSTaskGroupIndex, CSTaskImp, ChrSet, EnemyIns, FieldInsHandle, FieldInsSelector, FieldInsType, GoalResult, WorldChrMan
    },
    fd4::FD4TaskData,
    position::HavokPosition,
    util::{input, system::wait_for_system_init},
};
use fromsoftware_shared::{program::Program, task::*, FromStatic, Superclass};
use vtable_rs::VPtr;

#[unsafe(no_mangle)]
/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
pub unsafe extern "C" fn DllMain(_hmodule: u64, reason: u32) -> bool {
    // Exit early if we're not attaching a DLL
    if reason != 1 {
        return true;
    }

    let target: FieldInsHandle = FieldInsHandle {
        block_id: BlockId::from_parts(60, 42, 37, 0),
        selector: FieldInsSelector::from_parts(FieldInsType::Chr, 115, 38),
    };

    let mut patched = false;

    std::thread::spawn(move || {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        std::thread::sleep(Duration::from_secs(2));

        let instance = Box::leak(Box::new(CustomGoal {
            vftable: Default::default(),
        })) as *mut CustomGoal as isize;

        let cs_task = unsafe { CSTaskImp::instance().unwrap() };

        cs_task.run_recurring(
            move |_: &FD4TaskData| {
                if input::is_key_pressed(0x4F) {
                    // Select the SummonBuddy ChrSet from the WorldChrMan, bail if we cant.
                    let Some(chr_set) = unsafe { WorldChrMan::instance() }
                        .ok()
                        .map(|wcm| &mut wcm.summon_buddy_chr_set)
                    else {
                        return;
                    };

                    // Cast the retrieved ChrSet<ChrIns> to ChrSet<EnemyIns>
                    let chr_set: &mut ChrSet<EnemyIns> = unsafe { std::mem::transmute(chr_set) };

                    // Cycle over all the summon buddies (all spirit ashes)
                    for buddy in chr_set.characters() {
                        // Only apply to NPCs in the spirit ash range
                        if buddy.field_ins_handle.selector.index() < 20 {
                            continue;
                        }

                        // Acquire the AiIns if there is one, bail otherwise
                        let Some(ai_ins) = buddy
                            .com_manipulator
                            .ai_ins
                            .map(|mut ai| unsafe { ai.as_mut() })
                        else {
                            return;
                        };

                        // Request an attack.
                        ai_ins.action_request.ez_action_id_1 = 3001;
                        ai_ins.action_request.ez_action_id_2 = 3001;
                        ai_ins.action_request.is_request = true;
                    }
                }
            },
            CSTaskGroupIndex::ChrIns_AILogic,
        );
    });

    true
}

#[repr(C)]
pub struct CustomGoal {
    pub vftable: VPtr<dyn CSGoalBaseVmt, Self>,
}

impl CSGoalBaseVmt for CustomGoal {
    extern "C" fn destructor(&mut self) {}

    extern "C" fn activate(&mut self, ai_func: &mut CSAiFunc, goal_func: &mut CSGoalFunc) -> bool {
        true
    }

    extern "C" fn update(
        &mut self,
        ai_func: &mut CSAiFunc,
        goal_func: &mut CSGoalFunc,
        delta: f32,
    ) -> GoalResult {
        let ai_ins = unsafe { ai_func.ai_ins.as_mut() };

        if let Some(main_player) = unsafe { WorldChrMan::instance() }
            .ok()
            .map(|w| w.main_player.as_ref())
            .flatten()
        {
            // ai_ins.is_in_battle = false;
            // ai_ins.walk_type = 1;
            // ai_ins.want_to_move_to = main_player.module_container.physics.position;
        };

        GoalResult::Continue
    }

    extern "C" fn terminate(&mut self, ai_func: &mut CSAiFunc, goal_func: &mut CSGoalFunc) -> i32 {
        0
    }

    extern "C" fn interrupt(&mut self, ai_func: &mut CSAiFunc, goal_func: &mut CSGoalFunc) -> bool {
        true
    }

    extern "C" fn unk28(&mut self) {}

    extern "C" fn unk30(&mut self) {}
}
