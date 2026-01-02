use shared::{CCallback, OwnedPtr};
use vtable_rs::VPtr;

use crate::{dlkr::DLRunnableVmt, Vector};

#[repr(C)]
#[shared::singleton("CSTrophy")]
/// Manages the awarding of achievements.
pub struct CSTrophy {
    vtable: isize,
    /// Holds a structure for concrete achievement granting per platform.
    pub trophy_platform: OwnedPtr<CSTrophyPlatformImp_forSteam>,
    unk10: isize,
    unk18: isize,
}

#[repr(C)]
pub struct CSTrophyPlatformImp {
    vftable: VPtr<dyn CSTrophyPlatformImpVmt, Self>,
    /// Game specific achievement data.
    pub trophy_title_info: OwnedPtr<CSTrophyTitleInfo>,
    /// Seems to be related to some debug features.
    unk10: Vector<()>,
    unk30: isize,
}

#[vtable_rs::vtable]
trait CSTrophyPlatformImpVmt: DLRunnableVmt {
    fn unk10(&mut self);

    fn unk18(&mut self);

    fn unk20(&mut self);

    fn award_achievement(&mut self, achievement: &u32);

    fn unk30(&mut self);
}

#[repr(C)]
pub struct CSTrophyTitleInfo {
    pub vftable: VPtr<dyn CSTrophyTitleInfoVmt, Self>,
}

#[vtable_rs::vtable]
pub trait CSTrophyTitleInfoVmt {
    fn destructor(&mut self);

    /// Returns the highest achievement ID.
    fn max_achievement_id(&self) -> u32;

    fn unk10(&mut self);

    fn unk18(&mut self);

    /// Retrieves the platform-specific achievement key associated with the supplied achievement.
    /// For steam this will be an ascii string which is immediately fed to
    /// `ISteamUserStats::SetAchievement` as pchName.
    fn achievement_key_for_id(&self, achievement_id: &u32) -> *const u8;
}

/// Steam backed implement for achievement granting.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct CSTrophyPlatformImp_forSteam {
    pub base: CSTrophyPlatformImp,
    unk38: isize,
    pub achievements: OwnedPtr<[CSTrophyPlatformImp_forSteamAchievementItem; 42]>,
    unk48: u32,
    unk4c: u8,
    unk4d: u8,
    unk50: CCallback,
    unk70: CCallback,
    unk90: CCallback,
}

/// Steam backed implement for achievement granting.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct CSTrophyPlatformImp_forSteamAchievementItem {
    /// Has the player unlocked the achievement?
    pub unlocked: bool,
    /// Bytes holding the title of the achievement.
    title: [u16; 0x80],
}

impl CSTrophyPlatformImp_forSteamAchievementItem {
    pub fn title(&self) -> String {
        let length = self
            .title
            .iter()
            .position(|c| *c == 0)
            .unwrap_or(self.title.len());

        String::from_utf16(&self.title[..length]).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x20, size_of::<CSTrophy>());
        assert_eq!(0x38, size_of::<CSTrophyPlatformImp>());
        assert_eq!(0x8, size_of::<CSTrophyTitleInfo>());
        assert_eq!(0xb0, size_of::<CSTrophyPlatformImp_forSteam>());
    }
}
