pub struct EventFlag(u32);

impl EventFlag {
    pub fn world(&self) -> u8 {
        ((self.0 / 10000000) % 10) as u8
    }

    pub fn area(&self) -> u8 {
        ((self.0 / 100000) % 100) as u8
    }

    pub fn group(&self) -> u8 {
        ((self.0 / 10000) % 10) as u8
    }

    pub fn zone(&self) -> u8 {
        ((self.0 / 1000) % 10) as u8
    }

    pub fn is_valid(&self) -> bool {
        self.area() < 90
    }

    pub fn global_index(&self) -> Option<u8> {
        Some(match (self.area(), self.group()) {
            (12, 1) => 0,
            (20, 1) => 1,
            (21, 0) => 3,
            (29, 0) => 4,
            (29, 1) => 5,
            (29, 2) => 6,
            _ => return None
        })
    }
}

impl From<u32> for EventFlag {
    fn from(value: u32) -> Self {
        EventFlag(value)
    }
}
