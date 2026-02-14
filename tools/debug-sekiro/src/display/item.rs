use hudhook::imgui::Ui;

use sekiro::sprj::*;

use super::StatefulDebugDisplay;

#[derive(Default)]
pub struct MapItemManManDebugState {
    item: String,
}

impl StatefulDebugDisplay for MapItemMan {
    type State = MapItemManManDebugState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        {
            let _tok = ui.push_item_width(150.);
            ui.input_text("Item ID:", &mut state.item).build();
        }
        ui.same_line();

        let item_id = state
            .item
            .parse::<u32>()
            .ok()
            .and_then(|i| ItemId::try_from(i).ok());

        ui.same_line_with_pos(ui.window_content_region_max()[0] - 140.);
        {
            let _tok = ui.begin_enabled(item_id.is_some());
            if ui.button("Grant Item") {
                let item_id = item_id.unwrap();
                self.grant_item(item_id);
            }
        }
    }
}
