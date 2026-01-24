use std::ptr::NonNull;

use shared::Program;
use pelite::pe::Pe;
use vtable_rs::VPtr;
use bitfield::bitfield;

use crate::{
    cs::{AiIns, CSAiFunc},
    dlkr::DLAllocatorBase,
    dltx::{DLString, DLUTF16StringKind},
    rva, Deque, Tree, Vector,
};

#[repr(C)]
/// Source of name: RTTI
pub struct CSGoalBase {
    vftable: VPtr<dyn CSGoalBaseVmt, Self>,
    pub goal_name: DLString<DLUTF16StringKind>,
    pub activate_func_name: DLString<DLUTF16StringKind>,
    pub update_func_name: DLString<DLUTF16StringKind>,
    pub terminate_func_name: DLString<DLUTF16StringKind>,
    pub interrupt_func_name: DLString<DLUTF16StringKind>,
    pub goal_id: u32,
    pub update_time: bool,
    pub no_update: bool,
    pub no_interrupt: bool,
    pub no_subgoal: bool,
    pub use_avoid_chr: bool,
    pub update_time_min: f32,
    pub update_time_max: f32,
    unk108: bool,
    pub combo_attack_cancel: bool,
    /// Debug description tree for the parameters.
    unk110: Tree<()>,
}

#[vtable_rs::vtable]
pub trait CSGoalBaseVmt {
    fn destructor(&mut self);

    fn activate(&mut self, ai_func: &mut CSAiFunc, goal_func: &mut CSGoalFunc) -> bool;

    fn update(
        &mut self,
        ai_func: &mut CSAiFunc,
        goal_func: &mut CSGoalFunc,
        param_4: f32,
    ) -> GoalResult;

    fn terminate(&mut self, ai_func: &mut CSAiFunc, goal_func: &mut CSGoalFunc) -> i32;

    fn interrupt(&mut self, ai_func: &mut CSAiFunc, goal_func: &mut CSGoalFunc) -> bool;

    /// Seemingly unused debug method.
    fn unk28(&mut self);

    /// Seemingly unused debug method.
    fn unk30(&mut self);
}

#[repr(i32)]
#[derive(Debug)]
pub enum GoalResult {
    Failed = -0x1,
    Continue = 0x0,
    Success = 0x1,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSGoalFunc {
    vftable: isize,
    pub goal_ins: NonNull<GoalIns>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct GoalIns {
    pub ai_owner: NonNull<AiIns>,
    pub parent_goal: NonNull<GoalIns>,
    pub latest_sub_goal: NonNull<GoalIns>,
    pub goal_func: CSGoalFunc,
    pub life: f32,
    unk2c: u32,
    pub subgoals: Deque<NonNull<GoalIns>>,
    pub params_basic: [f32; 8],
    pub params_vec: Vector<f32>,
    pub goal_id: i32,
    pub goal_strategy: Option<NonNull<CSGoalBase>>,
    /// 1 = non-battle goal, 2 = battle goal
    pub goal_type: i32,
    unkb4: i32,
    pub tick_delta: f32,
    pub timers: [f32; 16],
    pub numbers: [f32; 8],
    pub result: GoalResult,
    pub subgoal_result: GoalResult,
    unk124: [u8; 0x2ac],
}

type FnAddSubGoal = extern "C" fn(
    *mut GoalIns,
    goal_id: u32,
    life: f32,
    param_1: f32,
    param_2: f32,
    param_3: f32,
    param_4: f32,
    param_5: f32,
    param_6: f32,
    param_7: f32,
    param_8: f32,
    param_9: f32,
    param_10: f32,
    param_11: f32,
    param_12: f32,
    param_13: f32,
    param_14: f32,
);

type FnClearSubGoal = extern "C" fn(
    *mut GoalIns,
);

impl GoalIns {
    pub fn add_sub_goal(
        &mut self,
        goal_id: u32,
        life: f32,
        param_1: f32,
        param_2: f32,
        param_3: f32,
        param_4: f32,
        param_5: f32,
        param_6: f32,
        param_7: f32,
        param_8: f32,
        param_9: f32,
        param_10: f32,
        param_11: f32,
        param_12: f32,
        param_13: f32,
        param_14: f32,
    ) {
        let target = unsafe {
            std::mem::transmute::<u64, FnAddSubGoal>(
                Program::current()
                    .rva_to_va(rva::get().goal_ins_add_sub_goal)
                    .unwrap(),
            )
        };

        target(
            self, goal_id, life, param_1, param_2, param_3, param_4, param_5, param_6, param_7,
            param_8, param_9, param_10, param_11, param_12, param_13, param_14,
        )
    }

    pub fn clear_sub_goal(&mut self) {
        let target = unsafe {
            std::mem::transmute::<u64, FnClearSubGoal>(
                Program::current()
                    .rva_to_va(rva::get().goal_ins_clear_sub_goal)
                    .unwrap(),
            )
        };

        target(self)
    }
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSScriptGoal {
    pub base: CSGoalBase,
    /// Container for flags?
    unk128: CSScriptGoal128,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSScriptGoal128 {
    pub owner: NonNull<CSScriptGoal>,
    pub flags: CSScriptGoal128Flags,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CSScriptGoal128Flags(u32);
    impl Debug;

    needs_activation, set_needs_activation: 0;
    needs_intialization, set_needs_initialization: 4;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSGoalBase>(), 0x128);
        assert_eq!(std::mem::size_of::<CSGoalFunc>(), 0x10);
        assert_eq!(std::mem::size_of::<GoalIns>(), 0x3d0);
        assert_eq!(std::mem::size_of::<CSScriptGoal>(), 0x140);
    }
}
