use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{CSGaitemGameData, GameDataMan, GameSettings, GameVersionData};

use super::DebugDisplay;

impl DebugDisplay for GameDataMan {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Death Count: {}", self.death_count));
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
        ui.text(format!("NG Level: {}", self.ng_lvl));
        ui.text(format!(
            "Post Map Load Chr Type: {:?}",
            self.post_map_load_chr_type
        ));

        ui.header("Gaitem Game Data", || {
            self.gaitem_game_data.render_debug(ui);
        });

        ui.separator();
        ui.text(format!("Boss Fight Active: {}", self.boss_fight_active));
        ui.text(format!(
            "Boss Fight Timer: {:?}",
            self.boss_fight_timer.time
        ));
        ui.text(format!(
            "Boss Health Bar Entity ID: {}",
            self.boss_health_bar_entity_id
        ));
        ui.text(format!(
            "Boss Health Bar NPC Param ID: {}",
            self.boss_health_bar_npc_param_id
        ));
        ui.text(format!("White Phantom Count: {}", self.white_phantom_count));

        ui.separator();
        ui.text(format!("Death State: {:?}", self.death_state));
        ui.text(format!("Just Died: {}", self.just_died));
        ui.text(format!(
            "Has Death Preventing Effect: {}",
            self.has_death_preventing_effect
        ));

        ui.separator();
        ui.text(format!(
            "Request Full Recovery: {}",
            self.request_full_recovery
        ));
        ui.text(format!(
            "Award Phantom Great Rune: {}",
            self.award_phantom_great_rune_requested
        ));
        ui.text(format!(
            "Award Rebreak-in Item: {}",
            self.award_rebreak_in_item_requested
        ));

        ui.separator();
        ui.text(format!(
            "Net Penalty Requested: {}",
            self.net_penalty_requested
        ));
        ui.text(format!("Net Penalty Points: {}", self.net_penalty_points));
        ui.text(format!(
            "Net Penalty Item Cooldown: {}",
            self.is_net_penalized
        ));
        ui.text(format!(
            "Net Penalty Limit Time: {}",
            self.net_penalty_forgive_item_limit_time
        ));

        ui.header("Main Player Game Data", || {
            self.main_player_game_data.render_debug(ui);
        });

        ui.header("Game Settings", || {
            self.game_settings.render_debug(ui);
        });

        ui.header("Game Version Data", || {
            self.game_version_data.render_debug(ui);
        });

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
            |ui, i, item| {
                ui.header(format!("Slot {}", i), || {
                    item.render_debug(ui);
                });
            },
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
            |ui, i, item| {
                if let Some(data) = item {
                    ui.header(format!("Slot {}", i), || {
                        data.render_debug(ui);
                    });
                }
            },
        );
    }
}

impl DebugDisplay for GameSettings {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Camera Speed: {}", self.camera_speed));
        ui.text(format!(
            "Rumble Strength: {}",
            self.controller_rumble_strength
        ));
        ui.text(format!("Brightness: {}", self.brightness));
        ui.separator();
        ui.text(format!("Master Volume: {}", self.master_volume));
        ui.text(format!("Music Volume: {}", self.music_volume));
        ui.text(format!("SFX Volume: {}", self.sfx_volume));
        ui.text(format!("Voice Volume: {}", self.voice_volume));
        ui.text(format!("Sound Type: {}", self.sound_type));

        ui.separator();
        ui.text(format!("Display Blood: {:?}", self.display_blood));
        ui.text(format!("HUD Type: {:?}", self.hud_type));
        ui.text(format!("Performance: {:?}", self.performance_setting));
        ui.text(format!("Ray Tracing: {}", self.enable_ray_tracing));
        ui.text(format!("Start Offline: {}", self.start_offline));
        ui.text(format!(
            "Cross Region Play: {}",
            self.enable_cross_region_play
        ));
        ui.text(format!("Show Subtitles: {}", self.show_subtitles));
        ui.text(format!("Show Gamer Tags: {}", self.show_gamer_tags));

        ui.separator();
        ui.text("Control Settings");
        ui.text(format!("Reverse Camera X: {}", self.reverse_camera_xaxis));
        ui.text(format!("Reverse Camera Y: {}", self.reverse_camera_yaxis));
        ui.text(format!("Auto Lock-on: {}", self.auto_lock_on));
        ui.text(format!(
            "Camera Auto Wall Recovery: {}",
            self.camera_auto_wall_recovery
        ));
        ui.text(format!(
            "Camera Auto Rotation: {}",
            self.camera_auto_rotation
        ));
        ui.text(format!("Reset Camera Y Axis: {}", self.reset_camera_yaxis));
        ui.text(format!("Jump Button L3: {}", self.jump_button_l3));
        ui.text(format!(
            "Manual Attack Aiming: {}",
            self.manual_attack_aiming
        ));
        ui.text(format!("Auto Target: {}", self.auto_target));

        ui.separator();
        ui.text("Misc Settings");
        ui.text(format!("Cinematic Effects: {}", self.cinematic_effects));
        ui.text(format!("Voice Chat: {}", self.voice_chat));
        ui.text(format!("Send Summon Signs: {}", self.send_summon_signs));
        ui.text(format!("Show Tutorials: {}", self.show_tutorials));
        ui.text(format!("Mark New Items: {}", self.mark_new_items));
        ui.text(format!("Show Recent Items: {}", self.show_recent_items));
        ui.text(format!(
            "Unused GR System 103000: {}",
            self.unused_gr_system_103000
        ));

        ui.separator();
        ui.text("HDR Settings");
        ui.text(format!("HDR Brightness: {}", self.hdr_brightness));
        ui.text(format!("HDR Max Brightness: {}", self.hdr_max_brightness));
        ui.text(format!("HDR Contrast: {}", self.hdr_contrast));
    }
}

impl DebugDisplay for CSGaitemGameData {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Entries Count: {}", self.gaitem_entries.len()));
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
        ui.text(format!("Game Data Version: {}", self.game_data_version));
        ui.text(format!(
            "Last Saved Game Data Version: {}",
            self.last_saved_game_data_version
        ));
        ui.text(format!(
            "Saved Game Data Version is the Latest: {}",
            self.saved_game_data_version_is_the_latest
        ));
        ui.text(format!("Unused: {}", self.unused));
    }
}
