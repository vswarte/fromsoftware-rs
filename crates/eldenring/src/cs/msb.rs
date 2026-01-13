use crate::fd4::{FD4FileCap, FD4ResCap, FD4ResRep};
use shared::{F32Vector3, ReadOnlyPtr, Subclass};

#[repr(C)]
/// Source of name: RTTI
#[shared::singleton("MsbRepository")]
#[derive(Subclass)]
#[subclass(base = FD4ResRep<MsbFileCap>, base = FD4ResCap)]
pub struct MsbRepository {
    pub res_rep: FD4ResRep<MsbFileCap>,
}

#[repr(C)]
#[derive(Subclass)]
#[subclass(base = FD4FileCap, base = FD4ResCap)]
pub struct MsbFileCap {
    pub file_cap: FD4FileCap,
}

#[repr(C)]
pub struct CSMsbPoint {
    vftable: isize,
    msb_res_cap: isize,
    unk10: isize,
    pub shape_data: ReadOnlyPtr<CSMsbPointShapeData>,
    unk20: isize,
    pub layer_mask: u32,
    unk30: isize,
    unk38: isize,
    unk40: isize,
    unk48: isize,
    unk50: bool,
}

#[repr(C)]
pub struct CSMsbPointShapeData {
    unk0: u32,
    unk4: u32,
    unk8: u32,
    unkc: u32,
    pub point_type: CSMsbPointShapeType,
    pub position: F32Vector3,
    pub rotation: F32Vector3,
    unk2c: [u8; 0x18],
    pub map_layer: i32,
}

#[repr(u32)]
#[derive(Debug)]
pub enum CSMsbPointShapeType {
    Point = 0x0,
    Circle = 0x1,
    Sphere = 0x2,
    Cylinder = 0x3,
    Rectangle = 0x4,
    Box = 0x5,
    Composite = 0x6,
}
