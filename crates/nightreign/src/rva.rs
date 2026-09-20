use pelite::pe64::PeView;
use shared::LANG_ID_EN;
use std::sync::LazyLock;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::core::PCSTR;

mod bundle;
mod rva_data;

pub use bundle::RvaBundle;

use fromsoftware_shared::game_version::GameVersion;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NigthreignGameVersion {
    Ww1330,
}

impl GameVersion for NigthreignGameVersion {
    const NAME: &'static str = "elden ring nightreign";

    fn from_lang_version(lang_id: u16, version: &str) -> Option<Self> {
        match (lang_id, version) {
            (LANG_ID_EN, "1.3.3.0") => Some(Self::Ww1330),
            _ => None,
        }
    }
}

impl NigthreignGameVersion {
    const fn rvas(self) -> RvaBundle {
        match self {
            Self::Ww1330 => rva_data::RVAS,
        }
    }
}

/// Returns the RVA bundle for the current executable region and version.
///
/// This will panic if the current executable isn't supported by this package.
pub fn get() -> &'static RvaBundle {
    static RVAS: LazyLock<RvaBundle> = LazyLock::new(|| {
        let module = unsafe {
            PeView::module(GetModuleHandleA(PCSTR(std::ptr::null())).unwrap().0 as *const u8)
        };
        NigthreignGameVersion::detect(&module)
            .unwrap_or_else(|e| panic!("{e}"))
            .rvas()
    });

    &RVAS
}
