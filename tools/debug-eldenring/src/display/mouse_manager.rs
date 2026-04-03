use eldenring::cs::CSMouseMan;

use crate::display::DebugDisplay;

impl DebugDisplay for CSMouseMan {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.text(format!("Show cursor: {}", self.show_cursor));

        ui.text(format!(
            "Mouse base coefficient: {}",
            self.mouse_base_coefficient
        ));
        ui.text(format!(
            "Mouse coefficient min: {}",
            self.mouse_coefficient_min
        ));
        ui.text(format!(
            "Mouse coefficient max: {}",
            self.mouse_coefficient_max
        ));

        ui.text(format!("Cursor X: {}", self.cursor_x));
        ui.text(format!("Cursor Y: {}", self.cursor_y));

        ui.text(format!("Is mouse moving: {}", self.is_mouse_moving));
        ui.text(format!("Is mouse in window: {}", self.is_mouse_in_window));
    }
}
