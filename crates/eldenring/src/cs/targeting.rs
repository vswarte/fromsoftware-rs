use std::ptr::NonNull;
use vtable_rs::VPtr;

use crate::position::HavokPosition;
use crate::rotation::Quaternion;
use shared::F32Vector4;

use super::{CSBulletIns, ChrIns, FieldInsHandle, SpecialEffect};

#[repr(C)]
pub struct NpcThinkParamLookupResult {
    pub row_id: u32,
    pub param_row: usize,
    pub battle_goal_id: i32,
    pub logic_id: i32,
}

#[vtable_rs::vtable]
pub trait CSTargetingSystemOwnerVmt {
    fn destructor(&mut self, should_free: bool);

    fn get_team_type<'a>(&self, out: &'a mut i8) -> &'a mut i8;

    fn get_position<'a>(&self, out: &'a mut HavokPosition) -> &'a mut HavokPosition;

    fn get_target_ene0_position<'a>(&self, out: &'a mut HavokPosition) -> &'a mut HavokPosition;

    /// The point on the hit capsule that is in front of the owner.
    ///
    /// = position + forward * hit_radius
    ///
    /// For bullets: uses bullet's position and hit radius but chr owner's forward
    fn get_outmost_forward_position<'a>(&self, out: &'a mut HavokPosition)
    -> &'a mut HavokPosition;

    /// The point on the hit capsule that is in front of the owner.
    ///
    /// = position + forward * hit_radius + (0, hit_height)
    ///
    /// For bullets: uses bullet's position and hit radius but chr owner's forward
    fn get_outmost_forward_position_height_offset<'a>(
        &self,
        out: &'a mut HavokPosition,
    ) -> &'a mut HavokPosition;

    fn get_orientation<'a>(&self, out: &'a mut Quaternion) -> &'a mut Quaternion;

    fn get_forward<'a>(&self, out: &'a mut F32Vector4) -> &'a mut F32Vector4;

    fn get_forward2<'a>(&self, out: &'a mut F32Vector4) -> &'a mut F32Vector4;

    fn get_hit_height(&self) -> f32;

    fn get_hit_radius(&self) -> f32;

    fn get_npc_think_entry(&self) -> &NpcThinkParamLookupResult;

    fn is_black_phantom(&self) -> bool;

    fn is_on_solid_ground(&self) -> bool;

    fn is_battle_state(&self) -> bool;

    fn is_not_in_any_search_state(&self) -> bool;

    fn is_ignore_effect_hear_modifiers(&self) -> bool;

    fn is_ignore_effect_sight_modifiers(&self) -> bool;

    fn get_ignore_fake_target_flags(&self) -> u8;

    fn get_owner_handle<'a>(&self, out: &'a mut FieldInsHandle) -> &'a mut FieldInsHandle;

    fn unka0(&mut self, param_2: usize);

    fn get_team_ai(&self) -> usize;

    fn get_unk_team_ai_struct(&self) -> usize;

    fn is_climbing_ladder(&self) -> bool;

    fn is_target_ene0_on_ladder(&self) -> bool;

    fn unkc8(&mut self) -> usize;

    fn get_sight_search_modifiers(
        &self,
        rate_out: &mut f32,
        add_out: &mut f32,
        is_nonpositive_rate_out: &mut bool,
    );

    fn get_hearing_search_rate(&self) -> f32;

    fn get_hearing_search_add(&self) -> f32;

    fn get_hearing_sound_level_overwrite(&self) -> i32;

    fn get_special_effect(&mut self) -> &mut SpecialEffect;

    fn unkf8(&self) -> f32;

    fn unk100(&self) -> u8;

    fn unk108(&self) -> u8;

    fn unk110<'a>(&self, out: &'a mut F32Vector4) -> &'a mut F32Vector4;

    fn get_hearing_head_size(&self) -> f32;

    fn unk120(&self) -> bool;

    fn is_system_owner_ai(&self) -> bool;

    fn unk130(&self) -> usize;

    fn unk138(&self, param_2: usize) -> u8;

    fn unk140(&self, out: usize) -> usize;

    fn is_in_attack_goal(&self) -> bool;

    fn unk150(&self) -> bool;

    fn unk158(&self) -> bool;

    fn is_disappear_action_approach(&self) -> bool;

    fn is_caution_important_action_approach(&self) -> bool;

    fn is_caution_action_approach(&self) -> bool;

    fn is_search_lvl1_action_approach(&self) -> bool;

    fn is_search_lvl2_action_approach(&self) -> bool;

    fn ai_get_targeting_system(&mut self) -> &mut CSTargetingSystemBase;
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSTargetingSystemOwner {
    vftable: VPtr<dyn CSTargetingSystemOwnerVmt, Self>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSBulletTargetingSystemOwner {
    pub base: CSTargetingSystemOwner,
    pub bullet: NonNull<CSBulletIns>,
    pub owner_chr_handle: FieldInsHandle,
    pub owner_think: NpcThinkParamLookupResult,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSAiTargetingSystemOwner {
    pub base: CSTargetingSystemOwner,
    pub ai: usize,
    pub owner: NonNull<ChrIns>,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSTargetingSystemBase {
    vftable: usize,
    pub system_owner: NonNull<CSTargetingSystemOwner>,
    pub search_sys: CSTargetSearchSys,
    unk8: [u8; 0x120],
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSTargetSearchSys {
    vftable: usize,
    pub system_owner: NonNull<CSTargetingSystemOwner>,
    pub search_slots: [usize; 14],
    unk80: u16,
    _pad82: [u8; 6],
    unk88: usize,
    pub latest_ai_sound_id: i32,
    pub latest_sound_rank: i8,
    unk95: [u8; 0xB],
}

enum SearchSlotIndex {
    Enemy = 0,
    Friend = 1,
    Sound = 2,
    LastSight = 5,
    LastTarget = 6,
    LowFriend = 7,
    Corpse = 8,
    LastMemory = 11,
}

#[cfg(test)]
mod test {
    use crate::cs::{
        CSAiTargetingSystemOwner, CSBulletTargetingSystemOwner, CSTargetingSystemBase,
        CSTargetingSystemOwner,
    };

    #[test]
    fn proper_sizes() {
        assert_eq!(0x8, size_of::<CSTargetingSystemOwner>());
        assert_eq!(0x30, size_of::<CSBulletTargetingSystemOwner>());
        assert_eq!(0x18, size_of::<CSAiTargetingSystemOwner>());
        assert_eq!(0x1d0, size_of::<CSTargetingSystemBase>());
    }
}
