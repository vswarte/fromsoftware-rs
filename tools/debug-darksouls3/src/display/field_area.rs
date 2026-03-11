use hudhook::imgui::Ui;

use darksouls3::sprj::*;
use debug::UiExt;

use super::DebugDisplay;

impl DebugDisplay for FieldArea {
    fn render_debug(&self, ui: &Ui) {
        self.world_info_owner.render_debug(ui);
    }
}

impl DebugDisplay for WorldInfoOwner {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            format!("Area infos: {} ##{:p}", self.area_info().len(), self),
            self.area_and_block_info(),
            |ui, _, (area_info, block_infos)| {
                ui.header(
                    format!("Area {} ##{:p}", area_info.area_number, area_info),
                    || {
                        for block_info in block_infos {
                            block_info.render_debug(ui);
                        }
                    },
                );
            },
        );
    }
}

impl DebugDisplay for WorldBlockInfo {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!(
            "Block {}: event index {}",
            self.block_id.group(),
            self.world_block_index
        ));
    }
}
