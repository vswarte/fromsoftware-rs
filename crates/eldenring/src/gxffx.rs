use std::ptr::NonNull;

use crate::DoublyLinkedList;
use shared::OwnedPtr;

#[repr(C)]
pub struct FxrWrapper {
    pub fxr: usize, // Pointer to the FXR object
    unk: u64,
}

#[repr(C)]
pub struct FxrListNode {
    pub id: u32,
    _pad14: u32,
    pub fxr_wrapper: OwnedPtr<FxrWrapper>,
}

#[repr(C)]
pub struct FxrResourceContainer {
    pub allocator1: u64,
    pub scene_ctrl: NonNull<OwnedPtr<GXFfxSceneCtrl>>,
    unk10: u64,
    pub fxr_definitions: DoublyLinkedList<FxrListNode>,
}

#[repr(C)]
pub struct GXFfxGraphicsResourceManager {
    pub vftable: u64,
    unk: [u8; 0x158],
    pub resource_container: OwnedPtr<FxrResourceContainer>,
}

#[repr(C)]
pub struct GXFfxSceneCtrl {
    pub vftable: u64,
    pub sg_entity: u64,
    pub allocator: u64,
    pub ffx_manager: u64,
    unk: u64,
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
