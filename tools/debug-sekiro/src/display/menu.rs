use hudhook::imgui::Ui;

use debug::UiExt;
use sekiro::app_menu::*;

use super::DebugDisplay;

impl DebugDisplay for NewMenuSystem {
    fn render_debug(&self, ui: &Ui) {
        ui.list("Windows", self.windows.iter(), |ui, i, window| {
            ui.pointer(format!("{i}"), window)
        });
    }
}
