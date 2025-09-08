use std::ptr::NonNull;

use shared::{F32Matrix4x4, F32Vector3, F32Vector4, OwnedPtr};

use crate::{cs::BlockId, Tree};

#[repr(C)]
pub struct AutoInvadePoint {
    pub position: F32Vector3,
    pub yaw: f32,
}

#[repr(C)]
pub struct AutoInvadePointBlockEntry {
    pub block_id: BlockId,
    // _pad4: [u8; 4],
    pub count: usize,
    head: OwnedPtr<AutoInvadePoint>,
}

impl AutoInvadePointBlockEntry {
    pub fn items(&self) -> &[AutoInvadePoint] {
        unsafe { std::slice::from_raw_parts(self.head.as_ptr(), self.count) }
    }
}

#[repr(C)]
/// Source of name: dlrf
/// Holds the list of automatically generated (by fromsoftware)
/// invasion points, when current region param has isAutoIntrudePoint set to true, game will use one of these
/// instead of looking for the point on msb
#[dlrf::singleton("CSAutoInvadePoint")]
pub struct CSAutoInvadePoint {
    pub entries: Tree<AutoInvadePointBlockEntry>,
    unk18: [u8; 0x28],
    unk40: F32Vector4,
    unk50: [u8; 0x20],
}
