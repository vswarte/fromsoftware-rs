use std::{borrow::Cow, ptr::NonNull};

use super::WorldInfoOwner;
use crate::rva;
use shared::*;

#[repr(C)]
pub struct FieldArea {
    _vftable: usize,

    pub world_info_owner: OwnedPtr<WorldInfoOwner>,

    _world_info_owner_2: NonNull<WorldInfoOwner>, // Always the same as [world_info_owner], apparently

    _game_rend: u64,
    _unk20: u32,
    _chr_cam: u64,
    _unk30: [u8; 0x30],
    _hit_ins: u64,
    _unk68: u64,
    _field_backread: usize,
    _unk78: [u8; 0x60],
    _self: NonNull<FieldArea>,
    _unke0: usize,
    _unke8: [u8; 8],
}

impl FromStatic for FieldArea {
    fn name() -> Cow<'static, str> {
        "FieldArea".into()
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { shared::load_static_indirect(rva::get().field_area_ptr) }
    }
}
