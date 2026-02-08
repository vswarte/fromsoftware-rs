use hudhook::imgui::Ui;

use darksouls3::sprj::*;

use super::StatefulDebugDisplay;

#[derive(Default)]
pub struct SprjEventFlagManDebugState {
    flag: String,
}

impl StatefulDebugDisplay for SprjEventFlagMan {
    type State = SprjEventFlagManDebugState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        {
            let _tok = ui.push_item_width(150.);
            ui.input_text("Flag value:", &mut state.flag).build();
        }
        ui.same_line();

        let flag_value = state
            .flag
            .parse::<u32>()
            .ok()
            .and_then(|i| EventFlag::try_from(i).ok());
        ui.text(if let Some(flag_value) = flag_value {
            if self.get_flag(flag_value) {
                "true"
            } else {
                "false"
            }
        } else {
            "invalid"
        });

        ui.same_line_with_pos(ui.window_content_region_max()[0] - 100.);
        {
            let _tok = ui.begin_enabled(flag_value.is_some());
            if ui.button("Toggle") {
                let flag_value = flag_value.unwrap();
                self.set_flag(flag_value, !self.get_flag(flag_value));
            }
        }
    }
}
