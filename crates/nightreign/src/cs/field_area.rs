#[repr(C)]
pub struct FieldArea {
    unk0: [u8; 0x18],
    pub world_info_owner: OwnedPtr<WorldInfoOwner>,
    pub game_rend: OwnedPtr<GameRend>,
    unk28: u32,
    pub map_id: BlockId,
    // TODO: more
}

