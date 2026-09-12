use shared::OwnedPtr;

#[repr(C)]
/// Part of the DLRF namespace, describes some aspects of a tracked class.
///
/// Source of name: RTTI
pub struct DLRuntimeClass {
    vftable: usize,
    pub base_class: OwnedPtr<DLRuntimeClass>,
    unk10: usize,
    unk18: usize,
    unk20: usize,
    unk28: usize,
    unk30: usize,
    allocator1: usize,
    allocator2: usize,
}

#[repr(C)]
pub struct DLRuntimeClassImpl {
    pub runtime_class: DLRuntimeClass,
    pub name: *const u8,
    pub name_w: *const u16,
}
