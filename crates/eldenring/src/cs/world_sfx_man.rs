use std::ptr::NonNull;

use crate::Tree;

use super::{BlockId, WorldInfoOwner};

#[repr(C)]
/// Source of name: RTTI
#[shared::singleton("WorldSfxMan")]
pub struct WorldSfxMan {
    vftable: usize,
    unk8: usize,
    world_info_owner: NonNull<WorldInfoOwner>,

    world_area_sfx_count: u32,
    _pad1c: u32,
    world_area_sfx_list: NonNull<WorldAreaSfx>,

    world_block_sfx_count: u32,
    _pad2c: u32,
    world_block_sfx_list: NonNull<WorldBlockSfx>,

    world_grid_area_sfx_count: u32,
    _pad3c: u32,
    world_grid_area_sfx_list: NonNull<WorldGridAreaSfx>,

    _pad48: u64,

    world_area_sfx: [WorldAreaSfx; 28],
    world_block_sfx: [WorldBlockSfx; 192],
    world_grid_area_sfx: [WorldAreaSfx; 6],

    unk6110: [u8; 0x180],
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldAreaSfxBase {
    vftable: usize,
    world_area_info: usize,
    unk10: u64,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldAreaSfx {
    base: WorldAreaSfxBase,
    world_area_info: usize,
    world_block_sfx_count: u32,
    _pad24: u32,
    world_block_sfx: NonNull<WorldBlockSfx>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldGridAreaSfx {
    base: WorldAreaSfxBase,
    world_area_info: usize,
    unk20: Tree<()>,
    unk38: usize,
}

#[repr(C)]
/// Source of name: RTTI
pub struct WorldBlockSfx {
    vftable: usize,
    world_block_info: usize,
    world_area_sfx: NonNull<WorldAreaSfx>,
    pub block_id: BlockId,
    unk1c: [u8; 0x40],
    pub total_sfx_count: u32,
    unk60: usize,
    unk68: u32,
    unk6c: u32,
    unk70: usize,
}
