use pelite::pe64::Pe;

use super::PlayerIns;
use crate::{
    position::{HavokPosition, PositionDelta},
    rva,
};
use shared::{program::Program, OwnedPtr};

// Source of name: RTTI
#[shared::singleton("CSHavokMan")]
#[repr(C)]
pub struct CSHavokMan {
    vftable: usize,
    unk8: [u8; 0x90],
    pub phys_world: OwnedPtr<CSPhysWorld>,
}

// Source of name: RTTI
#[repr(C)]
pub struct CSPhysWorld {
    // TODO
}

type FnCastRay = extern "C" fn(
    *const CSPhysWorld,
    u32,
    *const HavokPosition,
    *const HavokPosition,
    *mut HavokPosition,
    *const PlayerIns,
) -> bool;

impl CSPhysWorld {
    /// Casts a ray inside of the physics world. Returns a None if the ray
    /// didn't hit anything.
    pub fn cast_ray(
        &self,
        filter: u32,
        origin: &HavokPosition,
        delta: PositionDelta,
        owner: &PlayerIns,
    ) -> Option<HavokPosition> {
        let target = unsafe {
            std::mem::transmute::<u64, FnCastRay>(
                Program::current()
                    .rva_to_va(rva::get().cs_phys_world_cast_ray)
                    .unwrap(),
            )
        };

        let mut result = HavokPosition(0.0, 0.0, 0.0, 0.0);
        let extent = HavokPosition(delta.0, delta.1, delta.2, 0.0);
        if target(self, filter, origin, &extent, &mut result, owner) {
            Some(result)
        } else {
            None
        }
    }
}
