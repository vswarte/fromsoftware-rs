use hudhook::imgui::Ui;

use debug::UiExt;
use sekiro::sprj::*;

use super::{DebugDisplay, DisplayUiExt, StatefulDebugDisplay};

#[derive(Default)]
pub struct GameDataManDebugState {
    item: String,
}

impl StatefulDebugDisplay for GameDataMan {
    type State = GameDataManDebugState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        {
            let _tok = ui.push_item_width(150.);
            ui.input_text("Item ID", &mut state.item).build();
        }
        ui.same_line();

        let item_id = state
            .item
            .parse::<u32>()
            .ok()
            .and_then(|i| ItemId::try_from(i).ok());

        ui.same_line_with_pos(ui.window_content_region_max()[0] - 140.);
        {
            let _tok = ui.begin_enabled(item_id.is_some());
            if ui.button("Remove Item") {
                let item_id = item_id.unwrap();
                self.remove_item(item_id, 1);
            }
        }

        ui.nested("Options", &self.options_data);
        ui.nested("Local Player", self.local_player.as_ref());
    }
}

impl DebugDisplay for OptionsData {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Camera speed", self.camera_speed);
        ui.debug("Pad vibration", self.pad_vibration);
        ui.debug("Brightness (SDR)", self.brightness_sdr);
        ui.debug("Sound type", self.sound_type);
        ui.debug("Volume (Music)", self.volume_music);
        ui.debug("Volume (Effects)", self.volume_effects);
        ui.debug("Volume (Voice)", self.volume_voice);
        ui.debug("Blood level: ", self.blood_level);
        ui.debug("Show captions", self.show_captions);
        ui.debug("Hud visible: ", self.hud_visible);
        ui.debug("Invert Camera (X)", self.invert_camera_x);
        ui.debug("Invert Camera (Y)", self.invert_camera_y);
        ui.debug("Auto lock", self.auto_lock);
        ui.debug("Auto avoid wall", self.auto_avoid_wall);
        ui.debug("Enable bank register", self.enable_bank_register);
        ui.debug("Jump with L3", self.jump_with_l3);
        ui.debug("Reset camera (Y)", self.reset_camera_y);
        ui.debug("Camera direction", self.camera_direction);
        ui.debug(
            "Rank register profile index",
            self.rank_register_profile_index,
        );
        ui.debug("Allow global matching", self.allow_global_matching);
        ui.debug("Voice chat", self.voice_chat);
        ui.debug(
            "Other player name notation",
            self.other_player_name_notation,
        );
        ui.debug(
            "Auto lock on attack dir ctrl",
            self.auto_lock_on_attack_dir_ctrl,
        );
        ui.debug("Auto target", self.auto_target);
        ui.debug("Boot offline", self.boot_offline);
        ui.debug("Hide white sign", self.hide_white_sign);
    }
}
