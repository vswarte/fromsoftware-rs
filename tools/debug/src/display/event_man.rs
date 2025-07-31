use eldenring::cs::CSEventManImp;
use eldenring::cs::CSSosSignMan;
use eldenring::cs::DisplayGhostData;
use eldenring::cs::PhantomJoinData;
use eldenring::cs::SosSignData;

use hudhook::imgui::{TableColumnSetup, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSEventManImp {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("CSEventSosSignCtrl", TreeNodeFlags::empty()) {
            ui.indent();
            let sos_sign_ctrl = self.sos_sign.as_ref();
            if ui.collapsing_header("SosSignMan", TreeNodeFlags::empty()) {
                ui.indent();
                if let Some(sos_sign_man) = sos_sign_ctrl.sos_sign_man {
                    unsafe { sos_sign_man.as_ref().render_debug(ui) };
                }
                ui.unindent();
            }
            ui.unindent();
        }
    }
}

impl DebugDisplay for CSSosSignMan {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Signs", TreeNodeFlags::empty()) {
            ui.indent();
            self.signs.iter().for_each(|entry| {
                if ui.collapsing_header(format!("Sign {}", entry.sign_id), TreeNodeFlags::empty()) {
                    entry.sign_data.render_debug(ui);
                }
            });
            ui.unindent();
        }
        if ui.collapsing_header("Sign SFX", TreeNodeFlags::empty()) {
            ui.indent();
            self.sign_sfx.iter().for_each(|entry| {
                ui.text(format!("Sign ID: {}", entry.sign_id));
            });
            ui.unindent();
        }
        if ui.collapsing_header("Summon Requests", TreeNodeFlags::empty()) {
            ui.indent();
            self.summon_requests.iter().for_each(|entry| {
                ui.text(format!("Summon Request ID: {entry}"));
            });
            ui.unindent();
        }
        if ui.collapsing_header("Join Data", TreeNodeFlags::empty()) {
            ui.indent();
            self.join_data
                .iter()
                .map(|e| unsafe { e.as_ref() })
                .for_each(|entry| {
                    if ui.collapsing_header(
                        format!("Join Data (Sign ID: {})", entry.sign_id),
                        TreeNodeFlags::empty(),
                    ) {
                        entry.render_debug(ui);
                    }
                });
            ui.unindent();
        }

        ui.text(format!(
            "White Sign Cool Time Param ID: {}",
            self.white_sign_cool_time_param_id
        ));
        if ui.collapsing_header("Signs Cooldown", TreeNodeFlags::empty()) {
            ui.indent();
            self.signs_cooldown
                .items()
                .iter()
                .enumerate()
                .for_each(|(i, t)| {
                    ui.text(format!("Cooldown {i}: {t:.2}s"));
                });
            ui.unindent();
        }

        ui.text(format!(
            "Override Guardian of Rosalia Count Enabled: {}",
            self.override_guardian_of_rosalia_count_enabled
        ));
        ui.text(format!(
            "Override Guardian of Rosalia Count: {}",
            self.override_guardian_of_rosalia_count
        ));
        ui.text(format!(
            "Override Map Guardian Count Enabled: {}",
            self.override_map_guardian_count_enabled
        ));
        ui.text(format!(
            "Override Map Guardian Count: {}",
            self.override_map_guardian_count
        ));
        ui.text(format!(
            "Override Force Join Black Count Enabled: {}",
            self.override_force_join_black_count_enabled
        ));
        ui.text(format!(
            "Override Force Join Black Count: {}",
            self.override_force_join_black_count
        ));
        ui.text(format!(
            "Override Sinner Hunter Count Enabled: {}",
            self.override_sinner_hunter_count_enabled
        ));
        ui.text(format!(
            "Override Sinner Hunter Count: {}",
            self.override_sinner_hunter_count
        ));
        ui.text(format!(
            "Override Berserker White Count Enabled: {}",
            self.override_berserker_white_count_enabled
        ));
        ui.text(format!(
            "Override Berserker White Count: {}",
            self.override_berserker_white_count
        ));
        ui.text(format!(
            "Override Sinner Hero Count Enabled: {}",
            self.override_sinner_hero_count_enabled
        ));
        ui.text(format!(
            "Override Sinner Hero Count: {}",
            self.override_sinner_hero_count
        ));
        ui.text(format!(
            "Override Cult White Summon Count Enabled: {}",
            self.override_cult_white_summon_count_enabled
        ));
        ui.text(format!(
            "Override Cult White Summon Count: {}",
            self.override_cult_white_summon_count
        ));
        ui.text(format!(
            "Override Normal White Count Enabled: {}",
            self.override_normal_white_count_enabled
        ));
        ui.text(format!(
            "Override Normal White Count: {}",
            self.override_normal_white_count
        ));
        ui.text(format!(
            "Override Red Summon Type Count Enabled: {}",
            self.override_red_summon_type_count_enabled
        ));
        ui.text(format!(
            "Override Red Summon Type Count: {}",
            self.override_red_summon_type_count
        ));
    }
}

impl DebugDisplay for SosSignData {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Sign ID: {}", self.sign_id));
        ui.text(format!("Sign Identifier: {}", self.sign_identifier.0));
        ui.text(format!("Map ID: {}", self.map_id));
        ui.text(format!("Position: {:?}", self.pos));
        ui.text(format!("Yaw: {}", self.yaw));
        ui.text(format!("Play region: {}", self.play_region_id));
        ui.text(format!("Vow Type: {}", self.vow_type));
        ui.text(format!(
            "Apply Multiplayer Rules: {}",
            self.apply_multiplayer_rules
        ));
        ui.text(format!("Multiplay Type: {:?}", self.multiplay_type));
        ui.text(format!("Is Sign Puddle: {}", self.is_sign_puddle));
        ui.text(format!(
            "Steam ID: {}",
            self.steam_id.to_u64().unwrap_or_default()
        ));
        ui.text(format!("FMG Name ID: {}", self.fmg_name_id));
        ui.text(format!("NPC Param ID: {}", self.npc_param_id));
        if ui.collapsing_header("Display Ghost Data", TreeNodeFlags::empty()) {
            ui.indent();
            self.display_ghost.render_debug(ui);
            ui.unindent();
        }
        ui.text(format!(
            "Summoned NPC Entity ID: {}",
            self.summoned_npc_entity_id
        ));
        ui.text(format!(
            "Summon Event Flag ID: {}",
            self.summon_event_flag_id
        ));
        ui.text(format!(
            "Dismissal Event Flag ID: {}",
            self.dismissal_event_flag_id
        ));
        ui.text(format!("Summonee Player ID: {}", self.summonee_player_id));
        ui.text(format!("Character ID: {}", self.character_id));
    }
}

impl DebugDisplay for DisplayGhostData {
    fn render_debug(&self, ui: &&mut Ui) {
        if let Some(_t) = ui.begin_table_header(
            "sign-data-equipment",
            [
                TableColumnSetup::new("Weapon Left 1"),
                TableColumnSetup::new("Weapon Right 1"),
                TableColumnSetup::new("Weapon Left 2"),
                TableColumnSetup::new("Weapon Right 2"),
                TableColumnSetup::new("Weapon Left 3"),
                TableColumnSetup::new("Weapon Right 3"),
                TableColumnSetup::new("Arrow 1"),
                TableColumnSetup::new("Bolt 1"),
                TableColumnSetup::new("Arrow 2"),
                TableColumnSetup::new("Bolt 2"),
                TableColumnSetup::new("Arrow 3"),
                TableColumnSetup::new("Bolt 3"),
            ],
        ) {
            ui.indent();
            ui.table_next_row();
            for i in 0..12 {
                ui.table_next_column();
                ui.text(format!("{}", self.equipment_param_ids[i as usize]));
            }
            ui.unindent();
        }
        if let Some(_t) = ui.begin_table_header(
            "sign-data-protector",
            [
                TableColumnSetup::new("Head"),
                TableColumnSetup::new("Chest"),
                TableColumnSetup::new("Gauntlets"),
                TableColumnSetup::new("Greaves"),
                TableColumnSetup::new("Unused"),
            ],
        ) {
            ui.indent();
            ui.table_next_row();
            for i in 0..5 {
                ui.table_next_column();
                ui.text(format!("{:?}", self.armor_param_ids[i as usize]));
            }
            ui.unindent();
        }
        ui.text(format!("Gender: {}", self.gender));
        if ui.collapsing_header("Sign Equipment", TreeNodeFlags::empty()) {
            ui.indent();
            self.asm_equipment.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for PhantomJoinData {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Sign ID: {}", self.sign_id));
        ui.text(format!("Sign Identifier: {}", self.sign_identifier.0));
        ui.text(format!("Join Time: {}", self.join_time));
        ui.text(format!("Multiplay Type: {:?}", self.multiplay_type));
        ui.text(format!("Is Sign Puddle: {}", self.is_sign_puddle));
        ui.text(format!("State: {}", self.state));
        ui.text(format!(
            "Steam ID: {}",
            self.steam_id.to_u64().unwrap_or_default()
        ));
        ui.text(format!("NPC Entity ID: {}", self.npc_entity_id));
        ui.text(format!(
            "Summon Event Flag ID: {}",
            self.summon_event_flag_id
        ));
        ui.text(format!(
            "Dismissal Event Flag ID: {}",
            self.dismissal_event_flag_id
        ));
        ui.text(format!("Position: {:?}", self.pos));
        ui.text(format!("Rotation: {:?}", self.rotation));
        ui.text(format!("Map ID: {}", self.map_id));
        ui.text(format!("Summonee Player ID: {}", self.summonee_player_id));
        ui.text(format!(
            "Summon Job Error Code: {:?}",
            self.summon_job_error_code
        ));
        ui.text(format!(
            "Apply Multiplayer Rules: {}",
            self.apply_multiplayer_rules
        ));
    }
}
