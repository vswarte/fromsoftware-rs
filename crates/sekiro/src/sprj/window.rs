use shared::{OwnedPtr, UnknownStruct};

use windows::Win32::Foundation::HINSTANCE;

#[repr(C)]
#[shared::singleton("CSWindow")]
// Source of name: RTTI
pub struct CSWindow {
    _vftable: usize,
    _unk08: u64,
    _unk10: UnknownStruct<0x20>,
    pub hinstance: HINSTANCE,
    pub screen_mode_ctrl: OwnedPtr<UnknownStruct<0xc8>>,
    _unk40: u32,
    _unk44: u32,
    _unk48: u8,
    _unk50: u64,
    _unk58: u64,
    _unk60: u64,
    _unk68: u32,
    _unk6c: u8,
    _unk6e: u16,
    _unk70: UnknownStruct<0x50>,
    _unkc0: UnknownStruct<0x50>,
    _unk110: u64,
    _unk118: u64,
}

#[cfg(test)]
mod test {
    use crate::sprj::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x120, size_of::<CSWindow>());
    }
}
