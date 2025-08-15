use eldenring::cs::{FieldArea, WorldInfoOwner};
use hudhook::imgui::{TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for FieldArea {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("World Info Owner", TreeNodeFlags::empty()) {
            ui.indent();
            self.world_info_owner.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for WorldInfoOwner {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header(
            format!(
                "WorldAreaInfo - {}",
                self.world_res.world_info.world_area_info_count
            ),
            TreeNodeFlags::empty(),
        ) {
            ui.indent();
            for entry in self.world_res.world_info.world_area_info().iter() {
                if ui.collapsing_header(
                    format!("World Area Info {}", entry.base.block_id),
                    TreeNodeFlags::empty(),
                ) {
                    // chr_set.render_debug(ui);
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header(
            format!(
                "WorldGridAreaInfo - {}",
                self.world_res.world_info.world_grid_area_info_count
            ),
            TreeNodeFlags::empty(),
        ) {
            ui.indent();
            for entry in self.world_res.world_info.world_grid_area_info().iter() {
                if ui.collapsing_header(
                    format!("World Grid Area Info {}", entry.base.block_id),
                    TreeNodeFlags::empty(),
                ) {
                    ui.indent();
                    entry.blocks.iter().for_each(|entry| {
                        if ui.collapsing_header(
                            format!("World Block Info {}", entry.block_id),
                            TreeNodeFlags::empty(),
                        ) {
                            ui.indent();
                            ui.text(format!(
                                "Center physics coords: {}",
                                entry.block.physics_center
                            ));
                            ui.unindent();
                        }
                    });
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header(
            format!(
                "WorldBlockInfo - {}",
                self.world_res.world_info.world_block_info_count
            ),
            TreeNodeFlags::empty(),
        ) {
            ui.indent();
            for entry in self.world_res.world_info.world_block_info().iter() {
                if ui.collapsing_header(
                    format!("World Block Info {}", entry.block_id),
                    TreeNodeFlags::empty(),
                ) {
                    ui.indent();
                    ui.text(format!("Center physics coords: {}", entry.physics_center));
                    ui.unindent();
                }
            }
            ui.unindent();
        }
    }
}
