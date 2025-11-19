// source of name: singleton reflection data
#[repr(C)]
#[shared::singleton("CSFlipper")]
pub struct CSFlipper {
    vftable: usize,
    pub flip_mode_initial: FlipMode,
    pub flip_mode_current: FlipMode,
    pub flip_mode_platform: FlipMode,
    unk14: u8,
    pub previous_force_mode: bool,
    pub force_mode_pending: bool,
    unk17: u8,
    pub vsync_interval: u32,
    pub fixed_fps: f32,
    pub previous_frame_qpc: usize,
    pub current_frame_qpc: usize,
    unk30: [u8; 48],
    pub frame_time_history: [CSFlipperVsyncHistoryEntry; 32],
    pub frame_history_index: u32,
    pub last_frame_time: f32,
    /// Delta time used for task updates.
    /// Useful for making movement and coordinate changes consistent in your custom tasks.
    pub task_delta: f32,
    pub foreground_frame_history_count: u32,
    pub background_frame_history_count: u32,
    pub enable_frame_sync: bool,
    pub reset_frame_history_count: bool,
    pub force_no_sleep: bool,
    unk227: u8,
    pub frame_time_rolling_average: [f32; 16],
    pub calculated_fps: f32,
    pub override_foreground_history_count: i32,
    pub override_background_history_count: i32,
    pub dynamic_fps_lock: f32,
    pub use_dynamic_fps_lock: bool,
    pub dynamic_fps_active: bool,
    pub dynamic_fps_transition: bool,
    pub debug_disp_fps: bool,
    /// Game speed multiplier (1.0 = normal speed).
    pub game_speed: f32,
    pub use_special_timing_mode: bool,
    unk2d1: [u8; 3],
    unk2d4: f32,
    pub countdown_timer: f32,
    unk2dc: [u8; 4]
}

// Source of name: Shared FS Ghidra repository
// Source of values: Shared FS Ghidra repository.
// Flipped the FPS mode and the setting description, because enums shouldn't start with numbers.
// A.e "30FPS_VSYNC_ON" -> "VsyncOn30Fps" or "60FPS" -> "Default60Fps".
#[repr(u32)]
pub enum FlipMode {
    VsyncOn30Fps = 0x0,
    Adaptive30Fps = 0x1,
    Default60Fps = 0x2,
    ForcedSync30Fps = 0x3,
    VsyncOff30Fps = 0x4,
    VsyncOn60Fps = 0x5,
    NoSync360Fps = 0x6,
    Adaptive20Fps = 0x7,
    DynamicAdaptive = 0x8,
    Default85Fps = 0x9,
    NoSync85Fps = 0xa,
    Default120Fps = 0xb,
    NoSync120Fps = 0xc,
    PlatformDefault = 0xd
}

// Source of name: Shared FS Ghidra repository
#[repr(C)]
pub struct CSFlipperVsyncHistoryEntry {
    frame_delta_ticks: usize,
    /// Technically a BOOL (u32).
    /// Hence padding for alignment.
    pub vsync_state: bool,
    padding: [u8; 3],
    unkc: [u8; 4],
}
