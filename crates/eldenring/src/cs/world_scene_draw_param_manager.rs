use std::ptr::NonNull;

use crate::DoublyLinkedList;

use super::{BlockId, WorldInfoOwner};

/// Source of name: RTTI
#[repr(C)]
#[dlrf::singleton("CSWorldSceneDrawParamManager")]
pub struct CSWorldSceneDrawParamManager {
    vftable: usize,
    pub world_info_owner: NonNull<WorldInfoOwner>,
    world_block_info_count: u32,
    _pad14: u32,
    unk18: u64,
    unk20: u64,
    pub world_area_blocks: DoublyLinkedList<CSWorldAreaBlockSceneDrawParam>,
    pub scene_draw_param: CSFD4SceneDrawParam,
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSFD4SceneDrawParam {
    vftable: usize,
    pub lerpers: [CSGparamIdLerper; 15],
    pub lerper: CSGparamIdLerper,
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSGparamIdLerper {
    vftable: usize,
    pub unk8: u32,
    pub unkc: u32,
    pub destination_id: i32,
    pub unk14: u32,
    pub begin_id: i32,
    pub unk1c: u32,
    pub timer: f32,
    pub unk24: f32,
}

/// Source of name: RTTI
#[repr(C)]
pub struct CSWorldAreaBlockSceneDrawParam {
    vftable: usize,
    unk8: bool,
    pub is_gparam_ref_settings_for_overworld: bool,
    unka: [u8; 6],
    pub area: BlockId,
    unk14: u32,
    unk18: u32,
    unk1c: u32,
    unk20: u64,
    unk28: u64,
    unk30: [u8; 0xd8],
    weather_gparam_1: i32,
    weather_gparam_2: i32,
    unk110: u64,
    pub transition_to_override: bool,
    unk119: [u8; 3],
    pub override_gparam: u32,
    unk120: u32,
    pub override_transition_duration: f32,
    unk128: u32,
    unk12c: u32,
    // TODO: fill up to 0x168
}
