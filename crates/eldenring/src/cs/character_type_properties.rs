#[repr(C)]
pub struct CharacterTypePropertiesEntry {
    unk0: u8,
    /// Controls whether the character type can execute some TAE or HKS.
    pub disable_behavior: bool,
    /// Controls whether the character type can pick up items.
    pub can_use_item_lots: bool,
    /// Controls whether sound location should be based on the character's position instead of the camera's.
    pub use_chr_based_sound_location: bool,
    unk4: u8,
    unk5: u8,
    unk6: u8,
    /// Controls whether the character type can use the rune arc.
    pub can_use_rune_arcs: bool,
    /// Controls whether the character type can receive buffs when
    /// their message is rated.
    pub can_receive_message_rate_buff: bool,
    /// Controls whether the character type count toward the
    /// number of friendly phantoms.
    pub is_white_phantom: bool,
    /// Controls whether the character type count toward the
    /// number of hostile phantoms.
    pub is_black_phantom: bool,
    unkb: u8,
    unkc: i32,
    unk10: i32,
}

#[repr(C)]
pub struct CharacterTypePropertiesTable {
    pub entries: [CharacterTypePropertiesEntry; 22],
    pub default: CharacterTypePropertiesEntry,
}
