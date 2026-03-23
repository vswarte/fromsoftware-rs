use std::ops::{Deref, DerefMut};

use shared::{F32Vector3, F32Vector4, OwnedPtr};

use crate::{DLMap, cs::BlockId};

#[repr(C)]
pub struct AutoInvadePoint {
    pub position: F32Vector3,
    pub yaw: f32,
}

#[repr(C)]
pub struct AutoInvadePointBlockEntry {
    pub count: usize,
    head: OwnedPtr<AutoInvadePoint>,
}

impl AutoInvadePointBlockEntry {
    pub fn items(&self) -> &[AutoInvadePoint] {
        unsafe { std::slice::from_raw_parts(self.head.as_ptr(), self.count) }
    }

    pub fn items_mut(&mut self) -> &mut [AutoInvadePoint] {
        unsafe { std::slice::from_raw_parts_mut(self.head.as_ptr(), self.count) }
    }
}

impl Deref for AutoInvadePointBlockEntry {
    type Target = [AutoInvadePoint];

    fn deref(&self) -> &Self::Target {
        self.items()
    }
}

impl DerefMut for AutoInvadePointBlockEntry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.items_mut()
    }
}

#[repr(C)]
/// Source of name: dlrf
/// Holds the list of automatically generated (by fromsoftware)
/// invasion points, when current region param has isAutoIntrudePoint set to true, game will use one of these
/// instead of looking for the point on msb
#[shared::singleton("CSAutoInvadePoint")]
pub struct CSAutoInvadePoint {
    pub entries: DLMap<BlockId, AutoInvadePointBlockEntry>,
    unk18: [u8; 0x28],
    unk40: F32Vector4,
    unk50: [u8; 0x20],
}

#[cfg(test)]
mod test {
    use crate::cs::{AutoInvadePoint, AutoInvadePointBlockEntry, CSAutoInvadePoint};

    #[test]
    fn proper_sizes() {
        assert_eq!(std::mem::size_of::<CSAutoInvadePoint>(), 0x70);
        assert_eq!(std::mem::size_of::<AutoInvadePointBlockEntry>(), 0x10);
        assert_eq!(std::mem::size_of::<AutoInvadePoint>(), 0x10);
    }
}
