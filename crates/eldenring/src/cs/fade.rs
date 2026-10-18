use std::ops::{Index, IndexMut};

use crate::{dlkr::MainHeapAllocator, dltx::DLString, fd4::FD4Time};
use shared::{F32Vector4, OwnedPtr};

#[repr(C)]
/// Controls fades in the game. Used for cutscene transitions and such.
///
/// Source of name: RTTI
#[shared::singleton("CSFade")]
pub struct CSFade {
    vftable: usize,
    pub fade_system: OwnedPtr<CSFD4FadeSystem>,
    /// Holds the individual fade plates, these control the actual drawing of the dimming.
    pub fade_plates: [OwnedPtr<CSFD4FadePlate, MainHeapAllocator>; 9],
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
    pub title: DLString,
    unk98: F32Vector4,
    unka8: FD4Time,
    unkb8: u8,
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

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FadePlateId {
    Title = 0,
    MapIn = 1,
    InGame = 2,
    Cutscene = 3,
    InCutscene = 4,
    Ending = 5,
    Event = 6,
    InGameChroma = 7,
    InGameBokeh = 8,
}

impl Index<FadePlateId> for [OwnedPtr<CSFD4FadePlate, MainHeapAllocator>; 9] {
    type Output = OwnedPtr<CSFD4FadePlate, MainHeapAllocator>;

    fn index(&self, index: FadePlateId) -> &Self::Output {
        &self[index as usize]
    }
}

impl IndexMut<FadePlateId> for [OwnedPtr<CSFD4FadePlate, MainHeapAllocator>; 9] {
    fn index_mut(&mut self, index: FadePlateId) -> &mut Self::Output {
        &mut self[index as usize]
    }
}
