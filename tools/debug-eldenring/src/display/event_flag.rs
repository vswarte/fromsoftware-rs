use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{CSEventFlagMan, CSFD4VirtualMemoryFlag};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSEventFlagMan {
    fn render_debug(&self, ui: &Ui) {
        ui.debug_copiable("World type", self.world_type);
        ui.nested("CSFD4VirtualMemory", &self.virtual_memory_flag);
    }
}

impl DebugDisplay for CSFD4VirtualMemoryFlag {
    fn render_debug(&self, ui: &Ui) {
        ui.debug_copiable("Event flag divisor", self.event_flag_divisor);
        ui.debug_copiable("Event flag holder size", self.event_flag_holder_size);
        ui.debug_copiable("Event flag holder count", self.event_flag_holder_count);

        ui.header("Block Descriptors", || {
            ui.table(
                "event-flags-groups",
                [
                    TableColumnSetup::new("Group ID"),
                    TableColumnSetup::new("Location mode"),
                ],
                self.flag_block_descriptors.iter(),
                |ui, _i, e| {
                    let (group, block_descriptor) = e.into();
                    ui.table_next_column();
                    ui.text(group.to_string());

                    ui.table_next_column();
                    ui.text(block_descriptor.location_mode.to_string());
                },
            );
        });
    }
}
