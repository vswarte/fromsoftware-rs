use eldenring::cs::{CSWorldGeomIns, CSWorldGeomMan, CSWorldGeomManBlockData};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSWorldGeomMan {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Loaded blocks: {}", self.blocks.len()));
        if ui.collapsing_header("Loaded blocks", TreeNodeFlags::empty()) {
            ui.indent();
            for block in self.blocks.iter() {
                let label = format!("{}", block.map_id);
                if ui.collapsing_header(label, TreeNodeFlags::empty()) {
                    block.data.render_debug(ui);
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Current Unk Block", TreeNodeFlags::empty()) {
            ui.indent();
            self.curent_99_block_data.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for CSWorldGeomManBlockData {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Block ID: {}", self.map_id));
        ui.text(format!("World block info: {:x}", self.world_block_info));

        ui.text(format!(
            "Next GeomIns FieldIns index: {}",
            self.next_geom_ins_field_ins_index
        ));

        ui.text(format!("Objects in vector: {}", self.geom_ins_vector.len()));
        if ui.collapsing_header("Geometry Vector", TreeNodeFlags::empty()) {
            ui.indent();
            for geometry_ins in self.geom_ins_vector.items() {
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

                if ui.collapsing_header(
                    format!(
                        "{} - {} FieldInsSelector({}, {})",
                        name,
                        geometry_ins.field_ins_handle.map_id,
                        geometry_ins.field_ins_handle.selector.container(),
                        geometry_ins.field_ins_handle.selector.index()
                    ),
                    TreeNodeFlags::empty(),
                ) {
                    ui.indent();
                    geometry_ins.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Sign Geometry Vector", TreeNodeFlags::empty()) {
            ui.indent();
            for geometry_ins in self.sos_sign_geometry.items() {
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

                if ui.collapsing_header(
                    format!(
                        "{} - {} FieldInsSelector({}, {})",
                        name,
                        geometry_ins.field_ins_handle.map_id,
                        geometry_ins.field_ins_handle.selector.container(),
                        geometry_ins.field_ins_handle.selector.index()
                    ),
                    TreeNodeFlags::empty(),
                ) {
                    ui.indent();
                    geometry_ins.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }
    }
}

impl DebugDisplay for CSWorldGeomIns {
    fn render_debug(&self, _ui: &&mut Ui) {}
}
