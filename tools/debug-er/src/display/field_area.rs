use hudhook::imgui::Ui;

use debug::UiExt;
use eldenring::cs::{FieldArea, WorldInfoOwner};

use super::DebugDisplay;

impl DebugDisplay for FieldArea {
    fn render_debug(&self, ui: &Ui) {
        ui.header("World Info Owner", || {
            self.world_info_owner.render_debug(ui);
        });
    }
}

impl DebugDisplay for WorldInfoOwner {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            format!(
                "WorldAreaInfo - {}",
                self.world_res.world_info.world_area_info_count
            ),
            self.world_res.world_info.world_area_info().iter(),
            |ui, _i, entry| {
                ui.header(format!("World Area Info {}", entry.base.block_id), || {
                    // chr_set.render_debug(ui);
                });
            },
        );

        ui.list(
            format!(
                "WorldGridAreaInfo - {}",
                self.world_res.world_info.world_grid_area_info_count
            ),
            self.world_res.world_info.world_grid_area_info().iter(),
            |ui, _i, entry| {
                ui.header(
                    format!("World Grid Area Info {}", entry.base.block_id),
                    || {
                        ui.list("Blocks", entry.blocks.iter(), |ui, _i, block_entry| {
                            ui.header(format!("World Block Info {}", block_entry.block_id), || {
                                ui.text(format!(
                                    "Center physics coords: {}",
                                    block_entry.block.physics_center
                                ));
                            });
                        });
                    },
                );
            },
        );

        ui.list(
            format!(
                "WorldBlockInfo - {}",
                self.world_res.world_info.world_block_info_count
            ),
            self.world_res.world_info.world_block_info().iter(),
            |ui, _i, entry| {
                ui.header(format!("World Block Info {}", entry.block_id), || {
                    ui.text(format!("Center physics coords: {}", entry.physics_center));
                });
            },
        );
    }
}
