use std::ops::{Index, IndexMut};

use shared::{CCallback, OwnedPtr};
use vtable_rs::VPtr;

use crate::{Vector, dlkr::DLRunnableVmt};

#[repr(C)]
#[shared::singleton("CSTrophy")]
/// Manages the awarding of achievements.
pub struct CSTrophyImp {
    vtable: isize,
    /// Holds a structure for concrete achievement granting per platform.
    pub trophy_platform: OwnedPtr<CSTrophyPlatformImp_forSteam>,
    unk10: isize,
    unk18: u8,
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

    /// Gives the player an achievement based on the internal achievement ID.
    fn award_achievement(&mut self, achievement_id: &AchievementId);

    fn unk30(&mut self);
}

#[repr(C)]
pub struct CSTrophyTitleInfo {
    pub vftable: VPtr<dyn CSTrophyTitleInfoVmt, Self>,
}

#[vtable_rs::vtable]
pub trait CSTrophyTitleInfoVmt {
    fn destructor(&mut self);

    /// Returns the highest achievement ID. Used for validating requested achievement IDs.
    fn max_achievement_id(&self) -> u32;

    fn unk10(&mut self);

    /// Retrieves the internal name (in JP) for the achievement
    fn achievement_name_for_id(&self, achievement_id: &AchievementId) -> *const u8;

    /// Retrieves the platform-specific achievement key associated with the supplied achievement.
    /// For steam this will be an ascii string which is immediately fed to
    /// `ISteamUserStats::SetAchievement` as pchName.
    fn achievement_key_for_id(&self, achievement_id: &AchievementId) -> *const u8;
}

/// Steam backed implementation for achievement granting.
#[allow(non_camel_case_types)]
#[repr(C)]
pub struct CSTrophyPlatformImp_forSteam {
    pub base: CSTrophyPlatformImp,
    /// AppId these achievements are for
    pub steam_app_id: isize,
    pub achievements: OwnedPtr<[CSTrophyPlatformImp_forSteamAchievementItem; 42]>,
    /// Amount of the achievements this player unlocked
    pub unlocked_count: u32,
    /// Whether achievement info is initialized or not
    pub is_initialized: u8,
    /// Whether "master" achievement was unlocked or not
    pub is_master_unlocked: bool,
    pub on_user_stats_received_cb: CCallback<Self, [u8; 0x18]>,
    pub on_user_stats_stored_cb: CCallback<Self, [u8; 0x10]>,
    pub on_user_achievement_stored_cb: CCallback<Self, [u8; 0x98]>,
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
    /// The achievement's title as shown on the Steam profile achievements page.
    pub fn title(&self) -> String {
        let length = self
            .title
            .iter()
            .position(|c| *c == 0)
            .unwrap_or(self.title.len());

        String::from_utf16(&self.title[..length]).unwrap()
    }
}

impl Index<AchievementId> for [CSTrophyPlatformImp_forSteamAchievementItem; 42] {
    type Output = CSTrophyPlatformImp_forSteamAchievementItem;

    fn index(&self, index: AchievementId) -> &Self::Output {
        &self[index as usize]
    }
}
impl IndexMut<AchievementId> for [CSTrophyPlatformImp_forSteamAchievementItem; 42] {
    fn index_mut(&mut self, index: AchievementId) -> &mut Self::Output {
        &mut self[index as usize]
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AchievementId {
    EldenRing = 0,                          // エルデンリング
    EldenLord = 1,                          // エルデの王
    AgeOfTheStars = 2,                      // 星の世紀
    LordOfFrenziedFlame = 3,                // 狂い火の王
    ShardbearerGodrick = 4,                 // 破片の君主、ゴドリック
    ShardbearerRadahn = 5,                  // 破片の君主、ラダーン
    ShardbearerMorgott = 6,                 // 破片の君主、モーゴット
    ShardbearerRykard = 7,                  // 破片の君主、ライカード
    ShardbearerMalenia = 8,                 // 破片の君主、マレニア
    ShardbearerMohg = 9,                    // 破片の君主、モーグ
    MalikethTheBlackBlade = 10,             // 黒き剣のマリケス
    HoarahLouxTheWarrior = 11,              // 戦士、ホーラ・ルー
    DragonlordPlacidusax = 12,              // 竜王プラキドサクス
    GodSlayingArmament = 13,                // 神を殺す武器
    LegendaryArmaments = 14,                // 伝説の武器
    LegendaryAshenRemains = 15,             // 伝説の霊体
    LegendarySorceriesAndIncantations = 16, // 伝説の魔術／祈祷
    LegendaryTalismans = 17,                // 伝説のタリスマン
    RennalaQueenOfTheFullMoon = 18,         // 満月の女王、レナラ
    LichdragonFortissax = 19,               // 死竜フォルサクス
    GodskinDuo = 20,                        // 神肌のふたり
    FireGiant = 21,                         // 火の巨人
    DragonkinSoldierOfNokstella = 22,       // 竜人兵
    RegalAncestorSpirit = 23,               // 祖霊の王
    ValiantGargoyle = 24,                   // 英雄のガーゴイル
    MargitTheFellOmen = 25,                 // 忌み鬼、マルギット
    RedWolfOfRadagon = 26,                  // ラダゴンの赤狼
    GodskinNoble = 27,                      // 神肌の貴種
    MagmaWyrmMakar = 28,                    // 溶岩土竜
    GodfreyTheFirstLord = 29,               // 最初の王、ゴッドフレイ
    MohgTheOmen = 30,                       // 忌み鬼、モーグ
    MimicTear = 31,                         // 写し身の雫
    LorettaKnightOfTheHaligtree = 32,       // アーバーガード (Arbor Guard)
    AstelNaturalbornOfTheVoid = 33,         // 暗黒の落とし子、アステール
    LeonineMisbegotten = 34,                // 獅子の混種
    RoyalKnightLoretta = 35,                // カーリアの親衛騎士
    ElemerOfTheBriar = 36,                  // 鉄茨のエレメール
    AncestorSpirit = 37,                    // 祖霊
    CommanderNiall = 38,                    // 宿将二アール
    RoundtableHold = 39,                    // 円卓
    GreatRune = 40,                         // 大ルーン
    ErdtreeAflame = 41,                     // 燃える黄金樹
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x20, size_of::<CSTrophyImp>());
        assert_eq!(0x38, size_of::<CSTrophyPlatformImp>());
        assert_eq!(0x8, size_of::<CSTrophyTitleInfo>());
        assert_eq!(0xb0, size_of::<CSTrophyPlatformImp_forSteam>());
    }
}
