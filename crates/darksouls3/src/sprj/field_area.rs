use std::{borrow::Cow, ptr::NonNull};

use super::WorldRes;
use crate::rva;
use shared::{FromStatic, InstanceResult};

#[repr(C)]
pub struct FieldArea {
    _vftable: usize,

    pub world_res: Option<NonNull<WorldRes>>,

    _world_res_2: Option<NonNull<WorldRes>>, // Always the same as [world_res], apparently

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

impl FieldArea {
    pub fn world_res(&self) -> Option<&WorldRes> {
        self.world_res.map(|ptr| unsafe { ptr.as_ref() })
    }
}

impl FromStatic for FieldArea {
    fn name() -> Cow<'static, str> {
        "FieldArea".into()
    }

    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect(rva::get().field_area_ptr) }
    }
}
