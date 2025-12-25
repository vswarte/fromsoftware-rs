use fromsoftware_shared::{
    F32Matrix2x2, F32Matrix2x3, F32Matrix2x4, F32Matrix3x2, F32Matrix3x3, F32Matrix3x4,
    F32Matrix4x2, F32Matrix4x3, F32Matrix4x4, F32ModelMatrix, F32PackedModelMatrix, F32Vector2,
    F32Vector3, F32Vector4,
};
use hudhook::imgui::Ui;

use super::DebugDisplay;

macro_rules! impl_debug_display_for_tuple {
    ($t:ty, $($i:tt),+ $(,)?) => {
        impl DebugDisplay for $t {
            fn render_debug(&self, ui: &Ui) {
                $(self.$i.render_debug(ui); ui.separator();)+
            }
        }
    };
}

impl_debug_display_for_tuple!(F32Matrix4x4, 0, 1, 2, 3);
impl_debug_display_for_tuple!(F32Matrix4x3, 0, 1, 2);
impl_debug_display_for_tuple!(F32Matrix4x2, 0, 1, 2, 3);
impl_debug_display_for_tuple!(F32Matrix3x4, 0, 1, 2);
impl_debug_display_for_tuple!(F32Matrix3x3, 0, 1, 2);
impl_debug_display_for_tuple!(F32Matrix3x2, 0, 1, 2);
impl_debug_display_for_tuple!(F32Matrix2x4, 0, 1);
impl_debug_display_for_tuple!(F32Matrix2x3, 0, 1);
impl_debug_display_for_tuple!(F32Matrix2x2, 0, 1);

impl DebugDisplay for F32ModelMatrix {
    fn render_debug(&self, ui: &Ui) {
        self.rotation::<F32Matrix3x3>().render_debug(ui);
        self.translation::<F32Vector3>().render_debug(ui);
    }
}

impl DebugDisplay for F32PackedModelMatrix {
    fn render_debug(&self, ui: &Ui) {
        self.rotation::<F32Matrix3x3>().render_debug(ui);
        self.translation::<F32Vector3>().render_debug(ui);
    }
}

impl DebugDisplay for F32Vector4 {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("x: {}", self.0));
        ui.text(format!("y: {}", self.1));
        ui.text(format!("z: {}", self.2));
        ui.text(format!("w: {}", self.3));
    }
}

impl DebugDisplay for F32Vector3 {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("x: {}", self.0));
        ui.text(format!("y: {}", self.1));
        ui.text(format!("z: {}", self.2));
    }
}

impl DebugDisplay for F32Vector2 {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("x: {}", self.0));
        ui.text(format!("y: {}", self.1));
    }
}
