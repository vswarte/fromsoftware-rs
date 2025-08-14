use pelite::pe::Pe;
use std::sync::LazyLock;

mod rva_jp;
mod rva_ww;

use crate::program::Program;

const LANG_ID_EN: u16 = 0x0009;
const LANG_ID_JP: u16 = 0x0011;

pub fn get() -> &'static RvaBundle {
    static RVAS: LazyLock<RvaBundle> = LazyLock::new(|| {
        let program = Program::current();
        let resources = program.resources().unwrap();

        let (product, lang_id_base, version) = {
            let info = resources.version_info().unwrap();
            let product_version = info.fixed().unwrap().dwProductVersion;

            let version = format!(
                "{}.{}.{}.{}",
                product_version.Major,
                product_version.Minor,
                product_version.Patch,
                product_version.Build,
            );

            let mut product: Option<String> = None;
            let language = info.translation().first().unwrap();
            info.strings(*language, |k, v| {
                if k == "ProductName" {
                    product = Some(v.to_string())
                }
            });

            (product.unwrap(), language.lang_id & 0x03FF, version)
        };

        match (product.as_str(), lang_id_base, version.as_str()) {
            ("ELDEN RING™", LANG_ID_EN, "2.6.0.0") => RvaBundle {
                cs_ez_draw_draw_line: rva_ww::RVA_CS_EZ_DRAW_DRAW_LINE,
                cs_ez_draw_draw_capsule: rva_ww::RVA_CS_EZ_DRAW_DRAW_CAPSULE,
                cs_ez_draw_draw_sphere: rva_ww::RVA_CS_EZ_DRAW_DRAW_SPHERE,
                cs_ez_draw_draw_wedge: rva_ww::RVA_CS_EZ_DRAW_DRAW_WEDGE,
                cs_ez_draw_set_color: rva_ww::RVA_CS_EZ_DRAW_SET_COLOR,
                cs_world_geom_man_block_data_by_map:
                    rva_ww::RVA_CS_WORLD_GEOM_MAN_BLOCK_DATA_BY_MAP_ID,
                initialize_spawn_geometry_request: rva_ww::RVA_INITIALIZE_SPAWN_GEOMETRY_REQUEST,
                spawn_geometry: rva_ww::RVA_SPAWN_GEOMETRY,
                cs_phys_world_cast_ray: rva_ww::RVA_CS_PHYS_WORLD_CAST_RAY,
                cs_bullet_manager_spawn_bullet: rva_ww::RVA_CS_BULLET_MANAGER_SPAWN_BULLET,
                chr_ins_apply_speffect: rva_ww::RVA_CHR_INS_APPLY_SPEFFECT,
                chr_ins_remove_speffect: rva_ww::RVA_CHR_INS_REMOVE_SPEFFECT,
            },
            ("ELDEN RING™", LANG_ID_JP, "2.6.0.1") => RvaBundle {
                cs_ez_draw_draw_line: rva_jp::RVA_CS_EZ_DRAW_DRAW_LINE,
                cs_ez_draw_draw_capsule: rva_jp::RVA_CS_EZ_DRAW_DRAW_CAPSULE,
                cs_ez_draw_draw_sphere: rva_jp::RVA_CS_EZ_DRAW_DRAW_SPHERE,
                cs_ez_draw_draw_wedge: rva_jp::RVA_CS_EZ_DRAW_DRAW_WEDGE,
                cs_ez_draw_set_color: rva_jp::RVA_CS_EZ_DRAW_SET_COLOR,
                cs_world_geom_man_block_data_by_map:
                    rva_jp::RVA_CS_WORLD_GEOM_MAN_BLOCK_DATA_BY_MAP_ID,
                initialize_spawn_geometry_request: rva_jp::RVA_INITIALIZE_SPAWN_GEOMETRY_REQUEST,
                spawn_geometry: rva_jp::RVA_SPAWN_GEOMETRY,
                cs_phys_world_cast_ray: rva_jp::RVA_CS_PHYS_WORLD_CAST_RAY,
                cs_bullet_manager_spawn_bullet: rva_jp::RVA_CS_BULLET_MANAGER_SPAWN_BULLET,
                chr_ins_apply_speffect: rva_jp::RVA_CHR_INS_APPLY_SPEFFECT,
                chr_ins_remove_speffect: rva_jp::RVA_CHR_INS_REMOVE_SPEFFECT,
            },
            _ => panic!("could not fetch RVAs for executable. name = \"{product}\", lang = {lang_id_base:x}, version = {version}"),
        }
    });

    &RVAS
}

pub struct RvaBundle {
    pub cs_ez_draw_draw_line: u32,
    pub cs_ez_draw_draw_capsule: u32,
    pub cs_ez_draw_draw_sphere: u32,
    pub cs_ez_draw_draw_wedge: u32,
    pub cs_ez_draw_set_color: u32,
    pub cs_world_geom_man_block_data_by_map: u32,
    pub initialize_spawn_geometry_request: u32,
    pub spawn_geometry: u32,
    pub cs_phys_world_cast_ray: u32,
    pub cs_bullet_manager_spawn_bullet: u32,
    pub chr_ins_apply_speffect: u32,
    pub chr_ins_remove_speffect: u32,
}
