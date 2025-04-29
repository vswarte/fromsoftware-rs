use std::ptr::NonNull;

use shared::OwnedPtr;

use super::CSSosSignMan;

#[dlrf::singleton("CSEventMan")]
#[repr(C)]
pub struct CSEventManImp {
    vftable: usize,
    simple_info: usize,
    dead_reset: usize,
    obj_sfx: usize,
    parts_damage: usize,
    drop_item: usize,
    sound: usize,
    damage: usize,
    dam_obj_hit: usize,
    unk48: usize,
    unk50: usize,
    unk58: usize,
    pub sos_sign: OwnedPtr<CSEventSosSignCtrl>,
    unk68: usize,
    obj_act_exec: usize,
    unk78: usize,
    bloodstain: usize,
    script: usize,
    corpse: usize,
    unk98: usize,
    generator: usize,
    unka8: usize,
    system_flag: usize,
    turn: usize,
    world_area_time: usize,
    fade_warp: usize,
    unkd0: usize,
    unkd8: usize,
    retry_points: usize,
    network_error_return_title_step: usize,
    cutscene_warp: usize,
}

#[repr(C)]
pub struct CSEventSosSignCtrl {
    vftable: usize,
    unk8: [u8; 0x40],
    pub sos_sign_man: Option<NonNull<CSSosSignMan>>,
    unk50: u32,
    unk54: u32,
}
