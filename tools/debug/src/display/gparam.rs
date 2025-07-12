use eldenring::cs::{
    CSGparamIdLerper, CSWorldAreaBlockSceneDrawParam, CSWorldSceneDrawParamManager,
};
use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSWorldSceneDrawParamManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text("World Area Blocks");
        self.world_area_blocks.iter().for_each(|b| {
            if ui.collapsing_header(format!("{}", b.area), TreeNodeFlags::empty()) {
                ui.indent();
                b.render_debug(ui);
                ui.unindent();
            }
        });

        ui.text("Lerper");
        self.scene_draw_param.lerper.render_debug(ui);

        ui.text("Lerpers");
        if let Some(_t) = ui.begin_table_header_with_flags(
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
            TableFlags::RESIZABLE
                | TableFlags::BORDERS
                | TableFlags::ROW_BG
                | TableFlags::SIZING_STRETCH_PROP,
        ) {
            self.scene_draw_param.lerpers.iter().for_each(|lerper| {
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
            });
        }
    }
}

impl DebugDisplay for CSGparamIdLerper {
    fn render_debug(&self, ui: &&mut Ui) {
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
    fn render_debug(&self, _ui: &&mut Ui) {}
}
