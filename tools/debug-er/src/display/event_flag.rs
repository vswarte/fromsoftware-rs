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

        let virtual_memory_flag = &mut unsafe { CSEventFlagMan::instance() }
            .unwrap()
            .virtual_memory_flag;

        if ui.button("Nuke Caelid") {
            virtual_memory_flag.set_flag(62040, !virtual_memory_flag.get_flag(62040));
        }

        if ui.button("Toggle Godrick out of existence") {
            virtual_memory_flag.set_flag(9101, !virtual_memory_flag.get_flag(9101));
        }

        if ui.button("Close door after Godrick") {
            virtual_memory_flag.set_flag(10008540, !virtual_memory_flag.get_flag(10008540));
        }

        if virtual_memory_flag.get_flag(62040) {
            ui.text("Still have to nuke Caelid...");
        } else {
            ui.text("Caelid = nuked");
        }

        if !virtual_memory_flag.get_flag(9101) {
            ui.text("Godrick exists");
        } else {
            ui.text("Godrick doesn't exist");
        }

        if virtual_memory_flag.get_flag(10008540) {
            ui.text("Door behind Godrick open");
        } else {
            ui.text("Door behind Godrick closed");
        }
    }
}
