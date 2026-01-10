use bitfield::bitfield;
use std::ptr::NonNull;
use crate::cs::ChrIns;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrActionRequestModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// Actions that are currently being requested by the player.
    pub action_requests: ChrActions,
    unk18: [u8; 0x8],
    unk20: u64,
    unk28: [u8; 0x8],
    unk30: u64,
    unk38: [u8; 0x8],
    /// Controls what actions are currently can't be inputted by the player.
    pub disabled_action_inputs: ChrActions,
    unk48: [u8; 0x48],
    unk90: u32,
    unk94: [u8; 0x4],
    /// Controls what actions can be queued during current animation.
    pub possible_action_inputs: ChrActions,
    /// Controls what actions can interrupt current animation.
    pub possible_action_cancels: ChrActions,
    unka8: [u8; 0x8],
    /// Current action durations in seconds.
    /// Corresponds to how long each action button is held down.
    pub action_timers: ActionTimers,
    /// For how long movement request buttons are held down.
    /// Conflicting movement requests will be ignored (eg. W + S).
    pub movement_request_duration: f32,
    unkf4: [u8; 0x4],
    /// Param ID of the requested gesture from PadManipulator.
    pub requested_gesture: i32,
    unkfc: [u8; 0x4],
    pub ai_cancels: AiActionCancels,
    unk104: [u8; 0x3c],
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrActions(u64);
    impl Debug;
    pub r1, set_r1:                           0;
    pub r2, set_r2:                           1;
    pub l1, set_l1:                           2;
    pub l2, set_l2:                           3;
    /// Pouch slot submenu and weapon switch button (E)
    pub action, set_action:                   4;
    /// Roll button
    pub sp_move, set_sp_move:                 5;
    pub jump, set_jump:                       6;
    pub use_item, set_use_item:               7;
    pub switch_spell, set_switch_spell:       8;
    pub change_weapon_r, set_change_weapon_r: 9;
    pub change_weapon_l, set_change_weapon_l: 10;
    pub change_item, set_change_item:         11;
    /// Lock on
    pub r3, set_r3:                           12;
    /// Crouch
    pub l3, set_l3:                           13;
    pub touch_r, set_touch_r:                 14;
    pub touch_l, set_touch_l:                 15;
    pub backstep, set_backstep:               16;
    pub rolling, set_rolling:                 17;
    /// Magic casted by the spellcasting item when in main hand.
    pub magic_r, set_magic_r:                 19;
    /// Magic casted by the spellcasting item when in off hand.
    pub magic_l, set_magic_l:                 20;
    pub gesture, set_gesture:                 21;
    pub ladderup, set_ladderup:               22;
    pub ladderdown, set_ladderdown:           23;
    pub guard, set_guard:                     24;
    pub emergencystep, set_emergencystep:     25;
    /// Forward + R1 + L1 (ds kick)
    pub light_kick, set_light_kick:           26;
    /// Forward + R2 + L2 (ds kick)
    pub heavy_kick, set_heavy_kick:           27;
    pub change_style_r, set_change_style_r:   28;
    pub change_style_l, set_change_style_l:   29;
    pub rideon, set_rideon:                   30;
    /// Torrent boost
    pub rideoff, set_rideoff:                 31;
    pub buddy_disappear, set_buddy_disappear: 32;
    /// Magic casted by the spellcasting sword when in main hand.
    pub magic_r2, set_magic_r2:               33;
    /// Magic casted by the spellcasting sword when in off hand.
    pub magic_l2, set_magic_l2:               34;
}

#[repr(C)]
pub struct ActionTimers {
    /// Main hand light attack
    pub r1: f32,
    /// Main hand heavy attack
    pub r2: f32,
    /// Offhand light attack
    pub l1: f32,
    /// Offhand heavy attack
    pub l2: f32,
    /// Pouch slot submenu and weapon switch button (E)
    pub action: f32,
    /// Roll and Backstep
    pub roll: f32,
    /// Jump button
    pub jump: f32,
    /// Consumable item use
    pub use_item: f32,
    /// Spell switch
    pub switch_spell: f32,
    /// Change weapon in right hand
    pub change_weapon_r: f32,
    /// Change weapon in left hand
    pub change_weapon_l: f32,
    /// Change to next consumable
    pub change_item: f32,
    /// Right stick click
    pub r3: f32,
    /// Left stick click
    pub l3: f32,
    pub touch_r: f32,
    pub touch_l: f32,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct AiActionCancels(u32);
    impl Debug;

    /// Set by TAE Event 0 ChrActionFlag (action 32 CANCEL_JUMP_CROUCH_WEAPON_SWITCH)
    pub slot_switch, set_slot_switch:       3;
    /// Set by TAE Event 0 ChrActionFlag (action 4 CANCEL_RH_ATTACK & 23 CANCEL_AI_COMBOATTACK)
    pub rh_attack, set_rh_attack:           4;
    /// Set by TAE Event 0 ChrActionFlag (action 1 CANCEL_LS_MOVEMENT)
    pub ls_movement, set_ls_movement:       6;
    /// Set by TAE Event 0 ChrActionFlag (action 4 CANCEL_RH_ATTACK & 16 CANCEL_LH_ATTACK)
    pub action_general, set_action_general: 9;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSChrActionRequestModule>(), 0x140);
    }
}
