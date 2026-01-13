use std::ptr::NonNull;

use vtable_rs::VPtr;

use crate::{
    Tree, cs::{AiIns, CSAiFunc}, dlkr::DLAllocatorBase, dltx::{DLString, DLUTF16StringKind}
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
    pub no_update: bool,
    pub no_interrupt: bool,
    pub no_subgoal: bool,
    pub use_avoid_chr: bool,
    pub update_time_min: f32,
    pub update_time_max: f32,
    unk108: bool,
    pub combo_attack_cancel: bool,
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

    fn unk28(&mut self);

    fn unk30(&mut self);
}

#[repr(i32)]
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
    pub goal_func: NonNull<CSGoalFunc>,
    pub life: f32,
    pub next_update_cooldown: f32,
    pub children_info: GoalChildrenInfo,
    pub params_basic: [f32; 8],
}


#[repr(C)]
/// Source of name: RTTI
pub struct GoalChildrenInfo {
    unk0: NonNull<DLAllocatorBase>,
    unk8: isize,
    unk10: isize,
    unk18: isize,
    unk20: isize,
    pub subgoal_count: isize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSGoalBase>(), 0x128);
        assert_eq!(std::mem::size_of::<CSGoalFunc>(), 0x10);
        assert_eq!(std::mem::size_of::<GoalIns>(), 0x3d0);
    }
}
