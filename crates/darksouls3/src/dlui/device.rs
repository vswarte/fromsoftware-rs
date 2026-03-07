use crate::dlkr::DLPlainLightMutex;
use crate::dlui::DynamicBitset;

#[repr(C)]
/// Source of name: RTTI
pub struct DLUserInputDevice {
    _vftable: usize,
    pub allocator: NonNull<DLAllocator>,
    _unk10: [u8; 0x50],
    pub extensions: DLVector<usize>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct DLUserInputDeviceImpl {
    pub device: DLUserInputDevice,
    _unk80: [u8; 0x10],
    pub mutex: DLPlainLightMutex,
    _unkc8: [u8; 0x8],
    _key_info1: VirtualAnalogKeyInfo,
    _key_info2: VirtualAnalogKeyInfo,
    _unk120: DLVector<u64>,
    pub input_data: VirtualInputData,
}

#[repr(C)]
/// Source of name: RTTI
struct VirtualAnalogKeyInfo {
    _vftable: usize,
    _unk08: DLVector<u64>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct VirtualInputData {
    _vftable: usize,
    _key_info: VirtualAnalogKeyInfo,
    pub bitset: DynamicBitset,
}
