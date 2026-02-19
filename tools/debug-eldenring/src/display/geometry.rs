use hudhook::imgui::Ui;

use debug::UiExt;
use eldenring::cs::{CSWorldGeomIns, CSWorldGeomMan, CSWorldGeomManBlockData};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSWorldGeomMan {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            format!("Loaded blocks: {}", self.blocks.len()),
            self.blocks.iter(),
            |ui, _i, block| ui.nested(format!("{}", block.block_id), &block.data),
        );

        ui.nested("Current Unk Block", &self.curent_99_block_data);
    }
}

impl DebugDisplay for CSWorldGeomManBlockData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Block ID", self.block_id);
        ui.text(format!("World block info: {:x}", self.world_block_info));
        ui.display(
            "Next GeomIns FieldIns index",
            self.next_geom_ins_field_ins_index,
        );

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

                ui.nested(
                    format!(
                        "{} - {} FieldInsSelector({}, {})",
                        name,
                        geometry_ins.field_ins_handle.block_id,
                        geometry_ins.field_ins_handle.selector.container(),
                        geometry_ins.field_ins_handle.selector.index()
                    ),
                    geometry_ins,
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

                ui.nested(
                    format!(
                        "{} - {} FieldInsSelector({}, {})",
                        name,
                        geometry_ins.field_ins_handle.block_id,
                        geometry_ins.field_ins_handle.selector.container(),
                        geometry_ins.field_ins_handle.selector.index()
                    ),
                    geometry_ins,
                );
            },
        );
    }
}

impl DebugDisplay for CSWorldGeomIns {
    fn render_debug(&self, _ui: &Ui) {}
}
