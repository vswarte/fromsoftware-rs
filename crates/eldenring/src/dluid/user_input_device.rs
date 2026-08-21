use bitfield::bitfield;
use std::{
    ops::{Deref, DerefMut},
    ptr::NonNull,
};

use crate::{
    DLVector,
    dlkr::DLPlainLightMutex,
    dluid::{DLVirtualAnalogKeyInfo, DLVirtualInputData},
};
use shared::{Subclass, Superclass, UnknownStruct};

#[repr(C)]
pub struct DLUserInputDevice {
    vftable: *const (),
    allocator: *const (),
    /// The data in `DLUserInputDeviceImpl.virtual_input_data` gets copied over to this field.
    ///
    /// The game accesses this from `FD4PadManager` and it's `CSPad` instances to poll inputs.
    pub virtual_input_data: DLVirtualInputData,
    user_input_extensions: DLVector<UnknownStruct<0x8>>,
}

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
    device: DLUserInputDevice,
    unk080: usize,
    unk088: usize,
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
    user_input_mapper_slots: DLVector<*const ()>,
    /// The [DLVirtualInputData] is inserted here and gets memcpy'd over to `virtual_input_data`
    pub initial_virtual_input_data: DLVirtualInputData,
}

impl Deref for DLUserInputDeviceImpl {
    type Target = DLUserInputDevice;

    fn deref(&self) -> &Self::Target {
        &self.device
    }
}

impl DerefMut for DLUserInputDeviceImpl {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.device
    }
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
///
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
#[derive(Subclass)]
pub struct VirtualMultiDevice {
    device: DLUserInputDeviceImpl,
    /// Contains a list of pointers to PadDevice, MouseDevice and KeyboardDevice instances.
    pub user_input_devices: DLVector<NonNull<DLUserInputDeviceImpl>>,
}

/// Source of name: RTTI
///
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
#[derive(Subclass)]
pub struct DummyDevice {
    device: DLUserInputDeviceImpl,
}

/// Source of name: RTTI
///
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
#[derive(Subclass)]
pub struct PadDevice {
    device: DLUserInputDeviceImpl,
    //unk7d8: [u8; 0x290],
    unk7d8: i32,
    unk7dc: [u8; 4],
    unk7e0: [u8; 0x60],
    /// set by memset in vfptr[43]
    unk840: [u8; 80],
    /// `WORD` bitfield of `XInputGetState()`'s wButtons field.
    pub w_buttons: WButtons,
    // unk892: u16,
    /// Index of the user's controller. Can be 0..4.
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
    /// Source: <https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad>
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

    pub button_a,       set_button_a:       12;
    pub button_b,       set_button_b:       13;
    pub button_x,       set_button_x:       14;
    pub button_y,       set_button_y:       15;
}

/// Source of name: RTTI
///
/// Subclass of [DLUserInputDeviceImpl]
#[repr(C)]
#[derive(Subclass)]
pub struct MouseDevice {
    device: DLUserInputDeviceImpl,
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

impl MouseDevice {
    /// See [DIMouseButton] for reference.
    pub fn is_key_pressed<K: Into<usize>>(&self, button: K) -> bool {
        self.di_mouse_state.buttons[button.into()] & 0x80 != 0
    }
}

/// Source of name: <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/ee416631(v=vs.85)>
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
    device: DLUserInputDeviceImpl,
    unk7d8: i32,
    unk7dc: [u8; 4],
    unk7e0: *const (),
    /// DInput8 keyboard state, see [DIKey] for key indexes.
    pub di_keyboard_state: [u8; 256],
    unk8e8: [u8; 8],
}

impl KeyboardDevice {
    /// See [DIMouseButton] for reference.
    pub fn is_key_pressed<K: Into<usize>>(&self, key: K) -> bool {
        self.di_keyboard_state[key.into()] & 0x80 != 0
    }
}

/// Source: <https://learn.microsoft.com/en-us/previous-versions/windows/desktop/bb321074(v=vs.85)>
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DIKey {
    Escape = 0x01,
    D1 = 0x02,
    D2 = 0x03,
    D3 = 0x04,
    D4 = 0x05,
    D5 = 0x06,
    D6 = 0x07,
    D7 = 0x08,
    D8 = 0x09,
    D9 = 0x0A,
    D0 = 0x0B,
    /// - on main keyboard
    Minus = 0x0C,
    Equals = 0x0D,
    /// backspace
    Back = 0x0E,
    Tab = 0x0F,
    Q = 0x10,
    W = 0x11,
    E = 0x12,
    R = 0x13,
    T = 0x14,
    Y = 0x15,
    U = 0x16,
    I = 0x17,
    O = 0x18,
    P = 0x19,
    LeftBracket = 0x1A,
    RightBracket = 0x1B,
    /// Enter on main keyboard
    Return = 0x1C,
    LeftControl = 0x1D,
    A = 0x1E,
    S = 0x1F,
    D = 0x20,
    F = 0x21,
    G = 0x22,
    H = 0x23,
    J = 0x24,
    K = 0x25,
    L = 0x26,
    Semicolon = 0x27,
    Apostrophe = 0x28,
    /// accent grave
    Grave = 0x29,
    LeftShift = 0x2A,
    Backslash = 0x2B,
    Z = 0x2C,
    X = 0x2D,
    C = 0x2E,
    V = 0x2F,
    B = 0x30,
    N = 0x31,
    M = 0x32,
    Comma = 0x33,
    /// . on main keyboard
    Period = 0x34,
    /// / on main keyboard   
    Slash = 0x35,
    RightShift = 0x36,
    /// * on numeric keypad
    Multiply = 0x37,
    /// left Alt   
    LeftMenu = 0x38,
    Space = 0x39,
    Capital = 0x3A,
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
    Numlock = 0x45,
    /// Scroll Lock
    Scroll = 0x46,
    Numpad7 = 0x47,
    Numpad8 = 0x48,
    Numpad9 = 0x49,
    /// - on numeric keypad
    Subtract = 0x4A,
    Numpad4 = 0x4B,
    Numpad5 = 0x4C,
    Numpad6 = 0x4D,
    /// + on numeric keypad
    Add = 0x4E,
    Numpad1 = 0x4F,
    Numpad2 = 0x50,
    Numpad3 = 0x51,
    Numpad0 = 0x52,
    /// . on numeric keypad
    Decimal = 0x53,
    /// <> or \| on RT 102-key keyboard (Non-U.S.)     
    Oem102 = 0x56,
    F11 = 0x57,
    F12 = 0x58,
    F13 = 0x64,
    F14 = 0x65,
    F15 = 0x66,
    /// (Japanese keyboard)  
    Kana = 0x70,
    /// /? on Brazilian keyboard               
    AbntC1 = 0x73,
    /// (Japanese keyboard)
    Convert = 0x79,
    /// (Japanese keyboard)            
    NoConvert = 0x7B,
    /// (Japanese keyboard)         
    Yen = 0x7D,
    /// Numpad . on Brazilian keyboard             
    AbntC2 = 0x7E,
    /// = on numeric keypad
    NumpadEquals = 0x8D,
    /// Previous Track (CIRCUMFLEX on Japanese keyboard)   
    PrevTrack = 0x90,
    AT = 0x91,
    Colon = 0x92,
    Underline = 0x93,
    /// (Japanese keyboard)   
    Kanji = 0x94,
    Stop = 0x95,
    AX = 0x96,
    UnLabeled = 0x97,
    NextTrack = 0x99,
    /// Enter on numeric keypad  
    NumpadEnter = 0x9C,
    RightControl = 0x9D,
    Mute = 0xA0,
    Calculator = 0xA1,
    PlayPause = 0xA2,
    MediaStop = 0xA4,
    VolumeDown = 0xAE,
    VolumeUp = 0xB0,
    WebHome = 0xB2,
    /// , on numeric keypad
    NumpadComma = 0xB3,
    /// / on numeric keypad
    Divide = 0xB5,
    SYSRQ = 0xB7,
    /// right Alt
    RightMenu = 0xB8,
    Pause = 0xC5,
    /// Home on arrow keypad
    Home = 0xC7,
    // UpArrow on arrow keypad
    Up = 0xC8,
    /// PgUp on arrow keypad
    Prior = 0xC9,
    /// LeftArrow on arrow keypad
    Left = 0xCB,
    /// RightArrow on arrow keypad
    Right = 0xCD,
    /// End on arrow keypad
    End = 0xCF,
    /// DownArrow on arrow keypad
    Down = 0xD0,
    // PgDn on arrow keypad
    Next = 0xD1,
    /// Insert on arrow keypad
    Insert = 0xD2,
    /// Delete on arrow keypad
    Delete = 0xD3,
    /// Left Windows key
    LeftWin = 0xDB,
    /// Right Windows key
    RightWin = 0xDC,
    Apps = 0xDD,
    Power = 0xDE,
    Sleep = 0xDF,
    Wake = 0xE3,
    WebSearch = 0xE5,
    WebFavorites = 0xE6,
    WebRefresh = 0xE7,
    WebStop = 0xE8,
    WebForward = 0xE9,
    WebBack = 0xEA,
    MyComputer = 0xEB,
    Mail = 0xEC,
    MediaSelect = 0xED,
}

impl From<DIKey> for usize {
    fn from(key: DIKey) -> Self {
        key as usize
    }
}
