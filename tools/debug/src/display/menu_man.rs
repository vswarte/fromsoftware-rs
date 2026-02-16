use eldenring::cs::{CSMenuManImp, CSPopupMenu, WorldMapLegacyConverter, WorldMapViewModel};

use hudhook::imgui::{TableColumnSetup, Ui};

use crate::display::UiExt;

use super::DebugDisplay;

impl DebugDisplay for CSMenuManImp {
    fn render_debug(&self, ui: &Ui) {
        if let Some(popup_menu) = self.popup_menu.map(|m| unsafe { m.as_ref() }) {
            ui.header("Popup Menu", || {
                popup_menu.render_debug(ui);
            });
        }
    }
}

impl DebugDisplay for CSPopupMenu {
    fn render_debug(&self, ui: &Ui) {
        if let Some(world_map_view_model) = self.world_map_view_model.map(|m| unsafe { m.as_ref() })
        {
            ui.header("World Map View Model", || {
                world_map_view_model.render_debug(ui);
            });
        }
    }
}

impl DebugDisplay for WorldMapViewModel {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Main player block: {:?}", self.main_player_block));
        ui.text(format!(
            "Main player block position: {:?}",
            self.main_player_block_position
        ));
        ui.text(format!(
            "Main player orientation: {:?}",
            self.main_player_orientation
        ));
        ui.text(format!(
            "Disable main player marker: {:?}",
            self.disable_main_player_marker
        ));
        ui.text(format!(
            "Player map position: {:?}",
            self.main_player_map_position
        ));

        ui.header("World map legacy converter", || {
            self.legacy_converter.render_debug(ui);
        });

        ui.header("Warp Pins", || {
            ui.table(
                "world-map-warp-pins",
                [
                    TableColumnSetup::new("Position"),
                    TableColumnSetup::new("Entity ID"),
                    TableColumnSetup::new("Cleared event flag ID"),
                ],
                self.warp_pins.items.items(),
                |ui, _i, item| {
                    ui.table_next_column();
                    ui.text(format!("{:?}", item.pin.position));

                    ui.table_next_column();
                    ui.text(format!("{}", item.pin.entity_id));

                    ui.table_next_column();
                    ui.text(format!("{}", item.pin.cleared_event_flag_id));
                },
            );
        });
    }
}

impl DebugDisplay for WorldMapLegacyConverter {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Blocks", || {
            ui.table(
                "world-map-legacy-converter-blosk",
                [
                    TableColumnSetup::new("Block ID"),
                    TableColumnSetup::new("Override Block ID"),
                    TableColumnSetup::new("X"),
                    TableColumnSetup::new("Y"),
                    TableColumnSetup::new("Z"),
                ],
                self.entries.iter(),
                |ui, _i, item| {
                    ui.table_next_column();
                    ui.text(format!("{}", item.block_id));

                    ui.table_next_column();
                    ui.text(format!("{}", item.override_block_id));

                    ui.table_next_column();
                    ui.text(format!("{}", item.position.0));

                    ui.table_next_column();
                    ui.text(format!("{}", item.position.1));

                    ui.table_next_column();
                    ui.text(format!("{}", item.position.2));
                },
            );
        });
    }
}
