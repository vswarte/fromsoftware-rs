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
    /// Multiplier of *remapped mouse sensitivity*. Has a linear effect on the cursor speed.
    ///
    /// When the game calculates mouse sensitivity, it first takes the value present in game
    /// settings (0-10), remaps it linearly to the `[mouse_coefficient_min, mouse_coefficient_max]`
    /// interval. This is then multiplied by the base coefficient and the raw mouse movement in
    /// pixels to obtain the final, sensitivity adjusted mouse displacement.
    pub mouse_base_coefficient: f32,
    /// Value the minimum mouse sensitivity setting (0) is linearly mapped to.
    pub mouse_coefficient_min: f32,
    /// Value the maximum mouse sensitivity setting (10) is linearly mapped to.
    pub mouse_coefficient_max: f32,
    /// Value the maximum target change sensitivity setting (0) is linearly mapped to.
    pub target_change_sensitivity_coefficient_max: f32,
    /// Value the minimum target change sensitivity setting (0) is linearly mapped to.
    pub target_change_sensitivity_coefficient_min: f32,
    /// Seems to be related to keeping the cursor within the game window?
    /// Zeroed while in menu, climbs to 0.5 while in-game.
    unk24: f32,
    /// Horizontal position of the mouse relative to the game window's top left corner.
    pub cursor_x: i32,
    /// Vertical position of the mouse relative to the game window's top left corner.
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
