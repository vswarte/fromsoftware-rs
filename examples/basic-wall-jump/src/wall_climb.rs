use std::time::{Duration, Instant};

pub const JUMP_TIME_FRAME: Duration = Duration::from_millis(350);

#[repr(C)]
pub struct WallJumpManager {
    /// Tracks the time for the jump window.
    /// This is updated when "has_entered_window" is set to true.
    pub window_enter_time: Instant,
    /// Tracks if we entered the time window to jump.
    pub has_entered_window: bool,
    /// Represents wether a wall-jump was activated
    /// You cannot do another untill the `is_jumping` field in the ChrIns became false.
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
