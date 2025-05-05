use eldenring::cs::CSGaitemImp;
use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags};

use super::DebugDisplay;

impl DebugDisplay for CSGaitemImp {
    fn render_debug(&self, ui: &&mut hudhook::imgui::Ui) {
        if ui.collapsing_header("Gaitem Inses", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header_with_flags(
                "cs-gaitem-imp-gaiteminses",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Category"),
                    TableColumnSetup::new("Additional"),
                ],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            ) {
                for gaitem in self.gaitems.iter().filter_map(|f| f.as_ref()) {
                    let gaitem = gaitem.as_ref();
                    let index = gaitem.gaitem_handle.index() as i32;

                    ui.table_next_column();
                    ui.text(format!("{index:?}"));

                    ui.table_next_column();
                    ui.text(gaitem.gaitem_handle.to_string());

                    ui.table_next_column();
                    ui.text(gaitem.item_id.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", gaitem.gaitem_handle.category()));

                    ui.table_next_column();
                    if let Some(wep) = gaitem.as_wep() {
                        let gem_handle = wep.gem_slot_table.gem_slots[0].gaitem_handle;
                        if gem_handle.0 != 0 {
                            ui.text(format!("Gem: {:?}", gem_handle.index()))
                        }
                    } else if let Some(gem) = gaitem.as_gem() {
                        if gem.weapon_handle.0 != 0 {
                            ui.text(format!("Weapon: {:?}", gem.weapon_handle.index()))
                        }
                    }
                }
            }
            ui.unindent();
        }
    }
}
