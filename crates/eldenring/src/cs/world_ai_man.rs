use std::ptr::NonNull;

use crate::{
    Tree,
    cs::{BlockId, CSGoalBase, WorldAreaInfo, WorldBlockInfo, WorldGridAreaInfo, WorldInfoOwner},
    dlut::DLDateTime,
};

// Source of name: RTTI
#[repr(C)]
#[shared::singleton("CSWorldAiManager")]
pub struct CSWorldAiManagerImp {
    vtable: usize,
    unk8: u64,
    pub world_info_owner: NonNull<WorldInfoOwner>,
    pub area_count: u32,
    pub area_ptr: NonNull<WorldAreaInfo>,
    pub block_count: u32,
    pub block_ptr: NonNull<WorldAreaInfo>,
    pub grid_area_count: u32,
    pub grid_area_ptr: NonNull<WorldAreaInfo>,
    unk48: u64,
    pub areas: [CSWorldAreaAi; 28],
    pub blocks: [CSWorldBlockAi; 192],
    pub grid_areas: [CSWorldGridAreaAi; 6],
    pub all_areas: [Option<NonNull<CSWorldAreaAiBase>>; 34],
    unk6930: u32,
    pub current_block_index: i32,
    ai_lua: usize,
    unk6940: usize,
    unk6948: [u8; 0x60],
    pub goal_strategies: Tree<CSWorldAiManagerImpGoalStrategy>,
}

#[repr(C)]
pub struct CSWorldAiManagerImpGoalStrategy {
    pub goal_id: i32,
    unk8: usize,
    unk10: usize,
    pub goal_strategy: NonNull<CSGoalBase>,
}

#[repr(C)]
pub struct CSWorldAreaAiBase {
    vtable: usize,
    pub world_area_info: NonNull<WorldAreaInfo>,
    pub area_id: u32,
    unk10: u32,
    unk14: u32,
    unk18: u16,
}

#[repr(C)]
pub struct CSWorldAreaAi {
    pub base: CSWorldAreaAiBase,
    pub world_area_info: NonNull<WorldAreaInfo>,
    /// Number of blocks associated with this area under self.blocks.
    pub block_count: u32,
    pub blocks: NonNull<CSWorldBlockAi>,
}

#[repr(C)]
pub struct CSWorldGridAreaAi {
    pub base: CSWorldAreaAiBase,
    pub world_grid_area_info: NonNull<WorldGridAreaInfo>,
    pub blocks: Tree<CSWorldBlockAi>,
    team_ai: usize,
}

// impl CSWorldAreaAi {
//     pub fn blocks(&self) -> &[CSWorldBlockAi] {
//         if self.block_count == 0 {
//             return &[];
//         }
//
//         unsafe { std::slice::from_raw_parts(self.blocks.as_ref(), self.block_count as usize) }
//     }
// }

#[repr(C)]
pub struct CSWorldBlockAi {
    vtable: usize,
    pub world_block_info: NonNull<WorldBlockInfo>,
    pub world_area_ai: NonNull<CSWorldAreaAi>,
    pub block_id: BlockId,
    unk1c: u32,
    unk20: u8,
    unk21: u8,
    lua_bnd_file_cap: usize,
    pub goal_strategies: Tree<CSWorldBlockAiGoalStrategy>,
    unk48: [u8; 0x20],
    team_ai: usize,
    unk70: usize,
    unk78: usize,
}

#[repr(C)]
pub struct CSWorldBlockAiGoalStrategy {
    pub goal_id: i32,
    pub goal_strategy: NonNull<CSGoalBase>,
}
