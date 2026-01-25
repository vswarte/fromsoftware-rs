use crate::dlut::DLDateTime;

#[repr(C)]
#[shared::singleton("WorldAreaTime")]
pub struct WorldAreaTime {
    pub clock: DLDateTime,
    pub previous_tick_clock: DLDateTime,
    unk20: f32,
    unk24: f32,
    pub target_hour: u32,
    pub target_minute: u32,
    pub target_second: u32,
    unk34: f32,
    pub time_passage_multiplier: f32,
    unk3c: f32,
    // TODO: rest
}

impl WorldAreaTime {
    pub fn request_time(&mut self, hour: u32, minute: u32, second: u32) {
        self.target_hour = hour;
        self.target_minute = minute;
        self.target_second = second;
    }
}

