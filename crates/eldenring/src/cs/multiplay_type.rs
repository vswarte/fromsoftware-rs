use crate::cs::{ChrType, FullScreenMessage};
use bitfield::bitfield;
use shared::FromStatic;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Main multiplay-related types controlling what kind of [ChrType],
/// [MultiplayRole] and [SummonParamType] a character is treated as in multiplayer sessions.
/// Used as index in [MultiplayProperties] that contains all this info.
///
/// Source of names: [MultiplayPropertyEntry::debug_name]
pub enum MultiplayType {
    /// 白召喚
    WhiteSummon = 0,
    /// Invader type A (Bloody Finger)
    /// 乱入赤_A
    RedInvasionA = 1,
    /// Duelist
    /// 赤召喚
    RedSummon = 2,
    /// Invader type A Limited (Festering Bloody Finger)
    /// 乱入赤_A_有限
    RedInvasionALimited = 3,
    /// Invader type B (Recusant)
    /// 乱入赤_B
    RedInvasionB = 4,
    /// DS3 Mad Phantom (White Sign)
    /// バーサーカー白
    BerserkerWhite = 5,
    /// 罪人英雄白
    SinnerHeroWhite = 6,
    /// 罪人狩り
    SinnerHunt = 7,
    /// Blue hunter
    /// 赤狩り
    RedHunt = 8,
    /// DS3 Rosaria's Fingers
    /// ロザリア守護
    RosariaGuardian = 9,
    /// DS3 Watchdogs of Farron
    /// 森マップ守護
    ForestMapGuardian = 10,
    /// アノールマップ守護
    AnorMapGuardian = 11,
    /// アバター戦
    AvatarBattle = 12,
    /// Quickmatch Arena
    /// バトルロイヤル戦
    BattleRoyale = 13,
    /// 儀式召喚
    RitualSummon = 14,
    /// DS3 Warrior of Sunlight (White Sign)
    /// 太陽霊（白サイン）
    SunSpiritWhiteSign = 15,
    /// DS3 Warrior of Sunlight (Red Sign)
    /// 太陽霊（赤サイン）
    SunSpiritRedSign = 16,
    /// DS3 Mad Phantom (Red Sign)
    /// バーサーカー霊（赤サイン）
    BerserkerSpiritRedSign = 17,
    /// DS3 Warrior of Sunlight (Invasion)
    /// 太陽霊（乱入）
    SunSpiritInvasion = 18,
    /// DS3 Mad Phantom (Invasion)
    /// バーサーカー霊（乱入）
    BerserkerSpiritInvasion = 19,
    /// 白召喚_NPC
    WhiteSummonNpc = 20,
    /// 乱入赤_A_NPC
    RedInvasionANpc = 21,
    /// 乱入赤_B_NPC
    RedInvasionBNpc = 22,
    /// 乱入赤_C_NPC
    RedInvasionCNpc = 23,
    /// NPC擬似マルチ用白霊
    NpcPseudoWhiteSpirit = 24,
    /// NPC擬似マルチ用侵入_A
    NpcPseudoInvasionA = 25,
    /// NPC擬似マルチ用侵入_B
    NpcPseudoInvasionB = 26,
    /// NPC擬似マルチ用幻視イベント
    NpcPseudoPhantasmEvent = 27,
    /// 赤狩り2
    RedHunt2 = 28,
    /// NPC擬似マルチ用侵入_セッションホスト
    NpcPseudoInvasionSessionHost = 29,
    /// NPC擬似マルチ用侵入_セッションゲスト
    NpcPseudoInvasionSessionGuest = 30,
    None = 31,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Source of names:
/// [MultiplayType]
pub enum MultiplayRole {
    Host = 0,
    WhiteSummon = 1,
    RedSummon = 2,
    RedInvasionA = 3,
    RedInvasionALimited = 4,
    RedInvasionB = 5,
    BerserkerWhite = 6,
    RedHunt = 7,
    SinnerHeroWhite = 8,
    SinnerHunt = 9,
    RosariaGuardian = 10,
    ForestMapGuardian = 11,
    AnorMapGuardian = 12,
    AvatarBattle = 13,
    BattleRoyale = 14,
    RitualSummon = 15,
    SunSpiritWhiteSign = 16,
    SunSpiritRedSign = 17,
    BerserkerSpiritRedSign = 18,
    SunSpiritInvasion = 19,
    BerserkerSpiritInvasion = 20,
    WhiteSummonNpc = 21,
    RedInvasionANpc = 22,
    RedInvasionBNpc = 23,
    RedInvasionCNpc = 24,
    NpcPseudoWhiteSpirit = 25,
    NpcPseudoInvasionA = 26,
    NpcPseudoInvasionB = 27,
    NpcPseudoPhantasmEvent = 28,
    RedHunt2 = 29,
    NpcPseudoInvasionSessionHost = 30,
    NpcPseudoInvasionSessionGuest = 31,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Source of names:
/// [MultiplayType]
pub enum SummonParamType {
    NpcPseudoInvasionSessionGuest = -30,
    NpcPseudoInvasionSessionHost = -29,
    RedHunt2 = -28,
    NpcPseudoPhantasmEvent = -27,
    NpcPseudoInvasionB = -26,
    NpcPseudoInvasionA = -25,
    NpcPseudoWhiteSpirit = -24,
    RedInvasionCNpc = -23,
    RedInvasionBNpc = -22,
    RedInvasionANpc = -21,
    WhiteSummonNpc = -20,
    BerserkerSpiritInvasion = -19,
    SunSpiritInvasion = -18,
    BerserkerSpiritRedSign = -17,
    SunSpiritRedSign = -16,
    SunSpiritWhiteSign = -15,
    BattleRoyale = -14,
    AvatarBattle = -13,
    AnorMapGuardian = -12,
    ForestMapGuardian = -11,
    RosariaGuardian = -10,
    RedHunt = -9,
    SinnerHunt = -8,
    BerserkerWhite = -6,
    RedInvasionB = -5,
    RedInvasionALimited = -4,
    RedInvasionA = -3,
    RedSummon = -2,
    Summon = -1,
    Host = 0,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum JoinType {
    /// Host need approval for phantom to join as friendly phantom
    FriendlySign = 0,
    /// Host need approval for phantom to join as hostile phantom
    HostileSign = 1,
    /// No explicit approval needed, join regardless.
    /// Used for Hunters/Invaders/Area Defenders, etc.
    ForceJoin = 3,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MatchingCooldownType {
    None = -1,
    Invasion = 0,
    ForestMapInvasion = 1,
    AnorMapInvasion = 2,
    BlueHunter = 3,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MultiplayPropertyEntryFlags(u32);
    impl Debug;
    /// Whether this multiplayer type ignores network penalty when summoning/invading
    pub ignore_net_penalty, set_ignore_net_penalty: 3;
}

#[repr(C)]
pub struct MultiplayPropertyEntry {
    unk0: i32,
    /// SummonParamType associated with this MultiplayType
    pub summon_param_type: SummonParamType,
    /// ChrType to use for this multiplayer type
    pub chr_type: ChrType,
    /// [MultiplayRole] to use for this multiplayer type
    pub multiplay_role: MultiplayRole,
    /// Type of joining method used for this multiplayer type
    pub join_type: JoinType,
    unk14: i32,
    unk18: i32,
    unk1c: i32,
    unk20: i32,
    /// Type of cooldown used to limit this type of multiplayer type
    pub matching_cooldown_type: MatchingCooldownType,
    /// FMG ID for the message shown when interacting with this multiplay type's sign
    pub other_sign_interaction_fmg_id: i32,
    /// FMG ID for the message shown when interacting with own sign of this multiplay type
    pub self_sign_interaction_fmg_id: i32,
    /// FullScreenMessage shown when a character of this multiplayer type is killed
    pub kill_full_screen_message: FullScreenMessage,
    /// Different properties related to this multiplayer type
    pub flags: MultiplayPropertyEntryFlags,
    /// Debug name for this multiplayer type
    pub debug_name: *const u16,
}

#[repr(C)]
/// Container for all [MultiplayPropertyEntry] entries, indexed by [MultiplayType]
///
/// Used to lookup what [ChrType], [MultiplayRole] and [SummonParamType]
/// a character should be treated as in multiplayer sessions.
/// Additionally contains various other properties related to each multiplayer type.
pub struct MultiplayProperties {
    pub entries: [MultiplayPropertyEntry; 31],
}

impl FromStatic for MultiplayProperties {
    unsafe fn instance() -> shared::InstanceResult<&'static mut Self> {
        use crate::rva;
        use pelite::pe64::Pe;
        use shared::Program;

        let target = Program::current()
            .rva_to_va(rva::get().multiplay_properties)
            .map_err(|_| shared::InstanceError::NotFound)?
            as *mut MultiplayProperties;

        Ok(&mut *target)
    }
}
