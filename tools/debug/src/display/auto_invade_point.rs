use eldenring::cs::{AutoInvadePointBlockEntry, CSAutoInvadePoint};
use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSAutoInvadePoint {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Blocks", TreeNodeFlags::empty()) {
            ui.indent();
            for block_entry in self.entries.iter() {
                if ui.collapsing_header(block_entry.block_id.to_string(), TreeNodeFlags::empty()) {
                    ui.indent();
                    block_entry.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }
    }
}

impl DebugDisplay for AutoInvadePointBlockEntry {
    fn render_debug(&self, ui: &&mut Ui) {
        if let Some(_t) = ui.begin_table_header_with_flags(
            "cs-auto-invade-point-items",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("X"),
                TableColumnSetup::new("Y"),
                TableColumnSetup::new("Z"),
                TableColumnSetup::new("Yaw"),
            ],
            TableFlags::RESIZABLE
                | TableFlags::BORDERS
                | TableFlags::ROW_BG
                | TableFlags::SIZING_STRETCH_PROP,
        ) {
            for (i, item) in self.items().iter().enumerate() {
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
            }
        }
    }
}
