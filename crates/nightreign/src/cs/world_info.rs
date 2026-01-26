#[repr(C)]
pub struct WorldInfoOwner {
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
    pub world_area_info_all: [Option<NonNull<WorldAreaInfoBase>>; 30],
    /// Count of block infos.
    pub world_block_info_count: u32,
    _pad3c: u32,
    /// Pointer to start of list of world block infos.
    pub world_block_info_list_ptr: NonNull<WorldBlockInfo>,
    unk130: u32,
    unk134: u32,
    unk138: u64,
    _world_area_info: [WorldAreaInfo; 20],
    _world_block_info: [WorldBlockInfo; 128],
    _world_grid_area_info: [WorldGridAreaInfo; 6],
    // TODO: Add resource stuff
}

impl WorldInfoOwner {
    pub fn world_area_info(&self) -> &[WorldAreaInfo] {
        &self._world_area_info[0..self.world_area_info_count as usize]
    }

    pub fn world_grid_area_info(&self) -> &[WorldGridAreaInfo] {
        &self._world_grid_area_info[0..self.world_grid_area_info_count as usize]
    }

    pub fn world_block_info(&self) -> &[WorldBlockInfo] {
        &self._world_block_info[0..self.world_block_info_count as usize]
    }

    pub fn world_block_info_by_map(&self, block_id: &BlockId) -> Option<&WorldBlockInfo> {
        let mut block_id = *block_id;

        // Figure out overworld map ID to prevent storing data reliant on randomized features.
        if block_id.is_small_base_map() {
            // Figure out what grid area info stores the small bases
            let world_area_info = self
                .world_grid_area_info()
                .iter()
                .find(|w| w.base.hosts_small_bases)?;

            for small_base in world_area_info.small_bases.iter() {
                if small_base.block.small_base_block_id == block_id {
                    block_id = small_base.block.small_base_parent_block_id;
                }
            }
        }

        match block_id.is_overworld() {
            true => self
                .world_grid_area_info()
                .iter()
                .flat_map(|a| a.blocks.iter())
                .find(|b| b.map_id == block_id)
                .map(|b| b.block.as_ref()),
            false => self
                .world_block_info()
                .iter()
                .find(|b| b.map_id == block_id),
        }
    }
}

// Source of name: RTTI
#[repr(C)]
pub struct WorldAreaInfoBase {
    vtable: usize,
    pub map_id: BlockId,
    pub area_id: u32,
    pub world_info_owner: NonNull<WorldInfoOwner>,
    /// Points to _99 MSB for this area.
    overlay_msb_res_cap: Option<NonNull<()>>,
    unk20: u64,
    unk28: u64,
    pub hosts_small_bases: bool,
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
    pub skybox_map_id: BlockId,
    pub skybox_block_info: NonNull<WorldBlockInfo>,
    pub blocks: Tree<WorldGridAreaInfoBlockElement>,
    unka0: Tree<()>,
    unkb8: u64,
    unkc0: u64,
    pub small_bases: Tree<WorldGridAreaInfoSmallBaseBlockElement>,
}

#[repr(C)]
pub struct WorldGridAreaInfoBlockElement {
    pub map_id: BlockId,
    _pad4: u32,
    pub block: OwnedPtr<WorldBlockInfo>,
}

#[repr(C)]
pub struct WorldGridAreaInfoSmallBaseBlockElement {
    pub map_id: BlockId,
    _pad4: u32,
    // TODO: Might be a struct here instead of a pointer pointer.
    pub block: OwnedPtr<OwnedPtr<WorldBlockInfo>>,
}

// Source of name: RTTI
#[repr(C)]
pub struct WorldBlockInfo {
    vtable: usize,
    pub map_id: BlockId,
    unkc: [u8; 0x28],
    pub small_base_block_id: BlockId,
    pub small_base_parent_block_id: BlockId,
    unk3c: [u8; 0x44],
    pub physics_center: HavokPosition,
    unk90: [u8; 0x60],
}
