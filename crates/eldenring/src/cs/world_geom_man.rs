use std::{fmt::Formatter, mem::transmute, ptr::NonNull};

use pelite::pe64::Pe;
use pelite::util::CStr;
use shared::{F32ModelMatrix, F32Vector3, F32Vector4};
use vtable_rs::VPtr;
use windows::core::PCWSTR;

use super::{BlockId, FieldInsHandle, WorldInfoOwner};
use crate::Pair;
use crate::cs::{CSChrModelIns, CSModelIns, FieldInsBaseVmt};
use crate::dltx::{DLFixedString, DLUTF16StringKind};
use crate::position::BlockPosition;
use crate::{Tree, Vector, param::ASSET_GEOMETORY_PARAM_ST, rva};
use shared::{OwnedPtr, program::Program};

#[repr(C)]
/// Source of name: RTTI
#[shared::singleton("CSWorldGeomMan")]
pub struct CSWorldGeomMan {
    vftable: usize,
    unk8: usize,
    pub world_info_owner: NonNull<WorldInfoOwner>,
    /// A tree of loaded maps hosting their geometry instances.
    pub blocks: Tree<CSWorldGeomManBlocksEntry>,
    /// Seemingly points to the current overlay world tile's map data
    pub curent_99_block_data: OwnedPtr<CSWorldGeomManBlockData>,
}

impl CSWorldGeomMan {
    pub fn geom_block_data_by_id(&self, block_id: &BlockId) -> Option<&CSWorldGeomManBlockData> {
        self.blocks.iter().find_map(|b| {
            if &b.block_id == block_id {
                Some(b.data.as_ref())
            } else {
                None
            }
        })
    }

    pub fn geom_block_data_by_id_mut(
        &mut self,
        block_id: &BlockId,
    ) -> Option<&mut CSWorldGeomManBlockData> {
        self.blocks.iter().find_map(|b| {
            if &b.block_id == block_id {
                Some(b.data.as_mut())
            } else {
                None
            }
        })
    }
}

#[repr(C)]
pub struct CSWorldGeomManBlocksEntry {
    pub block_id: BlockId,
    _pad4: u32,
    pub data: OwnedPtr<CSWorldGeomManBlockData>,
}

#[repr(C)]
/// Seems to host any spawned geometry for a given map. It
pub struct CSWorldGeomManBlockData {
    /// The map ID this container hosts the assets for.
    pub block_id: BlockId,
    /// Might be padding?
    unk4: u32,
    pub world_block_info: usize,
    unk10: [u8; 0xF0],
    unk100: Vector<()>,
    unk120: Vector<()>,
    unk140: Vector<()>,
    pub activation_fade_modules: Vector<()>,
    unk180: [u8; 0x108],
    /// Holds refs to some geometry instances for this map.
    pub geom_ins_vector: Vector<OwnedPtr<CSWorldGeomIns>>,
    unk2a8: [u8; 0x20],
    pub geometry_array_count: u32,
    unk2cc: u32,
    pub geometry_array: OwnedPtr<CSWorldGeomIns>,
    unk2d8: [u8; 0x58],
    /// Seems to be the next field ins index that will be assiged.
    pub next_geom_ins_field_ins_index: u32,
    /// Seems to indicate if the geometry_ins vector has reached some hardcoded capacity?
    pub reached_geom_ins_vector_capacity: bool,
    _pad335: [u8; 3],
    pub geom_event_entity_id_map: Tree<Pair<u32, FieldInsHandle>>,
    unk350: usize,
    unk358: usize,
    unk360: usize,
    ladder_geometry: Vector<()>,
    pub sos_sign_geometry: Vector<OwnedPtr<OwnedPtr<CSWorldGeomIns>>>,
    pub disable_on_singleplay_geometry: Vector<OwnedPtr<OwnedPtr<CSWorldGeomIns>>>,
    unk3c8: [u8; 0x2E0],
}

pub struct GeometrySpawnParameters {
    pub position: BlockPosition,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
}

impl CSWorldGeomManBlockData {
    pub fn spawn_geometry(
        &mut self,
        asset: &str,
        parameters: &GeometrySpawnParameters,
    ) -> Option<NonNull<CSWorldGeomIns>> {
        let initialize_spawn_geometry_request_va = Program::current()
            .rva_to_va(rva::get().initialize_spawn_geometry_request)
            .unwrap();
        let spawn_geometry_va = Program::current()
            .rva_to_va(rva::get().spawn_geometry)
            .unwrap();

        let initialize_spawn_geometry_request = unsafe {
            transmute::<u64, extern "C" fn(&mut GeometrySpawnRequest, u32)>(
                initialize_spawn_geometry_request_va,
            )
        };

        let spawn_geometry = unsafe {
            transmute::<
                u64,
                extern "C" fn(
                    &mut CSWorldGeomManBlockData,
                    &GeometrySpawnRequest,
                ) -> Option<NonNull<CSWorldGeomIns>>,
            >(spawn_geometry_va)
        };

        let mut request = GeometrySpawnRequest {
            asset_string: Default::default(),
            block_pos: F32Vector3(0.0, 0.0, 0.0),
            rotation: F32Vector3(0.0, 0.0, 0.0),
            scale: F32Vector3(1.0, 1.0, 1.0),
            unk94: [0u8; 0x6C],
        };

        initialize_spawn_geometry_request(&mut request, 0x5);
        request.set_asset(asset);

        let BlockPosition { x, y, z, yaw: _ } = parameters.position;
        request.block_pos = F32Vector3(x, y, z);

        request.rotation = F32Vector3(parameters.rot_x, parameters.rot_y, parameters.rot_z);
        request.scale = F32Vector3(parameters.scale_x, parameters.scale_y, parameters.scale_z);

        spawn_geometry(self, &request)
    }
}

#[repr(C)]
/// Abstract base class for geometry instances.
///
/// Source of name: RTTI
pub struct CSWorldGeomIns {
    vftable: VPtr<dyn FieldInsBaseVmt, Self>,
    pub field_ins_handle: FieldInsHandle,
    /// Points to the map data hosting this GeomIns.
    pub block_data: NonNull<CSWorldGeomManBlockData>,
    /// Points to the world placement data for this geometry instance.
    pub info: CSWorldGeomInfo,
    pub res_proxy: CSWorldGeomResProxy,
    unk1e0: usize,
    unk1e8: [u8; 0x28],
    geombnd_res_cap: usize,
    pub model_matrix: F32ModelMatrix,
    unk260: [u8; 0x20],
    pub render_data: CSGeomInsRenderData,
    unk340: [u8; 0xf0],
}

#[repr(C)]
pub struct CSGeomInsRenderData {
    pub model_tint: F32Vector4,
    pub use_alpha_blend: bool,
    unk20: F32Vector4,
    unk30: F32Vector4,
    unk40: F32Vector4,
    unk50: F32Vector4,
    pub transparency: f32,
    unk70: F32Vector4,
    unk80: F32Vector4,
    unk90: u32,
    unk94: u32,
    unk98: u8,
    unk99: u8,
    unka0: F32Vector4,
    unkb0: f32,
    unkb8: f32,
}

#[repr(C)]
pub struct CSGeomModelIns {
    pub base: CSModelIns,
    unk150: [u8; 0x20],
}

#[repr(C)]
pub struct CSWorldGeomResProxy {
    unk0: usize,
    unk8: usize,
    unk10: usize,
    pub owner: NonNull<CSWorldGeomIns>,
    unk20: usize,
    pub model_ins: OwnedPtr<CSGeomModelIns>,
    unk30: i32,
}

#[repr(C)]
/// Holds the asset details in regard to placement in the world, drawing, etc.
///
/// Source of name: "..\\..\\Source\\Game\\Geometry\\CSWorldGeomInfo.cpp" in exception.
pub struct CSWorldGeomInfo {
    /// Points to the map data hosting the GeomIns for this info struct.
    pub block_data: OwnedPtr<CSWorldGeomManBlockData>,
    /// Points to the param row this geometry instance uses.
    pub asset_geometry_param: NonNull<ASSET_GEOMETORY_PARAM_ST>,
    unk10: u32,
    pub msb_parts_geom: CSMsbPartsGeom,
    unk68: u32,
    unk6c: u32,
    unk70: u32,
    unk74: u32,
    unk78: CSWorldGeomInfoRenderInfo,
    unke0: CSWorldGeomInfoRenderInfo,
    unk148: u16,
    unk14a: u8,
    unk14b: u8,
    /// Source of name: Params being copied over
    pub far_clip_distance: f32,
    /// Source of name: Params being copied over
    pub distant_view_model_border_dist: f32,
    /// Source of name: Params being copied over
    pub distant_view_model_play_dist: f32,
    /// Source of name: Params being copied over
    pub limted_activate_border_dist_for_grid: f32,
    /// Source of name: Params being copied over
    pub limted_activate_play_dist_for_grid: f32,
    /// Source of name: Params being copied over
    pub z_sort_offset_for_no_far_clip_draw: u32,
    unk164: u32,
    unk168: f32,
    unk16c: f32,
    unk170: f32,
    pub sound_obj_enable_dist: f32,
    unk178: u8,
    /// Whether this geometry is part of the skybox.
    pub is_on_skybox: bool,
    unk17a: u8,
    /// Source of name: Params being copied over
    pub has_tex_lv01_border_dist: bool,
    /// Source of name: Params being copied over
    pub is_no_far_clip_draw: bool,
    /// Source of name: Params being copied over
    pub is_trace_camera_xz: bool,
    /// Source of name: Params being copied over
    pub is_sky_dome_draw_phase: bool,
    /// Source of name: Params being copied over
    pub forward_draw_envmap_blend_type: bool,
    unk180: u16,
    unk182: u16,
    /// Hides the object whenever the player is alone, used for fogwalls and such.
    pub disable_on_singleplay: bool,
    unk185: u8,
    unk186: u16,
    unk188: usize,
}

#[repr(C)]
pub struct CSWorldGeomInfoRenderInfo {
    pub render_group_mask: [u8; 0x20],
    unk20: [u8; 0x40],
    unk60: usize,
}

#[repr(C)]
/// Seems to describe how to draw the MSB part.
pub struct CSMsbPartsGeom {
    pub msb_parts: CSMsbParts,
}

#[repr(C)]
/// Seems to describe how to draw the MSB part.
pub struct CSMsbParts {
    vfptr: usize,
    msb_res_cap: usize,
    unk10: usize,
    /// Owned by MsbResCap
    pub msb_part: NonNull<MsbPart>,
    unk20: usize,
    pub msb_geom_info: OwnedPtr<MsbGeomModelInfo>,
    /// Temporary storage for the MsbPart during some processing.
    /// Should be used instead of [Self::msb_part] if set.
    msb_part_temp_storage: Option<NonNull<MsbPart>>,
    /// Temporary storage for the MsbGeomModelInfo during some processing.
    /// Should be used instead of [Self::msb_geom_info] if set.
    pub msb_geom_info_temp_storage: Option<NonNull<MsbGeomModelInfo>>,
    pub map_studio_layer_mask: i32,
    unk44: F32Vector3,
}

#[repr(C)]
/// Source of name: RTTI
pub struct CSMsbPartsEne {
    pub cs_msb_parts: CSMsbParts,
}

#[repr(C)]
pub struct MsbGeomModelInfo {
    /// Actual name of AEG or map piece.
    pub model_name: PCWSTR,
    unk8: u32,
    unkc: i32,
    /// Path to SIB file used by this model.
    pub sib_path: PCWSTR,
    unk18: [u8; 0x10],
}

#[repr(C)]
pub struct MsbPart {
    /// Name of the part as defined in the MSB.
    ///
    /// IMPORTANT: This is NOT the model name, see [MsbGeomModelInfo::model_name] for that.
    pub name: PCWSTR,
    pub instance_id: i32,
    pub part_type: MsbPartType,
    /// Same as [crate::cs::FieldInsSelector::index] on Enemy parts.
    /// Used to create FieldInsHandles.
    pub field_ins_index: i32,
    unk14: i32,
    /// Path to SIB file, set on msb load
    pub model_placeholder_path: PCWSTR,
    pub position: F32Vector3,
    pub rotation: F32Vector3,
    pub scale: F32Vector3,
    unk44: i32,
    pub map_studio_layer: i32,
    pub display_data: OwnedPtr<MsbPartDisplayData>,
    /// Only set for [MsbPartType::Asset], [MsbPartType::ConnectCollision] and [MsbPartType::Collision] parts.
    pub display_group_data: Option<OwnedPtr<MsbPartDisplayGroupData>>,
    pub msb_part_entity: OwnedPtr<MsbPartEntity>,
    /// Type-specific data
    unk68: usize,
    /// Set for all types except [MsbPartType::Player] and [MsbPartType::ConnectCollision].
    pub gparam_config: Option<OwnedPtr<MsbGeomGparamConfig>>,
    /// Only set for [MsbPartType::Collision] parts.
    pub scene_gparam: Option<OwnedPtr<MsbSceneGparamConfig>>,
    /// Only set for [MsbPartType::MapPiece] and [MsbPartType::Asset].
    pub grass_config: Option<OwnedPtr<MsbPartsGrassConfig>>,
    unk90: OwnedPtr<MsbPartsUnk8>,
    /// Only set for [MsbPartType::MapPiece] and [MsbPartType::Asset].
    unk98: Option<OwnedPtr<MsbPartsUnk9>>,
    pub tile_load_config: OwnedPtr<MsbPartTileLoadConfig>,
    /// Only set for [MsbPartType::MapPiece], [MsbPartType::Asset], [MsbPartType::ConnectCollision] and [MsbPartType::Collision].
    unka0: Option<OwnedPtr<MsbPartUnk11>>,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MsbPartType {
    MapPiece = 0,
    Enemy = 2,
    Player = 4,
    Collision = 5,
    DummyAsset = 9,
    DummyEnemy = 10,
    ConnectCollision = 11,
    Asset = 13,
}

#[repr(C)]
pub struct MsbPartDisplayData {
    pub display_groups: [u32; 8],
    pub draw_groups: [u32; 8],
    /// Source of name: PrimDispMask_%s in CSRemoModelPrimDispMaskAct ctor
    pub prim_disp_masks: [u32; 32],
    unkc0: u8,
    unkc1: u8,
    unkc2: u8,
    unkc3: u8,
    unkc4: u16,
    unkc6: u16,
    reserved: [u32; 48],
}

#[repr(C)]
pub struct MsbPartDisplayGroupData {
    pub condition: i32,
    pub display_groups: [u32; 8],
    unk24: u16,
    unk26: i16,
    reserved: [u32; 8],
}

#[repr(C)]
pub struct MsbPartEntity {
    pub entity_id: i32,
    pub is_use_parts_draw_param_id: bool,
    unk5: u8,
    unk6: u8,
    pub lantern_id: u8,
    pub parts_draw_param_id: i16,
    pub point_light_shadow_source: i8,
    unkb: i8,
    pub shadow_source: bool,
    pub static_shadow_source: bool,
    pub cascade3_shadow_source: bool,
    unk10: u8,
    unk11: u8,
    pub is_shadow_destination: bool,
    pub is_shadow_only: bool,
    pub draw_by_reflect_camera: bool,
    pub draw_only_reflect_camera: bool,
    pub use_depth_bias: bool,
    pub disable_point_light_effect: u8,
    unk18: u8,
    pub entity_group_ids: [i32; 8],
    unk3c: u16,
    unk3e: u8,
    pub disable_rtao: bool,
}

#[repr(C)]
pub struct MsbGeomGparamConfig {
    pub light_set_id: i32,
    pub fog_id: i32,
    pub light_scattering_id: i32,
    pub environment_map_id: i32,
    reserved: [u32; 4],
}

#[repr(C)]
pub struct MsbSceneGparamConfig {
    /// Supposedly unused
    unk0: [u32; 4],
    pub transition_time: f32,
    unk14: u32,
    pub gparam_sub_id: i32,
    unk1c: i8,
    unk1d: i8,
    unk20: i8,
    unk21: i8,
    unk24: [u32; 11],
}

#[repr(C)]
pub struct MsbPartsGrassConfig {
    pub grass_type_params: [u32; 5],
    unk18: i16,
    unk1a: i16,
}

#[repr(C)]
pub struct MsbPartsUnk8 {
    unk0: [u8; 0x20],
}

#[repr(C)]
pub struct MsbPartsUnk9 {
    unk0: [u8; 0x20],
}

#[repr(C)]
pub struct MsbPartTileLoadConfig {
    /// Block ID this part is associated with.
    /// Some MSBs can host parts for completely different maps, so this field is used to track that.
    pub target_block_id: BlockId,
    /// Offset in characters where to start reading [MsbPart::name] for some of the operations (eg search by name).
    pub part_name_string_start_offset: u8,
    unk8: u32,
    unkc: u32,
    unk10: u32,
    pub culling_height_behavior: u32,
    unk18: u32,
    unk1c: u32,
}

#[repr(C)]
pub struct MsbPartUnk11 {
    unk0: [u8; 0x20],
}

#[repr(C)]
/// Used by the game to seperate geometry spawning code (like MSB parser) from the actual GeomIns
/// construction details.
pub struct GeometrySpawnRequest {
    pub asset_string: DLFixedString<DLUTF16StringKind, 32>,
    pub block_pos: F32Vector3,
    pub rotation: F32Vector3,
    pub scale: F32Vector3,
    pub unk94: [u8; 0x6C],
}

impl GeometrySpawnRequest {
    pub fn asset(&self) -> String {
        // let mut result = String::new();
        // for val in self.asset_string.iter() {
        //     let c: u8 = (*val & 0xFF) as u8;
        //     if c == 0 {
        //         break;
        //     } else {
        //         result.push(c as char);
        //     }
        // }
        // result
        todo!()
    }

    // TODO: guard against strings that are too long
    pub fn set_asset(&mut self, asset: &str) {
        // for (i, char) in asset.as_bytes().iter().enumerate() {
        //     self.asset_string[i] = *char as u16;
        // }
        todo!()
    }
}

impl std::fmt::Debug for GeometrySpawnRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeometrySpawnRequest")
            .field("asset", &self.asset())
            .field("positionX", &self.block_pos.0)
            .field("positionY", &self.block_pos.1)
            .field("positionZ", &self.block_pos.2)
            .field("rotationX", &self.rotation.0)
            .field("rotationY", &self.rotation.1)
            .field("rotationZ", &self.rotation.2)
            .field("scaleX", &self.scale.0)
            .field("scaleY", &self.scale.1)
            .field("scaleZ", &self.scale.2)
            .finish()
    }
}
