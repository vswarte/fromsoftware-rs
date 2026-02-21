use eldenring::cs::WorldAreaTime;
use hudhook::imgui::Ui;

use debug::UiExt;

use super::DebugDisplay;

impl DebugDisplay for WorldAreaTime {
    fn render_debug(&self, ui: &Ui) {
        ui.debug_copiable("Hours", self.clock.hours());
        ui.debug_copiable("Minutes", self.clock.minutes());
        ui.debug_copiable("Seconds", self.clock.seconds());
    }
}
