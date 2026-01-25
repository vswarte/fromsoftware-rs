#[repr(C)]
pub struct GameRend {
    unk0: [u8; 0xa4],
    // 0 = inactive, 1 = active and paused, 2 = active and running world, 3 = stationary
    pub freecam_mode: FreecamMode,
    // TODO: rest
}

#[derive(PartialEq)]
#[repr(u8)]
pub enum FreecamMode {
    Inactive = 0x0,
    ActivePaused = 0x1,
    Active = 0x2,
    Stationary = 0x3,
}
