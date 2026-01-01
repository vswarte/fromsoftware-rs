use std::borrow::Cow;

use bitfield::bitfield;

use crate::rva;
use shared::{FromStatic, InstanceResult};

bitfield! {
    /// The handle providing information about a single gesture in the player's
    /// inventory.
    pub struct GestureHandle(u32);
    impl Debug;

    /// Whether the player currently has this gesture.
    bool, acquired, set_acquired: 0;

    /// The index of the gesture's data in the [GestureDataStore]. This is
    /// **not** the same as the gesture's index that's used in EMEVD.
    u16, index, _: 15, 1;

    /// The index of this handle in the [GestureGameData] that contains it.
    /// Always 1 less than [index].
    u16, local_index, _: 31, 16;
}

// Source of name: RTTI
#[repr(C)]
pub struct GestureGameData {
    _vftable: usize,
    _unk08: u64,

    /// Data about each gesture in the game.
    pub gestures: [GestureHandle; 40],

    _unkb0: u64,
}

impl GestureGameData {
    /// Returns whether the player has the gesture with the given index.
    pub fn has_gesture(&self, gesture_index: u32) -> bool {
        let id = gesture_index + 22;
        if let Ok(data_store) = unsafe { GestureDataStore::instance() } {
            for handle in &self.gestures {
                if data_store.entries[handle.index() as usize].id == id {
                    return handle.acquired();
                }
            }
        }

        false
    }

    /// Setss whether the player has the gesture with the given index. Returns
    /// whether the gesture was found.
    pub fn set_gesture_acquired(&mut self, gesture_index: u32, acquired: bool) -> bool {
        let id = gesture_index + 22;
        if let Ok(data_store) = unsafe { GestureDataStore::instance() } {
            for handle in &mut self.gestures {
                if data_store.entries[handle.index() as usize].id == id {
                    handle.set_acquired(acquired);
                    return true;
                }
            }
        }

        false
    }
}

/// A static, global store of information about each gesture in the game.
#[repr(C)]
pub struct GestureDataStore {
    /// The contents of the store. The first entry contains meaningless data.
    /// Note that these are *not* in ID order.
    pub entries: [GestureDataStoreEntry; 41],
}

impl FromStatic for GestureDataStore {
    fn name() -> Cow<'static, str> {
        "GestureDataStore".into()
    }

    unsafe fn instance() -> InstanceResult<&'static mut Self> {
        unsafe { shared::load_static_direct(rva::get().gesture_data_store) }
    }
}

/// An entry describing global properties of a gesture.
#[repr(C)]
pub struct GestureDataStoreEntry {
    /// The icon to display for this gesture.
    pub icon_id: u32,

    _unk04: u32,

    /// The gesture's ID. This is always the gesture's EMEVD index plus 22.
    pub id: u32,

    _unkc: u32,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn proper_sizes() {
        assert_eq!(0x10, size_of::<GestureDataStoreEntry>());
    }
}
