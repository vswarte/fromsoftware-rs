use std::ops::Mul;

use crate::cs::{BlockId, ChrType, MultiplayRole, SummonParamType};

use super::FieldInsHandle;

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemberType {
    Host = 0,
    RemotePlayer = 1,
    Npc = 2,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PartyMemberEntryState {
    HostDefault = 0,
    Unk1 = 1,
    Unk2 = 2,
    Unk3 = 3,
    RemotePlayerDefault = 4,
    Dead = 5,
    DisconnectRequest = 6,
    DisconnectWait = 7,
    Unk8 = 8,
    Unk9 = 9,
}

#[repr(C)]
pub struct PartyMemberInfoEntry {
    pub field_ins_handle: FieldInsHandle,
    pub member_type: MemberType,
    pub state: PartyMemberEntryState,
    /// Event flag ID for the npc's invasion event
    pub npc_invasion_event_flag: u32,
    /// Event flag ID for the npc's return event
    pub npc_return_event_flag_id: u32,
    /// Time since the player was asked to leave the session
    pub disconnect_request_delta_time: f32,
    unk1c: u8,
    /// Whether the player should be considered for multiplayer rules
    /// eg. invader sent home when hosts starts a boss fight
    pub apply_multiplayer_rules: bool,
    unk1e: u8,
    unk1f: u8,
    /// ChrType to use for the npc member
    pub npc_chr_type: ChrType,
    /// MultiplayRole to use for the npc member
    pub npc_multiplay_role: MultiplayRole,
    unk25: u8,
    unk26: u8,
    unk27: u8,
    pub npc_name_fmg_id: u32,
    unk2c: u8,
    unk2d: u8,
    unk2e: u8,
    unk2f: u8,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CeremonyState {
    Inactive = 0,
    Requested = 1,
    Loading = 2,
    Active = 3,
}

#[repr(C)]
pub struct PartyMemberInfo {
    vftable: usize,
    /// Number of loaded characters considering to be friendly phantoms
    ///
    /// See [`crate::cs::CharacterTypePropertiesEntry::is_friendly_phantom`]
    pub friendly_phantom_count: i32,
    /// Number of loaded characters considering to be hostile phantoms by their character type
    ///
    /// See [`crate::cs::CharacterTypePropertiesEntry::is_hostile_phantom`]
    pub hostile_phantom_count: i32,
    /// all loaded players without npc
    pub in_world_online_player_count: i32,
    /// all loaded players including npc
    pub in_world_players_count: i32,
    /// same as loaded_online_player_count
    pub non_npc_player_count: i32,
    /// all players including npc
    pub all_players_count: i32,
    /// in session player count excluding npc
    pub session_online_player_count: i32,
    unk24: u8,
    unk25: u8,
    unk26: u8,
    unk27: u8,
    pub party_members: [PartyMemberInfoEntry; 6],
    pub npc_host_entities: [FieldInsHandle; 5],
    pub npc_host_entity_count: u32,
    pub pseudo_mp_ceremony_state: CeremonyState,
    pub pseudo_mp_host_entity_id: u32,
    /// Used in pseudo multiplayer
    pub pseudo_mp_event_flag: u32,
    /// Host entity ID + 10000?
    pseudo_mp_event_flag_unk180: i32,
    /// Used to determine the message contents before hitting the loading screen.
    pub pseudo_mp_event_text_for_map_id: i32,
    /// Summon param type of current player in multiplayer session
    pub summon_param_type: SummonParamType,
    /// ID of a NPC (1-21) to use when reading field from [crate::param::NETWORK_MSG_PARAM_ST]
    pub pseudo_mp_network_msg_npc_id: i8,
    /// Default ceremony role param override in ceremony based on
    /// PseudoMultiplayer event point in MSB
    pub pseudo_mp_role_param_override: i32,
    /// Host ceremony role param when in a multiplayer ceremony
    pub pseudo_mp_role_param_override_host: i32,
    /// Guest ceremony role param when in a multiplayer ceremony
    pub pseudo_mp_role_param_override_guest: i32,
    /// Multiplay role that will be assigned to the host in a ceremony
    pub pseudo_mp_role_host: MultiplayRole,
    /// Multiplay role that will be assigned to the guest in a ceremony
    pub pseudo_mp_role_guest: MultiplayRole,
    unk19e: u8,
    unk19f: u8,
    pub needs_update: bool,
    unk1a1: u8,
    unk1a2: u8,
    unk1a3: u8,
    unk1a4: u8,
    unk1a5: u8,
    unk1a6: u8,
    unk1a7: u8,
}
