use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{
    CSSessionManager, CSStayInMultiplayAreaWarpData, SessionManagerPlayerEntry,
    SessionManagerPlayerEntryBase,
};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSSessionManager {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Lobby state", self.lobby_state);
        ui.debug("Protocol state", self.protocol_state);
        ui.display("Session player limit", self.session_player_limit);

        ui.display(
            "Session player limit override",
            self.session_player_limit_override,
        );

        ui.list("Members", self.players.items(), |ui, _i, player| {
            player.render_debug(ui)
        });

        ui.nested_opt("Host", self.host_player.as_option());

        ui.nested(
            "Stay in Multiplay Area Warp Data",
            self.stay_in_multiplay_area_warp_data.as_ref(),
        );
    }
}

impl DebugDisplay for SessionManagerPlayerEntryBase {
    fn render_debug(&self, ui: &Ui) {
        ui.display_copiable("Steam Name", &self.steam_name);
        ui.display_copiable("Steam ID", self.steam_id);
    }
}

impl DebugDisplay for SessionManagerPlayerEntry {
    fn render_debug(&self, ui: &Ui) {
        self.base.render_debug(ui);
        ui.display("Game data index", self.game_data_index);
        ui.display("Is host", self.is_host);
    }
}

impl DebugDisplay for CSStayInMultiplayAreaWarpData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Multiplay Start Area ID", self.multiplay_start_area_id);
        ui.display("Saved block ID", self.saved_block_id);
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

        ui.display("Warp Request Delay", self.warp_request_delay);
        ui.display(
            "Disable Multiplay Restriction",
            self.disable_multiplay_restriction,
        );
        ui.display("Is Warp Possible", self.is_warp_possible);
    }
}
