use std::time::{Duration, Instant};

pub const JUMP_TIME_FRAME: Duration = Duration::from_millis(350);

#[repr(C)]
pub struct WallJumpManager {
    pub window_enter_time: Instant,
    pub has_entered_window: bool,
    pub has_jumped: bool,
}

impl WallJumpManager {
    pub fn new() -> Self {
        WallJumpManager {
            window_enter_time: Instant::now(),
            has_jumped: true,
            has_entered_window: false,
        }
    }
}