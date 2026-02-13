use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{
    CSBattleRoyalContext, CSNetBloodMessageDb, CSNetBloodMessageDbItem, CSNetMan,
    CSQuickMatchingCtrl, QuickmatchManager,
};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSNetMan {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("Blood Messages", &self.blood_message_db);
        ui.nested("Quickmatch", &self.quickmatch_manager);
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
        ui.display_copiable("Match state", format!("{:?}", self.stepper.current_state));
    }
}
