use crate::display::DebugDisplay;
use debug::UiExt;
use eldenring::cs::CSChrTimeActModule;
use hudhook::imgui::{TableColumnSetup, Ui};

impl DebugDisplay for CSChrTimeActModule {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "chr-ins-time-act-module",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("Anim ID"),
                TableColumnSetup::new("Play Time"),
                TableColumnSetup::new("Length"),
            ],
            self.anim_queue.iter(),
            |ui, index, entry| {
                ui.table_next_column();
                ui.text(index.to_string());

                ui.table_next_column();
                ui.text(entry.anim_id.to_string());

                ui.table_next_column();
                ui.text(entry.play_time.to_string());

                ui.table_next_column();
                ui.text(entry.anim_length.to_string());
            },
        );
        ui.display("Read IDX", self.read_idx);
        ui.display("Write IDX", self.write_idx);
        ui.header("Current Anim Info", || {
            let current_anim_info = &self.anim_queue[self.read_idx as usize];
            ui.display("Anim ID", current_anim_info.anim_id);
            ui.display("Play Time", current_anim_info.play_time);
            ui.display("Anim Length", current_anim_info.anim_length);
        });
    }
}
