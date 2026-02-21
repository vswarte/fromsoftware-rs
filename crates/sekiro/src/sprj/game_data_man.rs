use std::borrow::Cow;

use shared::{FromStatic, InstanceResult, OwnedPtr, UnknownStruct};

use super::PlayerGameData;
use crate::{Vector, fd4::FD4Time, rva};

#[repr(C)]
// Source of name: RTTI
pub struct GameDataMan {
    _trophy_equip_data: OwnedPtr<UnknownStruct<0xc10>>,

    /// The main player.
    pub local_player: OwnedPtr<PlayerGameData>,

    /// Networked players. Probably unused in Sekiro.
    pub net_players: OwnedPtr<[PlayerGameData; 0x5]>,

    _unk18: u64,
    _unk20: OwnedPtr<[u8; 5]>,
    _unk28: OwnedPtr<UnknownStruct<0x8c>>,
    _unk30: u8,
    _unk38: OwnedPtr<UnknownStruct<0x44>>,
    _unk40: u32,
    _unk44: u8,
    _unk48: OwnedPtr<UnknownStruct<0x44>>,

    /// The options the player has set.
    pub options_data: OwnedPtr<OptionsData>,

    _menu_system_save_load: OwnedPtr<UnknownStruct<0xf30>>,
    _profile_summary: OwnedPtr<UnknownStruct<0x1510>>,
    _pc_option_data: OwnedPtr<UnknownStruct<0xc8>>,
    _unk70: u64,
    _unk78: u32,
    _unk7c: u32,
    _unk80: u32,
    _unk84: u32,
    _unk88: u16,
    _unk8c: u32,
    _unk90: u32,
    _unk94: u32,
    _unk98: u16,
    _unk9c: u32,
    _unka0: u32,
    _unka4: u32,
    _unka8: FD4Time,
    _unkb8: u8,
    _unkbc: u32,
    _unkc0: u32,
    _unkc4: u32,
    _unkc8: u32,
    _unkcc: u8,
    _unkcd: u8,
    _unkce: u8,
    _unkcf: u8,
    _unkd0: u8,
    _unkd1: u8,
    _unkd4: u32,
    _unkd8: u32,
    _unkdc: u32,
    _unke0: u32,
    _unke4: u32,
    _unke8: u8,
    _unke9: [u8; 0x7],
    _unkf0: u16,
    _unkf8: Vector<()>,
    _unk118: UnknownStruct<0x28>,
    _unk140: u8,
    _unk148: u64,
    _unk150: u32,
    _unk154: u32,
    _unk158: u32,
    _unk15c: u32,
    _unk160: u16,
    _unk164: u32,
}

impl FromStatic for GameDataMan {
    fn name() -> Cow<'static, str> {
        "GameDataMan".into()
    }

    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect(rva::get().game_data_man_ptr) }
    }
}

#[repr(C)]
// Source of name: debug string
pub struct OptionsData {
    pub camera_speed: u8,
    pub pad_vibration: u8,
    pub brightness_sdr: u8,
    pub sound_type: u8,
    pub volume_music: u8,
    pub volume_effects: u8,
    pub volume_voice: u8,
    pub blood_level: BloodLevel,
    pub show_captions: bool,
    pub hud_visible: HudVisibility,
    pub invert_camera_x: bool,
    pub invert_camera_y: bool,
    pub auto_lock: bool,
    pub auto_avoid_wall: bool,
    pub enable_bank_register: bool,
    pub jump_with_l3: bool,
    pub reset_camera_y: bool,
    pub camera_direction: bool,
    pub rank_register_profile_index: u8,
    pub allow_global_matching: bool,
    pub voice_chat: bool,
    pub other_player_name_notation: u8,
    pub auto_lock_on_attack_dir_ctrl: u8,
    pub auto_target: bool,
    pub boot_offline: bool,
    pub hide_white_sign: bool,
    _unk1a: [u8; 0x66],
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HudVisibility {
    AlwaysHide = 0,
    AlwaysShow = 1,
    AutoHideAll = 2,
    AutoHidePartial = 3,
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BloodLevel {
    Off = 0,
    On = 1,
    Mild = 2,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x80, size_of::<OptionsData>());
        assert_eq!(0x168, size_of::<GameDataMan>());
    }
}
