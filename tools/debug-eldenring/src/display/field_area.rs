use hudhook::imgui::Ui;

use debug::UiExt;
use eldenring::cs::{FieldArea, WorldInfoOwner};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for FieldArea {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("World Info Owner", unsafe {
            self.world_info_owner.as_ref()
        });
        ui.display(
            "Enable Fast Travel Event Flag",
            self.enable_fast_travel_event_flag,
        );
        ui.display("Map Place Name ID", self.map_place_name_id);
        ui.display("Save Map Name ID", self.save_map_name_id);
        ui.display("Current Play Region ID", self.current_play_region_id);
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
                            let (block_id, data) = block_entry.into();
                            ui.header(format!("World Block Info {}", block_id), || {
                                ui.display("Center physics coords", data.physics_center);
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
                    ui.display("Center physics coords", entry.physics_center);
                });
            },
        );
    }
}
