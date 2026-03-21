use shared::OwnedPtr;

use crate::dlkr::DLPlainLightMutex;
use crate::dlui::DLUserInputDeviceImpl;

#[repr(C)]
/// Source of name: RTTI
pub struct DLUserInputManager {
    _vftable: usize,
    pub mutex: DLPlainLightMutex,
    pub allocator: NonNull<DLAllocatorBase>,
    _unk48: u64,
    _unk50: DLVector<u64>,
    _unk70: DLVector<u64>,
    pub devices: DLVector<OwnedPtr<DLUserInputDeviceImpl>>,
    _unkb0: u64,
    pub dummy_device: DLUserInputDeviceImpl,
    pub com_initialized: bool,
    _unk249: u32,
    _unk24b: bool,
    _unk24c: bool,
    pub window_active: bool,
    _unk24e: [u8; 0xA],
    _unk258: DLVector<u64>,
    _unk278: [u8; 0x50],
}
