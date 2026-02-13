use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{
    BreakInAreaList, BreakInData, BreakInManager, BreakInTarget, CSBattleRoyalContext,
    CSNetBloodMessageDb, CSNetBloodMessageDbItem, CSNetMan, CSQuickMatchingCtrl, QuickmatchManager,
};

use super::DebugDisplay;

impl DebugDisplay for CSNetMan {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Blood Messages", || {
            self.blood_message_db.render_debug(ui);
        });

        ui.header("Break In", || {
            self.breakin_manager.render_debug(ui);
        });

        ui.header("Quickmatch", || {
            self.quickmatch_manager.render_debug(ui);
        });
    }
}

impl DebugDisplay for BreakInManager {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Multiplay Type: {:?}", self.multiplay_type));

        ui.list("Break In Targets", self.targets.items(), |ui, _i, item| {
            item.render_debug(ui);
        });

        ui.header("Break In Data", || {
            self.data.render_debug(ui);
        });
        ui.text(format!("Error Code: {:?}", self.error_code));
        ui.header("Break In Areas", || {
            self.areas.render_debug(ui);
        });
        ui.text(format!(
            "Invasion Search State: {:?}",
            self.invasion_search_state
        ));

        ui.text(format!(
            "Prev Invasion Search State: {:?}",
            self.last_update_invasion_search_state
        ));

        ui.text(format!(
            "Attempt Interval Timer: {:?}",
            self.attempt_interval_timer.time
        ));
        ui.text(format!("Time Out Timer: {:?}", self.time_out_timer.time));

        ui.text(format!(
            "Is Yellow Monk: {:?}",
            self.is_yellow_costume_region
        ));

        ui.text(format!("Is Multi Region: {:?}", self.is_multi_region));
    }
}

impl DebugDisplay for BreakInTarget {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Player ID: {}", self.player_id));
        let mut steam_id_str =
            String::from_utf8(self.external_id.items().to_vec()).unwrap_or("Invalid".to_owned());
        ui.input_text("Steam ID", &mut steam_id_str)
            .read_only(true)
            .build();
        ui.text(format!("Play Region: {}", self.play_region))
    }
}

impl DebugDisplay for BreakInAreaList {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Count: {:?}", self.count));
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
        ui.text(format!("Block ID: {}", self.block_id));
        ui.header("Block Position", || {
            self.block_pos.render_debug(ui);
        });
        ui.text(format!("Entry File List Id: {:?}", self.entryfilelist_id));
        ui.text(format!("Summon Param Type: {:?}", self.summon_param_type));
        ui.text(format!("Multi Play Role: {:?}", self.multiplay_role));
        ui.text(format!("Has Password: {:?}", self.has_password));
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
        ui.header("CSQuickMatchingCtrl", || {
            self.quickmatching_ctrl.render_debug(ui);
        });

        ui.header("CSBattleRoyalContext", || {
            self.battle_royal_context.render_debug(ui);
        });
    }
}

impl DebugDisplay for CSBattleRoyalContext {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!(
            "Error State: {:?}",
            self.quickmatch_context.error_state
        ));

        ui.text(format!(
            "Match settings: {:?}",
            self.quickmatch_context.match_settings
        ));

        ui.text(format!(
            "Match map (arena): {:?}",
            self.quickmatch_context.match_map
        ));

        ui.text(format!("Max players: {}", self.match_player_count));
        ui.text(format!("Current players: {}", self.current_player_count));

        ui.input_text("Password", &mut self.password.to_string())
            .read_only(true)
            .build();
    }
}

impl DebugDisplay for CSQuickMatchingCtrl {
    fn render_debug(&self, ui: &Ui) {
        ui.input_text("Match state", &mut format!("{:?}", self.current_state))
            .read_only(true)
            .build();
    }
}
