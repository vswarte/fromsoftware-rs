use hudhook::imgui::Ui;

use debug::UiExt;
use eldenring::cs::{CSCam, CSCamera};

use crate::display::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSCamera {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("Pers cam 1", &self.pers_cam_1);
        ui.nested("Pers cam 2", &self.pers_cam_2);
        ui.nested("Pers cam 3", &self.pers_cam_3);
        ui.nested("Pers cam 4", &self.pers_cam_4);
        ui.display("Camera mask", self.camera_mask);
    }
}

impl DebugDisplay for CSCam {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Fov", self.fov);
        ui.display("Aspect ratio", self.aspect_ratio);
        ui.display("Far plane", self.far_plane);
        ui.display("Near plane", self.near_plane);
        ui.nested("Matrix", self.matrix);
    }
}
