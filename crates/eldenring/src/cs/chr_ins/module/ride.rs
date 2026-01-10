use shared::{F32ModelMatrix, OwnedPtr};

use crate::{Vector, cs::{CSPairAnimNode, ChrIns}, dltx::{DLString, DLUTF8StringKind}, position::{HavokPosition, PositionDelta}, rotation::{EulerAngles, Quaternion}};
use std::ptr::NonNull;

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrRideModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub ride_node: OwnedPtr<CSRideNode>,
    /// Gets populated when mounting another `ChrIns`.
    /// Note: This will be null if you load into the world and you've already mounted a `ChrIns`
    /// during the previous session. This is not populated on the receiving `ChrIns`.
    pub last_mounted: NonNull<ChrIns>,
    unk20: i32,
    unk24: i32,
    unk28: i32,
    unk2c: i32,
    unk30: bool,
    /// True when a RideParam entry was found matching both entries.
    pub has_ride_param: bool,
    unk32: bool,
    /// True when this `ChrIns` is the ridden character.
    pub is_ride_character: bool,
    unk34: i32,
    unk38: i32,
    unk3c: i32,
    /// Some additional details describing the mount. This is only updated on the rider's end.
    /// For example the player will have data in here, but Torrent's `ChrIns` will not.
    pub mount_data: CSChrRideModuleMountData,
    unk140: Vector<()>,
    unk160: bool,
    unk161: bool,
    /// Is in the mounting animation?
    pub is_mounting: bool,
    /// Is done with the mounting animation?
    pub is_mounted: bool,
    unk164: [u8; 0x4C],
    unk1b0: DLString<DLUTF8StringKind>,
}

#[repr(C)]
pub struct CSChrRideModuleMountData {
    unk0: F32ModelMatrix,
    unk40: F32ModelMatrix,
    /// Rotation in euler angles.
    pub rotation: EulerAngles,
    /// Position of the ride's `ChrIns`.
    pub mount_position: HavokPosition,
    /// Seems to be the position of the ride dmypoly (where the two `ChrIns`es snap together).
    pub dummy_poly_position: HavokPosition,
    // I believe this to be a quaternion but in the wrong order? Other quats appear to be xyzw.
    // This quanternion seemingly only encodes rotations along the y (up)/yaw but the order seems
    // to be wxyz with this one?
    unkb0: Quaternion,
    /// Speed of the mount as a directional vector.
    pub velocity: PositionDelta,
    unkd0: u64,
    pub attack_direction: u8,
    unkdc: u32,
    unke0: u32,
    pub received_damage_type: u32,
    unke8: u32,
    pub mount_health: u32,
    pub fall_height: f32,
    /// Is the the mount `ChrIns` touching solid ground?
    pub is_touching_solid_ground: bool,
    // Seems to be related to landing?
    unkf5: bool,
    /// Is the mount `ChrIns` in the falling loop?
    pub is_falling: bool,
    /// Is the mount `ChrIns` sliding?
    pub is_sliding: bool,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSRideNode {
    pub pair_anim_node: CSPairAnimNode,
    /// 0 = none, 3 = getting on, 5 = riding, 7 = dismounting
    pub ride_state: u32,
    unk54: f32,
    unk58: f32,
    unk5c: f32,
    pub ride_param_id: i32,
    unk64: i32,
    unk68: i32,
    unk6c: i32,
    unk70: bool,
    pub camera_mount_control: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSChrRideModule>(), 0x1e0);
        assert_eq!(std::mem::size_of::<CSRideNode>(), 0x80);
    }
}
