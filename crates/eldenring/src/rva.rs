use pelite::pe64::{Pe, PeView};
use std::sync::LazyLock;
use windows::core::PCSTR;
use windows::Win32::System::LibraryLoader::GetModuleHandleA;

mod rva_jp;
mod rva_ww;

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
            ("ELDEN RING™", LANG_ID_EN, "2.6.1.0") => Some(Self::Ww261),
            ("ELDEN RING", LANG_ID_JP, "2.6.1.1") => Some(Self::Jp2611),
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
            .expect("This game version or distribution is not supported")
    });

    &RVAS
}

/// Determines the region and version of the current executable and, if it's
/// known, returns the [RvaBundle] for it.
fn detect_version_and_get_rvas(module: &PeView) -> Option<RvaBundle> {
    let resources = module.resources().ok()?;
    let info = resources.version_info().ok()?;

    // Extract version info
    let product_version = info.fixed()?.dwProductVersion;
    let version = format!(
        "{}.{}.{}.{}",
        product_version.Major, product_version.Minor, product_version.Patch, product_version.Build,
    );

    // Extract product name
    let language = *info.translation().first()?;
    let mut product_name: Option<String> = None;
    info.strings(language, |k, v| {
        if k == "ProductName" {
            product_name = Some(v.to_string());
        }
    });

    let product = product_name?;
    let lang_id_base = language.lang_id & 0x03FF;

    // Detect version and return appropriate RVAs
    let version = GameVersion::from_metadata(&product, lang_id_base, &version)?;
    Some(RvaBundle::for_version(version))
}

/// A struct containing offsets (relative to the beginning of the executable) of
/// various addresses of structures and functions. They can be converted to a
/// usable address using the [Pe::rva_to_va] trait function of [Program].
///
/// These are populated from `mapper-profile.toml` in the root of this package
/// using `tools/binary-generator`.
pub struct RvaBundle {
    pub cs_ez_draw_draw_line: u32,
    pub cs_ez_draw_draw_capsule: u32,
    pub cs_ez_draw_draw_sphere: u32,
    pub cs_ez_draw_draw_wedge: u32,
    pub cs_ez_draw_draw_triangle: u32,
    pub cs_ez_draw_draw_dodecadron: u32,
    pub initialize_spawn_geometry_request: u32,
    pub spawn_geometry: u32,
    pub cs_phys_world_cast_ray: u32,
    pub cs_bullet_manager_spawn_bullet: u32,
    pub chr_ins_apply_speffect: u32,
    pub chr_ins_remove_speffect: u32,
    pub cs_action_button_man_execute_action_button: u32,
    pub cs_menu_man_imp_display_status_message: u32,
    pub global_hinstance: u32,
    pub register_task: u32,
}

macro_rules! rva_bundle {
    ($module:ident) => {
        Self {
            cs_ez_draw_draw_line: $module::RVA_CS_EZ_DRAW_DRAW_LINE,
            cs_ez_draw_draw_capsule: $module::RVA_CS_EZ_DRAW_DRAW_CAPSULE,
            cs_ez_draw_draw_sphere: $module::RVA_CS_EZ_DRAW_DRAW_SPHERE,
            cs_ez_draw_draw_wedge: $module::RVA_CS_EZ_DRAW_DRAW_WEDGE,
            cs_ez_draw_draw_triangle: $module::RVA_CS_EZ_DRAW_DRAW_TRIANGLE,
            cs_ez_draw_draw_dodecadron: $module::RVA_CS_EZ_DRAW_DRAW_DODECADRON,
            initialize_spawn_geometry_request: $module::RVA_INITIALIZE_SPAWN_GEOMETRY_REQUEST,
            spawn_geometry: $module::RVA_SPAWN_GEOMETRY,
            cs_phys_world_cast_ray: $module::RVA_CS_PHYS_WORLD_CAST_RAY,
            cs_bullet_manager_spawn_bullet: $module::RVA_CS_BULLET_MANAGER_SPAWN_BULLET,
            chr_ins_apply_speffect: $module::RVA_CHR_INS_APPLY_SPEFFECT,
            chr_ins_remove_speffect: $module::RVA_CHR_INS_REMOVE_SPEFFECT,
            cs_action_button_man_execute_action_button:
                $module::RVA_CS_ACTION_BUTTON_MAN_EXECUTE_ACTION_BUTTON,
            cs_menu_man_imp_display_status_message: $module::RVA_CS_MENU_MAN_DISPLAY_STATUS_MESSAGE,
            global_hinstance: $module::RVA_GLOBAL_HINSTANCE,
            register_task: $module::RVA_REGISTER_TASK,
        }
    };
}

impl RvaBundle {
    fn for_version(version: GameVersion) -> Self {
        match version {
            GameVersion::Ww261 => rva_bundle!(rva_ww),
            GameVersion::Jp2611 => rva_bundle!(rva_jp),
        }
    }
}
