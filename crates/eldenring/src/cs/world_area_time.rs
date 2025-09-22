use crate::dlut::DLDateTime;

#[repr(C)]
#[dlrf::singleton("WorldAreaTime")]
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
    fn request_time(&mut self, hour: u32, minute: u32, second: u32) {
        self.target_hour = hour;
        self.target_minute = minute;
        self.target_second = second;
    }
}

#[repr(u32)]
pub enum AiSightTimeOfDay {
    Morning = 0x0,
    Noon = 0x1,
    Evening = 0x2,
    Night = 0x3,
    Midnight = 0x4,
}
