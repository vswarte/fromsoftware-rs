use eldenring::cs::CSWorldAiManagerImp;
use hudhook::imgui::{TableColumnSetup, Ui};

use super::{DebugDisplay, UiExt};

impl DebugDisplay for CSWorldAiManagerImp {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            "Goal strategies",
            self.goal_strategies.iter(),
            |ui, _i, entry| {
                ui.text(format!("{:?}", entry.goal_id));
            },
        );
    }
}
