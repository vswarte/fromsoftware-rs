use std::{fmt::Formatter, mem::transmute, ptr::NonNull};

use pelite::pe64::Pe;
use windows::core::PCWSTR;

use super::{BlockId, FieldInsHandle, WorldInfoOwner};
use crate::position::BlockPosition;
use crate::{Tree, Vector, param::ASSET_GEOMETORY_PARAM_ST, rva};
use shared::{OwnedPtr, Subclass, Superclass, program::Program};

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
    unk334: bool,
    _pad335: [u8; 3],
    unk338: [u8; 0x50],
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
            asset_string: [0u16; 0x20],
            unk40: 0,
            unk44: 0,
            asset_string_ptr: 0,
            unk50: 0,
            unk54: 0,
            unk58: 0,
            unk5c: 0,
            unk60: 0,
            unk64: 0,
            unk68: 0,
            unk6c: 0,
            pos_x: 0.0,
            pos_y: 0.0,
            pos_z: 0.0,
            rot_x: 0.0,
            rot_y: 0.0,
            rot_z: 0.0,
            scale_x: 0.0,
            scale_y: 0.0,
            scale_z: 0.0,
            unk94: [0u8; 0x6C],
        };

        initialize_spawn_geometry_request(&mut request, 0x5);
        request.set_asset(asset);

        let BlockPosition { x, y, z, yaw: _ } = parameters.position;
        request.pos_x = x;
        request.pos_y = y;
        request.pos_z = z;

        request.rot_x = parameters.rot_x;
        request.rot_y = parameters.rot_y;
        request.rot_z = parameters.rot_z;
        request.scale_x = parameters.scale_x;
        request.scale_y = parameters.scale_y;
        request.scale_z = parameters.scale_z;

        spawn_geometry(self, &request)
    }
}

#[repr(C)]
/// Abstract base class for geometry instances.
///
/// Source of name: RTTI
pub struct CSWorldGeomIns {
    vfptr: usize,
    pub field_ins_handle: FieldInsHandle,
    /// Points to the map data hosting this GeomIns.
    pub block_data: NonNull<CSWorldGeomManBlockData>,
    /// Points to the world placement data for this geometry instance.
    pub info: CSWorldGeomInfo,
    unk1a8: [u8; 0x288],
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
    unk14: u32,
    pub msb_parts_geom: CSMsbPartsGeom,
    unk68: u32,
    unk6c: u32,
    unk70: u32,
    unk74: u32,
    unk78: CSWorldGeomInfoUnk,
    unke0: CSWorldGeomInfoUnk,
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
    unk179: u8,
    unk17a: u8,
    unk17c: u8,
    /// Source of name: Params being copied over
    pub has_tex_lv01_border_dist: bool,
    /// Source of name: Params being copied over
    pub is_no_far_clip_draw: bool,
    /// Source of name: Params being copied over
    pub is_trace_camera_xz: bool,
    /// Source of name: Params being copied over
    pub forward_draw_envmap_blend_type: bool,
    unk180: u16,
    unk182: u16,
    /// Hides the object whenever the player is alone, used for fogwalls and such.
    pub disable_on_singleplay: u8,
    unk185: u8,
    unk186: u16,
    unk188: usize,
}

#[repr(C)]
pub struct CSWorldGeomInfoUnk {
    unk0: u32,
    unk4: u32,
    unk8: u32,
    unkc: u32,
    unk10: u32,
    unk14: u32,
    unk18: u32,
    unk1c: u32,
    unk20: usize,
    unk28: [u8; 0x38],
    unk60: usize,
}

#[repr(C)]
#[derive(Subclass)]
/// Seems to describe how to draw the MSB part.
pub struct CSMsbPartsGeom {
    pub msb_parts: CSMsbParts,
}

#[repr(C)]
#[derive(Superclass)]
#[superclass(children(CSMsbPartsGeom, CSMsbPartsEne))]
/// Seems to describe how to draw the MSB part.
pub struct CSMsbParts {
    vfptr: usize,
    pub draw_flags: u32,
    _padc: u32,
    unk10: usize,
    pub msb_part: OwnedPtr<MsbPart>,
    unk20: [u8; 0x30],
}

#[repr(C)]
#[derive(Subclass)]
/// Source of name: RTTI
pub struct CSMsbPartsEne {
    pub cs_msb_parts: CSMsbParts,
}

#[repr(C)]
pub struct MsbPart {
    pub name: PCWSTR,
    // TODO: rest
}

#[repr(C)]
/// Used by the game to seperate geometry spawning code (like MSB parser) from the actual GeomIns
/// construction details.
pub struct GeometrySpawnRequest {
    /// Contains the asset string, ex. "AEG020_370"
    pub asset_string: [u16; 0x20],
    pub unk40: u32,
    pub unk44: u32,
    /// Contains a pointer to the asset string
    pub asset_string_ptr: u64,
    pub unk50: u32,
    pub unk54: u32,
    pub unk58: u32,
    pub unk5c: u32,
    pub unk60: u32,
    pub unk64: u32,
    pub unk68: u32,
    pub unk6c: u32,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
    pub unk94: [u8; 0x6C],
}

impl GeometrySpawnRequest {
    pub fn asset(&self) -> String {
        let mut result = String::new();
        for val in self.asset_string.iter() {
            let c: u8 = (*val & 0xFF) as u8;
            if c == 0 {
                break;
            } else {
                result.push(c as char);
            }
        }
        result
    }

    // TODO: guard against strings that are too long
    pub fn set_asset(&mut self, asset: &str) {
        for (i, char) in asset.as_bytes().iter().enumerate() {
            self.asset_string[i] = *char as u16;
        }
    }
}

impl std::fmt::Debug for GeometrySpawnRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GeometrySpawnRequest")
            .field("asset", &self.asset())
            .field("positionX", &self.pos_x)
            .field("positionY", &self.pos_y)
            .field("positionZ", &self.pos_z)
            .field("rotationX", &self.rot_x)
            .field("rotationY", &self.rot_y)
            .field("rotationZ", &self.rot_z)
            .field("scaleX", &self.scale_x)
            .field("scaleY", &self.scale_y)
            .field("scaleZ", &self.scale_z)
            .finish()
    }
}
