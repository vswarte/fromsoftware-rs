use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{CSGaitemImp, CSGaitemInsSubclass};

use super::DebugDisplay;

impl DebugDisplay for CSGaitemImp {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Gaitem Inses", || {
            ui.table(
                "cs-gaitem-imp-gaiteminses",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Category"),
                    TableColumnSetup::new("Additional"),
                ],
                self.gaitems.iter().filter_map(|f| f.as_ref()),
                |ui, index, gaitem| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(gaitem.gaitem_handle.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", gaitem.item_id));

                    ui.table_next_column();
                    ui.text(format!("{:?}", gaitem.gaitem_handle.category()));

                    ui.table_next_column();
                    match gaitem.as_ref().into() {
                        CSGaitemInsSubclass::CSWepGaitemIns(wep) => {
                            let gem_handle = wep.gem_slot_table.gem_slots[0].gaitem_handle;
                            if gem_handle.0 != 0 {
                                ui.text(format!("Gem: {:?}", gem_handle.index()))
                            }
                        }
                        CSGaitemInsSubclass::CSGemGaitemIns(gem) if gem.weapon_handle.0 != 0 => {
                            ui.text(format!("Weapon: {:?}", gem.weapon_handle.index()))
                        }
                        _ => {}
                    }
                },
            );
        });
    }
}
