use hudhook::imgui::Ui;

use debug::UiExt;
use eldenring::cs::{CSBulletIns, CSBulletManager};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSBulletManager {
    fn render_debug(&self, ui: &Ui) {
        ui.list("BulletInses", self.bullets(), |ui, _i, bullet| {
            ui.nested(format!("{}", bullet.field_ins_handle), bullet);
        });
    }
}

impl DebugDisplay for CSBulletIns {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Physics", || {
            ui.debug("Position", self.physics.position);
            ui.debug("Orientation", self.physics.orientation);
            ui.debug("Velocity", self.physics.velocity);
        });
    }
}
