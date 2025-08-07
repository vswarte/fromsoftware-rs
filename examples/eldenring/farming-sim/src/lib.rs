use eldenring::position::{BlockPosition, HavokPosition, PositionDelta};
use eldenring::DynamicSizeSpan;
use eldenring_util::ez_state::{
    CustomTalkScriptMenu, CustomTalkScriptMenuOption, EzStateExpressionExt, EzStateInstruction,
    EZSTATE_COMMAND_ADD_TALK_LIST_DATA, EZSTATE_COMMAND_CLEAR_TALK_LIST_DATA,
    EZSTATE_COMMAND_CLOSE_SHOP_MESSAGE, EZSTATE_COMMAND_OPEN_REPOSITORY,
    EZSTATE_COMMAND_SHOW_SHOP_MESSAGE,
};
use eldenring_util::geometry::{CSWorldGeomManBlockDataExt, GeometrySpawnParameters};
use nalgebra_glm as glm;
use nalgebra_glm::{Mat4, Quat};
use pelite::pe64::Pe;
use retour::static_detour;
use shared::FSVector4;
use std::ptr::NonNull;
use std::time::Duration;
use thiserror::Error;
use tracing_panic::panic_hook;

use eldenring::cs::{
    CSCamera, CSTaskGroupIndex, CSTaskImp, CSWorldGeomMan, EzStateEvent, EzStateExpression,
    EzStateMachineImpl, EzStateState, EzStateTransition, FieldArea, GeometrySpawnRequest, MapId,
    PlayerIns, RendMan, WorldChrMan,
};
use eldenring::fd4::FD4TaskData;
use eldenring_util::ez_draw::CSEzDrawExt;
use eldenring_util::input::is_key_pressed;
use eldenring_util::program::Program;
use eldenring_util::singleton::get_instance;
use eldenring_util::system::wait_for_system_init;
use eldenring_util::task::CSTaskImpExt;

static_detour! {
    static EZ_STATE_ENTER_STATE: extern "C" fn(NonNull<EzStateState>, NonNull<EzStateMachineImpl>, usize);
}

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

    unsafe {
        EZ_STATE_ENTER_STATE
            .initialize(
                std::mem::transmute::<
                    usize,
                    extern "C" fn(NonNull<EzStateState>, NonNull<EzStateMachineImpl>, usize),
                >(0x1420887f0usize),
                |state, mut machine, unk| {
                    let machine_ref = machine.as_mut();

                    // Check if the current state group contains the sort chest menu option
                    // somewhere.
                    let state_group = machine.as_ref().state_group.as_ref();
                    let contains_repository_transition = machine_ref
                        .state_group
                            .as_mut()
                            .states
                            .iter_mut()
                            // Are we in the grace's state group?
                            .any(|s| {
                                s.entry_events
                                    .iter()
                                    .any(|e| e.command == EZSTATE_COMMAND_OPEN_REPOSITORY)
                            });

                    // Check if we're entering the state group newly.
                    let is_entering_initial_state = state == state_group.initial_state;

                    if contains_repository_transition && is_entering_initial_state {
                        let mut transition_index = usize::MAX;
                        let mut add_menu_state = None;
                        let mut menu_transition_state = None;

                        for state in state_group.states.iter() {
                            // Find the state that adds the Sort chest option
                            if state.entry_events.iter().any(|event| {
                                event.command == EZSTATE_COMMAND_ADD_TALK_LIST_DATA
                                    && event
                                        .arguments
                                        .as_slice()
                                        .get(1)
                                        .and_then(|arg| arg.disassemble().ok())
                                        .is_some_and(|f| {
                                            f.low[0] == EzStateInstruction::PushInt32(15000395)
                                        })
                            }) {
                                add_menu_state = Some(state as *const _ as *mut EzStateState);
                            }

                            // Find the state that adds the Sort chest transition
                            if let Some(index) = state.transitions.iter().position(|transition| {
                                transition
                                    .as_ref()
                                    .target_state
                                    .as_ref()
                                    .is_some_and(|target| {
                                        target.as_ref().entry_events.iter().any(|event| {
                                            event.command == EZSTATE_COMMAND_OPEN_REPOSITORY
                                        })
                                    })
                            }) {
                                transition_index = index;
                                menu_transition_state = Some(state as *const _ as *mut EzStateState);
                            }
                        }

                        if let Some(add_menu_state) = add_menu_state
                            && let Some(menu_transition_state) = menu_transition_state
                            && transition_index != usize::MAX {

                            let test_state_4 =
                                Box::leak(Box::new(CustomTalkScriptMenu::from_options(
                                    73,
                                    &[CustomTalkScriptMenuOption::new(
                                        1,
                                        15000372,
                                        machine_ref.state_group.as_ref().initial_state.as_ptr(),
                                    )],
                                )));

                            let test_state_2 =
                                Box::leak(Box::new(CustomTalkScriptMenu::from_options(
                                    71,
                                    &[CustomTalkScriptMenuOption::new(
                                        1,
                                        15000372,
                                        test_state_4.entry_state(),
                                    )],
                                )));

                            let test_state_3 =
                                Box::leak(Box::new(CustomTalkScriptMenu::from_options(
                                    70,
                                    &[CustomTalkScriptMenuOption::new(
                                        1,
                                        15000395,
                                        test_state_2.entry_state(),
                                    )],
                                )));

                            let test_state =
                                Box::leak(Box::new(CustomTalkScriptMenu::from_options(
                                    69,
                                    &[
                                        CustomTalkScriptMenuOption::new(
                                            1,
                                            15000395,
                                            test_state_3.entry_state(),
                                        ),
                                        // Cancel button
                                        CustomTalkScriptMenuOption::new(
                                            2,
                                            15000372,
                                            machine_ref.state_group.as_ref().initial_state.as_ptr(),
                                        ),
                                    ],
                                )));

                            let test_state_transition = Box::leak(Box::new(EzStateTransition {
                                // target_state: Some(NonNull::from_ref(test_state)),
                                target_state: Some(NonNull::from_ref(test_state.entry_state())),
                                pass_events: DynamicSizeSpan::empty(),
                                sub_transitions: DynamicSizeSpan::empty(),
                                evaluator: EzStateExpression::from_static_slice(
                                    TEST_TRANSITION_EVALUATOR,
                                ),
                            }));

                            {
                                // Safety: at this point we know that add_menu-state is populated
                                let add_menu_state = add_menu_state.as_mut().unwrap();
                                let old_count = add_menu_state.entry_events.len();
                                let new_count = old_count + 1;

                                let layout =
                                    std::alloc::Layout::array::<EzStateEvent>(new_count).unwrap();
                                let alloc = std::alloc::alloc(layout) as *mut EzStateEvent;

                                std::ptr::copy_nonoverlapping(
                                    add_menu_state.entry_events.as_ptr(),
                                    alloc,
                                    old_count,
                                );

                                std::ptr::write(
                                    alloc.add(old_count),
                                    EzStateEvent {
                                        command: EZSTATE_COMMAND_ADD_TALK_LIST_DATA,
                                        arguments: DynamicSizeSpan::from_static_slice(
                                            ENTRY_EVENT_ARGUMENTS,
                                        ),
                                    },
                                );

                                add_menu_state.entry_events =
                                    DynamicSizeSpan::from_raw_parts(alloc, new_count);
                            }

                            // Inject new transition as well
                            {
                                // tracing::info!("Patching transitions");
                                let menu_transition_state =
                                    menu_transition_state.as_mut().unwrap();

                                let old_count = menu_transition_state.transitions.len();
                                let new_count = old_count + 1;

                                let layout =
                                    std::alloc::Layout::array::<NonNull<EzStateTransition>>(
                                        new_count,
                                    )
                                    .unwrap();
                                let alloc =
                                    std::alloc::alloc(layout) as *mut NonNull<EzStateTransition>;

                                // Copy up to the sort chest index
                                std::ptr::copy_nonoverlapping(
                                    menu_transition_state.transitions.as_ptr(),
                                    alloc,
                                    transition_index,
                                );
                                // Put our own transition right after
                                std::ptr::write(
                                    alloc.add(transition_index),
                                    NonNull::from_ref(test_state_transition),
                                );

                                // Copy the rest of the list after we added our entry
                                std::ptr::copy_nonoverlapping(
                                    menu_transition_state
                                        .transitions
                                        .as_ptr()
                                        .add(transition_index),
                                    alloc.add(transition_index + 1),
                                    new_count - transition_index,
                                );

                                menu_transition_state.transitions =
                                    DynamicSizeSpan::from_raw_parts(alloc, new_count);
                            }
                        }
                    }

                    EZ_STATE_ENTER_STATE.call(state, machine, unk);
                },
            )
            .unwrap()
            .enable()
            .unwrap();
    }

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

const ENTRY_EVENT_ARGUMENTS_1_BYTES: &[u8] = &[0x82, 0x69, 0x00, 0x00, 0x00, 0xa1];
const ENTRY_EVENT_ARGUMENTS_2_BYTES: &[u8] = &[0x82, 0x91, 0x95, 0x6F, 0x01, 0xa1];
const ENTRY_EVENT_ARGUMENTS_3_BYTES: &[u8] = &[0x3f, 0xa1];

const ENTRY_EVENT_ARGUMENTS: &[EzStateExpression] = &[
    EzStateExpression::from_static_slice(ENTRY_EVENT_ARGUMENTS_1_BYTES), // Set menu item idx
    EzStateExpression::from_static_slice(ENTRY_EVENT_ARGUMENTS_2_BYTES), // Set FMG,
    EzStateExpression::from_static_slice(ENTRY_EVENT_ARGUMENTS_3_BYTES), // ???
];

const TEST_TRANSITION_EVALUATOR: &[u8] = &[0xAF, 0x82, 0x69, 0x00, 0x00, 0x00, 0x95, 0xA1];

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

        let Some(block_geom_data) =
            world_geom_man.world_geom_block_by_map_id_mut(&self.origin_block_id)
        else {
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
