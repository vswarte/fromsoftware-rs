#[repr(C)]
#[derive(Copy, Clone)]
/// Tae Event Args for [`EnableTwistModifier`]
///
/// [`EnableTwistModifier`]: crate::cs::chr_ins::module::time_act::tae::TaeAnimEventId::EnableTwistModifier
pub struct EnableTwistModifierArgs {
    pub up_limit_angle: f32,
    pub down_limit_angle: f32,
    pub right_limit_angle: f32,
    pub left_limit_angle: f32,
    pub modifier_id: u32,
    pub target_type: u8,
    pub rank: u8,
    pub up_minimum_angle: f32,
    pub down_minimum_angle: f32,
    pub right_minimum_angle: f32,
    pub left_minimum_angle: f32,
}
