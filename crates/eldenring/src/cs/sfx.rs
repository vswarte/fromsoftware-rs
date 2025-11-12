use crate::gxffx::GXFfxSceneCtrl;
use crate::{Tree, Vector};
use fromsoftware_shared::{F32Matrix4x4, OwnedPtr};

#[repr(C)]
/// Source of name: RTTI
#[fromsoftware_shared::singleton("CSSfx")]
pub struct CSSfxImp {
    vftable: usize,
    unk8: usize,
    unk10: Tree<()>,
    unk28: Tree<()>,
    unk40: usize,
    unk48: usize,
    unk50: usize,
    unk58: usize,
    pub scene_ctrl: OwnedPtr<GXFfxSceneCtrl>,
    unk68: usize,
    unk70: usize,
    unk78: usize,
    unk80: usize,
    unk88: bool,
    unk89: bool,
    unk8a: bool,
    unk8b: bool,
    unk8c: u32,
    unk90: u32,
    unk94: u32,
    unk98: u32,
    unk9c: u32,
    unka0: u32,
    unka4: u32,
    unka8: u32,
    unkac: u32,
    unkb0: Tree<()>,
    unkc8: usize,
    unkd0: Vector<usize>,
    unkf0: Vector<usize>,
    unk110: usize,
    unk118: usize,
    unk120: f32,
    unk124: f32,
    unk128: bool,
    unk129: bool,
    unk12a: bool,
    unk12b: bool,
    pub debug_spawn_use_distance_from_camera: bool,
    unk12d: bool,
    unk12e: bool,
    unk12f: bool,
    pub debug_spawn_ffx_id: u32,
    pub debug_spawn_distance_from_camera: f32,
    unk138: [u8; 0x178],
}

#[cfg(test)]
mod test {
    use crate::cs::CSSfxImp;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x2b0, size_of::<CSSfxImp>());
    }
}
