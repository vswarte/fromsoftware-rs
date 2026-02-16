use std::ptr::NonNull;

use shared::F32Vector4;

use crate::cs::{ChrIns, FieldInsHandle};

/// Source of name: RTTI
#[repr(C)]
pub struct CSChrLadderModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    /// FieldInsHandle of the ladder's [`crate::cs::GeomIns`].
    pub ladder_handle: FieldInsHandle,
    /// Animation and control state for the currently ongoing ladder interaction. 
    pub state: LadderState,
    /// Havok coordinates of the ladder's top.
    pub top: F32Vector4,
    /// Havok coordinates of the ladder's bottom.
    pub bottom: F32Vector4,
    unk40: u8,
    unk44: u32,
    flags: u8,
}

/// Left and right are viewed from the ladder's perspective (character's front) rather than
/// from the character's back.
/// When moving up or down the ladder your character starts alternating between left and right with
/// every step.
///
/// Source of entry names: common_define.hks.
#[repr(i8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LadderState {
    /// Character is not on a ladder.
    None = -1,
    /// Getting onto the ladder from the bottom.
    StartBottom = 0,
    /// Getting onto the ladder fromn the top.
    StartTop = 1,
    /// Moving up using your left hand.
    UpRight = 2,
    /// Moving up using your right hand.
    UpLeft = 3,
    /// Moving down using your left hand.
    DownRight = 4,
    /// Moving down using your right hand.
    DownLeft = 5,
    /// Getting off the ladder at the top.
    EndTop = 6,
    /// Getting off the ladder at the bottom.
    EndBottom = 7,
    /// Not moving on the ladder with the left hand being the upper one.
    IdleRight = 8,
    /// Not moving on the ladder with the right hand being the upper one.
    IdleLeft = 9,
    /// Attacking (punching) up with your left hand.
    AttackUpRight = 10,
    /// Attacking (punching) up with your right hand.
    AttackUpLeft = 11,
    /// Attacking (kicking) down with your left leg.
    AttackDownRight = 12,
    /// Attacking (kicking) down with your right leg.
    AttackDownLeft = 13,
    /// Starting to slide down the ladder.
    CoastStart = 14,
    /// Slide down the ladder. Left was our last side to the dominant.
    CoastRight = 15,
    /// Stopping the slide downwards.
    CoastStop = 16,
    /// Slide down the ladder. Right was our last side to the dominant.
    CoastLeft = 18,
    /// Stopping the slide downwards by landing on the ground.
    CoastLanding = 20,
    /// Taking small amounts of damage.
    DamageSmall = 21,
    /// Taking large amounts of damage.
    DamageLarge = 22,
}
