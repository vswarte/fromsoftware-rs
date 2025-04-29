use std::{num::ParseIntError, ptr::NonNull};

use crate::cs::{ChrAsmArmStyle, ChrAsmEquipment, FaceDataBuffer, MapId};
use crate::dlkr::DLAllocatorBase;
use crate::dltx::DLString;
use crate::fd4::FD4Time;
use crate::{stl::DoublyLinkedList, Tree, Vector};

use shared::FSVector3;

use shared::OwnedPtr;

#[repr(C)]
pub struct CSSosSignMan {
    vftable: usize,
    /// Tree of the sign entries
    pub signs: Tree<SignTreeEntry>,
    /// Tree of sfx's for signs
    pub sign_sfx: Tree<CSSosSignSfx>,
    /// List of signs that were requested to be summoned
    /// Inserting values here will not do anything unless you also have data in `join_data`
    pub summon_requests: DoublyLinkedList<i32>,
    unk50: [u8; 8],
    /// List of data for join push notifications
    pub join_data: DoublyLinkedList<NonNull<PhantomJoinData>>,
    /// Completely unused, no reads or writes other then initialization and destruction
    unk70: DoublyLinkedList<[u8; 0x28]>,
    unk88: [u8; 0x8],
    display_ghost: usize,
    timer: FD4Time,
    /// Param ID for WhiteSignCoolTimeParam, incremented with each level and capped at 10
    pub white_sign_cool_time_param_id: u8,
    // pada9: [u8; 3],
    unkac: u32,
    /// Vector of sign cooldowns from WhiteSignCoolTimeParam
    /// Each time your coop player dies and you have someone in your world
    /// you will get a cooldown depending on WhiteSignCoolTimeParam.
    /// All this cooldowns are stored in this vector.
    pub signs_cooldown: Vector<f32>,
    /// Leftover from Dark Souls 3, never set to true or changed
    /// Source of names: Sekiro debug menu
    pub override_guardian_of_rosalia_count_enabled: bool,
    // padd1: [u8; 3],
    pub override_guardian_of_rosalia_count: u32,
    pub override_map_guardian_count_enabled: bool,
    // padd9: [u8; 3],
    pub override_map_guardian_count: u32,
    pub override_force_join_black_count_enabled: bool,
    // pade1: [u8; 3],
    pub override_force_join_black_count: u32,
    pub override_sinner_hunter_count_enabled: bool,
    // pade9: [u8; 3],
    pub override_sinner_hunter_count: u32,
    pub override_berserker_white_count_enabled: bool,
    // padf1: [u8; 3],
    pub override_berserker_white_count: u32,
    pub override_sinner_hero_count_enabled: bool,
    // padf9: [u8; 3],
    pub override_sinner_hero_count: u32,
    pub override_cult_white_summon_count_enabled: bool,
    // pad101: [u8; 3],
    pub override_cult_white_summon_count: u32,
    pub override_normal_white_count_enabled: bool,
    // pad109: [u8; 3],
    pub override_normal_white_count: u32,
    pub override_red_summon_type_count_enabled: bool,
    // pad111: [u8; 3],
    pub override_red_summon_type_count: u32,
}

#[repr(C)]
pub struct SignTreeEntry {
    pub sign_id: i32,
    // pad4: u32,
    pub sign_data: OwnedPtr<SosSignData>,
}

#[repr(C)]
pub struct CSSosSignSfx {
    pub sign_id: i32,
    // pad4: u32,
    fxhgsfx: usize,
}

#[repr(C)]
pub struct DisplayGhostData {
    /// Param ID for the equipment
    /// See ChrAsmSlot enum to know which slot it is
    pub equipment_param_ids: [i32; 12],
    /// Param ID for the armor
    /// in order: head, chest, arms, legs, unsued
    pub armor_param_ids: [i32; 5],
    unk44: [u8; 4],
    /// Character gender
    pub gender: u8,
    unk49: [u8; 11],
    /// Info about selected slots and one/two handing
    pub asm_equipment: ChrAsmEquipment,
    /// Face data for the ghost
    pub face_data: FaceDataBuffer,
}

#[repr(C)]
pub struct SosSignData {
    pub sign_id: i32,
    // _pad4: [u8; 0x4],
    /// Server-assigned identifier for the sign
    pub sign_identifier: ObjectIdentifier,
    /// Map ID where the sign was placed
    pub map_id: MapId,
    /// Position of the sign (in physics space)
    pub pos: FSVector3,
    /// Rotation of the sign
    pub yaw: f32,
    unk24: [u8; 6],
    /// Covenant level of the sign owner
    pub vow_type: u8,
    unk2b: [u8; 3],
    /// Type of multiplayer
    pub multiplay_type: MultiplayType,
    /// Whether the sign is in the sign pool
    pub is_sign_puddle: bool,
    unk30: u8,
    unk31: u8,
    /// Whether to apply multiplayer rules for summoning frame check
    /// if true, the game will check if the player is allowed to see the signs in the area,
    /// has special effects for the sign, etc.
    pub apply_multiplayer_rules: bool,
    unk33: u8,
    /// Steam ID of the sign owner as a hex string
    /// 0 if the sign is NPC
    pub steam_id: SteamIdStr,
    // _pad56: [u8; 2],
    /// Id of the FMG text entry for npc name
    pub fmg_name_id: i32,
    /// Param ID of the NPC
    pub npc_param_id: i32,
    unk60: [u8; 0x44],
    /// Data for ghost shown when you near the sign
    pub display_ghost: DisplayGhostData,
    /// Entity ID of the NPC
    /// 0 if the sign is not an NPC
    pub summoned_npc_entity_id: u32,
    /// ID of the event flag that will be set when the NPC is summoned
    /// 0 if the sign is not an NPC
    pub summon_event_flag_id: u32,
    /// ID of the event flag that will be set when the NPC sign is dismissed
    /// 0 if the sign is not an NPC
    pub dismissal_event_flag_id: u32,
    /// Player id of the sign owner from the server
    /// 1 if the sign is NPC
    pub summonee_player_id: u32,
    unk244: [u8; 0x4],
    /// Character ID for player-like NPC data
    pub character_id: i32,
    unk2c4: [u8; 4],
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub struct ObjectIdentifier(pub i64);

#[repr(u32)]
pub enum PhantomJoinState {
    /// Push notification sent to other player, awaiting for response
    Waiting = 0,
    /// Player has accepted the join request
    Joining = 1,
}

#[repr(i32)]
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum SummonJobErrorCode {
    /// Default no error state
    Default = -70,
    /// Sign data not found for the given sign ID
    SignDataNotFound = -84,
    /// Sign coordinates are invalid and can't be converted to physics space
    InvalidCoordinates = -83,
}

#[repr(C)]
/// Data used for join push notification
/// This could be a sign, invasion or something else
pub struct PhantomJoinData {
    /// Sign ID if phantom is joining by a sign
    /// -1 if it's invasion or something else
    pub sign_id: i32,
    // _pad4: u32,
    /// Server-assigned identifier for the sign
    pub sign_identifier: ObjectIdentifier,
    /// Time since phantom started joining
    /// if exceeds 55 seconds in `Waiting` state or 180 in `Joining` state,
    /// the join request will be cancelled
    pub join_time: f32,
    /// Multiplay type
    pub multiplay_type: MultiplayType,
    /// Whether the sign is in the sign pool
    pub is_sign_puddle: bool,
    // _pad16: [u8; 2],
    /// State of the joining player
    /// 0 - waiting for response
    /// 1 - joining
    pub state: u32,
    /// Steam ID encoded as hex wide char string with null terminator
    pub steam_id: SteamIdStr,
    // _pad3e: [u8; 0x2],
    /// Entity ID of the NPC
    /// 0 if it's not an NPC
    pub npc_entity_id: u32,
    /// ID of the event flag that will be set when the NPC is summoned
    pub summon_event_flag_id: u32,
    /// ID of the event flag that will be set when the NPC sign is dismissed
    pub dismissal_event_flag_id: u32,
    /// Position where phantom will be summoned (in physics space)
    pub pos: FSVector3,
    /// Rotation for the phantom
    /// This is the same as the sign's rotation if phantom is joining by a sign
    pub rotation: FSVector3,
    pub map_id: MapId,
    /// Player id of the sign owner from the server
    /// 1 if the sign is NPC
    pub summonee_player_id: u32,
    /// Error code in case of failure to join
    pub summon_job_error_code: SummonJobErrorCode,
    /// Whether to apply multiplayer rules for summoning frame check
    /// if true, the game will check if the player is allowed to see the signs in the area,
    /// has special effects for the sign, etc.
    pub apply_multiplayer_rules: bool,
    // _pad71: [u8; 0x7],
}

#[repr(C)]
#[derive(Clone, Copy)]
/// SteamID as a hex wchar string with null terminator
pub struct SteamIdStr(pub [u16; 17]);

impl SteamIdStr {
    pub fn to_u64(&self) -> Result<u64, ParseIntError> {
        let len = self.0.iter().position(|&c| c == 0).unwrap_or(self.0.len());
        let s = String::from_utf16_lossy(&self.0[..len]);
        u64::from_str_radix(&s, 16)
    }
}

impl From<SteamIdStr> for u64 {
    fn from(val: SteamIdStr) -> Self {
        val.to_u64().unwrap_or(0)
    }
}

impl std::fmt::Display for SteamIdStr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_u64().unwrap_or(0))
    }
}

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MultiplayType {
    WhiteSign = 0,
    Invasion = 1,
    RedSign = 2,
    Unk3 = 3,
    Unk4 = 4,
    BerserkerWhite = 5,
    SinnerHero = 6,
    SinnerHunterInvasion = 7,
    BlueHunterSummon = 8,
    RosariaGuardian = 9,
    Unk10 = 10,
    Unk11 = 11,
    Unk12 = 12,
    Unk13 = 13,
    CultWhiteSummon = 14,
    Unk15 = 15,
    Unk16 = 16,
    Unk17 = 17,
    Unk18 = 18,
    Unk19 = 19,
    NpcWhiteSign = 20,
    Unk21 = 21,
    Unk22 = 22,
    NpcInvasion1 = 23,
    Unk24 = 24,
    Unk25 = 25,
    Unk26 = 26,
    AlwaysAllow = 27,
    Unk28 = 28,
    NpcInvasion2 = 29,
    Unk30 = 30,
    None = 31,
}
