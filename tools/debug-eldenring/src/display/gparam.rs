use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{
    CSGparamIdLerper, CSWorldAreaBlockSceneDrawParam, CSWorldSceneDrawParamManager,
};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSWorldSceneDrawParamManager {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            "World Area Blocks",
            self.world_area_blocks.iter(),
            |ui, _i, b| ui.nested(format!("{}", b.area), b),
        );

        ui.text("Lerper");
        self.scene_draw_param.lerper.render_debug(ui);

        ui.header("Lerpers", || {
            ui.table(
                "cs-world-scene-draw-param-manager-lerpers",
                [
                    TableColumnSetup::new("Unk8"),
                    TableColumnSetup::new("UnkC"),
                    TableColumnSetup::new("Destination ID"),
                    TableColumnSetup::new("Unk14"),
                    TableColumnSetup::new("Begin ID"),
                    TableColumnSetup::new("Unk1C"),
                    TableColumnSetup::new("Timer"),
                    TableColumnSetup::new("Unk24"),
                ],
                self.scene_draw_param.lerpers.iter(),
                |ui, _i, lerper| {
                    ui.table_next_column();
                    ui.text(format!("{:x}", lerper.unk8));
                    ui.table_next_column();
                    ui.text(format!("{:x}", lerper.unkc));
                    ui.table_next_column();
                    ui.text(format!("{:x}", lerper.destination_id));
                    ui.table_next_column();
                    ui.text(format!("{:x}", lerper.unk14));
                    ui.table_next_column();
                    ui.text(format!("{:x}", lerper.begin_id));
                    ui.table_next_column();
                    ui.text(format!("{:x}", lerper.unk1c));
                    ui.table_next_column();
                    ui.text(format!("{}", lerper.timer));
                    ui.table_next_column();
                    ui.text(format!("{}", lerper.unk24));
                },
            );
        });
    }
}

impl DebugDisplay for CSGparamIdLerper {
    fn render_debug(&self, ui: &Ui) {
        ui.display_copiable("Unk8", self.unk8);
        ui.display_copiable("UnkC", self.unkc);
        ui.display_copiable("Destination ID", self.destination_id);
        ui.display_copiable("Unk14", self.unk14);
        ui.display_copiable("Begin ID", self.begin_id);
        ui.display_copiable("Unk1C", self.unk1c);
        ui.display_copiable("Timer", self.timer);
        ui.display_copiable("Unk24", self.unk24);
    }
}

impl DebugDisplay for CSWorldAreaBlockSceneDrawParam {
    fn render_debug(&self, _ui: &Ui) {}
}
