use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{AutoInvadePointBlockEntry, CSAutoInvadePoint};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSAutoInvadePoint {
    fn render_debug(&self, ui: &Ui) {
        ui.list("Entries", self.entries.iter(), |ui, _i, block_entry| {
            ui.nested(format!("Block {}", block_entry.block_id), block_entry);
        });
    }
}

impl DebugDisplay for AutoInvadePointBlockEntry {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.table(
            "items",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("X"),
                TableColumnSetup::new("Y"),
                TableColumnSetup::new("Z"),
                TableColumnSetup::new("Yaw"),
            ],
            self.items().iter(),
            |ui, i, item| {
                ui.table_next_column();
                ui.text(i.to_string());
                ui.table_next_column();
                ui.text(format!("{:.2}", item.position.0));
                ui.table_next_column();
                ui.text(format!("{:.2}", item.position.1));
                ui.table_next_column();
                ui.text(format!("{:.2}", item.position.2));
                ui.table_next_column();
                ui.text(format!("{:.2}", item.yaw));
            },
        );
    }
}
