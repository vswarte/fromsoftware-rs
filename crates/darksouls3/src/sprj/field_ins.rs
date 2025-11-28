use std::{fmt, mem};

use bitfield::bitfield;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq, Eq, Hash, Debug)]
pub enum FieldInsType {
    Hit = 0,
    Chr = 1,
    Obj = 2,
    Bullet = 3,
}

bitfield! {
    /// A selector that encodes the information to look up a specific FieldIns
    /// managed by its respective (external) domain.
    #[derive(Copy, Clone, PartialEq, Eq, Hash)]
    pub struct FieldInsSelector(u32);

    /// The index of this FieldIns in its container.
    pub u32, index, _: 19, 0;
    u32, _, set_index: 19, 0;

    /// The index of the container that holds this FieldIns.
    pub u32, container, _: 27, 20;
    u32, _, set_container: 27, 20;

    u8, type_raw, set_type_raw: 31, 28;
}

impl FieldInsSelector {
    /// Creates a new FieldInsSelector from its components.
    pub fn new(field_ins_type: FieldInsType, container: u32, index: u32) -> Self {
        let mut selector = FieldInsSelector(0);
        selector.set_type_raw(field_ins_type as u8);
        selector.set_container(container);
        selector.set_index(index);
        selector
    }

    /// The type of FieldIns this represents, which also indicates which object
    /// to look it up in.
    pub fn field_ins_type(&self) -> FieldInsType {
        // Safety: Rust can't construct an invalid selector, and we don't know
        // any game APIs that provide one.
        unsafe { mem::transmute(self.type_raw()) }
    }
}

impl fmt::Debug for FieldInsSelector {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.type_raw() > 3 {
            write!(f, "<invalid 0x{:x}>", self.0)
        } else {
            write!(
                f,
                "{:?}:{:x}:{:x}",
                self.field_ins_type(),
                self.container(),
                self.index()
            )
        }
    }
}
