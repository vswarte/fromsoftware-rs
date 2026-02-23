use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use sekiro::sprj::SoloParamRepository;

use super::DebugDisplay;

impl DebugDisplay for SoloParamRepository {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Resources", || {
            ui.table(
                "fd4-param-repository-rescaps",
                [
                    TableColumnSetup::new("Param Name"),
                    TableColumnSetup::new("Struct Name"),
                    TableColumnSetup::new("Row Count"),
                    TableColumnSetup::new("Paramdef Version"),
                    TableColumnSetup::new("Bytes"),
                ],
                self.params(),
                |ui, _i, res_cap| {
                    ui.table_next_column();
                    ui.text(res_cap.name.to_string());

                    ui.table_next_column();
                    ui.text(res_cap.data.struct_name());

                    ui.table_next_column();
                    let row_count = res_cap.data.row_count();
                    ui.text(format!("{row_count:?}"));

                    ui.table_next_column();
                    let paramdef_version = res_cap.data.paramdef_version();
                    ui.text(format!("{paramdef_version:?}"));

                    ui.table_next_column();
                    let bytes_ptr = res_cap.data.as_ptr();
                    ui.text(format!("{:x?}", bytes_ptr));
                },
            );
        });
    }
}
