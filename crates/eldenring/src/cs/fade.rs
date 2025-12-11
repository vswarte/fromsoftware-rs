use crate::fd4::FD4Time;
use shared::OwnedPtr;

#[repr(C)]
/// Controls fades in the game. Used for cutscene transitions and such.
///
/// Source of name: RTTI
#[shared::singleton("CSFade")]
pub struct CSFade {
    vftable: usize,
    pub fade_system: OwnedPtr<CSFD4FadeSystem>,
    /// Holds the individual fade plates, these control the actual drawing of the dimming.
    pub fade_plates: [OwnedPtr<CSFD4FadePlate>; 9],
    unk58: u32,
    unk5c: f32,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSFD4FadeSystem {
    vftable: usize,
}

#[repr(C)]
/// A fade plate
///
/// Source of name: RTTI
pub struct CSFD4FadePlate {
    vftable: usize,
    pub reference_count: u32,
    _padc: u32,
    /// Stores the currently interpolated color.
    pub current_color: CSFD4FadePlateColor,
    /// Stores the color we're transitioning away from.
    pub start_color: CSFD4FadePlateColor,
    /// Stores the color we're transitioning towards.
    pub end_color: CSFD4FadePlateColor,
    /// Stores the amount of seconds pending until the LERP to end_color is finished.
    pub fade_timer: FD4Time,
    /// Stores the time a transition to the target color should take in total.
    pub fade_duration: FD4Time,
    unk60: u8,
    _pad64: [u8; 7],
    allocator: usize,
    pub title: [u16; 8],
    unk80: u64,
    unk88: u64,
    unk90: u64,
    unk98: u64,
    unka0: u64,
    unka8: FD4Time,
    unkb8: u64,
}

impl CSFD4FadePlate {
    pub fn fade_in(&mut self, time: f32) {
        self.end_color.a = 0.0;
        self.start_color.a = 1.0;
        self.fade_duration.time = time;
        self.fade_timer.time = time;
    }

    pub fn fade_out(&mut self, time: f32) {
        self.end_color.a = 1.0;
        self.start_color.a = 0.0;
        self.fade_duration.time = time;
        self.fade_timer.time = time;
    }
}

#[repr(C)]
pub struct CSFD4FadePlateColor {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl From<&CSFD4FadePlateColor> for [f32; 4] {
    fn from(val: &CSFD4FadePlateColor) -> Self {
        [val.r, val.g, val.b, val.a]
    }
}

impl From<[f32; 4]> for CSFD4FadePlateColor {
    fn from(val: [f32; 4]) -> Self {
        Self {
            r: val[0],
            g: val[1],
            b: val[2],
            a: val[3],
        }
    }
}
