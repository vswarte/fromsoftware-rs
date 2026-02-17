use hudhook::imgui::Ui;

use debug::UiExt;
use eldenring::cs::CSEventManImp;
use eldenring::cs::CSSosSignMan;
use eldenring::cs::DisplayGhostData;
use eldenring::cs::PhantomJoinData;
use eldenring::cs::SosSignData;

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSEventManImp {
    fn render_debug(&self, ui: &Ui) {
        ui.header("CSEventSosSignCtrl", || {
            let sos_sign_ctrl = self.sos_sign.as_ref();
            ui.nested_opt(
                "SosSignMan",
                sos_sign_ctrl.sos_sign_man.map(|m| unsafe { m.as_ref() }),
            );
        });
    }
}

impl DebugDisplay for CSSosSignMan {
    fn render_debug(&self, ui: &Ui) {
        ui.list("Signs", self.signs.iter(), |ui, _i, entry| {
            ui.nested(format!("Sign {}", entry.sign_id), &entry.sign_data);
        });
        ui.list("Sign SFX", self.sign_sfx.iter(), |ui, _i, entry| {
            ui.display("Sign ID", entry.sign_id);
        });
        ui.list(
            "Summon Requests",
            self.summon_requests.iter(),
            |ui, _i, entry| ui.display("Summon Request ID", entry),
        );

        ui.list(
            "Join Data",
            self.join_data.iter().map(|e| unsafe { e.as_ref() }),
            |ui, _i, entry| ui.nested(format!("Join Data (Sign ID: {})", entry.sign_id), entry),
        );

        ui.display(
            "White Sign Cool Time Param ID",
            self.white_sign_cool_time_param_id,
        );

        ui.list(
            "Sign Cooldowns",
            self.signs_cooldown.items().iter(),
            |ui, i, t| {
                ui.text(format!("Cooldown {}: {:.2}s", i, t));
            },
        );

        ui.display(
            "Override Guardian of Rosalia Count Enabled",
            self.override_guardian_of_rosalia_count_enabled,
        );
        ui.display(
            "Override Guardian of Rosalia Count",
            self.override_guardian_of_rosalia_count,
        );
        ui.display(
            "Override Map Guardian Count Enabled",
            self.override_map_guardian_count_enabled,
        );
        ui.display(
            "Override Map Guardian Count",
            self.override_map_guardian_count,
        );
        ui.display(
            "Override Force Join Black Count Enabled",
            self.override_force_join_black_count_enabled,
        );
        ui.display(
            "Override Force Join Black Count",
            self.override_force_join_black_count,
        );
        ui.display(
            "Override Sinner Hunter Count Enabled",
            self.override_sinner_hunter_count_enabled,
        );
        ui.display(
            "Override Sinner Hunter Count",
            self.override_sinner_hunter_count,
        );
        ui.display(
            "Override Berserker White Count Enabled",
            self.override_berserker_white_count_enabled,
        );
        ui.display(
            "Override Berserker White Count",
            self.override_berserker_white_count,
        );
        ui.display(
            "Override Sinner Hero Count Enabled",
            self.override_sinner_hero_count_enabled,
        );
        ui.display(
            "Override Sinner Hero Count",
            self.override_sinner_hero_count,
        );
        ui.display(
            "Override Cult White Summon Count Enabled",
            self.override_cult_white_summon_count_enabled,
        );
        ui.display(
            "Override Cult White Summon Count",
            self.override_cult_white_summon_count,
        );
        ui.display(
            "Override Normal White Count Enabled",
            self.override_normal_white_count_enabled,
        );
        ui.display(
            "Override Normal White Count",
            self.override_normal_white_count,
        );
        ui.display(
            "Override Red Summon Type Count Enabled",
            self.override_red_summon_type_count_enabled,
        );
        ui.display(
            "Override Red Summon Type Count",
            self.override_red_summon_type_count,
        );
    }
}

impl DebugDisplay for SosSignData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Sign ID", self.sign_id);
        ui.display("Sign Identifier", self.sign_identifier.0);
        ui.display("Map ID", self.block_id);
        ui.debug("Position", self.pos);
        ui.display("Yaw", self.yaw);
        ui.display("Play region", self.play_region_id);
        ui.display("Vow Type", self.vow_type);
        ui.display("Apply Multiplayer Rules", self.apply_multiplayer_rules);
        ui.debug("Multiplay Type", self.multiplay_type);
        ui.display("Is Sign Puddle", self.is_sign_puddle);
        ui.display("Steam ID", self.steam_id.to_u64().unwrap_or_default());
        ui.display("FMG Name ID", self.fmg_name_id);
        ui.display("NPC Param ID", self.npc_param_id);
        ui.nested("Display Ghost Data", &self.display_ghost);
        ui.display("Summoned NPC Entity ID", self.summoned_npc_entity_id);
        ui.display("Summon Event Flag ID", self.summon_event_flag_id);
        ui.display("Dismissal Event Flag ID", self.dismissal_event_flag_id);
        ui.display("Summonee Player ID", self.summonee_player_id);
        ui.display("Character ID", self.character_id);
    }
}

impl DebugDisplay for DisplayGhostData {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            "Equipment Param IDs",
            self.equipment_param_ids.iter().zip([
                "Weapon Left 1",
                "Weapon Right 1",
                "Weapon Left 2",
                "Weapon Right 2",
                "Weapon Left 3",
                "Weapon Right 3",
                "Arrow 1",
                "Bolt 1",
                "Arrow 2",
                "Bolt 2",
                "Arrow 3",
                "Bolt 3",
            ]),
            |ui, _i, item| {
                ui.text(format!("{}: {}", item.1, item.0));
            },
        );

        ui.list(
            "Protector Param IDs",
            self.armor_param_ids
                .iter()
                .zip(["Head", "Chest", "Gauntlets", "Greaves", "Unused"]),
            |ui, _i, item| {
                ui.text(format!("{}: {}", item.1, item.0));
            },
        );
        ui.display("Gender", self.gender);
        ui.nested("Selected Slots", &self.asm_equipment);
    }
}

impl DebugDisplay for PhantomJoinData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Sign ID", self.sign_id);
        ui.display("Sign Identifier", self.sign_identifier.0);
        ui.display("Join Time", self.join_time);
        ui.debug("Multiplay Type", self.multiplay_type);
        ui.display("Is Sign Puddle", self.is_sign_puddle);
        ui.display("State", self.state);
        ui.display("Steam ID: {}", self.steam_id.to_u64().unwrap_or_default());
        ui.display("NPC Entity ID", self.npc_entity_id);
        ui.display("Summon Event Flag ID", self.summon_event_flag_id);
        ui.display("Dismissal Event Flag ID", self.dismissal_event_flag_id);
        ui.debug("Position", self.pos);
        ui.debug("Rotation", self.rotation);
        ui.display("Block ID", self.block_id);
        ui.display("Summonee Player ID", self.summonee_player_id);
        ui.debug("Summon Job Error Code", self.summon_job_error_code);
        ui.display("Apply Multiplayer Rules", self.apply_multiplayer_rules);
    }
}
