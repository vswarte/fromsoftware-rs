use vtable_rs::VPtr;

use crate::fd4::FD4SlotInsBaseVmt;

#[repr(C)]
pub struct ChrSlotBase {
    pub vftable: VPtr<dyn ChrSlotBaseVmt, Self>,
}

#[vtable_rs::vtable]
pub trait ChrSlotBaseVmt: FD4SlotInsBaseVmt {
    fn unk28(&mut self) -> bool;

    fn unk30(&mut self) -> bool;
}
