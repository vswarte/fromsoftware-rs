use std::ptr::NonNull;

use bitfield::bitfield;
use pelite::pe64::Pe;
use shared::program::Program;
use shared::{F32Vector4, OwnedPtr, Triangle};

use crate::dlkr::{DLAllocatorBase, DLPlainLightMutex};
use crate::position::{HavokPosition, PositionDelta};
use crate::rva;

#[repr(C)]
#[shared::singleton("RendMan")]
pub struct RendMan {
    vftable: usize,
    allocator: usize,
    stage_rend: usize,
    gx_sg_layer_flat: usize,
    unk20: usize,
    pub debug_ez_draw: OwnedPtr<CSEzDraw>,
    // TODO: rest
}

#[repr(C)]
pub struct CSEzDraw {
    vftable: usize,
    pub draw_context: OwnedPtr<FD4HkEzDrawContext>,
    /// Double buffered command buffers for rendering
    /// one is being written to while the other is being read by the GPU
    draw_command_buffers: [OwnedPtr<FD4HkEzDrawCommandBuffer>; 2],
    /// Index of the current writeable command buffer (0 or 1)
    pub current_buffer_index: u32,
    /// Lock to make writing to the command buffer thread-safe
    pub command_queue_lock: DLPlainLightMutex,
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

impl CSEzDraw {
    pub fn current_buffer(&self) -> &FD4HkEzDrawCommandBuffer {
        &self.draw_command_buffers[self.current_buffer_index as usize]
    }

    pub fn current_buffer_mut(&mut self) -> &mut FD4HkEzDrawCommandBuffer {
        &mut self.draw_command_buffers[self.current_buffer_index as usize]
    }

    /// Set the fill and line color for the to-be-rendered primitives.
    pub fn set_color(&mut self, color: &F32Vector4) {
        self.set_line_color(color);
        self.set_fill_color(color);
    }

    /// Set the line color for the to-be-rendered primitives.
    pub fn set_line_color(&mut self, color: &F32Vector4) {
        let buffer = self.current_buffer_mut();
        if buffer.ez_draw_state.base.line_color != *color {
            buffer.ez_draw_state.base.line_color = *color;
            buffer.ez_draw_state.base.draw_flags.set_line_color(true);
        }
    }

    /// Set the fill color for the to-be-rendered primitives.
    pub fn set_fill_color(&mut self, color: &F32Vector4) {
        let buffer = self.current_buffer_mut();
        if buffer.ez_draw_state.base.fill_color != *color {
            buffer.ez_draw_state.base.fill_color = *color;
            buffer.ez_draw_state.base.draw_flags.set_fill_color(true);
        }
    }

    /// Set the fill mode for the to-be-rendered primitives.
    pub fn set_fill_mode(&mut self, mode: EzDrawFillMode) {
        let buffer = self.current_buffer_mut();
        if buffer.ez_draw_state.base.fill_mode != mode {
            buffer.ez_draw_state.base.fill_mode = mode;
            buffer.ez_draw_state.base.draw_flags.set_fill_mode(true);
        }
    }

    /// Set the depth mode for the to-be-rendered primitives.
    pub fn set_depth_mode(&mut self, mode: u32) {
        let buffer = self.current_buffer_mut();

        if buffer.ez_draw_state.base.depth_mode != mode {
            buffer.ez_draw_state.base.depth_mode = mode;
            buffer.ez_draw_state.base.draw_flags.set_depth_mode(true);
        }
    }

    pub fn draw_line(&mut self, from: &HavokPosition, to: &HavokPosition) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawLine>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_line)
                    .unwrap(),
            )
        };

        target(self, from, to);
    }

    pub fn draw_capsule(&mut self, top: &HavokPosition, bottom: &HavokPosition, radius: f32) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawCapsule>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_capsule)
                    .unwrap(),
            )
        };

        target(self, top, bottom, radius);
    }

    pub fn draw_sphere(&mut self, origin: &HavokPosition, radius: f32) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawSphere>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_sphere)
                    .unwrap(),
            )
        };

        target(self, origin, radius);
    }

    pub fn draw_wedge(
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

    pub fn draw_triangle(&mut self, triangle: &Triangle) {
        let target = unsafe {
            std::mem::transmute::<u64, FnDrawTriangle>(
                Program::current()
                    .rva_to_va(rva::get().cs_ez_draw_draw_triangle)
                    .unwrap(),
            )
        };

        target(self, triangle);
    }

    pub fn draw_dodecadron(&mut self, top: &HavokPosition, bottom: &HavokPosition, radius: f32) {
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

#[repr(C)]
pub struct FD4HkEzDrawCommandBuffer {
    vftable: usize,
    pub buffer_allocator: NonNull<DLAllocatorBase>,
    pub initial_size: usize,
    pub capacity: usize,
    pub buffer_start: NonNull<u8>,
    pub write_ptr: NonNull<u8>,
    pub draw_state_allocator: NonNull<DLAllocatorBase>,
    pub ez_draw_context: NonNull<FD4HkEzDrawContext>,
    pub ez_draw_state: OwnedPtr<FD4HkEzDrawState>,
}

#[repr(C)]
pub struct FD4HkEzDrawContext {
    vftable: usize,
    unk8: usize,
    unk10: usize,
    pub ez_draw_state: NonNull<FD4HkEzDrawState>,
    unk20: usize,
    unk28: bool,
    unk2c: u32,
    unk30: NonNull<DLAllocatorBase>,
}

#[repr(C)]
pub struct FD4EzDrawState {
    pub vftable: usize,
    unk8: usize,
    /// Current draw flags, each setting should set it's respective bits
    /// to take effect
    pub draw_flags: EzDrawFlags,
    unk14: u32,
    unk18: u32,
    unk1c: u32,
    unk20: u32,
    pub depth_mode: u32,
    /// Fill mode used for drawing
    pub fill_mode: EzDrawFillMode,
    unk28: u32,
    unk30: u32,
    unk34: u8,
    /// Color used for outline when drawing
    pub line_color: F32Vector4,
    /// Color used for filling when drawing
    /// will use line_color if not set
    pub fill_color: F32Vector4,
    unk96: F32Vector4,
    unk112: F32Vector4,
    unk128: F32Vector4,
    /// Mod for text position interpretation
    pub text_coord_mode: EzDrawTextCoordMode,
    /// Color used for text rendering
    pub text_color: DlColor32,
    /// Font size for text rendering
    pub font_size: f32,
    /// Scale for text position x
    pub text_pos_width_scale: f32,
    /// Scale for text position y
    pub text_pos_height_scale: f32,
    unka4: u32,
    unka8: u32,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct DlColor32(u32);
    impl Debug;
    u8;
    pub r, set_r: 7, 0;
    pub g, set_g: 15, 8;
    pub b, set_b: 23, 16;
    pub a, set_a: 31, 24;
}

impl DlColor32 {
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        let mut color = DlColor32(0);
        color.set_r(r);
        color.set_g(g);
        color.set_b(b);
        color.set_a(a);
        color
    }
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EzDrawFillMode {
    /// Filled polygons
    Fill = 0,
    /// Only polygon edges
    Wireframe = 1,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EzDrawTextCoordMode {
    /// Coordinates are in screen space
    /// will be scaled by text_pos_width_scale and text_pos_height_scale
    ScreenSpace0 = 0,
    /// Same as 0
    ScreenSpace1 = 1,
    /// Coordinates are in world space
    /// as x,y,z HavokPosition
    HavokPosition2 = 2,
    /// Same as 2
    HavokPosition3 = 3,
    /// Coordinates are in normalized screen space
    /// relative to 1920x1080 canvas
    /// will be scaled down for screen resolution
    Normalized1080p = 4,
    /// Coordinates are in normalized screen space
    /// relative to 4096x2160 canvas
    /// will be scaled down for screen resolution
    Normalized4k = 5,
}

bitfield! {
    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct EzDrawFlags(u32);
    impl Debug;

    unk18, set_unk18: 0;
    unk18_1c, set_unk18_1c: 1;
    unk20, set_unk20: 2;
    pub depth_mode, set_depth_mode: 3;
    pub fill_mode, set_fill_mode: 4;
    unk2c, set_unk2c: 5;
    unk30, set_unk30: 6;
    unk34, set_unk34: 7;
    pub line_color, set_line_color: 8;
    pub fill_color, set_fill_color: 9;
    unk60, set_unk60: 10;
    pub text_coord_mode, set_text_coord_mode: 11;
    pub font_size, set_font_size: 12;
    pub reset_text_pos_scale, set_reset_text_pos_scale: 13;
    pub text_color, set_text_color: 14;
    unk_a4, set_unk_a4: 15;
}

impl EzDrawFlags {
    pub fn all() -> Self {
        EzDrawFlags(0xFFFF_FFFF)
    }
}

#[repr(C)]
pub struct FD4HkEzDrawState {
    pub base: FD4EzDrawState,
    unkb0: u32,
    unkc0: F32Vector4,
    unkd0: F32Vector4,
    unke0: f32,
    unkf0: F32Vector4,
    unk100: F32Vector4,
    unk110: f32,
    unk120: F32Vector4,
    unk130: F32Vector4,
    unk140: f32,
    unk150: F32Vector4,
    unk160: F32Vector4,
    unk170: f32,
    unk180: F32Vector4,
    unk190: F32Vector4,
    unk1a0: f32,
    unk1b0: F32Vector4,
    unk1c0: F32Vector4,
    unk1d0: f32,
    unk1d4: [u8; 0xc],
    unk1e0: u32,
    unk1e4: [u8; 0x1c],
}

impl AsRef<FD4EzDrawState> for FD4HkEzDrawState {
    fn as_ref(&self) -> &FD4EzDrawState {
        &self.base
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size_of() {
        assert_eq!(std::mem::size_of::<CSEzDraw>(), 0x58);
        assert_eq!(std::mem::size_of::<FD4HkEzDrawCommandBuffer>(), 0x48);
        assert_eq!(std::mem::size_of::<FD4HkEzDrawContext>(), 0x38);
        assert_eq!(std::mem::size_of::<FD4HkEzDrawState>(), 0x200);
        assert_eq!(std::mem::size_of::<FD4EzDrawState>(), 0xb0);
    }
}
