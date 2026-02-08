use hudhook::imgui::*;

use darksouls3::sprj::*;
use debug::UiExt;

use super::{StatefulDebugDisplay, world_chr_man::ChrSetState};

#[derive(Default)]
pub struct WorldBlockChrState {
    chr_set_state: ChrSetState,
}

impl StatefulDebugDisplay for WorldBlockChr {
    type State = WorldBlockChrState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        self.chr_set.render_debug_mut(ui, &mut state.chr_set_state);

        ui.header("Mappings", || {
            ui.table(
                "world-block-chr-mappings",
                [
                    TableColumnSetup::new("Entity ID"),
                    TableColumnSetup::new("FieldIns Type"),
                    TableColumnSetup::new("Container"),
                    TableColumnSetup::new("Index"),
                ],
                self.mappings(),
                |ui, _, mapping| {
                    ui.table_next_column();
                    ui.text(format!("{}", mapping.entity_id));

                    ui.table_next_column();
                    ui.text(format!("{:?}", mapping.selector.field_ins_type()));

                    ui.table_next_column();
                    ui.text(format!("0x{:x}", mapping.selector.container()));

                    ui.table_next_column();
                    ui.text(format!("0x{:x}", mapping.selector.index()));
                },
            );
        });
    }
}
