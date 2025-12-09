use eldenring::cs::{
    ChrIns, ChrSet, NetChrSetSync, OpenFieldChrSet, PlayerIns, SummonBuddyGroupEntry,
    SummonBuddyManager, SummonBuddyWarpEntry, SummonBuddyWarpManager, WorldChrMan,
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

        if ui.collapsing_header("SummonBuddyManager", TreeNodeFlags::empty()) {
            ui.indent();
            self.summon_buddy_manager.render_debug(ui);
            ui.unindent();
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
                .for_each(|e| ui.text(format!("{} {:016b}", e.0, e.1.0)));
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
                        chr_ins.field_ins_handle.block_id,
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
                        player_ins.chr_ins.field_ins_handle.block_id,
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

impl DebugDisplay for SummonBuddyWarpEntry {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Handle: {}", self.handle));
        ui.text(format!("Warp stage: {:?}", self.warp_stage));
        ui.text(format!("Target position: {}", self.target_position));
        ui.text(format!("Target rotation: {:?}", self.q_target_rotation));
        ui.text(format!("Flags: {:032b}", self.flags));
        ui.text(format!("Time ray blocked: {}", self.time_ray_blocked));
        ui.text(format!("Time path stacked: {}", self.time_path_stacked));
    }
}

impl DebugDisplay for SummonBuddyWarpManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Warp entry count: {}", self.entries.len()));

        for (index, entry) in self.entries.iter().enumerate() {
            if ui.collapsing_header(format!("Warp Entry {index}"), TreeNodeFlags::empty()) {
                ui.indent();
                entry.render_debug(ui);
                ui.unindent();
            }
        }
    }
}

impl DebugDisplay for SummonBuddyGroupEntry {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!("Buddy param ID: {}", self.buddy_param_id));
        ui.text(format!("Has mount: {}", self.has_mount));
        ui.text(format!(
            "Buddy stone param ID: {}",
            self.buddy_stone_param_id
        ));
        ui.text(format!("Doping SpEffect ID: {}", self.doping_sp_effect_id));
        ui.text(format!(
            "Dopping level SpEffect ID: {}",
            self.dopping_level_sp_effect_id
        ));
        ui.text(format!("Spawn animation ID: {}", self.spawn_animation));
        ui.text(format!("Warp requested: {}", self.warp_requested));
        ui.text(format!("Disappear requested: {}", self.disappear_requested));
        ui.text(format!("Disappear delay sec: {}", self.disappear_delay_sec));
        ui.text(format!("Has spawn point: {}", self.has_spawn_point));
        ui.text(format!(
            "Disable PC target share: {}",
            self.disable_pc_target_share
        ));
        ui.text(format!("Follow type: {}", self.follow_type));
        ui.text(format!("Is remote: {}", self.is_remote));
        ui.text(format!(
            "Has Mogh's Great Rune buff: {}",
            self.has_mogh_great_rune_buff
        ));
    }
}

impl DebugDisplay for SummonBuddyManager {
    fn render_debug(&self, ui: &&mut Ui) {
        ui.text(format!(
            "Request summon SpEffect ID: {}",
            self.request_summon_speffect_id
        ));
        ui.text(format!(
            "Active summon SpEffect ID: {}",
            self.active_summon_speffect_id
        ));
        ui.text(format!("Disappear requested: {}", self.disappear_requested));
        ui.text(format!(
            "Buddy stone entity ID: {}",
            self.buddy_stone_entity_id
        ));
        ui.text(format!(
            "Active summon buddy stone entity ID: {}",
            self.active_summmon_buddy_stone_entity_id
        ));
        ui.text(format!(
            "Buddy disappear delay sec: {}",
            self.buddy_disappear_delay_sec
        ));
        ui.text(format!(
            "Item use cooldown timer: {}",
            self.item_use_cooldown_timer
        ));
        ui.text(format!("Spawn rotation: {}", self.spawn_rotation));
        ui.text(format!(
            "Player has alive summon: {}",
            self.player_has_alive_summon
        ));
        ui.text(format!(
            "Is within activation range: {}",
            self.is_within_activation_range
        ));
        ui.text(format!(
            "Is within warn range: {}",
            self.is_within_warn_range
        ));
        ui.text(format!("Last buddy slot: {}", self.last_buddy_slot));
        ui.text(format!(
            "Debug buddy stone param ID: {}",
            self.debug_buddy_stone_param_id
        ));
        ui.text(format!(
            "Requested summon buddy goods ID: {}",
            self.requested_summon_goods_id
        ));
        ui.text(format!(
            "Active summon buddy goods ID: {}",
            self.active_summon_goods_id
        ));

        if ui.collapsing_header("Spawn Origin", TreeNodeFlags::empty()) {
            ui.indent();
            ui.text(format!("X: {}", self.spawn_origin.0));
            ui.text(format!("Y: {}", self.spawn_origin.1));
            ui.text(format!("Z: {}", self.spawn_origin.2));
            ui.text(format!("W: {}", self.spawn_origin.3));
            ui.unindent();
        }

        if ui.collapsing_header("Groups", TreeNodeFlags::empty()) {
            ui.indent();
            for group in self.groups.iter() {
                if ui.collapsing_header(
                    format!("Group {}", group.owner_event_id),
                    TreeNodeFlags::empty(),
                ) {
                    ui.indent();
                    for (index, v) in group.entries.iter().enumerate() {
                        if ui.collapsing_header(format!("Entry {index}"), TreeNodeFlags::empty()) {
                            ui.indent();
                            v.render_debug(ui);
                            ui.unindent();
                        }
                    }
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Eliminate Target Entries", TreeNodeFlags::empty()) {
            ui.indent();
            for (index, entry) in self.eliminate_target_entries.iter().enumerate() {
                if ui.collapsing_header(format!("Entry {index}"), TreeNodeFlags::empty()) {
                    ui.indent();
                    ui.text(format!(
                        "Buddy field ins handle: {}",
                        entry.buddy_field_ins_handle
                    ));
                    ui.text(format!(
                        "Buddy stone param ID: {}",
                        entry.target_calc.buddy_stone_param_id
                    ));
                    ui.text(format!(
                        "Target event entity ID: {}",
                        entry.target_calc.target_event_entity_id
                    ));
                    ui.text(format!(
                        "Target in range: {}",
                        entry.target_calc.target_in_range
                    ));
                    ui.text(format!(
                        "Range check counter: {}",
                        entry.target_calc.range_check_counter
                    ));
                    ui.unindent();
                }
            }
            ui.unindent();
        }

        if ui.collapsing_header("Warp Manager", TreeNodeFlags::empty()) {
            ui.indent();
            self.warp_manager.render_debug(ui);
            ui.unindent();
        }
    }
}
