use eldenring::cs::{CSEventFlagMan, CSFD4VirtualMemoryFlag};
use fromsoftware_shared::FromStatic;
use hudhook::imgui::{TableColumnSetup, Ui};

use super::{DebugDisplay, UiExt};

impl DebugDisplay for CSEventFlagMan {
    fn render_debug(&self, ui: &Ui) {
        ui.input_text("World type", &mut self.world_type.to_string())
            .read_only(true)
            .build();

        ui.header("CSFD4VirtualMemory", || {
            self.virtual_memory_flag.render_debug(ui);
        });
    }
}

impl DebugDisplay for CSFD4VirtualMemoryFlag {
    fn render_debug(&self, ui: &Ui) {
        ui.input_text(
            "Event flag divisor",
            &mut self.event_flag_divisor.to_string(),
        )
        .read_only(true)
        .build();

        ui.input_text(
            "Event flag holder size",
            &mut self.event_flag_holder_size.to_string(),
        )
        .read_only(true)
        .build();

        ui.input_text(
            "Event flag holder count",
            &mut self.event_flag_holder_count.to_string(),
        )
        .read_only(true)
        .build();

        ui.header("Block Descriptors", || {
            ui.table(
                "event-flags-groups",
                [
                    TableColumnSetup::new("Group ID"),
                    TableColumnSetup::new("Location mode"),
                ],
                self.flag_block_descriptors.iter(),
                |ui, _i, e| {
                    ui.table_next_column();
                    ui.text(e.group.to_string());

                    ui.table_next_column();
                    ui.text(e.location_mode.to_string());
                },
            );
        });
    }
}
