use eldenring::{
    cs::CSSfxImp,
    gxffx::{FxrListNode, FxrWrapper, GXFfxGraphicsResourceManager, GXFfxSceneCtrl},
};
use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for CSSfxImp {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Scene Ctrl", TreeNodeFlags::empty()) {
            ui.indent();
            self.scene_ctrl.render_debug(ui);
            ui.unindent();
        }
    }
}

impl DebugDisplay for GXFfxSceneCtrl {
    fn render_debug(&self, ui: &&mut Ui) {
        if ui.collapsing_header("Graphics Resource Manager", TreeNodeFlags::empty()) {
            ui.indent();
            ui.text(format!(
                "graphics_resource_manager: {:#01x}",
                self.graphics_resource_manager.as_ptr() as *const _ as usize
            ));
            unsafe {
                self.graphics_resource_manager.as_ref().render_debug(ui);
            }
            ui.unindent();
        }
    }
}

impl DebugDisplay for GXFfxGraphicsResourceManager {
    fn render_debug(&self, ui: &&mut Ui) {
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
    ui: &&mut Ui,
) {
    ui.text(format!(
        "fx_resource_container_scene_ctrl {:#x}",
        fx_resource_container_scene_ctrl as *const _ as usize
    ));

    if let Some(_t) = ui.begin_table_header_with_flags(
        "gx-ffx-graphics-resource-manager",
        [
            TableColumnSetup::new("ID"),
            TableColumnSetup::new("FXR Ptr"),
        ],
        TableFlags::RESIZABLE
            | TableFlags::BORDERS
            | TableFlags::ROW_BG
            | TableFlags::SIZING_STRETCH_PROP,
    ) {
        fxr_nodes.for_each(|fxr_node| {
            fxr_node.render_debug(ui);
        });
    }
}

impl DebugDisplay for FxrWrapper {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("{:#01x}", self.fxr));
    }
}

impl DebugDisplay for FxrListNode {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.table_next_column();
        ui.text(format!("{}", self.id));
        ui.table_next_column();
        self.fxr_wrapper.render_debug(ui);
    }
}
