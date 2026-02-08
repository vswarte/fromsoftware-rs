use hudhook::imgui::Ui;

use darksouls3::sprj::*;
use debug::UiExt;

use super::DebugDisplay;

impl DebugDisplay for FieldArea {
    fn render_debug(&self, ui: &Ui) {
        if let Some(world_res) = self.world_res() {
            world_res.super_world_info.render_debug(ui);
        } else {
            ui.text("World res: null");
        }
    }
}

impl DebugDisplay for WorldInfo {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            format!("Area infos: {} ##{:p}", self.area_info().len(), self),
            self.area_info(),
            |ui, _, area_info| {
                ui.header(
                    format!("Area {} ##{:p}", area_info.area_number, area_info),
                    || area_info.render_debug(ui),
                );
            },
        );
    }
}

impl DebugDisplay for WorldAreaInfo {
    fn render_debug(&self, ui: &Ui) {
        for block in self.block_info() {
            ui.text(format!(
                "Block {}: event index {}",
                block.block_id.group(),
                block.world_block_index
            ));
        }
    }
}
