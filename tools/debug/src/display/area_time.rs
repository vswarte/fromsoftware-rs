use eldenring::cs::WorldAreaTime;
use hudhook::imgui::Ui;

use super::DebugDisplay;

impl DebugDisplay for WorldAreaTime {
    fn render_debug(&self, ui: &Ui) {
        ui.input_text("Hours", &mut self.clock.hours().to_string())
            .read_only(true)
            .build();

        ui.input_text("Minutes", &mut self.clock.minutes().to_string())
            .read_only(true)
            .build();

        ui.input_text("Seconds", &mut self.clock.seconds().to_string())
            .read_only(true)
            .build();
    }
}
