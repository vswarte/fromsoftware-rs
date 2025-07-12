use super::FieldInsHandle;

#[repr(u32)]
pub enum MemberType {
    Host = 0,
    RemotePlayer = 1,
    Npc = 2,
}

#[repr(u32)]
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
    pub apply_multiplayer_rules: u8,
    unk1e: u8,
    unk1f: u8,
    pub chr_type: u32,
    pub team_type: u8,
    unk25: u8,
    unk26: u8,
    unk27: u8,
    pub npc_name_fmg_id: u32,
    unk2c: u8,
    unk2d: u8,
    unk2e: u8,
    unk2f: u8,
}

#[repr(C)]
pub struct PartyMemberInfo {
    vftable: usize,
    pub white_phantom_count: i32,
    pub red_phantom_count: i32,
    /// all loaded players without npc
    pub in_world_online_player_count: i32,
    /// all loaded players including npc
    pub in_world_players_count: i32,
    /// same as loaded_online_player_count
    pub non_npc_player_count: i32,
    /// in session player count including npc
    pub session_player_count: i32,
    /// in session player count excluding npc
    pub session_online_player_count: i32,
    unk24: u8,
    unk25: u8,
    unk26: u8,
    unk27: u8,
    pub party_members: [PartyMemberInfoEntry; 6],
    pub npc_host_entities: [FieldInsHandle; 5],
    pub npc_host_entity_count: i32,
    pub ceremony_state: i32,
    pub ceremony_host_entity_id: u32,
    pub npc_invasion_event_flag: u32,
    unk180: u32,
    unk184: u8,
    unk185: u8,
    unk186: u8,
    unk187: u8,
    unk188: i32,
    unk18c: u8,
    unk18d: u8,
    unk18e: u8,
    unk18f: u8,
    unk190: i32,
    unk194: i32,
    unk198: i32,
    unk19c: u8,
    unk19d: u8,
    unk19e: u8,
    unk19f: u8,
    pub needs_update: u8,
    unk1a1: u8,
    unk1a2: u8,
    unk1a3: u8,
    unk1a4: u8,
    unk1a5: u8,
    unk1a6: u8,
    unk1a7: u8,
}
