use eldenring::{
    cs::CSEzDraw,
    position::{HavokPosition, PositionDelta},
};
use pelite::pe64::Pe;
use shared::F32Vector4;

use crate::{program::Program, rva};

pub trait CSEzDrawExt {
    /// Set the color for the to-be-rendered primitives.
    fn set_color(&self, color: &F32Vector4);

    fn draw_line(&self, from: &HavokPosition, to: &HavokPosition);

    fn draw_capsule(&self, top: &HavokPosition, bottom: &HavokPosition, radius: f32);

    fn draw_sphere(&self, origin: &HavokPosition, radius: f32);

    fn draw_wedge(
        &self,
        origin: &HavokPosition,
        direction: &PositionDelta,
        inner_length: f32,
        outer_length: f32,
        degrees: f32,
    );
}

type FnSetColor = extern "C" fn(*const CSEzDraw, *const F32Vector4);
type FnDrawLine = extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition);
type FnDrawCapsule =
    extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition, f32);
type FnDrawSphere = extern "C" fn(*const CSEzDraw, *const HavokPosition, f32);
type FnDrawFan =
    extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition, f32, f32, f32);

impl CSEzDrawExt for CSEzDraw {
    fn set_color(&self, color: &F32Vector4) {
        let target = unsafe {
            std::mem::transmute::<u64, FnSetColor>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_set_color)
                    .unwrap(),
            )
        };

        target(self, color);
    }

    fn draw_line(&self, from: &HavokPosition, to: &HavokPosition) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawLine>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_line)
                    .unwrap(),
            )
        };

        target(self, from, to);
    }

    fn draw_capsule(&self, top: &HavokPosition, bottom: &HavokPosition, radius: f32) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawCapsule>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_capsule)
                    .unwrap(),
            )
        };

        target(self, top, bottom, radius);
    }

    fn draw_sphere(&self, origin: &HavokPosition, radius: f32) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawSphere>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_sphere)
                    .unwrap(),
            )
        };

        target(self, origin, radius);
    }

    fn draw_wedge(
        &self,
        origin: &HavokPosition,
        direction: &PositionDelta,
        inner_length: f32,
        outer_length: f32,
        degrees: f32,
    ) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawFan>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_wedge)
                    .unwrap(),
            )
        };

        let direction = HavokPosition(direction.0, direction.1, direction.2, 0.0);

        target(
            self,
            origin,
            &direction,
            inner_length,
            outer_length,
            degrees,
        );
    }
}
