use std::ptr::NonNull;

use shared::{F32Vector3, F32Vector4, OwnedPtr, ReadOnlyPtr};
use vtable_rs::VPtr;

use crate::{Tree, cs::CSMsbPoint, rotation::Quaternion};

#[shared::singleton("CSEventRegionMan")]
#[repr(C)]
/// Keeps track of event regions in the world.
/// 
/// Source of name: RTTI
pub struct CSEventRegionMan {
    vftable: isize,
    pub regions: Tree<CSEventRegionManEntry>,
    point_menu_helper: [u8; 0x188],
    unk1a8: [u8; 0xb0],
}

#[repr(C)]
pub struct CSEventRegionManEntry {
    pub region_id: u32,
    pub event_region: OwnedPtr<CSEventRegion>,
}

#[repr(C)]
/// Event region usually made by 
pub struct CSEventRegion {
    vftable: VPtr<dyn CSEventRegionVmt, Self>,
    unk8: isize,
    /// MSB point this event region is for.
    pub msb_point: CSMsbPoint,
    /// Center of the event region's block in physics space.
    /// Used in point position (msb space) to point position (physics space) conversion.
    pub block_center: F32Vector4,
    /// Rotation of the event region.
    pub rotation: Quaternion,
    unk90: u32,
    unk94: u32,
}

#[vtable_rs::vtable]
pub trait CSEventRegionVmt {
    fn populate_from_msb_data(&mut self, msb_point: &CSMsbPoint, unk3: bool);

    fn is_inside_region(&mut self, param_2: u8, param_3: isize, param_4: u8) -> bool;

    fn unk10(&mut self, out: &mut F32Vector4) -> &mut F32Vector3;

    fn get_position(&mut self, out: &mut F32Vector3) -> &mut F32Vector3;

    fn get_rotation(&mut self, out: &mut F32Vector3) -> &mut F32Vector3;
}
