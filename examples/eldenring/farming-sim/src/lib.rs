use eldenring::position::{BlockPosition, HavokPosition, PositionDelta};
use eldenring_util::geometry::{CSWorldGeomManBlockDataExt, GeometrySpawnParameters};
use nalgebra_glm as glm;
use nalgebra_glm::{Mat4, Quat};
use pelite::pe64::Pe;
use shared::FSVector4;
use std::time::Duration;
use thiserror::Error;
use tracing_panic::panic_hook;

use eldenring::cs::{
    CSCamera, CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, FieldArea, GeometrySpawnRequest, MapId,
    PlayerIns, RendMan, WorldChrMan,
};
use eldenring::fd4::FD4TaskData;
use eldenring_util::ez_draw::CSEzDrawExt;
use eldenring_util::input::is_key_pressed;
use eldenring_util::program::Program;
use eldenring_util::singleton::get_instance;
use eldenring_util::system::wait_for_system_init;
use eldenring_util::task::CSTaskImpExt;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn DllMain(_hmodule: usize, reason: u32) -> bool {
    if reason != 1 {
        return true;
    }

    std::panic::set_hook(Box::new(panic_hook));

    let appender = tracing_appender::rolling::never("./", "farming-sim.log");
    tracing_subscriber::fmt().with_writer(appender).init();

    std::thread::spawn(move || {
        wait_for_system_init(&Program::current(), Duration::from_secs(5))
            .expect("System initialization timed out");

        init().expect("Could not initialize mod");
    });

    true
}

#[derive(Debug, Error)]
pub enum ModError {
    #[error("Missing static {0}")]
    MissingStatic(&'static str),
}

fn init() -> Result<(), Box<dyn std::error::Error>> {
    let task = unsafe { get_instance::<CSTaskImp>() }?.ok_or(ModError::MissingStatic("CSTask"))?;
    let mut farming_sim = FarmingSimMod::default();

    let task = task.run_recurring(
        move |_: &FD4TaskData| {
            let Some(camera) = unsafe { get_instance::<CSCamera>() }.unwrap() else {
                return;
            };

            let Some(main_player) = unsafe { get_instance::<WorldChrMan>() }
                .unwrap()
                .and_then(|w| w.main_player.as_mut())
            else {
                return;
            };

            let Some(world_geom_man) = unsafe { get_instance::<CSWorldGeomMan>() }.unwrap() else {
                return;
            };

            let Some(rend_man) = unsafe { get_instance::<RendMan>() }.unwrap() else {
                return;
            };

            let field_area = unsafe { get_field_area() };
            let Some(field_area) = field_area else {
                return;
            };

            is_key_pressed(0x42).then(|| farming_sim.toggle_camera());
            is_key_pressed(0x68).then(|| farming_sim.move_camera(CameraMovement::Forward));
            is_key_pressed(0x62).then(|| farming_sim.move_camera(CameraMovement::Backward));
            is_key_pressed(0x64).then(|| farming_sim.move_camera(CameraMovement::Left));
            is_key_pressed(0x66).then(|| farming_sim.move_camera(CameraMovement::Right));
            is_key_pressed(0x65).then(|| farming_sim.place_asset(world_geom_man));

            farming_sim.update_camera(field_area, camera);

            if farming_sim.camera_enabled {
                farming_sim.draw_highlight(field_area, rend_man);
            }
        },
        CSTaskGroupIndex::Draw_Pre,
    );

    std::mem::forget(task);

    Ok(())
}

// Grid bounds in grid cell count from (-COUNT to COUNT)
const GRID_SIZE: i32 = 20;
// Size of a builder grid tile (meters).
const GRID_TILE_SIZE: f32 = 1.0;
// Height (meters) from the ground for the builder camera.
const BUILDER_CAMERA_HEIGHT: f32 = 10.0;

// Static lookdown quat to make the builder camera stare downward.
const LOOKDOWN_QUAT: glm::Quat = glm::Quat::new(0.7071068, -0.7071068, 0.0, 0.0);

pub struct FarmingSimMod {
    camera_enabled: bool,

    // Origin data for the builder mode grid
    origin_block_id: MapId,
    origin_position: BlockPosition,

    // Current highlighted position for the camera and object placement. This represents a x
    // and y position in the grid itself.
    cursor_position: (i32, i32),

    // Adjust for the awkward 61deg angle that RTH has.
    yaw_rotation_matrix: Mat4,
}

impl Default for FarmingSimMod {
    fn default() -> Self {
        Self {
            camera_enabled: false,

            //  Downstairs area in RTH is the building origin.
            origin_block_id: MapId::from_parts(11, 10, 00, 00),
            origin_position: BlockPosition(-296.27, -32.60, -292.65, 0.0),

            cursor_position: (0, 0),

            yaw_rotation_matrix: glm::rotate_y(&glm::identity(), 61.0_f32.to_radians()),
        }
    }
}

impl FarmingSimMod {
    fn toggle_camera(&mut self) {
        self.camera_enabled = !self.camera_enabled;
    }

    fn update_camera(&self, field_area: &FieldArea, camera: &mut CSCamera) {
        if !self.camera_enabled {
            return;
        }

        let Some(camera_position) = self
            .cursor_position(field_area)
            .map(|p| p + PositionDelta(0.0, BUILDER_CAMERA_HEIGHT, 0.0))
        else {
            return;
        };

        // Apply camera position
        camera.pers_cam_1.matrix.3 =
            FSVector4(camera_position.0, camera_position.1, camera_position.2, 1.0);

        let rotation = glm::quat_to_mat4(&(LOOKDOWN_QUAT * yaw_quat(-61.0)));

        // Apply camera orientation
        camera.pers_cam_1.matrix.0 =
            FSVector4(rotation.m11, rotation.m12, rotation.m13, rotation.m14);
        camera.pers_cam_1.matrix.1 =
            FSVector4(rotation.m21, rotation.m22, rotation.m23, rotation.m24);
        camera.pers_cam_1.matrix.2 =
            FSVector4(rotation.m31, rotation.m32, rotation.m33, rotation.m34);
    }

    // Get current cursor position
    fn cursor_position(&self, field_area: &FieldArea) -> Option<HavokPosition> {
        // Figure out position of origin in current havok AABB.
        let origin_block = field_area
            .world_info_owner
            .world_res
            .world_info
            .world_block_info_by_map(&self.origin_block_id)?;

        let origin_physics_position = HavokPosition(
            origin_block.physics_center.0 + self.origin_position.0,
            origin_block.physics_center.1 + self.origin_position.1,
            origin_block.physics_center.2 + self.origin_position.2,
            1.0,
        );

        let cursor_grid_position = glm::vec3(
            self.cursor_position.0 as f32 * GRID_TILE_SIZE,
            0.0,
            self.cursor_position.1 as f32 * GRID_TILE_SIZE,
        );

        let cursor_grid_position_rotated = self
            .yaw_rotation_matrix
            .transform_vector(&cursor_grid_position);

        Some(
            origin_physics_position
                + PositionDelta(
                    cursor_grid_position_rotated.x,
                    0.0,
                    cursor_grid_position_rotated.z,
                ),
        )
    }

    // Get current cursor position in block space relative to the origin map.
    fn cursor_block_position(&self) -> Option<BlockPosition> {
        let cursor_grid_position = glm::vec3(
            self.cursor_position.0 as f32 * GRID_TILE_SIZE,
            0.0,
            self.cursor_position.1 as f32 * GRID_TILE_SIZE,
        );

        let cursor_grid_position_rotated = self
            .yaw_rotation_matrix
            .transform_vector(&cursor_grid_position);

        Some(
            self.origin_position
                + PositionDelta(
                    cursor_grid_position_rotated.x,
                    0.0,
                    cursor_grid_position_rotated.z,
                ),
        )
    }

    // Handle movement inputs
    fn move_camera(&mut self, movement: CameraMovement) {
        match movement {
            CameraMovement::Forward => {
                self.cursor_position.1 = (self.cursor_position.1 + 1).clamp(-GRID_SIZE, GRID_SIZE)
            }
            CameraMovement::Backward => {
                self.cursor_position.1 = (self.cursor_position.1 - 1).clamp(-GRID_SIZE, GRID_SIZE)
            }
            CameraMovement::Right => {
                self.cursor_position.0 = (self.cursor_position.0 + 1).clamp(-GRID_SIZE, GRID_SIZE)
            }
            CameraMovement::Left => {
                self.cursor_position.0 = (self.cursor_position.0 - 1).clamp(-GRID_SIZE, GRID_SIZE)
            }
        };
    }

    // Draw the highlight for users to know what they're placing.
    fn draw_highlight(&self, field_area: &FieldArea, rend_man: &mut RendMan) {
        let Some(cursor_position) = self.cursor_position(field_area) else {
            return;
        };

        rend_man
            .debug_ez_draw
            .set_color(&FSVector4(1.0, 0.0, 0.0, 1.0));
        rend_man
            .debug_ez_draw
            .draw_sphere(&cursor_position, GRID_TILE_SIZE / 2.0);
    }

    fn place_asset(&self, world_geom_man: &mut CSWorldGeomMan) {
        let Some(position) = self.cursor_block_position() else {
            return;
        };

        let Some(block_geom_data) = world_geom_man.world_geom_block_by_map_id_mut(&self.origin_block_id) else {
            return;
        };

        block_geom_data.spawn_geometry(
            "AEG099_721",
            &GeometrySpawnParameters {
                position,
                rot_x: 0.0,
                rot_y: 0.0,
                rot_z: 0.0,
                scale_x: 1.0,
                scale_y: 1.0,
                scale_z: 1.0,
            },
        );
    }
}

pub enum CameraMovement {
    Forward,
    Backward,
    Left,
    Right,
}

unsafe fn get_field_area() -> Option<&'static FieldArea> {
    (*(Program::current().rva_to_va(0x3d691d8).unwrap() as *const *const FieldArea)).as_ref()
}

fn yaw_quat(degrees: f32) -> Quat {
    let radians = degrees.to_radians();
    let half = radians / 2.0;
    Quat::new(half.cos(), 0.0, half.sin(), 0.0)
}
