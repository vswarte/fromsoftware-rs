use crate::Vector;
use crate::cs::ChrIns;
use bitfield::bitfield;
use std::ptr::NonNull;

#[repr(C)]
/// Source of name: RTTI
///
/// Manages player and NPC action inputs, queuing, and cancel logic. Updated during [`ChrIns_PreBehavior`].
///
/// [`ChrIns_PreBehavior`]: crate::cs::task::CSTaskGroupIndex::ChrIns_PreBehavior
pub struct CSChrActionRequestModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// Raw action inputs from the pad manipulator
    pub action_requests: ChrActions,
    /// Previous frame's [`Self::action_requests`].
    /// Used to detect new presses and releases.
    pub previous_action_requests: ChrActions,
    /// Actions newly pressed this frame.
    pub new_action_presses: ChrActions,
    /// Actions released this frame.
    pub released_actions: ChrActions,
    /// Actions queued and eligible to cancel the current animation.
    ///
    /// Set when an action is in both [`Self::queued_action_inputs`] AND [`Self::possible_action_cancels`],
    /// and [`TaeCancelFlags::cancel_disable`] is not set.
    pub cancel_ready_actions: ChrActions,
    /// Intermediate: actions pressed while their input was allowed.
    ///
    /// Set when an action is in both [`Self::new_action_presses`] AND [`Self::possible_action_inputs`].
    /// Cleared when [`ChrActionAnimationFlags::stay_state`] is active.
    ///
    /// [`ChrActionAnimationFlags::stay_state`]: crate::cs::chr_ins::module::action_flag::ChrActionAnimationFlags::stay_state
    pub queued_action_inputs: ChrActions,
    /// Bitmask of actions currently blocked from input.
    pub disabled_action_inputs: ChrActions,
    /// Queue for action request and cancel tracking.
    ///
    /// Active when [`Self::queue_mode_enabled`] is true.
    pub action_request_queue: ActionRequestQueue,
    /// Controls what actions can be queued during current animation.
    pub possible_action_inputs: ChrActions,
    /// Controls what actions can interrupt current animation.
    pub possible_action_cancels: ChrActions,
    /// Snapshot of `possible_action_inputs` preserved across frames.
    pub prev_possible_action_inputs: ChrActions,
    /// Current action durations in seconds.
    /// How long each action button has been held down.
    ///
    /// Conflicting movement requests will be ignored (eg. W + S).
    ///
    /// HKS: `env(ActionDuration, ACTION_ARM_*)`.
    pub action_timers: ActionTimers,
    /// Duration the movement request has been held (seconds).
    /// Reset to 0 when no movement input.
    pub movement_request_duration: f32,
    /// NPC EzState action ID, set from `AiIns::actionRequestData.ezActionId`.
    /// -1 when not set.
    /// Second argument of `GOAL_COMMON_Attack` (e.g. 3110 for the backstab logic).
    pub npc_action_id: i32,
    /// Param ID of the requested gesture from PadManipulator.
    pub requested_gesture: i32,
    /// Movement request state flags.
    pub movement_request_flags: MovementRequestFlags,
    /// TAE-driven animation cancel control flags.
    pub tae_cancels: TaeCancelFlags,
    /// Readback copy of `new_action_presses`.
    pub readback_new_presses: ChrActions,
    /// Readback of `cancel_ready_actions`.
    pub readback_cancel_ready: ChrActions,
    /// Readback copy of `queued_action_inputs`.
    pub readback_queued_inputs: ChrActions,
    /// Readback copy of `possible_action_inputs`.
    pub readback_possible_inputs: ChrActions,
    /// Readback of `possible_action_cancels`.
    pub readback_possible_cancels: ChrActions,
    /// Saved `npc_action_id` from the previous frame.
    pub readback_npc_action_id: i32,
    /// Override for [`ActionRequestQueue::current_index`].
    ///
    /// When >= 0, used instead of the queue's own current_index.
    pub queue_index_override: i32,
    /// Enables the queue-based action request system.
    ///
    /// When true, per-state action requests and cancels are tracked in
    /// [`ActionRequestQueue`] vector entries instead of the simple bitfield path.
    pub queue_mode_enabled: bool,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MovementRequestFlags(u32);
    impl Debug;

    /// Raw movement input from pad manipulator.
    pub raw_input, set_raw_input: 0;
    /// Movement can cancel current animation.
    /// Set when [`TaeCancelFlags::movement_cancel`] is active AND raw input is present.
    pub cancel_eligible, set_cancel_eligible: 1;
    /// Dash/sprint input active.
    ///
    /// Set from `CS::AiIns::isDashing` or pad input.
    pub dash, set_dash: 2;
}

/// Queue for tracking action requests and cancels per animation state.
#[repr(C)]
pub struct ActionRequestQueue {
    pub owner: NonNull<CSChrActionRequestModule>,
    /// Per-state input tracking. Bit 35 = movement request for this state.
    pub input_entries: Vector<ActionQueueEntry>,
    /// Per-state cancel tracking. Cleared at end of `UpdateFromManipulator`.
    pub cancel_entries: Vector<ActionQueueEntry>,
    /// Current state index used to look up entries in both vectors.
    pub current_index: i32,
}

#[repr(C)]
pub struct ActionQueueEntry {
    /// State index this entry belongs to.
    pub state_index: i32,
    /// Action bitfield. Bits 0-34 match [`ChrActions`].
    /// Extra queue-specific bits:
    /// - 35: Movement request (input) / movement cancel current (cancel)
    /// - 36: Movement cancel previous frame (cancel only)
    /// - 37: Slot switch cancel (cancel only, parallel to [`TaeCancelFlags::slot_switch`])
    ///
    /// HKS `env(IsMoveCancelPossible)` in queue mode requires bits 35 AND 36 both set.
    /// HKS `env(MovementRequest)` in queue mode checks bit 35 of the input entry.
    pub actions: ChrActions,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ChrActions(u64);
    impl Debug;
    /// ACTION_ARM_R1 (0) - Main hand light attack
    pub r1, set_r1:                           0;
    /// ACTION_ARM_R2 (1) - Main hand heavy attack
    pub r2, set_r2:                           1;
    /// ACTION_ARM_L1 (2) - Off hand light attack
    pub l1, set_l1:                           2;
    /// ACTION_ARM_L2 (3) - Off hand heavy attack
    pub l2, set_l2:                           3;
    /// ACTION_ARM_ACTION (4) - Pouch slot submenu and weapon switch button (E)
    pub action, set_action:                   4;
    /// ACTION_ARM_SP_MOVE (5) - Dodge / Roll / Backstep
    pub sp_move, set_sp_move:                 5;
    /// ACTION_ARM_CHANGE_STYLE (6) - Change style / Jump button.
    /// Repurposed to jump in Elden Ring.
    pub jump, set_jump:                       6;
    /// ACTION_ARM_USE_ITEM (7) - Use consumable item
    pub use_item, set_use_item:               7;
    /// ACTION_ARM_SWITCH_FORM (8) - Switch spell / Switch form
    pub switch_form, set_switch_form:         8;
    /// ACTION_ARM_CHANGE_WEAPON_R (9)
    pub change_weapon_r, set_change_weapon_r: 9;
    /// ACTION_ARM_CHANGE_WEAPON_L (10)
    pub change_weapon_l, set_change_weapon_l: 10;
    /// ACTION_ARM_CHANGE_ITEM (11)
    pub change_item, set_change_item:         11;
    /// ACTION_ARM_R3 (12) - Lock on
    pub r3, set_r3:                           12;
    /// ACTION_ARM_L3 (13) - Crouch
    pub l3, set_l3:                           13;
    /// ACTION_ARM_TOUCH_R (14)
    pub touch_r, set_touch_r:                 14;
    /// ACTION_ARM_TOUCH_L (15)
    pub touch_l, set_touch_l:                 15;
    /// ACTION_ARM_BACKSTEP (16)
    pub backstep, set_backstep:               16;
    /// ACTION_ARM_ROLLING (17)
    pub rolling, set_rolling:                 17;
    /// ACTION_ARM_MAGIC_R (19) - Magic from main hand catalyst
    pub magic_r, set_magic_r:                 19;
    /// ACTION_ARM_MAGIC_L (20) - Magic from off hand catalyst
    pub magic_l, set_magic_l:                 20;
    /// ACTION_ARM_GESTURE (21)
    pub gesture, set_gesture:                 21;
    /// ACTION_ARM_LADDERUP (22)
    pub ladderup, set_ladderup:               22;
    /// ACTION_ARM_LADDERDOWN (23)
    pub ladderdown, set_ladderdown:           23;
    /// ACTION_ARM_GUARD (24)
    pub guard, set_guard:                     24;
    /// ACTION_ARM_EMERGENCYSTEP (25)
    pub emergencystep, set_emergencystep:     25;
    /// ACTION_ARM_LIGHT_KICK (26) - Forward + R1 + L1
    pub light_kick, set_light_kick:           26;
    /// ACTION_ARM_HEAVY_KICK (27) - Forward + R2 + L2
    pub heavy_kick, set_heavy_kick:           27;
    /// ACTION_ARM_CHANGE_STYLE_R (28) - Two-hand right weapon
    pub change_style_r, set_change_style_r:   28;
    /// ACTION_ARM_CHANGE_STYLE_L (29) - Two-hand left weapon
    pub change_style_l, set_change_style_l:   29;
    /// ACTION_ARM_RIDEON (30) - Mount Torrent
    pub rideon, set_rideon:                   30;
    /// ACTION_ARM_RIDEOFF (31) - Torrent boost / dismount
    pub rideoff, set_rideoff:                 31;
    /// ACTION_ARM_BUDDY_DISAPPEAR (32) - Dismiss spirit summon
    pub buddy_disappear, set_buddy_disappear: 32;
    /// ACTION_ARM_MAGIC_R2 (33) - Magic from main hand weapon catalyst
    pub magic_r2, set_magic_r2:               33;
    /// ACTION_ARM_MAGIC_L2 (34) - Magic from off hand weapon catalyst
    pub magic_l2, set_magic_l2:               34;

    /// Queue bit 35: Movement request (input entries) / movement cancel current (cancel entries).
    ///
    /// Not used in the main `action_requests` bitfield.
    pub movement, set_movement:               35;
    /// Queue bit 36: Movement cancel previous frame (cancel entries only).
    ///
    /// Not used in the main `action_requests` bitfield.
    pub movement_prev, set_movement_prev:     36;
    /// Queue bit 37: Slot switch cancel (cancel entries only).
    ///
    /// Not used in the main `action_requests` bitfield.
    pub slot_switch, set_slot_switch:         37;
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
    /// Jump button / Change style
    pub jump: f32,
    /// Consumable item use
    pub use_item: f32,
    /// Switch form / Spell switch
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
    pub struct TaeCancelFlags(u32);
    impl Debug;

    bool;
    /// Input queue flush latch.
    ///
    /// Set by HKS `act(ResetInputQueue)`.
    ///
    /// When set, next update clears [`CSChrActionRequestModule::queued_action_inputs`] and consumes this bit.
    pub input_queue_flush, set_input_queue_flush: 0;
    /// Movement cancel request.
    ///
    /// Set by TAE `CANCEL_LS_MOVEMENT` and `CANCEL_AI_MOVE`.
    /// In queue mode, propagated to [`ChrActions::movement`].
    /// [`Self::movement_cancel_prev`] receives this value before it's cleared.
    pub movement_cancel, set_movement_cancel: 1;
    /// Previous frame's [`Self::movement_cancel`].
    ///
    /// HKS `env(IsMoveCancelPossible)` requires both bits 1 AND 2,
    /// meaning the TAE event must be active for 2 consecutive frames.
    pub movement_cancel_prev, set_movement_cancel_prev: 2;
    /// Slot switch cancel.
    ///
    /// Enables cancel into slot-switch actions.
    ///
    /// TAE: `CANCEL_CHANGE_STYLE_L_R_CHANGE_ITEM_L_R_R3_L3_TOUCH_L_R_BUDDY_DISAPPEAR`.
    ///
    /// Transient
    pub slot_switch, set_slot_switch: 3;
    /// RH attack cancel.
    /// TAE `CANCEL_R1_R2_LIGHT_KICK_HEAVY_KICK` and `CANCEL_AI_COMBOATTACK`.
    ///
    /// Transient.
    pub rh_attack, set_rh_attack: 4;
    /// AI queued attack cancel.
    ///
    /// TAE `CANCEL_AI_ATTACK_QUEUED`
    /// Queried by HKS `env(GetAIAtkCancelType)`
    ///
    /// Transient.
    pub ai_attack_queued, set_ai_attack_queued: 5;
    /// Step/movement cancel (AI).
    /// TAE `CANCEL_LS_MOVEMENT` and `CANCEL_AI_STEP`
    /// Queried by `CS::CSAiFunc::IsEnableCancelStep` / HKS `env(GetAIChainStepType)`.
    ///
    /// Transient.
    pub ai_cancel_step, set_ai_cancel_step: 6;
    /// General action cancel. Set alongside many cancel TAE events.
    ///
    /// TAE: `CANCEL_R1_R2_LIGHT_KICK_HEAVY_KICK`, `CANCEL_MAGIC_L_R_MAGIC_R2_L2`,
    /// `CANCEL_CHANGE_STYLE_L_R_...`, `CANCEL_L2_ATTACK`, `CANCEL_L1_L2`,
    /// `CANCEL_L1_L2_FOR_DUAL_BLADE`, `CANCEL_QUICK_GOODS`.
    ///
    /// Transient.
    pub action_general, set_action_general: 9;
    /// Falling/jump frames.
    ///
    /// Set by TAE `FALLING_JUMP_FRAMES_USE_W_51`.
    ///
    /// **Persistent**.
    pub falling_jump_frames, set_falling_jump_frames: 10;
    /// Global cancel disable.
    ///
    /// When set, prevents all actions from being
    /// added to [`CSChrActionRequestModule::cancel_ready_actions`].
    ///
    /// **Persistent**.
    pub cancel_disable, set_cancel_disable: 11;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSChrActionRequestModule>(), 0x140);
        assert_eq!(std::mem::size_of::<ActionRequestQueue>(), 0x50);
        assert_eq!(std::mem::size_of::<ActionQueueEntry>(), 0x10);
    }
}
