use eldenring::cs::{
    CSFeManImp, ChrEnemyTagEntry, ChrFriendTagEntry, FrontEndViewValues, TagHudData,
};

use debug::UiExt;
use hudhook::imgui::Ui;

use crate::display::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSFeManImp {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("HUD State", self.hud_state);
        ui.nested("Debug Tag", &self.debug_tag);

        ui.list(
            "Enemy Character Tags",
            self.enemy_chr_tag_displays.iter(),
            |ui, i, tag| ui.nested(format!("Enemy Tag {i}"), tag),
        );

        ui.list(
            "Friendly Character Tags",
            self.friendly_chr_tag_displays.iter(),
            |ui, i, tag| ui.nested(format!("Friendly Tag {i}"), tag),
        );

        ui.list(
            "Boss Health Displays",
            self.boss_health_displays.iter(),
            |ui, i, boss| {
                ui.header(format!("Boss {i}"), || {
                    ui.display("FMG ID", boss.fmg_id);
                    ui.debug("Handle", boss.field_ins_handle);
                    ui.display("Damage Taken", boss.damage_taken);
                });
            },
        );

        ui.header("Status Messages", || {
            ui.display("Read Index", self.proc_status_messages_read_index);
            ui.display("Write Index", self.proc_status_messages_write_index);

            ui.list(
                "Message Buffer",
                self.proc_status_messages.iter(),
                |ui, i, msg_id| ui.text(format!("Message {i}: {msg_id}")),
            );

            ui.display("Subarea Name Popup ID", self.subarea_name_popup_message_id);
            ui.display(
                "Area Welcome Message Request",
                self.area_welcome_message_request,
            );
            ui.text(format!(
                "Damage Number Decay Time: {:.1}s",
                self.damage_number_decay_time
            ));
        });

        ui.nested("FrontEndView", &self.frontend_values);
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

        ui.display("Name", &self.chr_name);
        ui.display("Role", &self.role_string);
        ui.display("Role Name Color", self.role_name_color);
        ui.display("Has Rune Arc", self.has_rune_arc);
        ui.display("Is Visible", self.is_visible);
        ui.display("Update Position", self.update_position);
        ui.display("Not On Screen", self.is_not_on_screen);
        ui.display("Is Down Scaled", self.is_down_scaled);
        ui.display("Last Damage Taken", self.last_damage_taken);
        ui.display("Last HP Value", self.last_hp_value);
        ui.text(format!(
            "Screen Position: ({:.1}, {:.1})",
            self.screen_pos_x, self.screen_pos_y
        ));
        ui.debug("Handle", self.field_ins_handle);
    }
}

impl DebugDisplay for ChrFriendTagEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Is Visible", self.is_visible);
        ui.display("Line of Sight Blocked", self.is_line_of_sight_blocked);
        ui.display("Not On Screen", self.is_not_on_screen);
        ui.display("Is Debug Summon", self.is_debug_summon);
        ui.display("Is Down Scaled", self.is_down_scaled);
        ui.display("Has Rune Arc", self.has_rune_arc);

        ui.display("Team Type", self.team_type);
        ui.display("Role Name Color", self.role_name_color);
        ui.display("Voice Chat State", self.voice_chat_state);

        ui.display("Name", &self.name_string);
        ui.display("Role", &self.role_string);

        ui.text(format!(
            "HP: {}/{} (Max Uncapped: {})",
            self.hp, self.max_hp, self.hp_max_uncapped
        ));
        ui.display("Max Recoverable HP", self.max_recoverable_hp);
        ui.text(format!(
            "Last Damage Time: {:.1}s",
            self.last_damage_time_delta
        ));
        ui.text(format!(
            "Screen Position: ({:.1}, {:.1}, {:.1}, {:.1})",
            self.screen_pos.0, self.screen_pos.1, self.screen_pos.2, self.screen_pos.3
        ));
        ui.debug("Handle", self.field_ins_handle);
    }
}

impl DebugDisplay for ChrEnemyTagEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Is Visible", self.is_visible);
        ui.display("Damage Taken", self.damage_taken);
        ui.display("Pre-Damage HP", self.pre_damage_hp);
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
        ui.debug("Handle", self.field_ins_handle);
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
            ui.display("Max Recoverable HP", self.max_recoverable_hp);
            ui.text(format!("FP: {}/{}", self.fp, self.fp_max));
            ui.text(format!("Stamina: {}/{}", self.stamina, self.stamina_max));
            ui.display("HP Rally Enabled", self.enable_hp_rally);
            ui.display("Equip HUD Enabled", self.enable_equip_hud);
            ui.display("Sword Arts Name", &self.sword_arts_name_string);
        });

        ui.list(
            "Enemy Tags",
            self.enemy_chr_tag_data.iter(),
            |ui, i, tag| ui.nested(format!("Enemy Tag {i}"), tag),
        );

        ui.list(
            "Boss List Tags",
            self.boss_list_tag_data.iter(),
            |ui, i, tag| ui.nested(format!("Boss {i}"), tag),
        );

        ui.list(
            "Friendly Tags",
            self.friendly_chr_tag_data.iter(),
            |ui, i, tag| ui.nested(format!("Friendly {i}"), tag),
        );

        ui.header("Status Message", || {
            ui.display("Message", &self.proc_status_message);
            ui.text(format!("Timer: {:.1}s", self.proc_status_message_timer));
            ui.debug("Full Screen Message", self.full_screen_message_request_id);
        });

        ui.header("Spirit Ashes", || {
            ui.display("Summoned Spirit Ash Count", self.summoned_spirit_ash_count);

            ui.list(
                "Spirit Ash Displays",
                self.spirit_ash_display.iter(),
                |ui, i, spirit| {
                    ui.header(format!("Spirit Ash {i}"), || {
                        ui.text(format!(
                            "HP: {}/{} (Max Uncapped: {})",
                            spirit.hp,
                            spirit.hp_max_uncapped - spirit.hp_max_uncapped_difference,
                            spirit.hp_max_uncapped
                        ));
                        ui.debug("Handle", spirit.field_ins_handle);
                    });
                },
            );
        });

        ui.header("Arena Info", || {
            ui.display("Elimination Count", self.quickmatch_elimination_count);
        });
    }
}
