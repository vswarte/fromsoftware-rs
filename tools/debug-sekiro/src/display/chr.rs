use debug::UiExt;
use hudhook::imgui::{TableColumnSetup, Ui};
use sekiro::sprj::*;

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for PlayerGameData {
    fn render_debug(&self, ui: &Ui) {
        self.player_info.render_debug(ui);

        ui.text(format!(
            "Vigor: {} (effective {})",
            self.player_info.vigor, self.effective_vigor,
        ));
        ui.text(format!(
            "Attunement: {} (effective {})",
            self.player_info.attunement, self.effective_attunement,
        ));
        ui.text(format!(
            "Life Force: {} (effective {})",
            self.player_info.life_force, self.effective_life_force,
        ));
        ui.text(format!(
            "Willpower: {} (effective {})",
            self.player_info.willpower, self.effective_willpower,
        ));
        ui.text(format!(
            "Vitality: {} (effective {})",
            self.player_info.vitality, self.effective_vitality,
        ));
        ui.text(format!(
            "Strength: {} (effective {})",
            self.player_info.strength, self.effective_strength,
        ));
        ui.text(format!(
            "Dexterity: {} (effective {})",
            self.player_info.dexterity, self.effective_dexterity,
        ));
        ui.text(format!(
            "Intelligence: {} (effective {})",
            self.player_info.intelligence, self.effective_intelligence,
        ));
        ui.text(format!(
            "Faith: {} (effective {})",
            self.player_info.faith, self.effective_faith,
        ));
        ui.text(format!(
            "Luck: {} (effective {})",
            self.player_info.luck, self.effective_luck,
        ));

        ui.nested("EquipGameData", &self.equip_game_data);
    }
}

impl DebugDisplay for PlayerInfo {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Number", self.player_number);
        ui.debug("ID", self.player_id);
        ui.text(format!(
            "HP: {}/{} (base {})",
            self.hp, self.max_hp, self.base_max_hp
        ));
        ui.text(format!(
            "SP: {}/{} (base {})",
            self.sp, self.max_sp, self.base_max_sp
        ));
        ui.debug("Total Kills", self.total_kills);
        ui.debug("Poison Resistance", self.poison_resist);
        ui.debug("Bleed Resistance", self.bleed_resist);
        ui.debug("Toxic Resistance", self.toxic_resist);
        ui.debug("Curse Resistance", self.curse_resist);
        ui.debug("Region", self.region);
        ui.debug("Skill Level", self.skill_level);
        ui.debug("Skill Points", self.skill_level);
        ui.debug("Experience Points", self.total_experience_points);
        ui.text(format!(
            "Sen: {} (total {})",
            self.sen, self.total_sen_earned
        ));

        ui.header("Possibly Unused", || {
            ui.text(format!("MP: {}/{}", self.mp, self.max_mp));
            ui.debug("Soul Level", self.soul_level);
            ui.debug("Total Soul Over for Old", self.total_soul_over_for_old);
            ui.debug("Total Soul Over", self.total_soul_over);
            ui.debug("Hero Points", self.hero_points);
            ui.debug("Male", self.is_male);
            ui.debug("Shop Level", self.shop_level);
            ui.debug("Archetype", self.archetype);
            ui.debug("Appearance", self.appearance);
            ui.debug("Gift", self.gift);
            ui.debug("Max Weapon Upgrade Level", self.max_weapon_upgrade_level);
            ui.debug("Egg Soul", self.egg_soul);
            ui.debug("Curse Level", self.curse_level);
            ui.debug("Death Level", self.death_level);
            ui.debug("Dead", self.is_dead);
        });
    }
}

impl DebugDisplay for EquipGameData {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("EquipInventoryData", &self.equip_inventory_data);
    }
}

impl DebugDisplay for EquipInventoryData {
    fn render_debug(&self, ui: &Ui) {
        let label = format!(
            "Items ({}/{})",
            self.items_data.items_len(),
            self.items_data.total_capacity
        );
        ui.header(label.as_str(), || {
            ui.table(
                "equip-inventory-data-items",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                ],
                self.items_data.items(),
                |ui, index, item| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(item.gaitem_handle.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", item.item_id));

                    ui.table_next_column();
                    ui.text(item.quantity.to_string());
                },
            );
        });
    }
}
