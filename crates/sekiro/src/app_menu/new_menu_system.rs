use std::{borrow::Cow, ptr::NonNull, sync::atomic::AtomicI32};

use shared::{FromStatic, OwnedPtr, Subclass, Superclass, UnknownPtr, UnknownStruct};

use crate::{Vector, dlkr::DLAllocatorRef, dlut::DLFixedVector, rva, sprj::SprjScaleformValue};

#[repr(C)]
// Source of name: RTTI
pub struct NewMenuSystem {
    pub vftable: usize,
    _array_menu_window_job_1: usize,
    _unk10: DLFixedVector<u64, 4>,
    pub windows: DLFixedVector<OwnedPtr<MenuWindow>, 16>,
    _array_menu_window_job_2: usize,
    _unkd8: u64,
    _menu_window_job_1: UnknownPtr,
    _menu_window_job_2: UnknownPtr,
    _menu_window_job_3: UnknownPtr,
    _menu_window_job_4: UnknownPtr,
    finalize_callback_job: UnknownPtr,
    _unk108: u64,
    _unk110: u64,
    _unk118: u64,
    _unk120: u64,
    _menu_window_job_5: UnknownPtr,
    _unk130: [u8; 0x38],
    _unk168: u64,
    _unk170: DLFixedVector<UnknownPtr, 8>,
    _unk1c0: u32,
    _unk1c4: u32,
    _unk1c8: u32,
    _unk1d0: u64,
    _unk1d8: u16,
    _unk1da: u8,
    _unk1e0: UnknownStruct<0x40e0>,
    _unk3428: UnknownStruct<0xc3428>,
    _unkc76e8: [u8; 0x6c1c8],
}

impl NewMenuSystem {
    /// Returns whether an in-game menu (including bonfires, shops, and so on)
    /// is currently open. This is always false on the main menu, even if a
    /// settings sub-menu is open.
    pub fn is_menu_open(&self) -> bool {
        // This is a function pointer for a callback used to clean up a menu.
        // It's always set for menus in the main game, but never for other
        // menus.
        !self.finalize_callback_job.is_null()
    }
}

impl FromStatic for NewMenuSystem {
    fn name() -> Cow<'static, str> {
        "NewMenuSystem".into()
    }

    unsafe fn instance() -> fromsoftware_shared::InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_indirect(rva::get().app_menu_new_menu_system_ptr) }
    }
}

#[repr(C)]
#[derive(Superclass)]
// Source of name: RTTI
pub struct MenuWindow {
    _vftable: usize,
    pub ref_count: AtomicI32,
    _unk10: u64,
    _unk18: [u8; 0x28],
    _unk40: u64,
    _unk48: [u8; 0x88],
    _unkd0: u64,
    _unkd8: [u8; 0x38],
    _unk110: u64,
    _unk118: [u8; 0x38],
    _unk150: u64,
    _scene_obj_proxy_1: SceneObjProxy,
    _scene_obj_proxy_2: SceneObjProxy,
    _unk218: u64,
    _allocator: DLAllocatorRef,
    _unk228: u64,
    _unk230: u64,
    _unk238: u64,
    _unk240: Vector<u8>,
    _component_holder: ComponentProxy,
    _scaleform_value: SprjScaleformValue,
    _scene_obj_proxy_3: SceneObjProxy,
    _scene_obj_proxy_4: SceneObjProxy,
    _scene_obj_proxy_5: SceneObjProxy,
    _scene_obj_proxy_6: SceneObjProxy,
    _unk428: u8,
    _scene_obj_proxy_7: SceneObjProxy,
    _unk4a0: [u8; 0x608],
    _unkaa8: u64,
    _unkab0: u64,
}

#[repr(C)]
#[derive(Superclass)]
#[superclass(children(SceneObjProxy))]
// Source of name: RTTI
pub struct ComponentProxy {
    _vftable: usize,
    _unk08: NonNull<ComponentProxy>,
    _unk10: NonNull<ComponentProxy>,
    _unk18: NonNull<ComponentProxy>,
}

#[repr(C)]
#[derive(Subclass)]
// Source of name: RTTI
pub struct SceneObjProxy {
    pub component_proxy: ComponentProxy,
    _scene_proxy: UnknownPtr,
    pub scaleform_value: SprjScaleformValue,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x20, size_of::<ComponentProxy>());
        assert_eq!(0x60, size_of::<SceneObjProxy>());
        assert_eq!(0xab8, size_of::<MenuWindow>());
        assert_eq!(0x1338b0, size_of::<NewMenuSystem>());
    }
}
