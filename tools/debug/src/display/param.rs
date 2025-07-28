use eldenring::fd4::FD4ParamRepository;
use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags};

use super::DebugDisplay;

impl DebugDisplay for FD4ParamRepository {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        ui.text(format!(
            "ResCapHolder map bucket count: {:?}",
            self.res_rep.res_cap_holder.bucket_count
        ));

        if ui.collapsing_header("Resources", TreeNodeFlags::empty()) {
            if let Some(_t) = ui.begin_table_header_with_flags(
                "fd4-param-repository-rescaps",
                [
                    TableColumnSetup::new("Name"),
                    TableColumnSetup::new("Row Count"),
                    TableColumnSetup::new("Paramdef Version"),
                    TableColumnSetup::new("Bytes"),
                ],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            ) {
                ui.indent();
                for res_cap in self.res_rep.res_cap_holder.entries() {
                    ui.table_next_column();
                    ui.text(res_cap.data.name());

                    ui.table_next_column();
                    let row_count = res_cap.data.header.row_count;
                    ui.text(format!("{row_count:?}"));

                    ui.table_next_column();
                    let paramdef_version = res_cap.data.header.paramdef_version;
                    ui.text(format!("{paramdef_version:?}"));

                    ui.table_next_column();
                    let bytes_ptr = res_cap.data.as_ptr();
                    ui.text(format!("{:x?}", { bytes_ptr }));
                }
                ui.unindent();
            }
        }
    }
}
