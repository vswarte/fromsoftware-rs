use std::{borrow::Cow, ptr::NonNull};

use shared::{
    FromStatic, InstanceError, InstanceResult, OwnedPtr, Subclass, Superclass, UnknownStruct,
    for_all_subclasses,
};

use super::{ChrInsModuleContainer, ChrSetEntry, PlayerGameData, WorldChrMan};
use crate::{dlkr::DLAllocatorRef, fd4::FD4Time};

#[repr(C)]
#[derive(Superclass)]
#[superclass(children(PlayerIns, ReplayGhostIns))]
/// Source of name: RTTI
pub struct ChrIns {
    _vftable: usize,
    pub field_ins_handle: u32,
    _unk10: i64,
    pub chr_set_entry: NonNull<ChrSetEntry<ChrIns>>,
    _unk20: OwnedPtr<UnknownStruct<0xe0>>,
    _unk28: u16,
    _chr_res: usize,
    _unk38: u64,
    _unk40: u32,
    _model: usize,
    _player_ctrl: usize,
    _pad_manipulator: usize,
    _chr_tae_anim_event: usize,
    _unk68: u32,
    _unk6c: i32,
    _unk70: i32,
    _unk74: u32,
    _unk78: u32,
    _mdl_mtx: MdlMtx,
    _unk90: u64,
    _unk98: [u8; 8],
    _unka0: UnknownStruct<0x30>,
    _unkd0: u64,
    _unkd8: u64,
    _unke0: u64,
    _unke8: u64,
    _unkf0: u64,
    _unkf8: u64,
    _unk100: FD4Time,
    _unk110: u32,
    _gx_sg_layer_dynamic_tree: usize,
    _unk120: OwnedPtr<UnknownStruct<0x20>>,
    _unk128: [u8; 8],
    _target_velocity_recorder: usize,
    _unk138: [u8; 0xf10],
    _unk1048: u64,
    _unk1050: u64,
    _unk1058: u64,
    _unk1060: u32,
    _unk1064: u32,
    _unk1068: u32,
    _unk106c: u32,
    _unk1070: [u8; 0x10],
    _unk1080: u32,
    _unk1084: u32,
    _unk1088: u32,
    _unk108c: u32,
    _unk1090: u64,
    _cloth_state: SprjClothState,
    _unk10c8: u32,
    _unk10cc: u32,
    _unk10d0: u32,
    _unk10d4: u32,
    _unk10d8: u32,
    _unk10dc: u32,
    _unk10e0: u32,
    _unk10e4: u32,
    _slot_base_seed: FD4SlotBaseSeed,
    _unk1190: UnknownStruct<0x30>,
    _unk11c0: u64,
    _special_effect: usize,
    _unk11d0: u32,
    _unk11d4: u32,
    _unk11d8: [u8; 4],
    _unk11dc: u32,
    _unk11e0: [u8; 8],
    _unk11e8: UnknownStruct<0x820>,
    _unk1a08: u32,
    _unk1a10: u64,
    _unk1a18: u32,
    _unk1a1c: u32,
    _unk1a20: OwnedPtr<UnknownStruct<0x40>>,
    _special_effect_equip_ctrl: usize,
    _unk1a30: [u8; 4],
    _unk1a34: u32,
    _unk1a38: u32,
    _unk1a3c: u32,
    _unk1a40: u32,
    _unk1a44: u32,
    _unk1a48: u32,
    _unk1a4c: u32,
    _unk1a50: u32,
    _unk1a54: u16,
    _unk1a58: [u32; 26],
    _unk1ac0: [u8; 4],
    _unk1ac4: u32,
    _hit_ins_1: usize,
    _hit_ins_2: usize,
    _unk1ad8: u32,
    _unk1adc: u16,
    _chr_attach_sys: ChrAttachSys,
    _allocator1: DLAllocatorRef,
    _unk1b10: [u8; 8],
    _unk1b18: u64,
    _unk1b20: u64,
    _allocator2: DLAllocatorRef,
    _unk1b30: u32,
    _unk1b34: u32,
    _unk1b38: [u8; 8],
    _unk1b40: u64,
    _unk1b48: u64,
    _unk1b50: [u8; 8],
    _unk1b58: u64,
    _unk1b60: u64,
    _unk1b68: u32,
    _unk1b6c: u32,
    _unk1b70: u32,
    _unk1b74: u32,
    _unk1b78: u32,
    _unk1b7c: u32,
    _unk1b80: u16,
    _unk1b84: u32,
    _unk1b88: u16,
    _unk1b8c: u32,
    _unk1b90: u32,
    _unk1b94: u16,
    _unk1b96: u16,
    _unk1b98: u16,
    _void_tasks: [UnknownStruct<0x30>; 0x11],
    _pad11d1: [u8; 8],
    _unk1ed8: u64,
    _unk1ee0: u64,
    _unk1ee8: u16,
    _unk1eea: u8,
    _unk1eec: u32,
    _unk1ef0: u32,
    _unk1ef4: u32,
    _unk1ef8: u32,
    _unk1efc: u32,
    _unk1f00: u32,
    _unk1f04: u32,
    _unk1f08: u32,
    _unk1f0c: u32,
    _unk1f10: u64,
    _unk1f18: u64,
    _unk1f20: u32,
    _unk1f24: u32,
    _unk1f28: u32,
    _unk1f2c: u32,
    _unk1f30: u32,
    _unk1f34: u32,
    _unk1f38: u64,
    _unk1f40: u64,
    _unk1f48: u64,
    _unk1f50: [u8; 4],
    _unk1f54: u32,
    _unk1f58: u32,
    _unk1f5c: u32,
    _unk1f60: u32,
    _unk1f64: u32,
    _unk1f68: [u8; 4],
    _unk1f6c: u32,
    _unk1f70: u32,
    _unk1f78: u64,
    _unk1f80: u64,
    _unk1f88: u32,
    _unk1f8c: u32,
    pub modules: OwnedPtr<ChrInsModuleContainer>,
    _unk1f98: [u8; 8],
}

#[for_all_subclasses]
pub impl ChrInsExt for Subclass<ChrIns> {
    /// Returns the character ID string for this character, of the form `c1234`.
    fn id(&self) -> String {
        self.superclass().modules.data.id()
    }

    /// Set this character's HP to zero, killing it.
    fn kill(&mut self) {
        self.superclass_mut().modules.data.hp = 0;
    }
}

type MdlMtx = UnknownStruct<0x10>;
type SprjClothState = UnknownStruct<0x30>;
type FD4SlotBaseSeed = UnknownStruct<0xa8>;
type ChrAttachSys = UnknownStruct<0x28>;

#[repr(C)]
#[derive(Subclass)]
/// Source of name: RTTI
pub struct PlayerIns {
    pub super_chr_ins: ChrIns,
    pub player_game_data: NonNull<PlayerGameData>,
    _unk1fa8: u64,
    _net_ai_manipulator: usize,
    pub player_session_holder: PlayerSessionHolder,
    _unk1fe0: u32,
    _replay_recorder: usize,
    _unk1ff0: u8,
    _unk1ff4: u32,
    _unk1ff8: u32,
    _unk1ffc: u32,
    _unk2000: u32,
    _unk2008: usize,
    _unk2010: usize,
    _unk2018: usize,
    _unk2020: usize,
    _fg_model: usize,
    _unk2030: u32,
    _unk2038: OwnedPtr<UnknownStruct<0x10>>,
    _ring_equip_ctrl: usize,
    _wep_equip_ctrl: usize,
    _pro_equip_ctrl: usize,
    _unk2058: u64,
    _unk2060: u8,
    _unk2064: u32,
    _unk2068: u64,
    _unk2070: u32,
    _unk2074: u32,
    _unk2078: u32,
    _fdp_chr_face_animation_module: usize,
    _fdp_chr_gender_mapper: usize,
    _event_chr_entry: usize,
    _unk2098: u16,
    _chr_asm: usize,
    _chr_asm_model_res: usize,
    _chr_asm_model: usize,
    _mdl_obj_idx_trans: usize,
    _unk20c0: u64,
    _unk20c8: UnknownStruct<0x28>,
    _unk20f0: [u8; 0x68],
    _player_menu_ctrl: usize,
    _unk2160: [u8; 8],
    _unk2168: FD4Time,
    _unk2178: [u8; 8],

    /// A pointer to this PlayerIns instance.
    pub this: NonNull<PlayerIns>,

    _unk2188: [u8; 0x18],
}

impl FromStatic for PlayerIns {
    fn name() -> Cow<'static, str> {
        "PlayerIns".into()
    }

    /// Returns the singleton instance of `PlayerIns` for the main player
    /// character, if it exists.
    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe {
            WorldChrMan::instance()
                .and_then(|man| man.main_player.ok_or(InstanceError::NotFound))
                .map(|mut ptr| ptr.as_mut())
        }
    }
}

#[repr(C)]
#[derive(Subclass)]
/// Source of name: RTTI
pub struct ReplayGhostIns {
    pub super_chr_ins: ChrIns,
}

#[repr(C)]
/// Source of name: RTTI
pub struct PlayerSessionHolder {
    pub _vftable: usize,
    _debug_session: usize,
    _unk_session: usize,
    _session: usize,
    _unk20: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x28, size_of::<PlayerSessionHolder>());
        assert_eq!(0x1fa0, size_of::<ChrIns>());
        assert_eq!(0x21a0, size_of::<PlayerIns>());
    }
}
