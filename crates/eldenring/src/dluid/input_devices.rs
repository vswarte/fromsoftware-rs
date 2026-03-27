use std::ptr::NonNull;

use crate::dluid::{KeyboardDevice, MouseDevice, PadDevice, VirtualMultiDevice};

#[repr(C)]
pub struct InputDevices {
    vftable: *const (),
    pub virtual_multi_device: NonNull<VirtualMultiDevice>,
    pub pad_devices: [NonNull<PadDevice>; 4],
    unk30: [u8; 0x10],
    pub mouse_device: NonNull<MouseDevice>,
    pub keyboard_device: NonNull<KeyboardDevice>,
    unk50: [u8; 0x28],
    pub unk78: MultiDevices0x78,
    unk3b0: [u8; 16],
}

#[repr(C)]
pub struct MultiDevices0x78 {
    vftable: *const (),
    allocator: *const (),
    pub bitset_fallback: [bool; 162],
    padding: [u8; 6],
    /// Some weird array, see comments below this struct.
    unkb8: [u8; 0x280],
    unk334: u8,
}

// #[repr(C)]
// pub struct MultiDevices0x78ArrayEntry {
//     pub unk00: u32,
//     pub unk04: u32,
//     pub unk08: bool,
//     padding: [u8; 7],
// }
