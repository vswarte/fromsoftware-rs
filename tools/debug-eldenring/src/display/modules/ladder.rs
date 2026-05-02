use crate::display::DebugDisplay;
use eldenring::cs::CSChrLadderModule;
use hudhook::imgui::Ui;
impl DebugDisplay for CSChrLadderModule {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Ladder handle: {:?}", self.ladder_handle));
        ui.text(format!("State: {:?}", self.state));
        ui.text(format!("Top: {:?}", self.top));
        ui.text(format!("Bottom: {:?}", self.bottom));
    }
}
