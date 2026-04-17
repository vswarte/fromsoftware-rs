use pelite::pe64::{Pe, PeView};
use std::sync::LazyLock;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::core::PCSTR;

mod bundle;
mod rva_jp;
mod rva_ww;

pub use bundle::RvaBundle;

const NAME: &str = "ELDEN RING™";

const LANG_ID_EN: u16 = 0x0009;
const LANG_ID_JP: u16 = 0x0011;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GameVersion {
    Ww261,
    Jp2611,
}

impl GameVersion {
    fn from_metadata(product: &str, lang_id: u16, version: &str) -> Option<Self> {
        match (product, lang_id, version) {
            (NAME, LANG_ID_EN, "2.6.1.0") => Some(Self::Ww261),
            (NAME, LANG_ID_JP, "2.6.1.1") => Some(Self::Jp2611),
            _ => None,
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
        detect_version_and_get_rvas(&module)
    });

    &RVAS
}

/// Determines the region and version of the current executable and returns the
/// [RvaBundle] for it. Panics if the version isn't known.
fn detect_version_and_get_rvas(module: &PeView) -> RvaBundle {
    let resources = module.resources().unwrap();
    let info = resources.version_info().unwrap();

    // Extract version info
    let product_version = info
        .fixed()
        .expect("Executable doesn't contain version metdata")
        .dwProductVersion;
    let version = format!(
        "{}.{}.{}.{}",
        product_version.Major, product_version.Minor, product_version.Patch, product_version.Build,
    );

    // Extract product name
    let language = *info
        .translation()
        .first()
        .expect("Executable doesn't contain language metdata");
    let mut product_name: Option<String> = None;
    info.strings(language, |k, v| {
        if k == "ProductName" {
            product_name = Some(v.to_string());
        }
    });

    let product = product_name.expect("Executable doesn't contain product name metadata");
    if product != NAME {
        panic!(
            "Expected executable name to be \"{}\", was \"{}\"",
            NAME, &product
        );
    }

    let lang_id_base = language.lang_id & 0x03FF;
    if lang_id_base != LANG_ID_EN && lang_id_base != LANG_ID_JP {
        panic!(
            "Expected executable language ID to be {:#04x} or {:#04x}, was {:#04x}",
            LANG_ID_EN, LANG_ID_JP, lang_id_base
        );
    }

    // Detect version and return appropriate RVAs
    let version = GameVersion::from_metadata(&product, lang_id_base, &version)
        .unwrap_or_else(|| panic!("Unsupported game version {}", &version));
    RvaBundle::for_version(version)
}

impl RvaBundle {
    fn for_version(version: GameVersion) -> Self {
        match version {
            GameVersion::Ww261 => rva_ww::RVAS,
            GameVersion::Jp2611 => rva_jp::RVAS,
        }
    }
}
