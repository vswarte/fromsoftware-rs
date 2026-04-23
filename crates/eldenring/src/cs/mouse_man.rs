#[repr(C)]
#[shared::singleton("CSMouseMan")]
pub struct CSMouseMan {
    unk00: bool,
    unk01: bool,
    unk02: bool,
    pub show_cursor: bool,
    unk04: f32,
    unk08: f32,
    unk0c: bool,
    unk0d: bool,
    unk0e: bool,
    unk0f: bool,
    pub mouse_base_coefficient: f32,
    pub mouse_coefficient_min: f32,
    pub mouse_coefficient_max: f32,
    unk1c: f32,
    unk20: f32,
    unk24: f32,
    /// Horizontal position of the mouse relative to the window the game is opened in.
    pub cursor_x: i32,
    /// Vertical position of the mouse relative to the window the game is opened in.
    pub cursor_y: i32,
    /// `true` if the current mouse position doesn't match the current `cursor_x` and `cursor_y`.
    //
    // Translated Ghidra decomp below:
    // ```
    // let buffer = (i32, i32);
    //
    // GetCursorPos(&buffer);
    //
    // if self.cursor_x == buffer.0 && self.cursor_y == buffer.1 {
    //     self.is_mouse_moving = false;
    // } else {
    //     self.is_mouse_moving = true;
    // }
    //
    // self.cursor_x = buffer.0;
    // self.cursor_y = buffer.1;
    // ```
    pub is_mouse_moving: bool,
    /// `true` if the current position of the mouse is inside the client rectangle.
    //
    // Translated Ghidra decomp below:
    // ```
    // let window_handle = CS::CSWindowImp::GetWindowHandle(GLOBAL_CSWindow);
    //
    // let buffer = (i32, i32, i32, i32);
    //
    // if GetClientRect(window_handle, &buffer) {
    //     self.is_mouse_in_window = self.cursor_x < buffer.0
    //         || self.cursor_y < buffer.1
    //         || buffer.2 < self.cursor_x
    //         || buffer.3 < self.cursor_y;
    // }
    //
    // ```
    pub is_mouse_in_window: bool,
    unk32: bool,
    unk33: bool,
}
