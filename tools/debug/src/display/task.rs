use eldenring::cs::{CSTaskGroup, CSTaskImp};
use hudhook::imgui::*;

use super::DebugDisplay;

impl DebugDisplay for CSTaskGroup {
    fn render_debug(&self, ui: &&mut Ui) {
        for task_group in self.task_groups.iter() {
            ui.text(task_group.base.name.to_string());
        }
    }
}

impl DebugDisplay for CSTaskImp {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Task Groups", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header_with_flags(
                "task-group-table",
                [
                    TableColumnSetup::new("ID"),
                    TableColumnSetup::new("Name"),
                    TableColumnSetup::new("Active"),
                ],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            ) {
                for task_group in self.inner.task_base.task_groups.items() {
                    ui.table_next_column();
                    ui.text(format!("{:x}", task_group.index));

                    let name_bytes = task_group
                        .name
                        .iter()
                        .take_while(|c| **c != 0x0)
                        .cloned()
                        .collect::<Vec<_>>();
                    let name = String::from_utf16(name_bytes.as_slice()).unwrap();

                    ui.table_next_column();
                    ui.text(name);

                    ui.table_next_column();
                    ui.text(format!("{}", task_group.active));
                }
            }
            ui.unindent();
        }
    }
}
