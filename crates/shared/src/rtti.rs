use std::ptr::NonNull;

use pelite::pe64::{msvc::RTTICompleteObjectLocator, Pe, Rva, Va};
use undname::Flags;

use crate::program::Program;

// TODO: this cast to u32 is probably not going to cause panics but can be prettier.
const VA_SIZE: u32 = size_of::<Va>() as u32;

/// Builds an iterator that walks over the entire .rdata section looking for
/// recoverable classes.
pub fn find_rtti_classes<'a>(program: &'a Program) -> impl Iterator<Item = Class<'a>> + 'a {
    let text = program
        .section_headers()
        .by_name(".text")
        .expect("no .text section found");

    let rdata = program
        .section_headers()
        .by_name(".rdata")
        .expect("no .rdata section found");

    rdata
        .virtual_range()
        .step_by(size_of::<Va>())
        .filter_map(move |candidate_rva| {
            let vftable_meta_rva = candidate_rva;
            let vftable_rva = candidate_rva + VA_SIZE;

            let vftable_meta_rva = program
                .derva(vftable_meta_rva)
                .and_then(|va| program.va_to_rva(*va))
                .ok()?;

            let vftable_entry_rva = program
                .derva(vftable_rva)
                .and_then(|va| program.va_to_rva(*va))
                .ok()?;

            if rdata.virtual_range().contains(&vftable_meta_rva)
                && text.virtual_range().contains(&vftable_entry_rva)
            {
                let _: &RTTICompleteObjectLocator = program.derva(vftable_meta_rva).ok()?;

                Some((vftable_meta_rva, vftable_rva))
            } else {
                None
            }
        })
        .filter_map(|(meta, vftable)| {
            let col: &RTTICompleteObjectLocator = program.derva(meta).ok()?;

            let ty_name = program
                .derva_c_str(col.type_descriptor + 16)
                .ok()?
                .to_string();
            if !ty_name
                .chars()
                .all(|ch| (0x20..=0x7e).contains(&(ch as u8)))
            {
                return None;
            }

            let demangled = undname::demangle(ty_name.as_str(), Flags::NAME_ONLY)
                .map(|s| s.to_string())
                .ok()?;

            Some(Class {
                program,
                name: demangled,
                vftable,
            })
        })
}

// TODO: use better than usizes.
/// Attempts to extract the class name for a given vftable.
pub fn vftable_classname(program: &Program, vftable_va: usize) -> Option<String> {
    let vftable_rva = program.va_to_rva(vftable_va as u64).ok()?;
    let vftable_meta_rva = vftable_rva - VA_SIZE;

    let rdata = program
        .section_headers()
        .by_name(".rdata")
        .expect("no .rdata section found");

    let vftable_meta_rva = program
        .derva(vftable_meta_rva)
        .and_then(|va| program.va_to_rva(*va))
        .ok()?;

    if !rdata.virtual_range().contains(&vftable_meta_rva) {
        return None;
    }

    let col: &RTTICompleteObjectLocator = program.derva(vftable_meta_rva).ok()?;
    let ty_name = program
        .derva_c_str(col.type_descriptor + 16)
        .ok()?
        .to_string();
    if !ty_name
        .chars()
        .all(|ch| (0x20..=0x7e).contains(&(ch as u8)))
    {
        return None;
    }

    let demangled = undname::demangle(ty_name.as_str(), Flags::NAME_ONLY)
        .map(|s| s.to_string())
        .ok()?;

    Some(demangled)
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
#[allow(dead_code)]
struct RttiCandidate {
    vftable_meta_rva: Rva,
    vftable_rva: Rva,
}

pub struct Class<'a> {
    program: &'a Program<'a>,
    pub name: String,
    pub vftable: Rva,
}

impl Class<'_> {
    /// Retrieves the function pointer from the VMT.
    ///
    /// # Safety
    /// Does not validate whether or not the index is actually contained within the VMT.
    pub unsafe fn vmt_fn(&self, index: u32) -> Option<Va> {
        Some(*self.program.derva(self.vftable + VA_SIZE * index).ok()?)
    }

    /// Retrieves a mutable function pointer from the VMT.
    ///
    /// # Safety
    /// Does not validate whether or not the index is actually contained within the VMT.
    pub unsafe fn vmt_index(&self, index: u32) -> Option<NonNull<Va>> {
        let ptr = self
            .program
            .rva_to_va(self.vftable + VA_SIZE * index)
            .ok()? as *const u64 as *mut u64;

        NonNull::new(ptr)
    }
}
