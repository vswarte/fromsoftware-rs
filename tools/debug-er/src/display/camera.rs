use eldenring::cs::{CSCam, CSCamera};
use hudhook::imgui::Ui;

use super::{DebugDisplay, UiExt};

impl DebugDisplay for CSCamera {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Pers cam 1", || {
            self.pers_cam_1.render_debug(ui);
        });

        ui.header("Pers cam 2", || {
            self.pers_cam_2.render_debug(ui);
        });

        ui.header("Pers cam 3", || {
            self.pers_cam_3.render_debug(ui);
        });

        ui.header("Pers cam 4", || {
            self.pers_cam_4.render_debug(ui);
        });

        ui.text(format!("Camera mask: {}", self.camera_mask));
    }
}

impl DebugDisplay for CSCam {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Fov: {}", self.fov));
        ui.text(format!("Aspect ratio: {}", self.aspect_ratio));
        ui.text(format!("Far plane: {}", self.far_plane));
        ui.text(format!("Near plane: {}", self.near_plane));

        ui.header("Matrix", || {
            self.matrix.render_debug(ui);
        });
    }
}
