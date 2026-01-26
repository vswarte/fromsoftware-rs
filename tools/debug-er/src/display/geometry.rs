use hudhook::imgui::Ui;

use debug::UiExt;
use eldenring::cs::{CSWorldGeomIns, CSWorldGeomMan, CSWorldGeomManBlockData};

use super::DebugDisplay;

impl DebugDisplay for CSWorldGeomMan {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            format!("Loaded blocks: {}", self.blocks.len()),
            self.blocks.iter(),
            |ui, _i, block| {
                ui.header(format!("{}", block.block_id), || {
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
            format!("Geometry Vector ({})", self.geom_ins_vector.len()),
            self.geom_ins_vector.items(),
            |ui, _i, geometry_ins| {
                let name = unsafe {
                    geometry_ins
                        .info
                        .msb_parts_geom
                        .msb_parts
                        .msb_part
                        .name
                        .to_string()
                }
                .unwrap();

                ui.header(
                    format!(
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
            format!("Sign Geometry Vector ({})", self.sos_sign_geometry.len()),
            self.sos_sign_geometry.items(),
            |ui, _i, geometry_ins| {
                let name = unsafe {
                    geometry_ins
                        .info
                        .msb_parts_geom
                        .msb_parts
                        .msb_part
                        .name
                        .to_string()
                }
                .unwrap();

                ui.header(
                    format!(
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
    fn render_debug(&self, _ui: &Ui) {}
}
