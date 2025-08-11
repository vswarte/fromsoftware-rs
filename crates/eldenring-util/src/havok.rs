use eldenring::{
    cs::{CSPhysWorld, PlayerIns},
    position::{HavokPosition, PositionDelta},
};
use pelite::pe64::Pe;

use crate::{program::Program, rva};

type FnCastRay = extern "C" fn(
    *const CSPhysWorld,
    u32,
    *const HavokPosition,
    *const HavokPosition,
    *mut HavokPosition,
    *const PlayerIns,
) -> bool;

pub trait CSPhysWorldExt {
    /// Casts a ray inside of the physics world. Returns a None if the ray didn't hit anything.
    fn cast_ray(
        &self,
        filter: u32,
        origin: &HavokPosition,
        delta: PositionDelta,
        owner: &PlayerIns,
    ) -> Option<HavokPosition>;
}

impl CSPhysWorldExt for CSPhysWorld {
    fn cast_ray(
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
