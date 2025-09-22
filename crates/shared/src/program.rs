use std::sync::LazyLock;

use pelite::pe64::{Pe, PeFile, PeObject, PeView};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::core::PCSTR;

#[derive(Copy, Clone)]
pub enum Program<'a> {
    File(PeFile<'a>),
    Mapping(PeView<'a>),
}

static CURRENT_BASE: LazyLock<Program> = LazyLock::new(|| {
    let module = unsafe { GetModuleHandleA(PCSTR(std::ptr::null())).unwrap().0 } as *const u8;
    Program::Mapping(unsafe { PeView::module(module) })
});

impl Program<'_> {
    /// Returns the currently running programing.
    pub fn current() -> Self {
        *CURRENT_BASE
    }
}

unsafe impl<'a> Pe<'a> for Program<'a> {}
unsafe impl<'a> PeObject<'a> for Program<'a> {
    fn image(&self) -> &'a [u8] {
        match self {
            Self::File(file) => file.image(),
            Self::Mapping(mapping) => mapping.image(),
        }
    }

    fn align(&self) -> pelite::Align {
        match self {
            Self::File(file) => file.align(),
            Self::Mapping(mapping) => mapping.align(),
        }
    }

    fn image_base(&self) -> pelite::pe64::Va {
        self.optional_header().ImageBase
    }
}
