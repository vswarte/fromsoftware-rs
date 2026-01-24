use std::ptr::NonNull;
use vtable_rs::VPtr;

use crate::cs::{AiIns, BlockId, ChrSetEntry, NpcThinkParamLookupResult};
use crate::dlut::DLFixedVector;
use crate::position::HavokPosition;
use crate::rotation::Quaternion;
use shared::F32Vector4;

use super::{CSBulletIns, ChrIns, FieldInsHandle, SpecialEffect};

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
    pub owner: NonNull<AiIns>,
    pub owner_chr: NonNull<ChrIns>,
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

#[vtable_rs::vtable]
pub trait CSTargetAccessorVmt {
    fn destructor(&mut self);
    /// Get target characters team type.
    fn get_team_type(&self) -> i32;
    /// Checks if the target character is friendly to the main player.
    fn is_friendly_to_main_player(&self) -> i32;
    /// Checks if the target character is guarding.
    fn is_guarding(&self) -> bool;
    /// Checks if the target is alive.
    fn is_alive(&self) -> bool;
    /// Checks if the target character is two handing their weapon.
    fn is_two_handing(&self) -> bool;
    /// Checks if the target character has a paralysis speffect.
    fn is_paralyzed(&self) -> bool;
    /// Checks if the target character has a specific speffect by state info.
    fn has_speffect_with_state_info(&self, state_info: i32) -> bool;
    /// Checks if the target character has a specific speffect by category.
    fn has_speffect_with_category(&self, category: i16) -> bool;
    /// Checks if the target character has a specific speffect by ID.
    fn has_speffect(&self, id: i32) -> bool;
    fn has_sleep_speffect(&self) -> bool;
    fn get_aware_points_correction(&self) -> f32;
    /// Get the target characters HP.
    fn get_hp(&self) -> i32;
    /// Get the target characters HP as a fraction of the total.
    fn get_hp_rate(&self) -> f32;
    /// Get the target characters super armor durability as a fraction of the total.
    fn get_super_armor_durability_rate(&self) -> f32;
    /// Get the target characters FP.
    fn get_fp(&self) -> i32;
    /// Get the target characters stamina.
    fn get_stamina(&self) -> i32;
    /// Get the target characters super armor durability.
    fn get_super_armor_durability(&self) -> i32;
    /// Get the target characters HP as a fraction of the total.
    fn get_hp_rate_2(&self) -> f32;
    /// Get the target position. Writes the result to the out parameter.
    fn get_position<'a>(&self, out: &'a mut F32Vector4) -> &'a mut F32Vector4;
    /// Get the target position and hit radius. Writes the result to the out parameters.
    fn get_position_and_hit_radius<'a>(
        &self,
        position_out: &'a mut F32Vector4,
        param_3: u8,
        param_4: i32,
        hit_radius_out: &'a mut f32,
    );
    fn get_unk_position<'a>(&self, out: &'a mut F32Vector4) -> &'a mut F32Vector4;
    /// Gets the current orientation and rotates a forward vector by it. Result is written to the out
    /// parameter.
    fn get_forward<'a>(&self, out: &'a mut Quaternion) -> &'a mut Quaternion;
    /// Gets the current orientation and rotates a right vector by it. Result is written to the out
    /// parameter.
    fn get_right<'a>(&self, out: &'a mut Quaternion) -> &'a mut Quaternion;
    fn get_hit_capsule_data<'a>(&self, out: &'a mut HitCapsuleData) -> &'a mut HitCapsuleData;
    fn get_map_hit_radius(&self) -> f32;
    fn get_map_hit_height(&self) -> f32;
    /// Get the targets FieldInsHandle. Writes result to out parameter.
    fn get_field_ins_handle<'a>(&self, out: &'a mut FieldInsHandle) -> &'a mut FieldInsHandle;
    /// Get the targets event entity ID. Writes result to out parameter.
    fn get_event_entity_id<'a>(&self, out: &'a mut i32) -> &'a mut i32;
    /// Sums up the target priority of all the speffects.
    fn get_target_priority(&self) -> f32;
    /// Gets the targets currently equipped spell.
    fn get_currently_equipped_magic(&self) -> i32;
    fn is_riding(&self) -> bool;
    fn is_in_battle(&self) -> bool;
    fn unk108(&self);
    fn get_chr_ins(&self) -> Option<NonNull<ChrIns>>;
    fn unk118(&self);
    fn get_offset_y(&self) -> f32;
    fn unk128(&self);
    fn get_look_at_target_position_offset(&self) -> f32;
    fn unk138(&self);
    fn unk140(&self);
    fn unk148(&self);
    fn unk150(&self);
    fn unk158(&self);
    fn unk160(&self);
    fn unk168(&self);
    fn is_climbing_ladder(&self) -> bool;
    fn is_player_summon(&self) -> bool;
    /// Checks the equipped weapon category. Seemingly maps only special weapon types?
    /// - Torch (12) -> 1
    /// - Bow (10) -> 2
    /// - Crossbow (11) -> 2
    /// - Staff (8) -> 3
    /// - Everything else -> 0
    fn get_special_weapon_category(&self, side: u32) -> i32;
    /// Gets the targets MSB block ID.
    fn get_msb_block_id<'a>(&self, out: &'a mut BlockId) -> &'a mut BlockId;
    fn unk190(&self);
    fn unk198(&self);
    fn unk1a0(&self);
    fn unk1a8(&self);
    /// Get a weapon by its side and slot from the targets ChrAsm.
    fn get_equipped_weapon(&self, side: u32, slot: u32) -> i32;
    /// Get the targets currently "active" weapon.
    fn get_currently_equipped_weapon(&self, side: u32) -> i32;
    /// Get the targets weight type.
    fn get_weight_type(&self, side: u32) -> i32;
    /// Get the targets magic param ID by slot.
    fn get_equipped_magic(&self, slot: u32) -> i32;
    /// Get the targets goods param ID by slot in a specific slot type.
    /// Slot type 0 = Quick slots.
    /// Slot type 1 = Pouch.
    fn get_equipped_goods(&self, slot: u32, slot_type: u32) -> i32;
    /// Get the targets equipped arrows/bolts.
    /// - Type 0 = Arrow.
    /// - Type 1 = Bolt.
    fn get_equipped_projectile(&self, r#type: u32, slot: u32) -> i32;
    /// Condenses "both" state of both sides to a single value.
    /// - No side has both hands = -1
    /// - Both hands left = 0
    /// - Both hands right = 1
    fn get_weapon_both_hand_state(&self) -> i32;
    /// Distinguishes the accessor types.
    /// - CS::CSChrInsHandleTargetAccessor = 2
    fn get_accessor_type(&self) -> i32;
    /// Applies a havok world shift to any contained coordinates.
    fn apply_worldshift(&self, shift: &F32Vector4) -> i32;
}

#[vtable_rs::vtable]
pub trait CSChrInsTargetAccessorBaseVmt: CSTargetAccessorVmt {
    fn get_chr_ins(&self) -> Option<NonNull<ChrIns>>;

    fn get_chr_set_entry(&self) -> Option<NonNull<ChrSetEntry<ChrIns>>>;
}

#[repr(C)]
pub struct CSChrInsHandleTargetAccessor {
    vftable: VPtr<dyn CSChrInsTargetAccessorBaseVmt, Self>,
    pub chr_ins_handle: FieldInsHandle,
    unk10: u8,
    unk20: F32Vector4,
    unk30: F32Vector4,
    unk40: u32,
    unk44: i32,
    unk48: i32,
    unk50: F32Vector4,
}

pub struct HitCapsuleData {
    pub map_hit_radius: f32,
    pub map_hit_height: f32,
    pub chr_hit_radius: f32,
    pub chr_hit_height: f32,
}

#[repr(C)]
pub struct CSFixedPosTarget {
    vftable: VPtr<dyn CSTargetAccessorVmt, Self>,
    pub position: HavokPosition,
    unk20: F32Vector4,
    pub hit_radius: f32,
}

#[repr(C)]
pub struct CSRelativePosTarget {
    vftable: VPtr<dyn CSTargetAccessorVmt, Self>,
    unk4: u32,
    unk10: HavokPosition,
    unk20: f32,
}

#[repr(C)]
pub struct CSTargetVelocityRecorder {
    vtable: isize,
    /// A list of all the previously sampled deltas.
    pub deltas: DLFixedVector<F32Vector4, 240>,
    /// Targets position last frame.
    pub previous_position: HavokPosition,
    /// Targets position current frame.
    pub current_position: HavokPosition,
    unkf40: F32Vector4,
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
