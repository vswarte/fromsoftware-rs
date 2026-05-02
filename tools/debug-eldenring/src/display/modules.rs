use eldenring::cs::ChrInsModuleContainer;
use hudhook::imgui::Ui;

use crate::display::{DebugDisplay, DisplayUiExt};

mod action_request;
mod behavior_data;
mod ladder;
mod model_param_modifier;
mod physics;
mod ride;
mod time_act;

impl DebugDisplay for ChrInsModuleContainer {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("Time Act", &self.time_act);
        ui.nested("Physics", &self.physics);
        ui.nested("Ladder", &self.ladder);
        ui.nested("Action Request", &self.action_request);
        ui.nested("Behavior Data", &self.behavior_data);
        ui.nested("Model param modifier", &self.model_param_modifier);
        ui.nested("Ride", &self.ride);
    }
}
