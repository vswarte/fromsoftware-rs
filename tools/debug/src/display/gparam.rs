use eldenring::cs::{
    CSGparamIdLerper, CSWorldAreaBlockSceneDrawParam, CSWorldSceneDrawParamManager,
};
use hudhook::imgui::{TableColumnSetup, Ui};

use super::{DebugDisplay, UiExt};

impl DebugDisplay for CSWorldSceneDrawParamManager {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            "World Area Blocks",
            self.world_area_blocks.iter(),
            |ui, _i, b| {
                ui.header(&format!("{}", b.area), || {
                    b.render_debug(ui);
                });
            },
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
        ui.input_text("Unk8", &mut self.unk8.to_string())
            .read_only(true)
            .build();
        ui.input_text("UnkC", &mut self.unkc.to_string())
            .read_only(true)
            .build();
        ui.input_text("Destination ID", &mut self.destination_id.to_string())
            .read_only(true)
            .build();
        ui.input_text("Unk14", &mut self.unk14.to_string())
            .read_only(true)
            .build();
        ui.input_text("Begin ID", &mut self.begin_id.to_string())
            .read_only(true)
            .build();
        ui.input_text("Unk1C", &mut self.unk1c.to_string())
            .read_only(true)
            .build();
        ui.input_text("Timer", &mut self.timer.to_string())
            .read_only(true)
            .build();
        ui.input_text("Unk24", &mut self.unk24.to_string())
            .read_only(true)
            .build();
    }
}

impl DebugDisplay for CSWorldAreaBlockSceneDrawParam {
    fn render_debug(&self, _ui: &Ui) {}
}
