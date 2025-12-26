use eldenring::cs::{CSBulletIns, CSBulletManager};
use hudhook::imgui::Ui;

use super::{DebugDisplay, UiExt};

impl DebugDisplay for CSBulletManager {
    fn render_debug(&self, ui: &Ui) {
        ui.list("BulletInses", self.bullets(), |ui, _i, bullet| {
            ui.header(&format!("{}", bullet.field_ins_handle), || {
                bullet.render_debug(ui);
            });
        });
    }
}

impl DebugDisplay for CSBulletIns {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Physics", || {
            ui.text(format!("Position: {}", self.physics.position));
            ui.text(format!("Orientation: {}", self.physics.orientation));
            ui.text(format!("Velocity: {:?}", self.physics.velocity));
        });
    }
}
