use shared::FromStatic;

use crate::cs::ChrType;

#[repr(C)]
pub struct CharacterTypePropertiesEntry {
    /// Controls whether the character type needs to be included in AI target searches.
    /// For example, will be disabled for ghosts.
    pub include_in_ai_target_search: bool,
    /// Controls whether the character type can execute some TAE or HKS.
    pub disable_behavior: bool,
    /// Controls whether the character type can pick up items.
    pub can_use_item_lots: bool,
    /// Controls whether sound location should be based on the character's position instead of the camera's.
    pub use_chr_based_sound_location: bool,
    unk4: u8,
    unk5: u8,
    unk6: u8,
    /// Controls whether the character type considered to be a host-like player.
    /// For example, allows using rune arcs and interacting with NPC signs.
    pub is_host_like: bool,
    /// Controls whether the character type can receive buffs when
    /// their message is rated.
    pub can_receive_message_rate_buff: bool,
    /// Controls whether the character type count toward the
    /// number of friendly phantoms.
    ///
    /// See [`crate::cs::PartyMemberInfo::friendly_phantom_count`]
    pub is_friendly_phantom: bool,
    /// Controls whether the character type count toward the
    /// number of hostile phantoms.
    ///
    /// See [`crate::cs::PartyMemberInfo::hostile_phantom_count`]
    pub is_hostile_phantom: bool,
    unkb: u8,
    /// [DS3] Controls the character type assigned to the character after revival logic.
    /// Doesn't do anything in Elden Ring.
    pub post_revival_character_type: ChrType,
    unk10: i32,
}

#[repr(C)]
pub struct CharacterTypePropertiesTable {
    pub entries: [CharacterTypePropertiesEntry; 22],
    pub default: CharacterTypePropertiesEntry,
}

impl FromStatic for CharacterTypePropertiesTable {
    fn name() -> String {
        "CharacterTypePropertiesTable".to_string()
    }

    unsafe fn instance() -> shared::InstanceResult<&'static mut Self> {
        use crate::rva;
        use pelite::pe64::Pe;
        use shared::Program;

        let target = Program::current()
            .rva_to_va(rva::get().character_type_properties)
            .map_err(|_| shared::InstanceError::NotFound)?
            as *mut CharacterTypePropertiesTable;

        unsafe { Ok(&mut *target) }
    }
}
