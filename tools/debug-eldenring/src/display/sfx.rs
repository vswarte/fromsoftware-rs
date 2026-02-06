use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::{
    cs::CSSfxImp,
    gxffx::{FxrListNode, FxrWrapper, GXFfxGraphicsResourceManager, GXFfxSceneCtrl},
};

use super::DebugDisplay;

impl DebugDisplay for CSSfxImp {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Scene Ctrl", || {
            self.scene_ctrl.render_debug(ui);
        });
    }
}

impl DebugDisplay for GXFfxSceneCtrl {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Graphics Resource Manager", || {
            ui.text(format!(
                "graphics_resource_manager: {:#01x}",
                self.graphics_resource_manager.as_ptr() as *const _ as usize
            ));
            unsafe {
                self.graphics_resource_manager.as_ref().render_debug(ui);
            }
        });
    }
}

impl DebugDisplay for GXFfxGraphicsResourceManager {
    fn render_debug(&self, ui: &Ui) {
        let scene_ctrl = unsafe { &self.resource_container.scene_ctrl.as_ref() };
        render_graphics_resource_manager(
            scene_ctrl,
            self.resource_container.fxr_definitions.iter(),
            ui,
        );
    }
}

// TODO: Address crashing
fn render_graphics_resource_manager<'a>(
    fx_resource_container_scene_ctrl: &'a GXFfxSceneCtrl,
    fxr_nodes: impl Iterator<Item = &'a FxrListNode>,
    ui: &Ui,
) {
    ui.text(format!(
        "fx_resource_container_scene_ctrl {:#x}",
        fx_resource_container_scene_ctrl as *const _ as usize
    ));

    ui.table(
        "gx-ffx-graphics-resource-manager",
        [
            TableColumnSetup::new("ID"),
            TableColumnSetup::new("FXR Ptr"),
        ],
        fxr_nodes,
        |ui, _i, fxr_node| {
            ui.table_next_column();
            ui.text(format!("{}", fxr_node.id));
            ui.table_next_column();
            fxr_node.fxr_wrapper.render_debug(ui);
        },
    );
}

impl DebugDisplay for FxrWrapper {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("{:#01x}", self.fxr));
    }
}
