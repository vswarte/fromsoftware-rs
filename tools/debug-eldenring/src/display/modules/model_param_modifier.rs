use crate::display::DebugDisplay;
use debug::UiExt;
use eldenring::cs::CSChrModelParamModifierModule;
use hudhook::imgui::{TableColumnSetup, Ui};

impl DebugDisplay for CSChrModelParamModifierModule {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "chr-ins-model-param-modifier",
            [TableColumnSetup::new("Name")],
            self.modifiers.items().iter(),
            |ui, _i, modifier| {
                ui.table_next_column();
                ui.text(unsafe { modifier.name.to_string() }.unwrap());
            },
        );
    }
}
