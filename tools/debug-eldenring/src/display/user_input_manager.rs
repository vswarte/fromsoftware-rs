use debug::UiExt;
use eldenring::dluid::{DLUserInputManagerImpl, DLUserInputManagerImplBase};

use crate::display::DebugDisplay;

impl DebugDisplay for DLUserInputManagerImpl {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        self.base.render_debug(ui);
        ui.header("User Input Devices", || {
            self.user_input_devices.render_debug(ui);
        });
        ui.text(format!("Window handle: {:#}", self.window_handle));
        ui.header("DummyDevice", || {
            self.dummy_device.render_debug(ui);
        });
        ui.text(format!("is_co_initialized: {}", self.is_co_initialized));
        ui.text(format!("use_lib_sce_pad: {}", self.use_lib_sce_pad));
        ui.text(format!(
            "is_game_window_focused: {}",
            self.is_game_window_focused
        ));
        ui.text(format!("set_foreground_pad: {}", self.set_foreground_pad));
        ui.text(format!(
            "set_foreground_keyboard: {}",
            self.set_foreground_keyboard
        ));
        ui.text(format!(
            "set_foreground_mouse: {}",
            self.set_foreground_mouse
        ));
    }
}

impl DebugDisplay for DLUserInputManagerImplBase {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("DLPlainLightMutex", || {
            ui.text(format!("{:#?}", self.mutex.critical_section));
        });
    }
}
