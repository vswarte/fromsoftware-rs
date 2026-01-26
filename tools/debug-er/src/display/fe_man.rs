use eldenring::cs::{
    CSFeManImp, ChrEnemyTagEntry, ChrFriendTagEntry, FrontEndViewValues, TagHudData,
};
use hudhook::imgui::Ui;

use crate::display::UiExt;

use super::DebugDisplay;

impl DebugDisplay for CSFeManImp {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("HUD State: {:?}", self.hud_state));

        ui.header("Debug Tag", || {
            self.debug_tag.render_debug(ui);
        });

        ui.list(
            "Enemy Character Tags",
            self.enemy_chr_tag_displays.iter(),
            |ui, i, tag| {
                ui.header(&format!("Enemy Tag {i}"), || {
                    tag.render_debug(ui);
                });
            },
        );

        ui.list(
            "Friendly Character Tags",
            self.friendly_chr_tag_displays.iter(),
            |ui, i, tag| {
                ui.header(&format!("Friendly Tag {i}"), || {
                    tag.render_debug(ui);
                });
            },
        );

        ui.list(
            "Boss Health Displays",
            self.boss_health_displays.iter(),
            |ui, i, boss| {
                ui.header(&format!("Boss {i}"), || {
                    ui.text(format!("FMG ID: {}", boss.fmg_id));
                    ui.text(format!("Handle: {:?}", boss.field_ins_handle));
                    ui.text(format!("Damage Taken: {}", boss.damage_taken));
                });
            },
        );

        ui.header("Status Messages", || {
            ui.text(format!(
                "Read Index: {}",
                self.proc_status_messages_read_index
            ));
            ui.text(format!(
                "Write Index: {}",
                self.proc_status_messages_write_index
            ));

            ui.list(
                "Message Buffer",
                self.proc_status_messages.iter(),
                |ui, i, msg_id| {
                    ui.text(format!("Message {i}: {msg_id}"));
                },
            );

            ui.text(format!(
                "Subarea Name Popup ID: {}",
                self.subarea_name_popup_message_id
            ));
            ui.text(format!(
                "Area Welcome Message Request: {}",
                self.area_welcome_message_request
            ));
            ui.text(format!(
                "Damage Number Decay Time: {:.1}s",
                self.damage_number_decay_time
            ));
        });

        ui.header("FrontEndView", || {
            self.frontend_values.render_debug(ui);
        });
    }
}

impl DebugDisplay for TagHudData {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!(
            "HP: {}/{} (Max Uncapped: {})",
            self.hp,
            self.hp_max_uncapped - self.hp_max_uncapped_difference,
            self.hp_max_uncapped
        ));

        ui.text(format!("Name: {}", self.chr_name));
        ui.text(format!("Role: {}", self.role_string));
        ui.text(format!("Role Name Color: {}", self.role_name_color));
        ui.text(format!("Has Rune Arc: {}", self.has_rune_arc));
        ui.text(format!("Is Visible: {}", self.is_visible));
        ui.text(format!("Update Position: {}", self.update_position));
        ui.text(format!("Not On Screen: {}", self.is_not_on_screen));
        ui.text(format!("Is Down Scaled: {}", self.is_down_scaled));
        ui.text(format!("Last Damage Taken: {}", self.last_damage_taken));
        ui.text(format!("Last HP Value: {}", self.last_hp_value));
        ui.text(format!(
            "Screen Position: ({:.1}, {:.1})",
            self.screen_pos_x, self.screen_pos_y
        ));
        ui.text(format!("Handle: {:?}", self.field_ins_handle));
    }
}

impl DebugDisplay for ChrFriendTagEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Is Visible: {}", self.is_visible));
        ui.text(format!(
            "Line of Sight Blocked: {}",
            self.is_line_of_sight_blocked
        ));
        ui.text(format!("Not On Screen: {}", self.is_not_on_screen));
        ui.text(format!("Is Debug Summon: {}", self.is_debug_summon));
        ui.text(format!("Is Down Scaled: {}", self.is_down_scaled));
        ui.text(format!("Has Rune Arc: {}", self.has_rune_arc));

        ui.text(format!("Team Type: {}", self.team_type));
        ui.text(format!("Role Name Color: {}", self.role_name_color));
        ui.text(format!("Voice Chat State: {}", self.voice_chat_state));

        ui.text(format!("Name: {}", self.name_string));
        ui.text(format!("Role: {}", self.role_string));

        ui.text(format!(
            "HP: {}/{} (Max Uncapped: {})",
            self.hp, self.max_hp, self.hp_max_uncapped
        ));
        ui.text(format!("Max Recoverable HP: {}", self.max_recoverable_hp));
        ui.text(format!(
            "Last Damage Time: {:.1}s",
            self.last_damage_time_delta
        ));
        ui.text(format!(
            "Screen Position: ({:.1}, {:.1}, {:.1}, {:.1})",
            self.screen_pos.0, self.screen_pos.1, self.screen_pos.2, self.screen_pos.3
        ));
        ui.text(format!("Handle: {:?}", self.field_ins_handle));
    }
}

impl DebugDisplay for ChrEnemyTagEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Is Visible: {}", self.is_visible));
        ui.text(format!("Damage Taken: {}", self.damage_taken));
        ui.text(format!("Pre-Damage HP: {}", self.pre_damage_hp));
        ui.text(format!(
            "Last Update Time: {:.1}s",
            self.last_update_time_delta
        ));
        ui.text(format!(
            "Last Damage Time: {:.1}s",
            self.last_damage_time_delta
        ));
        ui.text(format!(
            "Screen Position: ({:.1}, {:.1}, {:.1}, {:.1})",
            self.screen_pos.0, self.screen_pos.1, self.screen_pos.2, self.screen_pos.3
        ));
        ui.text(format!("Handle: {:?}", self.field_ins_handle));
    }
}

impl DebugDisplay for FrontEndViewValues {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Player Stats", || {
            ui.text(format!(
                "HP: {}/{} (Max Uncapped: {})",
                self.player_hp,
                self.hp_max_uncapped - self.hp_max_uncapped_difference,
                self.hp_max_uncapped
            ));
            ui.text(format!("Max Recoverable HP: {}", self.max_recoverable_hp));
            ui.text(format!("FP: {}/{}", self.fp, self.fp_max));
            ui.text(format!("Stamina: {}/{}", self.stamina, self.stamina_max));
            ui.text(format!("HP Rally Enabled: {}", self.enable_hp_rally));
            ui.text(format!("Equip HUD Enabled: {}", self.enable_equip_hud));
            ui.text(format!("Sword Arts Name: {}", self.sword_arts_name_string));
        });

        ui.list(
            "Enemy Tags",
            self.enemy_chr_tag_data.iter(),
            |ui, i, tag| {
                ui.header(&format!("Enemy Tag {i}"), || {
                    tag.render_debug(ui);
                });
            },
        );

        ui.list(
            "Boss List Tags",
            self.boss_list_tag_data.iter(),
            |ui, i, tag| {
                ui.header(&format!("Boss {i}"), || {
                    tag.render_debug(ui);
                });
            },
        );

        ui.list(
            "Friendly Tags",
            self.friendly_chr_tag_data.iter(),
            |ui, i, tag| {
                ui.header(&format!("Friendly {i}"), || {
                    tag.render_debug(ui);
                });
            },
        );

        ui.header("Status Message", || {
            ui.text(format!("Message: {}", self.proc_status_message));
            ui.text(format!("Timer: {:.1}s", self.proc_status_message_timer));
            ui.text(format!(
                "Full Screen Message: {:?}",
                self.full_screen_message_request_id
            ));
        });

        ui.header("Spirit Ashes", || {
            ui.text(format!(
                "Summoned Spirit Ash Count: {}",
                self.summoned_spirit_ash_count
            ));

            ui.list(
                "Spirit Ash Displays",
                self.spirit_ash_display.iter(),
                |ui, i, spirit| {
                    ui.header(&format!("Spirit Ash {i}"), || {
                        ui.text(format!(
                            "HP: {}/{} (Max Uncapped: {})",
                            spirit.hp,
                            spirit.hp_max_uncapped - spirit.hp_max_uncapped_difference,
                            spirit.hp_max_uncapped
                        ));
                        ui.text(format!("Handle: {:?}", spirit.field_ins_handle));
                    });
                },
            );
        });

        ui.header("Arena Info", || {
            ui.text(format!(
                "Elimination Count: {}",
                self.quickmatch_elimination_count
            ));
        });
    }
}
