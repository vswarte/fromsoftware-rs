use darksouls3::sprj::*;
use debug::UiExt;
use hudhook::imgui::{TableColumnSetup, Ui};

use super::{DebugDisplay, DisplayUiExt, StatefulDebugDisplay};

impl StatefulDebugDisplay for PlayerIns {
    type State = ChrInsState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        self.super_chr_ins.render_debug_mut(ui, state);

        ui.nested("PlayerGameData", unsafe { self.player_game_data.as_ref() });
    }
}

impl DebugDisplay for PlayerGameData {
    fn render_debug(&self, ui: &Ui) {
        self.player_info.render_debug(ui);

        ui.nested("EquipGameData", &self.equipment);
        ui.nested_opt("Storage Box", self.storage.as_ref());
    }
}

impl DebugDisplay for PlayerInfo {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("ID", self.id);
        if !self.name().is_empty() {
            ui.display("Name", self.name());
        }
        ui.debug("Vigor", self.vigor);
        ui.debug("Attunement", self.attunement);
        ui.debug("Endurance", self.endurance);
        ui.debug("Vitality", self.vitality);
        ui.debug("Strength", self.strength);
        ui.debug("Dexterity", self.dexterity);
        ui.debug("Intelligence", self.intelligence);
        ui.debug("Faith", self.faith);
        ui.debug("Luck", self.luck);
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

pub type ChrInsState = ();

impl StatefulDebugDisplay for ChrIns {
    type State = ChrInsState;

    fn render_debug_mut(&mut self, ui: &Ui, _: &mut ChrInsState) {
        if ui.button("Kill") {
            self.kill();
        }

        let data = &self.modules.data;
        ui.text(format!("HP: {}/{}", data.hp, data.max_hp));
        ui.text(format!("MP: {}/{}", data.fp, data.max_fp));
        ui.text(format!("Stamina: {}/{}", data.stamina, data.max_stamina));
    }
}
