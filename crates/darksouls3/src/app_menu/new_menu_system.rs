use std::{borrow::Cow, ptr::NonNull};

use shared::{FromStatic, Subclass, Superclass, UnknownStruct};

use super::{GaitemSelectBaseMenu, GaitemSelectMenu};
use crate::{CxxVec, dlut::DLFixedVector, rva, sprj::SprjScaleformValue};

#[repr(C)]
// Source of name: RTTI
pub struct NewMenuSystem {
    _vftable: usize,
    _array_menu_window_job_1: usize,
    _unk10: [u8; 0x30],
    pub windows: DLFixedVector<NonNull<MenuWindow>, 8>,
    _menu_window_job_1: usize,
    _unk98: u64,
    finalize_callback_job: usize,
    _unka8: u64,
    _unkb0: u64,
    _unkb8: u64,
    _unkc0: u64,
    _menu_window_job_2: usize,
    _unkd0: [u8; 0x18],
    _callback: usize,
    _finalize_callback_jobs: DLFixedVector<usize, 8>,
    _unk140: bool,
    _unk144: u32,
    _unk148: u32,
    _unk150: u64,
    _unk158: u16,
    _unk160: UnknownStruct<0x2ec8>,
    _fe_emergency_notice: usize,
    _fe_summon_message: usize,
    _fade_screen: usize,
    _fe_view: usize,
    _unk3048: [u8; 0x28],
    _unk3070: u64,
    _array_menu_window_job_2: usize,
    _unk3080: u8,
    _unk3081: u8,
    _unk3082: u8,
    _unk3084: u16,
    _unk3088: u64,
    _unk3090: u32,
}

impl NewMenuSystem {
    /// Returns whether an in-game menu (including bonfires, shops, and so on)
    /// is currently open. This is always false on the main menu, even if a
    /// settings sub-menu is open.
    pub fn is_menu_open(&self) -> bool {
        // This is a function pointer for a callback used to clean up a menu.
        // It's always set for menus in the main game, but never for other
        // menus.
        self.finalize_callback_job != 0
    }

    /// Iterates over the currently active windows in the menu system.
    pub fn windows(&self) -> impl Iterator<Item = &MenuWindow> {
        // Safety: This is safe only on the presumption that because we have an
        // immutable reference to `self`, no code (including C++ code) is
        // mutating it during the lifetime of the references we return. That
        // should should be true, since the game does all menu logic on the main
        // thread.
        self.windows.iter().map(|p| unsafe { p.as_ref() })
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
#[superclass(children(GaitemSelectBaseMenu, GaitemSelectMenu))]
// Source of name: RTTI
pub struct MenuWindow {
    pub vftable: usize,
    _unk08: u32,
    _fix_order_job_sequence: usize,
    _unk18: [u8; 0x28],
    _unk40: u64,
    _scene_obj_modifiers: DLFixedVector<usize, 8>,
    _callback1: MenuWindowCallback,
    _callback2: MenuWindowCallback,
    _unkd8: SceneObjProxy,
    _unk138: SceneObjProxy,
    _unk198: u64,
    _unk1a0: CxxVec<u64>,
    _unk1c0: CxxVec<u64>,
    _component_holder: usize,
    _unk1e8: [u8; 0x18],
    _unk200: SprjScaleformValue,
    _unk238: SceneObjProxy,
    _unk298: SceneObjProxy,
    _unk2f8: SceneObjProxy,
    _unk358: u8,
    _unk360: SceneObjProxy,
    _unk3c0: [u8; 0x608],
    _unk9c8: u64,
    _unk9d0: u64,
}

#[repr(C)]
pub struct MenuWindowCallback {
    _vftable: usize,

    /// The window that owns this callback.
    pub menu_window: NonNull<MenuWindow>,

    _unk10: [u8; 0x8],
    _this: NonNull<MenuWindowCallback>,
}

#[repr(C)]
// Source of name: RTTI
pub struct SceneObjProxy {
    _vftable: usize,
    _unk08: [u8; 0x18],
    _scene_holder: usize,
    pub scaleform_value: SprjScaleformValue,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x60, size_of::<SceneObjProxy>());
        assert_eq!(0x9d8, size_of::<MenuWindow>());
        assert_eq!(0x3098, size_of::<NewMenuSystem>());
    }
}
