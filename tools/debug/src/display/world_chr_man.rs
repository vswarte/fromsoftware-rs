use eldenring::cs::{
    ChrIns, ChrSet, NetChrSetSync, OpenFieldChrSet, PlayerIns, SummonBuddyManager,
    SummonBuddyWarpManager, WorldChrMan,
};
use hudhook::imgui::{TableColumnSetup, TableFlags, TreeNodeFlags, Ui};

use super::DebugDisplay;

impl DebugDisplay for WorldChrMan {
    fn render_debug(&self, ui: &&mut Ui) {
        let world_area_chr_list_count = self.world_area_chr_list_count;
        ui.text(format!(
            "World Area Chr List Count: {world_area_chr_list_count}"
        ));

        let world_block_chr_list_count = self.world_block_chr_list_count;
        ui.text(format!(
            "World Block Chr List Count: {world_block_chr_list_count}"
        ));

        let world_grid_area_chr_list_count = self.world_grid_area_chr_list_count;
        ui.text(format!(
            "World Grid Area Chr List Count: {world_grid_area_chr_list_count}"
        ));

        let world_area_list_count = self.world_area_list_count;
        ui.text(format!("World Area List Count: {world_area_list_count}"));

        if ui.collapsing_header("Player ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.player_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("Ghost ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.ghost_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("SummonBuddy ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.summon_buddy_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("Debug ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.debug_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("OpenField ChrSet", TreeNodeFlags::empty()) {
            ui.indent();
            self.open_field_chr_set.render_debug(ui);
            ui.unindent();
        }

        if ui.collapsing_header("All ChrSets", TreeNodeFlags::empty()) {
            ui.indent();
            for (i, entry) in self.chr_sets.iter().enumerate() {
                let Some(chr_set) = entry else {
                    continue;
                };

                if ui.collapsing_header(format!("ChrSet {i}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    chr_set.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        match self.main_player.as_ref() {
            Some(p) => {
                if ui.collapsing_header("Main player", TreeNodeFlags::empty()) {
                    ui.indent();
                    p.render_debug(ui);
                    ui.unindent();
                }
            }
            None => ui.text("No Main player instance"),
        }

        match self.summon_buddy_manager.as_ref() {
            Some(s) => {
                if ui.collapsing_header("SummonBuddyManager", TreeNodeFlags::empty()) {
                    ui.indent();
                    s.render_debug(ui);
                    ui.unindent();
                }
            }
            None => ui.text("No SummonBuddyManager instance"),
        }

        if ui.collapsing_header("NetChrSync", TreeNodeFlags::empty()) {
            ui.indent();

            for (i, entry) in self
                .net_chr_sync
                .net_chr_set_sync
                .iter()
                .enumerate()
                .filter_map(|(i, s)| s.as_ref().map(|s| (i, s)))
            {
                if ui.collapsing_header(format!("NetChrSetSync {i}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    entry.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Debug Character Creator", TreeNodeFlags::empty()) {
            ui.indent();
            ui.input_text(
                "Last Created Character",
                &mut format!("{:x?}", self.debug_chr_creator.last_created_chr),
            )
            .read_only(true)
            .build();
            ui.unindent();
        }

        if ui.collapsing_header("ChrInses by distance", TreeNodeFlags::empty()) {
            ui.indent();
            for entry in self.chr_inses_by_distance.items().iter() {
                let distance = entry.distance;
                let chr_ins = unsafe { entry.chr_ins.as_ref() };

                if ui.collapsing_header(
                    format!("ChrIns {} - {}", chr_ins.field_ins_handle, distance),
                    TreeNodeFlags::empty(),
                ) {
                    ui.indent();
                    chr_ins.render_debug(ui);
                    ui.unindent();
                }
            }
            ui.unindent();
        }
    }
}

impl DebugDisplay for NetChrSetSync {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        if ui.collapsing_header("Readback Flags", TreeNodeFlags::empty()) {
            ui.indent();
            self.update_flags()
                .iter()
                .enumerate()
                .for_each(|e| ui.text(format!("{} {:016b}", e.0, e.1 .0)));
            ui.unindent();
        }

        ui.text(format!("Character capacity: {}", self.capacity));
    }
}

impl DebugDisplay for ChrSet<ChrIns> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        if ui.collapsing_header("Characters", TreeNodeFlags::empty()) {
            ui.indent();
            self.characters().for_each(|chr_ins| {
                if ui.collapsing_header(
                    format!(
                        "c{:0>4} - {} FieldInsSelector({}, {})",
                        chr_ins.character_id,
                        chr_ins.field_ins_handle.map_id,
                        chr_ins.field_ins_handle.selector.container(),
                        chr_ins.field_ins_handle.selector.index()
                    ),
                    TreeNodeFlags::empty(),
                ) {
                    chr_ins.render_debug(ui)
                }
            });
            ui.unindent();
        }

        if ui.collapsing_header("Character event ID mapping", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header_with_flags(
                "event-flags-groups",
                [
                    TableColumnSetup::new("Event ID"),
                    TableColumnSetup::new("Field Ins Handle"),
                ],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            ) {
                self.entity_id_mapping.iter().for_each(|e| {
                    ui.table_next_column();
                    ui.text(e.entity_id.to_string());

                    ui.table_next_column();
                    let chr_ins = unsafe { e.chr_set_entry.as_ref().chr_ins.as_ref() };
                    ui.text(format!("{}", unsafe {
                        &chr_ins.unwrap().as_ref().field_ins_handle
                    }));
                });
            }
            ui.unindent();
        }

        if ui.collapsing_header("Group mapping", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header_with_flags(
                "event-flags-groups",
                [
                    TableColumnSetup::new("Group"),
                    TableColumnSetup::new("Field Ins Handle"),
                ],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            ) {
                self.group_id_mapping.iter().for_each(|e| {
                    ui.table_next_column();
                    ui.text(e.group_id.to_string());

                    ui.table_next_column();
                    let chr_ins = unsafe { e.chr_set_entry.as_ref().chr_ins.as_ref() };
                    ui.text(format!("{}", unsafe {
                        &chr_ins.unwrap().as_ref().field_ins_handle
                    }));
                });
            }
            ui.unindent();
        }
    }
}

impl DebugDisplay for ChrSet<PlayerIns> {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        if ui.collapsing_header("Characters", TreeNodeFlags::empty()) {
            ui.indent();
            self.characters().for_each(|player_ins| {
                if ui.collapsing_header(
                    format!(
                        "c{:0>4} - {} FieldInsSelector({}, {})",
                        player_ins.chr_ins.character_id,
                        player_ins.chr_ins.field_ins_handle.map_id,
                        player_ins.chr_ins.field_ins_handle.selector.container(),
                        player_ins.chr_ins.field_ins_handle.selector.index()
                    ),
                    TreeNodeFlags::empty(),
                ) {
                    player_ins.render_debug(ui)
                }
            });
            ui.unindent();
        }

        if ui.collapsing_header("Character event ID mapping", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header_with_flags(
                "event-flags-groups",
                [
                    TableColumnSetup::new("Event ID"),
                    TableColumnSetup::new("Field Ins Handle"),
                ],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            ) {
                self.entity_id_mapping.iter().for_each(|e| {
                    ui.table_next_column();
                    ui.text(e.entity_id.to_string());

                    ui.table_next_column();
                    let chr_ins = unsafe { e.chr_set_entry.as_ref().chr_ins.as_ref() };
                    ui.text(format!("{}", unsafe {
                        &chr_ins.unwrap().as_ref().chr_ins.field_ins_handle
                    }));
                });
            }
            ui.unindent();
        }

        if ui.collapsing_header("Group mapping", TreeNodeFlags::empty()) {
            ui.indent();
            if let Some(_t) = ui.begin_table_header_with_flags(
                "event-flags-groups",
                [
                    TableColumnSetup::new("Group"),
                    TableColumnSetup::new("Field Ins Handle"),
                ],
                TableFlags::RESIZABLE
                    | TableFlags::BORDERS
                    | TableFlags::ROW_BG
                    | TableFlags::SIZING_STRETCH_PROP,
            ) {
                self.group_id_mapping.iter().for_each(|e| {
                    ui.table_next_column();
                    ui.text(e.group_id.to_string());

                    ui.table_next_column();
                    let chr_ins = unsafe { e.chr_set_entry.as_ref().chr_ins.as_ref() };
                    ui.text(format!("{}", unsafe {
                        &chr_ins.unwrap().as_ref().chr_ins.field_ins_handle
                    }));
                });
            }
            ui.unindent();
        }
    }
}

impl DebugDisplay for OpenFieldChrSet {
    fn render_debug(&self, ui: &&mut Ui) {
        self.base.render_debug(ui)
    }
}

impl DebugDisplay for SummonBuddyManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "To spawn buddy param: {}",
            self.to_spawn_buddy_param
        ));
        ui.text(format!("Spawned buddy param: {}", self.spawned_buddy_param));
        ui.text(format!("Next summony buddy slot: {}", self.next_buddy_slot));

        // self.w.render_debug(ui);
    }
}

impl DebugDisplay for SummonBuddyWarpManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "Trigger time ray block: {}",
            self.trigger_time_ray_block
        ));
        ui.text(format!(
            "Trigger dist to player: {}",
            self.trigger_dist_to_player
        ));
        ui.text(format!(
            "Trigger threshold time path stacked: {}",
            self.trigger_threshold_time_path_stacked
        ));
        ui.text(format!(
            "Trigger treshhold range path stacked: {}",
            self.trigger_threshold_range_path_stacked
        ));
    }
}
