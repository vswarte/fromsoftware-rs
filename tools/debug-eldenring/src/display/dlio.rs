use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::dlio::DLFileDeviceManager;

use crate::display::DebugDisplay;

impl DebugDisplay for DLFileDeviceManager {
    fn render_debug(&self, ui: &Ui) {
        ui.input_text("File Device Count", &mut self.devices.len().to_string())
            .read_only(true)
            .build();

        ui.header("Virtual Roots", || {
            ui.table(
                "dl-file-device-manager-virtual-roots",
                [
                    TableColumnSetup::new("Root"),
                    TableColumnSetup::new("Mount"),
                ],
                self.virtual_roots.items().iter(),
                |ui, _i, vr| {
                    ui.table_next_column();
                    ui.text(vr[0].to_string());
                    ui.table_next_column();
                    ui.text(vr[1].to_string());
                },
            );
        });

        ui.header("BND4 Files", || {
            ui.table(
                "dl-file-device-manager-bnd4-files",
                [
                    TableColumnSetup::new("Name"),
                    TableColumnSetup::new("File Size"),
                ],
                self.bnd4_files.items().iter(),
                |ui, _i, file| {
                    ui.table_next_column();
                    ui.text(file.name.to_string());

                    ui.table_next_column();
                    ui.text(file.file_size.to_string());
                },
            );
        });
    }
}
