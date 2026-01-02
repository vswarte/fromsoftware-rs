use eldenring::cs::{
    CSMsbParts, CSMsbPartsGeom, CSWorldGeomInfo, CSWorldGeomIns, CSWorldGeomMan,
    CSWorldGeomManBlockData, MsbPart,
};
use hudhook::imgui::Ui;

use super::{DebugDisplay, UiExt};

impl DebugDisplay for CSWorldGeomMan {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            &format!("Loaded blocks: {}", self.blocks.len()),
            self.blocks.iter(),
            |ui, _i, block| {
                ui.header(&format!("{}", block.block_id), || {
                    block.data.render_debug(ui);
                });
            },
        );

        ui.header("Current Unk Block", || {
            self.curent_99_block_data.render_debug(ui);
        });
    }
}

impl DebugDisplay for CSWorldGeomManBlockData {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Block ID: {}", self.block_id));
        ui.text(format!("World block info: {:x}", self.world_block_info));

        ui.text(format!(
            "Next GeomIns FieldIns index: {}",
            self.next_geom_ins_field_ins_index
        ));

        ui.list(
            &format!("Geometry Vector ({})", self.geom_ins_vector.len()),
            self.geom_ins_vector.items(),
            |ui, _i, geometry_ins| {
                let name = unsafe {
                    geometry_ins
                        .info
                        .msb_parts_geom
                        .msb_parts
                        .msb_part
                        .as_ref()
                        .name
                        .to_string()
                }
                .unwrap();

                ui.header(
                    &format!(
                        "{} - {} FieldInsSelector({}, {})",
                        name,
                        geometry_ins.field_ins_handle.block_id,
                        geometry_ins.field_ins_handle.selector.container(),
                        geometry_ins.field_ins_handle.selector.index()
                    ),
                    || {
                        geometry_ins.render_debug(ui);
                    },
                );
            },
        );

        ui.list(
            &format!("Sign Geometry Vector ({})", self.sos_sign_geometry.len()),
            self.sos_sign_geometry.items(),
            |ui, _i, geometry_ins| {
                let name = unsafe {
                    geometry_ins
                        .info
                        .msb_parts_geom
                        .msb_parts
                        .msb_part
                        .as_ref()
                        .name
                        .to_string()
                }
                .unwrap();

                ui.header(
                    &format!(
                        "{} - {} FieldInsSelector({}, {})",
                        name,
                        geometry_ins.field_ins_handle.block_id,
                        geometry_ins.field_ins_handle.selector.container(),
                        geometry_ins.field_ins_handle.selector.index()
                    ),
                    || {
                        geometry_ins.render_debug(ui);
                    },
                );
            },
        );
    }
}

impl DebugDisplay for CSWorldGeomIns {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Field Ins Handle: {}", self.field_ins_handle));
        ui.header("World geom info", || {
            self.info.render_debug(ui);
        });
    }
}

impl DebugDisplay for CSWorldGeomInfo {
    fn render_debug(&self, ui: &Ui) {
        ui.header("CSMsbPartsGeom", || {
            self.msb_parts_geom.render_debug(ui);
        });
        ui.text(format!("Far clip distance: {}", self.far_clip_distance));
        ui.text(format!(
            "Distant view model border distance: {}",
            self.distant_view_model_border_dist
        ));
        ui.text(format!(
            "Distant view model play distance: {}",
            self.distant_view_model_play_dist
        ));
        ui.text(format!(
            "Limited activate border distance for grid: {}",
            self.limted_activate_border_dist_for_grid
        ));
        ui.text(format!(
            "Limited activate play distance for grid: {}",
            self.limted_activate_play_dist_for_grid
        ));
        ui.text(format!(
            "Z sort offset for no far clip draw: {}",
            self.z_sort_offset_for_no_far_clip_draw
        ));
        ui.text(format!(
            "Sound object enable distance: {}",
            self.sound_obj_enable_dist
        ));
        ui.text(format!(
            "Has texture lv01 border distance: {}",
            self.has_tex_lv01_border_dist
        ));
        ui.text(format!("Is no far clip draw: {}", self.is_no_far_clip_draw));
        ui.text(format!("Is trace camera xz: {}", self.is_trace_camera_xz));
        ui.text(format!(
            "Forward draw envmap blend type: {}",
            self.forward_draw_envmap_blend_type
        ));
        ui.text(format!(
            "Disable on singleplay: {}",
            self.disable_on_singleplay
        ));
    }
}

impl DebugDisplay for CSMsbPartsGeom {
    fn render_debug(&self, ui: &Ui) {
        self.msb_parts.render_debug(ui);
    }
}

impl DebugDisplay for CSMsbParts {
    fn render_debug(&self, _ui: &Ui) {}
}

impl DebugDisplay for MsbPart {
    fn render_debug(&self, ui: &Ui) {
        unsafe {
            let name = self
                .name
                .to_string()
                .unwrap_or_else(|_| "<invalid utf16>".to_string());
            ui.text(format!("Name: {}", name));
        }
        ui.text(format!("Instance ID: {}", self.instance_id));
        ui.text(format!("Map studio layer: {}", self.map_studio_layer));
        ui.header("Position", || {
            self.position.render_debug(ui);
        });
        ui.header("Rotation", || {
            self.rotation.render_debug(ui);
        });
        ui.header("Scale", || {
            self.scale.render_debug(ui);
        });
    }
}
