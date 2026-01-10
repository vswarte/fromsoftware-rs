use std::ptr::NonNull;

use windows::core::PCWSTR;

use crate::{Vector, cs::ChrIns};

#[repr(C)]
/// Source of name: RTTI
pub struct CSChrModelParamModifierModule {
    vftable: usize,
    pub owner: NonNull<ChrIns>,
    pub modifiers: Vector<CSChrModelParamModifierModuleEntry>,
}

#[repr(C)]
pub struct CSChrModelParamModifierModuleEntry {
    unk0: u8,
    unk1: [u8; 0x3],
    unk4: u32,
    unk8: u32,
    unkc: u32,
    unk10: u64,
    unk18: u32,
    unk1c: u32,
    pub name: PCWSTR,
    unk28: CSChrModelParamModifierModuleEntryValue,
    unk40: CSChrModelParamModifierModuleEntryValue,
    unk58: CSChrModelParamModifierModuleEntryValue,
    unk70: u32,
    unk74: u32,
    unk78: u32,
    unk7c: u32,
    unk80: u64,
    unk88: CSChrModelParamModifierModuleEntryValue,
    unka0: CSChrModelParamModifierModuleEntryValue,
    unkb0: [u8; 0x20],
}

#[repr(C)]
pub struct CSChrModelParamModifierModuleEntryValue {
    unk0: u32,
    pub value1: f32,
    pub value2: f32,
    pub value3: f32,
    pub value4: f32,
    unk14: u32,
}


