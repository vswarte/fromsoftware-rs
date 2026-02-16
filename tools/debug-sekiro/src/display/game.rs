use hudhook::imgui::Ui;

use debug::UiExt;
use sekiro::sprj::*;

use super::DebugDisplay;

impl DebugDisplay for GameDataMan {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Options", || self.options_data.render_debug(ui));
    }
}

impl DebugDisplay for OptionsData {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Camera speed: {}", self.camera_speed));
        ui.text(format!("Pad vibration: {}", self.pad_vibration));
        ui.text(format!("Brightness (SDR): {}", self.brightness_sdr));
        ui.text(format!("Sound type: {}", self.sound_type));
        ui.text(format!("Volume (Music): {}", self.volume_music));
        ui.text(format!("Volume (Effects): {}", self.volume_effects));
        ui.text(format!("Volume (Voice): {}", self.volume_voice));
        ui.text(format!("Blood level: {:?}", self.blood_level));
        ui.text(format!("Show captions: {}", self.show_captions));
        ui.text(format!("Hud visible: {:?}", self.hud_visible));
        ui.text(format!("Invert Camera (X): {}", self.invert_camera_x));
        ui.text(format!("Invert Camera (Y): {}", self.invert_camera_y));
        ui.text(format!("Auto lock: {}", self.auto_lock));
        ui.text(format!("Auto avoid wall: {}", self.auto_avoid_wall));
        ui.text(format!(
            "Enable bank register: {}",
            self.enable_bank_register
        ));
        ui.text(format!("Jump with L3: {}", self.jump_with_l3));
        ui.text(format!("Reset camera (Y): {}", self.reset_camera_y));
        ui.text(format!("Camera direction: {}", self.camera_direction));
        ui.text(format!(
            "Rank register profile index: {}",
            self.rank_register_profile_index
        ));
        ui.text(format!(
            "Allow global matching: {}",
            self.allow_global_matching
        ));
        ui.text(format!("Voice chat: {}", self.voice_chat));
        ui.text(format!(
            "Other player name notation: {}",
            self.other_player_name_notation
        ));
        ui.text(format!(
            "Auto lock on attack dir ctrl: {}",
            self.auto_lock_on_attack_dir_ctrl
        ));
        ui.text(format!("Auto target: {}", self.auto_target));
        ui.text(format!("Boot offline: {}", self.boot_offline));
        ui.text(format!("Hide white sign: {}", self.hide_white_sign));
    }
}
