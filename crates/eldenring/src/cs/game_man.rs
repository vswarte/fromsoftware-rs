use std::borrow::Cow;

use crate::cs::{
    BlockId, CSEzTask, CSEzUpdateTask, CSRandSFMT, CSRandXorshift, MultiplayRole, PartyMemberInfo,
    SummonParamType,
};
use crate::dlut::DLDateTime;
use crate::position::BlockPosition;
use fromsoftware_shared::{FromStatic, OwnedPtr, load_static_indirect};
use shared::{F32Vector3, F32Vector4};

#[repr(C)]
pub struct GameMan {
    vftable: usize,
    unk8: usize,
    pub warp_requested: bool,
    unk14: BlockId,
    unk18: BlockId,
    unk1c: [u8; 0x4],
    /// Backup of local time when player is in someone else's world
    /// See [crate::cs::WorldAreaTime::clock]
    pub world_area_time: DLDateTime,
    unk30: [u8; 0xc],
    /// Set by emevd 2003[14] WarpPlayer \
    /// See https://soulsmods.github.io/emedf/er-emedf.html#WarpPlayer
    pub initial_area_entity_id: u32,
    /// Set by emevd 2008[1] ChangeCamera \
    /// Overrides default camera parameters in area. \
    /// See [crate::param::LOCK_CAM_PARAM_ST] \
    /// and https://soulsmods.github.io/emedf/er-emedf.html#ChangeCamera
    pub normal_camera_param_id: i32,
    /// Identical to [Self::normal_camera_param_id]
    pub locked_camera_param_id: i32,
    /// Set by TalkESD 138 ChangeCamera
    /// Overrides default camera parameters when talking to NPC. \
    /// See [crate::param::LOCK_CAM_PARAM_ST]
    pub talk_esd_camera_param_id: i32,
    /// Set by TAE event 150 SetLockCamParamSelf
    /// See [crate::param::LOCK_CAM_PARAM_ST]
    pub lock_on_camera_param_id: i32,
    /// Set by TAE event 151 SetCameraFollowDummyPoly
    /// Overrides default dummy poly ID for camera to follow.
    pub camera_follow_dummy_poly_id: i32,
    /// Read from [crate::cs::CSChrActionFlagModule::camera_lock_on_param_id]
    /// of character being locked on to. \
    /// See [crate::param::LOCK_CAM_PARAM_ST]
    pub camera_chr_lock_on_param_id: i32,
    /// Set by TAE event 152 CameraZoomOut
    pub camera_zoom_target_dist_mult: f32,
    /// Set by TAE event 152 CameraZoomOut when SIMPLE_LERP type is used
    pub cam_override_lerp_factor: f32,
    /// Set by TAE event 152 CameraZoomOut when EASE_OUT_QUADRATIC type is used
    pub cam_zoom_interpolated_progress: f32,
    /// Set by TAE event 152 CameraZoomOut
    pub cam_timed_override_duration: f32,
    /// Set by TAE event 152 CameraZoomOut
    pub cam_zoom_override_lerp_factor: f32,
    /// Set by TAE event 152 CameraZoomOut
    pub cam_zoom_reset_previous_distance: bool,
    /// Set by TAE event 152 CameraZoomOut
    pub cam_override_check_collisions: bool,
    /// Set by TAE event 153 ForceCameraDirection
    pub force_cam_vertical_angle_rad: f32,
    /// Set by TAE event 153 ForceCameraDirection
    pub force_cam_horizontal_angle_rad: f32,
    /// Set by TAE event 153 ForceCameraDirection
    pub force_cam_rotation_method: ForceCamRotationMethod,
    /// Set by TAE event 153 ForceCameraDirection
    pub force_cam_interpolation_progress: f32,
    /// Set by TAE event 153 ForceCameraDirection
    pub force_cam_first_execution: bool,
    /// Set by TAE event 153 ForceCameraDirection
    pub force_cam_vertical_enabled: bool,
    /// Set by TAE event 153 ForceCameraDirection
    pub force_cam_horizontal_enabled: bool,
    pub rand_xorshift: CSRandXorshift,
    pub rand_sfmt: CSRandSFMT,
    unka90: [u8; 0x10],
    pub last_load_position: F32Vector4,
    pub last_load_orientation: F32Vector4,
    pub save_slot: i32,
    unkac4: [u8; 0x4],
    pub load_target_block_id: BlockId,
    pub multiplay_join_block_pos: BlockPosition,
    pub multiplay_join_orientation: F32Vector4,
    pub ceremony_entry_point_entity_id: u32,
    pub target_ceremony: i32,
    pub entryfilelist_id: i32,
    unkafc: [u8; 0x4],
    unkb00: i32,
    unkb04: F32Vector3,
    unkb10: F32Vector4,
    unkb20: BlockId,
    unkb24: u32,
    unkb28: bool,
    unkb2c: i32,
    unkb30: F32Vector3,
    unkb40: F32Vector4,
    unkb50: BlockId,
    unkb54: u32,
    unkb58: bool,
    unkb59: u8,
    /// Whether or not item replenishment from chest is requested.
    /// Will trigger refill of all the items in inventory from item storage during MoveMapStep::STEP_CreateDrawPlan
    pub item_replanish_from_chest_requested: bool,
    /// Whether or not item restoration after quickmatch is requested. \
    /// See [crate::cs::EquipGameData::qm_item_backup_vector]
    pub item_restore_after_qmrequested: bool,
    unkb5c: u8,
    unkb5d: u8,
    unkb5e: u8,
    unkb5f: u8,
    unkb60: u32,
    unkb64: u32,
    unkb68: u32,
    unkb6c: u32,
    pub new_game_plus_requested: bool,
    unkb71: u8,
    pub save_requested: bool,
    unkb73: u8,
    unkb74: u8,
    unkb75: u8,
    /// Save slot index requested for loading.
    pub requested_save_slot_load_index: i32,
    unkb7c: u8,
    unkb7d: u8,
    pub save_state: u32,
    unkb88: DLDateTime,
    unkb98: DLDateTime,
    unkba8: DLDateTime,
    unkbb8: u32,
    unkbbc: u32,
    unkbc0: u32,
    unkbc4: u32,
    pub is_in_online_mode: bool,
    unkbc9: u8,
    pub event_world_type: EventWorldType,
    unkbcb: u8,
    unkbcc: u8,
    unkbcd: u8,
    unkbce: u8,
    unkbcf: [u8; 0x19],
    unkbe8: u64,
    unkbf0: [u8; 0x10],
    /// See [crate::cs::CSStayInMultiplayAreaWarpData]
    pub stay_in_multiplay_area_saved_position: F32Vector3,
    /// See [crate::cs::CSStayInMultiplayAreaWarpData]
    pub stay_in_multiplay_area_saved_block_id: BlockId,
    /// See [crate::cs::CSStayInMultiplayAreaWarpData]
    pub stay_in_multiplay_area_saved_rotation: F32Vector4,
    unkc20: [u8; 0x20],
    unkc40: F32Vector4,
    unkc50: F32Vector4,
    pub sub_area_name_popup_message_id: i32,
    pub update_task: CSEzUpdateTask<CSEzTask, Self>,
    unkc90: [u8; 0xf4],
    pub summon_param_type: SummonParamType,
    pub multiplay_role: MultiplayRole,
    pub has_password: bool,
    pub party_member_info: OwnedPtr<PartyMemberInfo>,
    unld98: [u8; 0xd8],
    pub character_name_is_empty: bool,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EventWorldType {
    Local = 0,
    Remote = 1,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ForceCamRotationMethod {
    Default = 0,
    Closest = 1,
    CounterClockwise = 2,
    Clockwise = 3,
}

impl FromStatic for GameMan {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("GameMan")
    }

    unsafe fn instance() -> fromsoftware_shared::InstanceResult<&'static mut Self> {
        unsafe { load_static_indirect(crate::rva::get().game_man) }
    }
}

#[cfg(test)]
mod tests {
    use super::GameMan;
    #[test]
    fn test_size() {
        use std::mem::size_of;
        assert_eq!(size_of::<GameMan>(), 0xE80);
    }
}
