use eldenring::cs::{
    CSSessionManager, CSStayInMultiplayAreaWarpData, SessionManagerPlayerEntry,
    SessionManagerPlayerEntryBase,
};
use hudhook::imgui::{TableColumnSetup, Ui};

use super::{DebugDisplay, UiExt};

impl DebugDisplay for CSSessionManager {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Lobby state: {:?}", self.lobby_state));
        ui.text(format!("Protocol state: {:?}", self.protocol_state));
        ui.text(format!(
            "Session player limit: {}",
            self.session_player_limit
        ));
        ui.text(format!(
            "Session player limit override: {}",
            self.session_player_limit_override
        ));

        ui.list("Members", self.players.items(), |ui, _i, player| {
            player.render_debug(ui);
        });

        if self.host_player.steam_id != 0x0 {
            ui.header("Host", || {
                self.host_player.render_debug(ui);
            });
        }

        ui.header("Stay in Multiplay Area Warp Data", || {
            self.stay_in_multiplay_area_warp_data
                .as_ref()
                .render_debug(ui);
        });
    }
}

impl DebugDisplay for SessionManagerPlayerEntryBase {
    fn render_debug(&self, ui: &Ui) {
        ui.input_text("Steam Name", &mut self.steam_name.to_string())
            .read_only(true)
            .build();
        ui.input_text("Steam ID", &mut self.steam_id.to_string())
            .read_only(true)
            .build();
    }
}

impl DebugDisplay for SessionManagerPlayerEntry {
    fn render_debug(&self, ui: &Ui) {
        self.base.render_debug(ui);
        ui.text(format!("Game data index: {}", self.game_data_index));
        ui.text(format!("Is host: {}", self.is_host));
    }
}

impl DebugDisplay for CSStayInMultiplayAreaWarpData {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!(
            "Multiplay Start Area ID: {}",
            self.multiplay_start_area_id
        ));
        ui.text(format!("Saved block ID: {}", self.saved_block_id));
        ui.text(format!(
            "Saved Position: ({}, {}, {})",
            self.saved_position.0, self.saved_position.1, self.saved_position.2
        ));
        ui.header("Fade out tracker", || {
            ui.table(
                "session-manager-fade-out-tracker",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Steam ID"),
                    TableColumnSetup::new("Fade time"),
                ],
                self.player_fade_tracker.items().iter(),
                |ui, index, item| {
                    ui.table_next_column();
                    ui.text(index.to_string());
                    ui.table_next_column();
                    ui.text(item.steam_id.to_string());
                    ui.table_next_column();
                    ui.text(item.fade_time.to_string());
                },
            );
        });

        ui.text(format!("Warp Request Delay: {}", self.warp_request_delay));
        ui.text(format!(
            "Disable Multiplay Restriction: {}",
            self.disable_multiplay_restriction
        ));
        ui.text(format!("Is Warp Possible: {}", self.is_warp_possible));
    }
}
