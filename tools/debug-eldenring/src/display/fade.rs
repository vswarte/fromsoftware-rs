use eldenring::cs::{CSFD4FadePlate, CSFade};
use hudhook::imgui::Ui;

use debug::UiExt;

use super::DebugDisplay;

impl DebugDisplay for CSFade {
    fn render_debug(&self, ui: &Ui) {
        ui.text("Fade plates");
        for fade_plate in self.fade_plates.iter() {
            let _ = unsafe {
                windows::core::PCWSTR::from_raw(fade_plate.title.as_ptr())
                    .to_string()
                    .unwrap()
            };
        }
    }
}

impl DebugDisplay for CSFD4FadePlate {
    fn render_debug(&self, ui: &Ui) {
        let mut current_color: [f32; 4] = (&self.current_color).into();
        ui.color_edit4("current_color", &mut current_color);

        let mut start_color: [f32; 4] = (&self.start_color).into();
        ui.color_edit4("start_color", &mut start_color);

        let mut end_color: [f32; 4] = (&self.end_color).into();
        ui.color_edit4("end_color", &mut end_color);

        ui.display_copiable("Fade timer", self.fade_timer.time);
        ui.display_copiable("Fade duration", self.fade_duration.time);
    }
}
