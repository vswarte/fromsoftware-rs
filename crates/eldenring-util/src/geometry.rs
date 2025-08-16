use std::mem::transmute;

use eldenring::position::BlockPosition;
use pelite::pe64::Pe;
use thiserror::Error;

use eldenring::cs::CSWorldGeomMan;
use eldenring::cs::GeometrySpawnRequest;
use eldenring::cs::BlockId;

use crate::program::Program;
use crate::rva;

#[derive(Debug, Error)]
pub enum SpawnGeometryError {
    #[error("No map data found")]
    BlockDataUnavailable,
}

pub struct GeometrySpawnParameters {
    pub block_id: BlockId,
    pub position: BlockPosition,
    pub rot_x: f32,
    pub rot_y: f32,
    pub rot_z: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub scale_z: f32,
}

pub trait CSWorldGeomManExt {
    fn spawn_geometry(
        &self,
        asset: &str,
        parameters: &GeometrySpawnParameters,
    ) -> Result<(), SpawnGeometryError>;
}

impl CSWorldGeomManExt for CSWorldGeomMan {
    fn spawn_geometry(
        &self,
        asset: &str,
        parameters: &GeometrySpawnParameters,
    ) -> Result<(), SpawnGeometryError> {
        tracing::info!("Spawning {asset}");

        let cs_world_geom_man_block_data_by_block_id_va = Program::current()
            .rva_to_va(rva::get().cs_world_geom_man_block_data_by_block)
            .unwrap();
        let initialize_spawn_geometry_request_va = Program::current()
            .rva_to_va(rva::get().initialize_spawn_geometry_request)
            .unwrap();
        let spawn_geometry_va = Program::current()
            .rva_to_va(rva::get().spawn_geometry)
            .unwrap();

        let block_data_by_block_id = unsafe {
            transmute::<u64, fn(&CSWorldGeomMan, &BlockId) -> u64>(
                cs_world_geom_man_block_data_by_block_id_va,
            )
        };

        let initialize_spawn_geometry_request = unsafe {
            transmute::<u64, fn(&mut GeometrySpawnRequest, u32)>(
                initialize_spawn_geometry_request_va,
            )
        };

        let spawn_geometry =
            unsafe { transmute::<u64, fn(u64, &GeometrySpawnRequest) -> u64>(spawn_geometry_va) };

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

        let BlockPosition(x, y, z, _) = parameters.position;
        request.pos_x = x;
        request.pos_y = y;
        request.pos_z = z;

        request.rot_x = parameters.rot_x;
        request.rot_y = parameters.rot_y;
        request.rot_z = parameters.rot_z;
        request.scale_x = parameters.scale_x;
        request.scale_y = parameters.scale_y;
        request.scale_z = parameters.scale_z;

        // TODO: make this a nice as_ref call or something
        let block_data_ptr = block_data_by_block_id(self, &parameters.block_id);
        if block_data_ptr == 0x0 {
            return Err(SpawnGeometryError::BlockDataUnavailable);
        }

        let _geom = spawn_geometry(block_data_ptr, &request);

        Ok(())
    }
}
