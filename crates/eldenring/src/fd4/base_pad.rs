use std::ptr::NonNull;

use crate::{
    Pair, Tree,
    cs::CSKeyAssign,
    dltx::{DLString, DLUTF16StringKind},
    dluid::InputDevices,
};

#[repr(C)]
pub struct FD4BasePad {
    pub vftable: *const (),
    allocator: *const (),
    /// [InputDevices] instance referenced in `FD4PadManager.devices`
    pub input_devices: NonNull<InputDevices>,
    /// `パッド名未設定` | `Pad name not set`
    ///
    /// The game will sometimes change this to the name of the [CSPad] type.
    ///
    /// For example: `CSMenuViewerPad` or `CSDebugPausePad`.
    pub pad_name: *const u16,
    /// Only allows polling when `true`.
    pub allow_polling: bool,
    unk24: i32,
    /// [CSKeyAssign] instance referenced in `FD4PadManager.key_assigns`.
    ///
    /// This will be the same [Subclass<>] as the [CSPad] that holds this instance.
    pub key_assign: NonNull<CSKeyAssign>,
    /// Usually just 1.
    unk30: usize,
    /// Represents a [Map<>] that groups the given input code to an [InputTypeGroup].
    ///
    /// The [InputTypeGroup] contains the [InputType] for that input and the code to poll it with.
    pub input_type_group: NonNull<Tree<Pair<i32, InputTypeGroup>>>,
    /// Represents a [Map<>] that maps the given input code to two booleans.
    ///
    /// If the first boolean is true and the second boolean is false, the input can be polled.
    pub input_code_check: NonNull<Tree<Pair<i32, InputCodeState>>>,
    /// Emtpy [DLString<DLUTF16StringKind>].
    ///
    /// Maybe leftover from debug shenanigans?
    pub empty_str: DLString<DLUTF16StringKind>,
    /// Represents the Cursor relative to the game window.
    pub window_cursor_context: NonNull<WindowCursorContext>,
    /// Represents a [Map<>] that maps the given input code to a boolean representing whether the input is pressed or not.
    ///
    /// Couldn't find a single reference that inserts in to this Tree in Ghidra for `CSInGamePad`'s.
    pub unused_input_map: Tree<Pair<i32, bool>>,
}

#[repr(C)]
pub struct InputCodeState {
    pub state_1: bool,
    pub state_2: bool,
}

#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputType {
    AreKeysDown = 0x00,
    AreKeysUp = 0x01,
    IsStickMoving = 0x02,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct InputTypeGroup {
    pub mapped_input_list: [i32; 4],
    pub input_type_list: [InputType; 4],
}

impl InputTypeGroup {
    pub fn iter(&self) -> impl Iterator<Item = (i32, InputType)> + '_ {
        self.mapped_input_list
            .iter()
            .copied()
            .zip(self.input_type_list.iter().copied())
            .filter(|(mapped_input, _)| *mapped_input != -1)
    }
}

/// Structure that holds cursor position relative to the game window.
#[repr(C)]
pub struct WindowCursorContext {
    /// Copied over from FD4PadManager when constructed.
    pub window_handle: isize,
    unk8: [u8; 0x18],
    /// Horizontal position of the mouse relative to the window the game is opened in.
    pub cursor_x: i32,
    /// Vertical position of the mouse relative to the window the game is opened in.
    pub cursor_y: i32,
    unk28: [u8; 0x14],
}
