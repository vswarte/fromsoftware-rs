use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{
    BreakInAreaList, BreakInData, BreakInManager, BreakInTarget, CSBattleRoyalContext,
    CSNetBloodMessageDb, CSNetBloodMessageDbItem, CSNetMan, CSQuickMatchingCtrl, QuickmatchManager,
};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSNetMan {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("Blood Messages", &self.blood_message_db);
        ui.nested("Break In", &self.breakin_manager);
        ui.nested("Quickmatch", &self.quickmatch_manager);
    }
}

impl DebugDisplay for BreakInManager {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Multiplay Type", self.multiplay_type);

        ui.list("Break In Targets", self.targets.items(), |ui, _i, item| {
            item.render_debug(ui);
        });

        ui.nested("Break In Data", &self.data);
        ui.debug("Error Code", self.error_code);
        ui.nested("Break In Areas", &self.areas);
        ui.debug("Invasion Search State", self.invasion_search_state);

        ui.debug(
            "Prev Invasion Search State",
            self.last_update_invasion_search_state,
        );

        ui.debug("Attempt Interval Timer", self.attempt_interval_timer.time);
        ui.display("Time Out Timer", self.time_out_timer.time);
        ui.display("Is Yellow Monk", self.is_yellow_costume_region);
        ui.display("Is Multi Region", self.is_multi_region);
    }
}

impl DebugDisplay for BreakInTarget {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Player ID", self.player_id);
        let steam_id_str =
            String::from_utf8(self.external_id.items().to_vec()).unwrap_or("Invalid".to_owned());
        ui.debug_copiable("Steam ID", steam_id_str);
        ui.display("Play Region", self.play_region)
    }
}

impl DebugDisplay for BreakInAreaList {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Count", self.count);
        ui.header("Areas", || {
            ui.table(
                "breakin-areas-list",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Area"),
                ],
                self.areas.items(),
                |ui, i, e| {
                    ui.table_next_column();
                    ui.text(format!("{i}"));

                    ui.table_next_column();
                    ui.text(format!("{e}"));
                },
            );
        });
    }
}

impl DebugDisplay for BreakInData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Block ID", self.block_id);
        ui.nested("Block Position", self.block_pos);
        ui.display("Entry File List Id", self.entryfilelist_id);
        ui.debug("Summon Param Type", self.summon_param_type);
        ui.debug("Multi Play Role", self.multiplay_role);
        ui.display("Has Password", self.has_password);
    }
}

impl DebugDisplay for CSNetBloodMessageDb {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Entries", || {
            render_message_table(self.entries.iter().map(|f| f.as_ref()), ui);
        });

        ui.header("Created message data", || {
            self.created_data
                .iter()
                .for_each(|f| ui.text(format!("{f} {f:x}")));
        });

        ui.header("Discovered messages", || {
            render_message_table(
                self.discovered_messages.iter().map(|f| f.as_ref().as_ref()),
                ui,
            );
        });
    }
}

fn render_message_table<'a>(messages: impl Iterator<Item = &'a CSNetBloodMessageDbItem>, ui: &Ui) {
    ui.table(
        "cs-net-man-blood-messages-entries",
        [
            TableColumnSetup::new("Message ID"),
            TableColumnSetup::new("Map ID"),
            TableColumnSetup::new("Placement (x, y, z, angle)"),
            TableColumnSetup::new("Template 1"),
            TableColumnSetup::new("Part 1"),
            TableColumnSetup::new("Infix"),
            TableColumnSetup::new("Template 2"),
            TableColumnSetup::new("Part 2"),
            TableColumnSetup::new("Gesture"),
        ],
        messages,
        |ui, _i, message| {
            ui.table_next_column();
            ui.text(format!("{:x}", message.message_id));

            ui.table_next_column();
            ui.text(message.block_id.to_string());

            ui.table_next_column();
            ui.text(format!(
                "{}, {}, {}, {}",
                message.position_x, message.position_y, message.position_z, message.angle,
            ));

            ui.table_next_column();
            ui.text(message.template1.to_string());

            ui.table_next_column();
            ui.text(message.part1.to_string());

            ui.table_next_column();
            ui.text(message.infix.to_string());

            ui.table_next_column();
            ui.text(message.template2.to_string());

            ui.table_next_column();
            ui.text(message.part2.to_string());

            ui.table_next_column();
            ui.text(message.gesture_param.to_string());
        },
    );
}

impl DebugDisplay for QuickmatchManager {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("CSQuickMatchingCtrl", &self.quickmatching_ctrl);
        ui.nested("CSBattleRoyalContext", &self.battle_royal_context);
    }
}

impl DebugDisplay for CSBattleRoyalContext {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Error State", self.quickmatch_context.error_state);
        ui.debug("Match settings", self.quickmatch_context.match_settings);
        ui.debug("Match map (arena)", self.quickmatch_context.match_map);
        ui.display("Max players", self.match_player_count);
        ui.display("Current players", self.current_player_count);
        ui.display_copiable("Password", &self.password);
    }
}

impl DebugDisplay for CSQuickMatchingCtrl {
    fn render_debug(&self, ui: &Ui) {
        ui.display_copiable("Match state", format!("{:?}", self.current_state));
    }
}
