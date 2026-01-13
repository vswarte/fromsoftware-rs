use eldenring::{
    cs::{CSEventRegion, CSEventRegionMan, EzDrawFillMode, RendMan, WorldChrMan},
    position::HavokPosition,
};
use fromsoftware_shared::{F32Vector4, FromStatic};
use hudhook::imgui::{TableColumnSetup, Ui};

use super::{DebugDisplay, UiExt};

impl DebugDisplay for CSEventRegionMan {
    fn render_debug(&self, ui: &Ui) {
        ui.list("Entries", self.regions.iter(), |ui, _i, entry| {
            ui.header(&format!("Region ID {}", entry.region_id), || {
                entry.event_region.render_debug(ui);
            });
        });
    }
}

impl DebugDisplay for CSEventRegion {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Type: {:?}", self.msb_point.shape_data.point_type));
        ui.text(format!(
            "Position: {:?}",
            self.msb_point.shape_data.position
        ));
        ui.text(format!(
            "Rotation: {:?}",
            self.msb_point.shape_data.rotation
        ));
        ui.text(format!(
            "Map layer: {:?}",
            self.msb_point.shape_data.map_layer
        ));
        ui.text(format!("Layer mask: {:b}", self.msb_point.layer_mask));
        ui.text(format!("Block center: {:?}", self.block_center));
        ui.text(format!("Rotation: {:?}", self.rotation));
    }
}
