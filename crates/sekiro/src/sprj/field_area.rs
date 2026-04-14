use std::{borrow::Cow, ptr::NonNull};

use super::WorldRes;
use crate::rva;
use shared::*;

#[repr(C)]
pub struct FieldArea {
    _vftable: usize,
    _unk08: usize,

    pub world_info_owner: OwnedPtr<WorldRes>,

    _world_info_owner_2: Option<NonNull<WorldRes>>, // Always the same as [world_res], apparently

    _game_rend: UnknownPtr,
    _current_world_block_info_index: u32,
    _chr_cam: UnknownPtr,
    _unk38: u32,
    _unk40: u64,
    _unk48: u64,
    _unk50: u32,
    _unk58: u64,
    _unk60: u64,
    _hit_ins: UnknownPtr,
    _unk70: u64,
    _backread: OwnedPtr<UnknownStruct<0x78>>,
    _unk80: u64,
    _unk88: u64,
    _unk90: u64,
    _unk98: u64,
    _unka0: u32,
    _unka8: UnknownStruct<0x18>, // tree
    _unkc0: u16,

    pub debug_measurement_display: bool,
    pub debug_major_reset: bool,

    _unkc4: [u8; 0xc],
    _unkd0: u32,
    _unkd4: u32,
    _unkd8: u32,
    _unkdc: u32,
}

impl FromStatic for FieldArea {
    fn name() -> Cow<'static, str> {
        "FieldArea".into()
    }

    fn instance_ptr() -> InstanceResult<*mut Self> {
        unsafe { shared::load_static_indirect(rva::get().field_area_ptr) }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0xe0, size_of::<FieldArea>());
    }
}
