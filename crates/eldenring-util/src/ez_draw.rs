use eldenring::{
    cs::{CSEzDraw, EzDrawFillMode},
    position::{HavokPosition, PositionDelta},
};
use pelite::pe64::Pe;
use shared::{F32Vector4, Triangle};

use shared::program::Program;

use crate::rva;

pub trait CSEzDrawExt {
    /// Set the fill and line color for the to-be-rendered primitives.
    fn set_color(&mut self, color: &F32Vector4);
    /// Set the line color for the to-be-rendered primitives.
    fn set_line_color(&mut self, color: &F32Vector4);
    /// Set the fill color for the to-be-rendered primitives.
    fn set_fill_color(&mut self, color: &F32Vector4);
    /// Set the fill mode for the to-be-rendered primitives.
    fn set_fill_mode(&mut self, mode: EzDrawFillMode);
    /// Set the depth mode for the to-be-rendered primitives.
    fn set_depth_mode(&mut self, mode: u32);

    fn draw_line(&mut self, from: &HavokPosition, to: &HavokPosition);

    fn draw_capsule(&mut self, top: &HavokPosition, bottom: &HavokPosition, radius: f32);

    fn draw_sphere(&mut self, origin: &HavokPosition, radius: f32);

    fn draw_wedge(
        &mut self,
        origin: &HavokPosition,
        direction: &PositionDelta,
        inner_length: f32,
        outer_length: f32,
        degrees: f32,
    );

    fn draw_triangle(&mut self, triangle: &Triangle);
    fn draw_dodecadron(&mut self, top: &HavokPosition, bottom: &HavokPosition, radius: f32);
}

type FnDrawLine = extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition);
type FnDrawCapsule =
    extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition, f32);
type FnDrawSphere = extern "C" fn(*const CSEzDraw, *const HavokPosition, f32);
type FnDrawFan =
    extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition, f32, f32, f32);
type FnDrawDodecadron =
    extern "C" fn(*const CSEzDraw, *const HavokPosition, *const HavokPosition, f32);
type FnDrawTriangle = extern "C" fn(*const CSEzDraw, *const Triangle);

impl CSEzDrawExt for CSEzDraw {
    fn set_color(&mut self, color: &F32Vector4) {
        self.set_line_color(color);
        self.set_fill_color(color);
    }

    fn set_line_color(&mut self, color: &F32Vector4) {
        let buffer = self.current_buffer_mut();
        if buffer.ez_draw_state.base.line_color != *color {
            buffer.ez_draw_state.base.line_color = *color;
            buffer.ez_draw_state.base.draw_flags.set_line_color(true);
        }
    }

    fn set_fill_color(&mut self, color: &F32Vector4) {
        let buffer = self.current_buffer_mut();
        if buffer.ez_draw_state.base.fill_color != *color {
            buffer.ez_draw_state.base.fill_color = *color;
            buffer.ez_draw_state.base.draw_flags.set_fill_color(true);
        }
    }

    fn set_fill_mode(&mut self, mode: EzDrawFillMode) {
        let buffer = self.current_buffer_mut();
        if buffer.ez_draw_state.base.fill_mode != mode {
            buffer.ez_draw_state.base.fill_mode = mode;
            buffer.ez_draw_state.base.draw_flags.set_fill_mode(true);
        }
    }

    fn set_depth_mode(&mut self, mode: u32) {
        let buffer = self.current_buffer_mut();

        if buffer.ez_draw_state.base.depth_mode != mode {
            buffer.ez_draw_state.base.depth_mode = mode;
            buffer.ez_draw_state.base.draw_flags.set_depth_mode(true);
        }
    }

    fn draw_line(&mut self, from: &HavokPosition, to: &HavokPosition) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawLine>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_line)
                    .unwrap(),
            )
        };

        target(self, from, to);
    }

    fn draw_capsule(&mut self, top: &HavokPosition, bottom: &HavokPosition, radius: f32) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawCapsule>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_capsule)
                    .unwrap(),
            )
        };

        target(self, top, bottom, radius);
    }

    fn draw_sphere(&mut self, origin: &HavokPosition, radius: f32) {
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
        &mut self,
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

    fn draw_triangle(&mut self, triangle: &Triangle) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawTriangle>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_triangle)
                    .unwrap(),
            )
        };

        target(self, triangle);
    }

    fn draw_dodecadron(&mut self, top: &HavokPosition, bottom: &HavokPosition, radius: f32) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawDodecadron>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_dodecadron)
                    .unwrap(),
            )
        };

        target(self, top, bottom, radius);
    }
}
