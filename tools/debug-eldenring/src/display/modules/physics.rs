use crate::display::{DebugDisplay, DisplayUiExt};
use debug::UiExt;
use eldenring::cs::{CSChrPhysicsModule, ChrPhysicsMaterialInfo};
use hudhook::imgui::Ui;

impl DebugDisplay for CSChrPhysicsModule {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Position", self.position);
        ui.display("Orientation", self.orientation);
        ui.nested("Physics material", unsafe {
            self.slide_info.material_info.as_ref()
        });
    }
}

impl DebugDisplay for ChrPhysicsMaterialInfo {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Ground normal vector", self.normal_vector);
    }
}
