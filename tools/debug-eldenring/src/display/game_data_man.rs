use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{CSGaitemGameData, GameDataMan, GameSettings, GameVersionData};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for GameDataMan {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Death Count", self.death_count);
        ui.header("Play Time", || {
            let hours = self.play_time / 3_600_000;
            let minutes = (self.play_time % 3_600_000) / 60_000;
            let seconds = (self.play_time % 60_000) / 1000;
            let milliseconds = self.play_time % 1000;
            ui.text(format!(
                "{}:{:02}:{:02}.{:03} ({} ms)",
                hours, minutes, seconds, milliseconds, self.play_time
            ));
        });
        ui.display("NG Level", self.ng_lvl);
        ui.debug("Post Map Load Chr Type", self.post_map_load_chr_type);
        ui.nested("Gaitem Game Data", &self.gaitem_game_data);

        ui.separator();
        ui.display("Boss Fight Active", self.boss_fight_active);
        ui.debug("Boss Fight Timer", self.boss_fight_timer.time);
        ui.display("Boss Health Bar Entity ID", self.boss_health_bar_entity_id);
        ui.display(
            "Boss Health Bar NPC Param ID",
            self.boss_health_bar_npc_param_id,
        );
        ui.display("White Phantom Count", self.white_phantom_count);

        ui.separator();
        ui.debug("Death State", self.death_state);
        ui.display("Just Died", self.just_died);
        ui.display(
            "Has Death Preventing Effect",
            self.has_death_preventing_effect,
        );

        ui.separator();
        ui.display("Request Full Recovery", self.request_full_recovery);
        ui.display(
            "Award Phantom Great Rune",
            self.award_phantom_great_rune_requested,
        );
        ui.display(
            "Award Rebreak-in Item",
            self.award_rebreak_in_item_requested,
        );

        ui.separator();
        ui.display("Net Penalty Requested", self.net_penalty_requested);
        ui.display("Net Penalty Points", self.net_penalty_points);
        ui.display("Net Penalty Item Cooldown", self.is_net_penalized);
        ui.display(
            "Net Penalty Limit Time",
            self.net_penalty_forgive_item_limit_time,
        );

        ui.nested("Main Player Game Data", &self.main_player_game_data);
        ui.nested("Game Settings", &self.game_settings);
        ui.nested("Game Version Data", &self.game_version_data);

        ui.separator();
        ui.header("DLC List", || {
            ui.table(
                "dlc-list",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("DLC ID"),
                ],
                self.pending_dlc_list.items(),
                |ui, i, dlc_id| {
                    ui.table_next_column();
                    ui.text(format!("{}", i));
                    ui.table_next_column();
                    ui.text(format!("{}", dlc_id));
                },
            );
        });

        ui.list(
            "Player Game Data List",
            self.player_game_data_list.iter(),
            |ui, i, item| ui.nested(format!("Slot {}", i), item),
        );

        ui.header("Remote Game Data States", || {
            ui.table(
                "remote-game-data-states",
                [
                    TableColumnSetup::new("Slot"),
                    TableColumnSetup::new("State"),
                ],
                self.remote_game_data_states.iter(),
                |ui, i, state| {
                    ui.table_next_column();
                    ui.text(format!("Slot {}", i));
                    ui.table_next_column();
                    ui.text(format!("{:?}", state));
                },
            );
        });

        ui.header("Leave Requests", || {
            ui.table(
                "game-data-leave-requests",
                [
                    TableColumnSetup::new("Slot"),
                    TableColumnSetup::new("Requested"),
                ],
                self.leave_requests.iter(),
                |ui, i, req| {
                    ui.table_next_column();
                    ui.text(format!("Slot {}", i));
                    ui.table_next_column();
                    ui.text(format!("{}", req));
                },
            );
        });

        ui.list(
            "Session Player Game Data",
            self.session_player_game_data_list.iter(),
            |ui, i, item| ui.nested_opt(format!("Slot {}", i), item.as_ref()),
        );
    }
}

impl DebugDisplay for GameSettings {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Camera Speed", self.camera_speed);
        ui.display("Rumble Strength", self.controller_rumble_strength);
        ui.display("Brightness", self.brightness);

        ui.separator();
        ui.display("Master Volume", self.master_volume);
        ui.display("Music Volume", self.music_volume);
        ui.display("SFX Volume", self.sfx_volume);
        ui.display("Voice Volume", self.voice_volume);
        ui.display("Sound Type", self.sound_type);

        ui.separator();
        ui.debug("Display Blood", self.display_blood);
        ui.debug("HUD Type", self.hud_type);
        ui.debug("Performance", self.performance_setting);
        ui.display("Ray Tracing", self.enable_ray_tracing);
        ui.display("Start Offline", self.start_offline);
        ui.display("Cross Region Play", self.enable_cross_region_play);
        ui.display("Show Subtitles", self.show_subtitles);
        ui.display("Show Gamer Tags", self.show_gamer_tags);

        ui.separator();
        ui.text("Control Settings");
        ui.display("Reverse Camera X", self.reverse_camera_xaxis);
        ui.display("Reverse Camera Y", self.reverse_camera_yaxis);
        ui.display("Auto Lock-on", self.auto_lock_on);
        ui.display("Camera Auto Wall Recovery", self.camera_auto_wall_recovery);
        ui.display("Camera Auto Rotation", self.camera_auto_rotation);
        ui.display("Reset Camera Y Axis", self.reset_camera_yaxis);
        ui.display("Jump Button L3", self.jump_button_l3);
        ui.display("Manual Attack Aiming", self.manual_attack_aiming);
        ui.display("Auto Target", self.auto_target);

        ui.separator();
        ui.text("Misc Settings");
        ui.display("Cinematic Effects", self.cinematic_effects);
        ui.display("Voice Chat", self.voice_chat);
        ui.display("Send Summon Signs", self.send_summon_signs);
        ui.display("Show Tutorials", self.show_tutorials);
        ui.display("Mark New Items", self.mark_new_items);
        ui.display("Show Recent Items", self.show_recent_items);
        ui.display("Unused GR System 103000", self.unused_gr_system_103000);

        ui.separator();
        ui.text("HDR Settings");
        ui.display("HDR Brightness", self.hdr_brightness);
        ui.display("HDR Max Brightness", self.hdr_max_brightness);
        ui.display("HDR Contrast", self.hdr_contrast);
    }
}

impl DebugDisplay for CSGaitemGameData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Entries Count", self.gaitem_entries.len());
        ui.header("Gaitem Entries", || {
            ui.table(
                "gaitem-entries",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Is already acquired"),
                ],
                self.gaitem_entries.iter(),
                |ui, i, entry| {
                    ui.table_next_column();
                    ui.text(format!("{}", i));
                    ui.table_next_column();
                    ui.text(format!("{:?}", entry.item_id));
                    ui.table_next_column();
                    ui.text(format!("{}", entry.already_acquired));
                },
            );
        });
    }
}

impl DebugDisplay for GameVersionData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Game Data Version", self.game_data_version);
        ui.display(
            "Last Saved Game Data Version",
            self.last_saved_game_data_version,
        );
        ui.display(
            "Saved Game Data Version is the Latest",
            self.saved_game_data_version_is_the_latest,
        );
        ui.display("Unused", self.unused);
    }
}
