use bitfield::bitfield;
use core::slice;
use std::ptr::NonNull;

use crate::{Vector, dlkr::DLPlainLightMutex, dluid::UserInputExtension};
use shared::{Subclass, Superclass};

/// Source of name: RTTI
#[repr(C)]
#[derive(Superclass)]
#[superclass(children(
    VirtualMultiDevice,
    PadDevice,
    KeyboardDevice,
    MouseDevice,
    DummyDevice
))]

pub struct DLUserInputDeviceImpl {
    _vftable: *const (),
    unk008: *const (),
    /// Contains a reference to the same [DLVirtualInputData] from `initial_virtual_input_data`.
    ///
    /// The game accesses this from [FD4PadManager] and it's [CSPad] instances to poll inputs.
    pub virtual_input_data: DLVirtualInputData,
    pub user_input_extensions: Vector<UserInputExtension>,
    unk080: *const (),
    unk088: *const (),
    pub mutex: DLPlainLightMutex,
    unk0c0: f32,
    unk0c4: f32,
    pub analog_positive_axis: DLVirtualAnalogKeyInfo<f32>,
    pub analog_negative_axis: DLVirtualAnalogKeyInfo<f32>,
    unk118: u8,
    unk11c: u32,
    unk120: usize,
    unk128: u32,
    unk12c: u32,
    unk130: usize,
    unk138: DLuserInputDeviceImpl0x138,
    unk750: [u8; 0x18],
    user_input_mapper_slots: Vector<*const ()>,
    /// The [DLVirtualInputData] is inserted here and gets memcpy'd over to `virtual_input_data`
    pub initial_virtual_input_data: DLVirtualInputData,
}

#[repr(C)]
struct DLuserInputDeviceImpl0x138 {
    entries: [DLuserInputDeviceImpl0x138Entry; 0x40],
    /// index game will use to update from an entry
    index: usize,
    /// counter that gets incremented
    counter: u64,
    /// copied over from counter.
    counter_reference: u64,
}

#[repr(C)]
struct DLuserInputDeviceImpl0x138Entry {
    virtual_input_data: NonNull<DLVirtualInputData>,
    /// reference to counter in DLuserInputDeviceImpl0x138.counter
    counter_reference: u64,
    /// Result of Windows QueryPerformanceCounter.
    performance_counter: usize,
}

impl DLUserInputDeviceImpl {
    pub fn get_virtual_analog_state(&self, index: usize) -> f32 {
        self.virtual_input_data.get_analog(index)
    }
    pub fn set_virtual_analog_state(&mut self, index: usize, state: f32) {
        self.virtual_input_data.set_analog(index, state)
    }
    pub fn get_virtual_digital_state(&self, index: usize) -> bool {
        self.virtual_input_data.get_digital(index)
    }
    pub fn set_virtual_digital_state(&mut self, index: usize, state: bool) {
        self.virtual_input_data.set_digital(index, state)
    }
}

/// Source of name: RTTI
#[repr(C)]
pub struct DLVirtualAnalogKeyInfo<T> {
    vftable: *const (),
    pub vector: Vector<T>,
}

/// Source of name: RTTI
#[repr(C)]
pub struct DLVirtualInputData {
    vftable: *const (),
    /// Corresponds to movement inputs such as Mouse, Stick and character movement keys.
    pub analog_key_info: DLVirtualAnalogKeyInfo<f32>,
    /// Corresponds to action inputs such as jump, crouch and attacks.
    pub dynamic_bitset: DynamicBitset,
}

impl DLVirtualInputData {
    pub fn get_analog(&self, index: usize) -> f32 {
        let vector = &self.analog_key_info.vector;
        if index < vector.len() {
            let items = self.analog_key_info.vector.items();
            return items[index];
        }

        0.0
    }
    pub fn set_analog(&mut self, index: usize, state: f32) {
        let vector = &mut self.analog_key_info.vector;
        if index < vector.len() {
            let items = vector.items_mut();
            items[index] = state;
        }
    }
    pub fn get_digital(&self, index: usize) -> bool {
        self.dynamic_bitset.get(index)
    }
    pub fn set_digital(&mut self, index: usize, state: bool) {
        self.dynamic_bitset.set(index, state);
    }
}

/// Source of name: RTTI
#[repr(C)]
pub struct DynamicBitset {
    vftable: *const (),
    /// Corresponds to the amount of integers (32 bit-size) required to store the bitfield.
    ///
    /// Calculated during creation as:
    ///
    /// integer_count = bit_count // 32 * 4.
    integer_count: usize,
    /// Bitfield that this [DynamicBitset] corresponds to.
    ///
    /// It's allocated as an array of integers with the size of `integer_count`.
    /// 
    /// # SAFETY
    /// 
    /// We assume the `integer_count` field is always accurate to access this.
    bitset: NonNull<u32>,
    allocator: *const (),
}

impl DynamicBitset {
    pub fn as_slice(&self) -> &[u32] {
        unsafe {
            let data = self.bitset.as_ptr();
            slice::from_raw_parts(data, self.len())
        }
    }

    pub fn as_slice_mut(&mut self) -> &mut [u32] {
        unsafe {
            let data = self.bitset.as_ptr();
            slice::from_raw_parts_mut(data, self.len())
        }
    }

    pub fn len(&self) -> usize {
        self.integer_count
    }

    pub fn get(&self, bit_index: usize) -> bool {
        let slice: &[u32] = self.as_slice();

        let index: usize = bit_index / 32;
        let row: u32 = slice[index];
        let shift: usize = bit_index & 31;

        ((row >> shift) & 1) == 1
    }

    pub fn set(&mut self, bit_index: usize, state: bool) {
        let slice = self.as_slice_mut();

        let index = bit_index / 32;
        let row = &mut slice[index];
        let shift = bit_index & 31;

        let mask = 1u32 << shift;

        *row = (*row & !mask) | ((state as u32) << shift);
    }
}

/// Source of name: RTTI
///
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
#[derive(Subclass)]
pub struct VirtualMultiDevice {
    pub device: DLUserInputDeviceImpl,
    /// Contains a list of pointers to PadDevice, MouseDevice and KeyboardDevice instances.
    pub device_list: Vector<NonNull<DLUserInputDeviceImpl>>,
}

/// Source of name: RTTI
///
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
#[derive(Subclass)]
pub struct DummyDevice {
    pub device: DLUserInputDeviceImpl,
}

/// Source of name: RTTI
///
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
#[derive(Subclass)]
pub struct PadDevice {
    pub device: DLUserInputDeviceImpl,
    //unk7d8: [u8; 0x290],
    unk7d8: i32,
    unk7dc: [u8; 4],
    unk7e0: [u8; 0x60],
    /// set by memset in vfptr[43]
    unk840: [u8; 80],
    /// `WORD` bitfield of `XInputGetState()`'s wButtons field.
    pub w_buttons: WButtons,
    // unk892: u16,
    /// Index of the user's controller. Can be a value from 0 to 3.
    pub dw_user_index: i32,
    unk898: [u8; 4],
    pub s_thumb_lx: f32,
    pub s_thumb_ly: f32,
    unk8a4: [u8; 4],
    pub s_thumb_rx: f32,
    pub s_thumb_ry: f32,
    unk8b0: [u8; 12],
    pub b_left_trigger: f32,
    pub b_right_trigger: f32,
    //unk8c4: [u8; 0x1A4]
    // TODO: fill this out...
}

bitfield! {
    /// Source: https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad
    #[repr(C)]
    pub struct WButtons(u16);
    impl Debug;

    pub dpad_up,        set_dpad_up:        0;
    pub dpad_down,      set_dpad_down:      1;
    pub dpad_left,      set_dpad_left:      2;
    pub dpad_right,     set_dpad_right:     3;

    pub start,          set_start:          4;
    pub back,           set_back:           5;

    pub left_thumb,     set_left_thumb:     6;
    pub right_thumb,    set_right_thumb:    7;

    pub left_shoulder,  set_left_shoulder:  8;
    pub right_shoulder, set_right_shoulder: 9;

    pub button_a,       set_a:              12;
    pub button_b,       set_b:              13;
    pub button_x,       set_x:              14;
    pub button_y,       set_y:              15;
}

/// Source of name: RTTI
///
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
#[derive(Subclass)]
pub struct MouseDevice {
    pub device: DLUserInputDeviceImpl,
    unk7d8: i32,
    unk7dc: [u8; 4],
    // DirectInput8 interface?
    unk7e0: *const (),
    /// Result of DirectInput8 `GetDeviceState`.
    pub di_mouse_state: DIMouseState2,
    unk7fc: bool,
    unk7fd: u8,
    unk7fe: u8,
    unk7ff: u8,
    /// Horizontal mouse movement.
    pub normalized_lx: f32,
    /// Vertical mouse movement.
    pub normalized_ly: f32,
    /// Scroll mouse movement.
    pub normalized_lz: f32,
}

/// Source of name: https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee416631(v=vs.85)
#[repr(C)]
pub struct DIMouseState2 {
    /// Horizontal mouse movement.
    pub lx: i32,
    /// Vertical mouse movement.
    pub ly: i32,
    /// Scroll mouse movement.
    pub lz: i32,
    /// Mouse buttons 1-8
    pub buttons: [u8; 8],
}

impl DIMouseState2 {
    /// See [DIMouseButton] for reference.
    pub fn pressed<K: Into<usize>>(&self, button: K) -> bool {
        self.buttons[button.into()] & 0x80 != 0
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DIMouseButton {
    Left = 0x00,
    Right = 0x01,
    Middle = 0x02,
    Button4 = 0x03,
    Button5 = 0x04,
    Button6 = 0x05,
    Button7 = 0x06,
    Button8 = 0x07,
}

impl From<DIMouseButton> for usize {
    fn from(button: DIMouseButton) -> Self {
        button as usize
    }
}

/// Source of name: RTTI
///
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
#[derive(Subclass)]
pub struct KeyboardDevice {
    pub device: DLUserInputDeviceImpl,
    unk7d8: i32,
    unk7dc: [u8; 4],
    unk7e0: *const (),
    /// DInput8 keyboard state, see [DIKey] for key indexes.
    pub di_keyboard_state: [u8; 256],
    unk8e8: [u8; 8],
}

impl KeyboardDevice {
    /// See [DIMouseButton] for reference.
    pub fn pressed<K: Into<usize>>(&self, key: K) -> bool {
        self.di_keyboard_state[key.into()] & 0x80 != 0
    }
}

/// Source: https://learn.microsoft.com/en-us/previous-versions/windows/desktop/bb321074(v=vs.85)
#[repr(u8)]
#[allow(nonstandard_style)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DIKey {
    ESCAPE = 0x01,
    _1 = 0x02,
    _2 = 0x03,
    _3 = 0x04,
    _4 = 0x05,
    _5 = 0x06,
    _6 = 0x07,
    _7 = 0x08,
    _8 = 0x09,
    _9 = 0x0A,
    _0 = 0x0B,
    /// - on main keyboard
    MINUS = 0x0C,
    EQUALS = 0x0D,
    /// backspace
    BACK = 0x0E,
    TAB = 0x0F,
    _Q = 0x10,
    _W = 0x11,
    _E = 0x12,
    _R = 0x13,
    _T = 0x14,
    _Y = 0x15,
    _U = 0x16,
    _I = 0x17,
    _O = 0x18,
    _P = 0x19,
    LBRACKET = 0x1A,
    RBRACKET = 0x1B,
    /// Enter on main keyboard
    RETURN = 0x1C,
    LCONTROL = 0x1D,
    _A = 0x1E,
    _S = 0x1F,
    _D = 0x20,
    _F = 0x21,
    _G = 0x22,
    _H = 0x23,
    _J = 0x24,
    _K = 0x25,
    _L = 0x26,
    SEMICOLON = 0x27,
    APOSTROPHE = 0x28,
    /// accent grave
    GRAVE = 0x29,
    LSHIFT = 0x2A,
    BACKSLASH = 0x2B,
    _Z = 0x2C,
    _X = 0x2D,
    _C = 0x2E,
    _V = 0x2F,
    _B = 0x30,
    _N = 0x31,
    _M = 0x32,
    COMMA = 0x33,
    /// . on main keyboard
    PERIOD = 0x34,
    /// / on main keyboard   
    SLASH = 0x35,
    RSHIFT = 0x36,
    /// * on numeric keypad
    MULTIPLY = 0x37,
    /// left Alt   
    LMENU = 0x38,
    SPACE = 0x39,
    CAPITAL = 0x3A,
    F1 = 0x3B,
    F2 = 0x3C,
    F3 = 0x3D,
    F4 = 0x3E,
    F5 = 0x3F,
    F6 = 0x40,
    F7 = 0x41,
    F8 = 0x42,
    F9 = 0x43,
    F10 = 0x44,
    NUMLOCK = 0x45,
    /// Scroll Lock
    SCROLL = 0x46,
    NUMPAD7 = 0x47,
    NUMPAD8 = 0x48,
    NUMPAD9 = 0x49,
    /// - on numeric keypad
    SUBTRACT = 0x4A,
    NUMPAD4 = 0x4B,
    NUMPAD5 = 0x4C,
    NUMPAD6 = 0x4D,
    /// + on numeric keypad
    ADD = 0x4E,
    NUMPAD1 = 0x4F,
    NUMPAD2 = 0x50,
    NUMPAD3 = 0x51,
    NUMPAD0 = 0x52,
    /// . on numeric keypad
    DECIMAL = 0x53,
    /// <> or \| on RT 102-key keyboard (Non-U.S.)     
    OEM_102 = 0x56,
    F11 = 0x57,
    F12 = 0x58,
    F13 = 0x64,
    F14 = 0x65,
    F15 = 0x66,
    /// (Japanese keyboard)  
    KANA = 0x70,
    /// /? on Brazilian keyboard               
    ABNT_C1 = 0x73,
    /// (Japanese keyboard)
    CONVERT = 0x79,
    /// (Japanese keyboard)            
    NOCONVERT = 0x7B,
    /// (Japanese keyboard)         
    YEN = 0x7D,
    /// Numpad . on Brazilian keyboard             
    ABNT_C2 = 0x7E,
    /// = on numeric keypad
    NUMPADEQUALS = 0x8D,
    /// Previous Track (CIRCUMFLEX on Japanese keyboard)   
    PREVTRACK = 0x90,
    AT = 0x91,
    COLON = 0x92,
    UNDERLINE = 0x93,
    /// (Japanese keyboard)   
    KANJI = 0x94,
    STOP = 0x95,
    AX = 0x96,
    UNLABELED = 0x97,
    NEXTTRACK = 0x99,
    /// Enter on numeric keypad  
    NUMPADENTER = 0x9C,
    RCONTROL = 0x9D,
    MUTE = 0xA0,
    CALCULATOR = 0xA1,
    PLAYPAUSE = 0xA2,
    MEDIASTOP = 0xA4,
    VOLUMEDOWN = 0xAE,
    VOLUMEUP = 0xB0,
    WEBHOME = 0xB2,
    /// , on numeric keypad
    NUMPADCOMMA = 0xB3,
    /// / on numeric keypad
    DIVIDE = 0xB5,
    SYSRQ = 0xB7,
    /// right Alt
    RMENU = 0xB8,
    PAUSE = 0xC5,
    /// Home on arrow keypad
    HOME = 0xC7,
    // UpArrow on arrow keypad
    UP = 0xC8,
    /// PgUp on arrow keypad
    PRIOR = 0xC9,
    /// LeftArrow on arrow keypad
    LEFT = 0xCB,
    /// RightArrow on arrow keypad
    RIGHT = 0xCD,
    /// End on arrow keypad
    END = 0xCF,
    /// DownArrow on arrow keypad
    DOWN = 0xD0,
    // PgDn on arrow keypad
    NEXT = 0xD1,
    /// Insert on arrow keypad
    INSERT = 0xD2,
    /// Delete on arrow keypad
    DELETE = 0xD3,
    /// Left Windows key
    LWIN = 0xDB,
    /// Right Windows key
    RWIN = 0xDC,
    APPS = 0xDD,
    POWER = 0xDE,
    SLEEP = 0xDF,
    WAKE = 0xE3,
    WEBSEARCH = 0xE5,
    WEBFAVORITES = 0xE6,
    WEBREFRESH = 0xE7,
    WEBSTOP = 0xE8,
    WEBFORWARD = 0xE9,
    WEBBACK = 0xEA,
    MYCOMPUTER = 0xEB,
    MAIL = 0xEC,
    MEDIASELECT = 0xED,
}

impl From<DIKey> for usize {
    fn from(key: DIKey) -> Self {
        key as usize
    }
}
