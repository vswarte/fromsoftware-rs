use std::ptr::NonNull;

use crate::{Pair, Tree, Vector};

#[repr(C)]
pub struct FD4BaseKeyAssign {
    pub vftable: *const (),
    allocator: *const (),
    input_mapper: *const (),
    virtual_input_data_index_vector: Vector<Unk18VectorItem>,
    /// Takes the result from the [InputTypeGroup] and maps it to an index to the [DLVirtualInputData].
    pub virtual_input_data_index_map: NonNull<Tree<Pair<i32, i32>>>,
    /// See field 0x78 of MultiDevices.
    ///
    /// Still dont have a name for the struct it points to.
    pub unk78_index_map: Tree<Pair<i32, i32>>,
    /// Contains the same pointer that the [DLFixedVector<>] in `FD4PadManager.unka8` has.
    unk58: *const (),
    unk60: u32,
    unk64: u32,
    unk68: u32,
    unk6c: u32,
    unk70: u32,
    padding: [u8; 4],
}

#[repr(C)]
struct Unk18VectorItem {
    /// Key referenced inside `virtual_input_data_index_map`.
    pub key: i32,
    /// Value referenced inside `virtual_input_data_index_map`.
    pub value: i32,
    pub unk8: i32,
    pub unkc: i32,
    pub unk10: i32,
}
