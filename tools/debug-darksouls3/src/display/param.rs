use hudhook::imgui::{TableColumnSetup, Ui};

use darksouls3::sprj::CSRegulationManager;
use debug::UiExt;

use super::DebugDisplay;

impl DebugDisplay for CSRegulationManager {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Resources", || {
            ui.table(
                "fd4-param-repository-rescaps",
                [
                    TableColumnSetup::new("Name"),
                    TableColumnSetup::new("Row Count"),
                    TableColumnSetup::new("Bytes"),
                ],
                &self.params,
                |ui, _, res_cap| {
                    let table = &res_cap.param.table;
                    ui.table_next_column();
                    ui.text(table.name());

                    ui.table_next_column();
                    ui.text(format!("{}", table.length));

                    ui.table_next_column();
                    ui.text(format!("{:p}", table.data()));
                },
            );
        });
    }
}
