use std::ptr::NonNull;

use crate::{
    DLList,
    dlkr::{DLAllocator, GfxHeapAllocator},
};
use shared::OwnedPtr;

#[repr(C)]
pub struct FxrWrapper {
    fxr: usize,
    unk: usize,
}

#[repr(C)]
pub struct FxrListNode {
    pub id: u32,
    pub fxr_wrapper: NonNull<FxrWrapper>,
}

#[repr(C)]
pub struct FxrResourceContainer {
    pub allocator: &'static DLAllocator,
    pub scene_ctrl: NonNull<GXFfxSceneCtrl>,
    unk10: usize,
    pub fxr_definitions: DLList<FxrListNode>,
}

#[repr(C)]
pub struct GXFfxGraphicsResourceManager {
    vftable: usize,
    unk: [u8; 0x158],
    pub resource_container: OwnedPtr<FxrResourceContainer, GfxHeapAllocator>,
}

#[repr(C)]
pub struct GXFfxSceneCtrl {
    vftable: usize,
    sg_entity: usize,
    pub allocator: &'static DLAllocator,
    ffx_manager: usize,
    unk: usize,
    pub graphics_resource_manager: NonNull<GXFfxGraphicsResourceManager>,
}

#[cfg(test)]
mod test {
    use crate::gxffx::{
        FxrListNode, FxrResourceContainer, FxrWrapper, GXFfxGraphicsResourceManager, GXFfxSceneCtrl,
    };
    use std::mem::size_of;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x10, size_of::<FxrWrapper>());
        assert_eq!(0x10, size_of::<FxrListNode>());
        assert_eq!(0x30, size_of::<FxrResourceContainer>());
        assert_eq!(0x168, size_of::<GXFfxGraphicsResourceManager>());
        assert_eq!(0x30, size_of::<GXFfxSceneCtrl>());
    }
}
