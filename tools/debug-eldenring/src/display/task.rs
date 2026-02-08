use hudhook::imgui::*;

use debug::UiExt;
use eldenring::cs::{CSTaskGroup, CSTaskImp};

use super::DebugDisplay;

impl DebugDisplay for CSTaskGroup {
    fn render_debug(&self, ui: &Ui) {
        for task_group in self.task_groups.iter() {
            ui.text(task_group.base.name.to_string());
        }
    }
}

impl DebugDisplay for CSTaskImp {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Task Groups", || {
            ui.table(
                "task-group-table",
                [
                    TableColumnSetup::new("ID"),
                    TableColumnSetup::new("Name"),
                    TableColumnSetup::new("Active"),
                ],
                self.inner.task_base.task_groups.items(),
                |ui, _i, task_group| {
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
                },
            );
        });
    }
}
