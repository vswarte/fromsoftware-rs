use std::ptr::NonNull;

use crate::{
    param::CEREMONY_PARAM_ST,
    position::{BlockPosition, HavokPosition},
    Tree,
};
use shared::OwnedPtr;

use super::BlockId;

// Source of name: RTTI
#[dlrf::singleton("FieldArea")]
#[repr(C)]
pub struct FieldArea {
    vtable: usize,
    unk8: usize,
    pub world_info_owner: OwnedPtr<WorldInfoOwner>,
    world_info_owner_2: NonNull<WorldInfoOwner>,
    // TODO: rest
}

// Source of name: RTTI
#[repr(C)]
pub struct WorldInfoOwner {
    pub world_res: WorldRes,
}

// Source of name: RTTI
#[repr(C)]
pub struct WorldRes {
    pub world_info: WorldInfo,
}

// Source of name: RTTI
#[repr(C)]
pub struct WorldInfo {
    vtable: usize,

    /// Count of legacy + ordinary dungeons area infos.
    pub world_area_info_count: u32,
    _padc: u32,
    /// Pointer to start of list of world area infos for legacy + ordinary dungeons.
    pub world_area_info_list_ptr: NonNull<WorldAreaInfo>,
    /// Count of overworld area infos.
    pub world_grid_area_info_count: u32,
    _pad1c: u32,
    /// Pointer to start of list of world area infos for overworld areas.
    pub world_grid_area_info_list_ptr: NonNull<WorldGridAreaInfo>,
    /// Count of combined dungeon + overworld area infos.
    pub world_area_info_all_count: u32,
    _pad2c: u32,
    /// Combined list of pointers to all overworld and dungeon world area infos.
    pub world_area_info_all: [Option<NonNull<WorldAreaInfoBase>>; 34],
    /// Count of block infos.
    pub world_block_info_count: u32,
    _pad3c: u32,
    /// Pointer to start of list of world block infos.
    pub world_block_info_list_ptr: NonNull<WorldBlockInfo>,
    unk150: u64,
    unk158: u64,
    _world_area_info: [WorldAreaInfo; 28],
    _world_block_info: [WorldBlockInfo; 192],
    _world_grid_area_info: [WorldGridAreaInfo; 6],
    // TODO: Add resource stuff
}

impl WorldInfo {
    pub fn world_area_info(&self) -> &[WorldAreaInfo] {
        &self._world_area_info[0..self.world_area_info_count as usize]
    }

    pub fn world_grid_area_info(&self) -> &[WorldGridAreaInfo] {
        &self._world_grid_area_info[0..self.world_grid_area_info_count as usize]
    }

    pub fn world_block_info(&self) -> &[WorldBlockInfo] {
        &self._world_block_info[0..self.world_block_info_count as usize]
    }

    pub fn world_block_info_by_map(&self, map: &BlockId) -> Option<&WorldBlockInfo> {
        match map.is_overworld() {
            true => self
                .world_grid_area_info()
                .iter()
                .flat_map(|a| a.blocks.iter())
                .find(|b| b.block_id.0 == map.0)
                .map(|b| b.block.as_ref()),
            false => self.world_block_info().iter().find(|b| b.block_id.0 == map.0),
        }
    }
}

// Source of name: RTTI
#[repr(C)]
pub struct WorldAreaInfoBase {
    vtable: usize,
    pub block_id: BlockId,
    pub area_id: u32,
    pub world_info_owner: NonNull<WorldInfoOwner>,
    /// Points to _99 MSB for this area.
    overlay_msb_res_cap: Option<NonNull<()>>,
    unk20: u64,
    unk28: u64,
    unk30: u8,
    _pad31: [u8; 0x7],
}

// Source of name: RTTI
#[repr(C)]
pub struct WorldAreaInfo {
    pub base: WorldAreaInfoBase,
    /// List index in the WorldInfoOwner
    pub list_index: u32,
    /// Starting offset of the areas blocks in the block list in WorldInfoOwner
    pub block_list_start_index: u32,
    /// Amount of blocks associated with this area in the blocks list.
    pub block_count: u32,
    _pad44: u32,
    /// Pointer to start of the areas block in the WorldInfoOwner block list.
    blocks: *const WorldBlockInfo,
}

// Source of name: RTTI
#[repr(C)]
pub struct WorldGridAreaInfo {
    pub base: WorldAreaInfoBase,
    unk38: [u32; 3],
    unk44: u32,
    unk48: u32,
    unk4c: u32,
    unk50: [u32; 3],
    unk5c: [f32; 4],
    unk6c: [f32; 4],
    pub skybox_block_id: BlockId,
    pub skybox_block_info: NonNull<WorldBlockInfo>,
    pub blocks: Tree<WorldGridAreaInfoBlockElement>,
    unka0: Tree<()>,
    unkb8: u64,
    unkc0: Tree<()>,
    unkd8: u64,
}

#[repr(C)]
pub struct WorldGridAreaInfoBlockElement {
    pub block_id: BlockId,
    _pad4: u32,
    pub block: OwnedPtr<WorldBlockInfo>,
}

// Source of name: RTTI
#[repr(C)]
pub struct WorldBlockInfo {
    vtable: usize,
    pub block_id: BlockId,
    unkc: u32,
    pub world_info_owner: NonNull<WorldInfoOwner>,
    /// Effective world area info. Either area or grid area.
    pub area_info: NonNull<WorldAreaInfoBase>,
    /// World area info. Seemingly only used if block is not part of the overworld.
    pub world_area_info: Option<NonNull<WorldAreaInfo>>,
    /// World area info. Seemingly only used if block is part of the overworld.
    pub world_grid_area_info: Option<NonNull<WorldGridAreaInfo>>,
    unk30: u32,
    block_id_2: BlockId,
    /// Index in WorldAreaInfo's block list. Will be -1 if this is an overworld block.
    pub world_area_info_index: i32,
    unk3c: u32,
    unk40: bool,
    unk41: [u8; 0x7],
    msb_res_cap: NonNull<()>,
    unk50: u64,
    unk58: u64,
    unk60: u64,
    unk68: u64,
    /// Havok position of the blocks center.
    pub physics_center: HavokPosition,
    unk80: u64,
    btl_file_cap: NonNull<()>,
    unk90: u64,
    fvb_file_cap: NonNull<()>,
    unka0: u64,
    pre_map_decal_file_cap: NonNull<()>,
    unkb0: u64,
    unkb8: bool,
    unkb9: [u8; 0x7],
    pub ceremony: WorldBlockInfoCeremony,
    unkd0: u32,
    _padd4: u32,
    unkd8: u64,
}

#[repr(C)]
pub struct WorldBlockInfoCeremony {
    pub param_id: i32,
    _pad4: u32,
    pub param_row: Option<NonNull<CEREMONY_PARAM_ST>>,
}
