use hudhook::imgui::{TableColumnSetup, Ui};

use debug::{StateMap, UiExt};
use eldenring::cs::{
    ChrIns, ChrSet, FieldInsHandle, NetChrSetSync, SummonBuddyGroupEntry, SummonBuddyManager,
    SummonBuddyWarpEntry, SummonBuddyWarpManager, WorldChrMan,
};
use fromsoftware_shared::Subclass;

use super::{DebugDisplay, DisplayUiExt, StatefulDebugDisplay, chr::ChrInsState};

#[derive(Default)]
pub struct WorldChrManState {
    chr_ins_states: StateMap<FieldInsHandle, ChrInsState>,
    player_chr_set_state: ChrSetState,
    ghost_chr_set_state: ChrSetState,
    summon_buddy_chr_set_state: ChrSetState,
    debug_chr_set_state: ChrSetState,
    open_field_chr_set_state: ChrSetState,
    chr_set_states: StateMap<usize, ChrSetState>,
}

impl StatefulDebugDisplay for WorldChrMan {
    type State = WorldChrManState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        state.chr_ins_states.track_reads();

        ui.display("World Area Chr List Count", self.world_area_chr_list_count);

        let world_block_chr_list_count = self.world_block_chr_list_count;
        ui.display("World Block Chr List Count", world_block_chr_list_count);

        ui.display(
            "World Grid Area Chr List Count",
            self.world_grid_area_chr_list_count,
        );

        ui.display("World Area List Count", self.world_area_list_count);

        ui.header("Player ChrSet", || {
            self.player_chr_set
                .render_debug_mut(ui, &mut state.player_chr_set_state);
        });

        ui.header("Ghost ChrSet", || {
            self.ghost_chr_set
                .render_debug_mut(ui, &mut state.ghost_chr_set_state);
        });

        ui.header("SummonBuddy ChrSet", || {
            self.summon_buddy_chr_set
                .render_debug_mut(ui, &mut state.summon_buddy_chr_set_state);
        });

        ui.header("Debug ChrSet", || {
            self.debug_chr_set
                .render_debug_mut(ui, &mut state.debug_chr_set_state);
        });

        ui.header("OpenField ChrSet", || {
            self.open_field_chr_set
                .superclass_mut()
                .render_debug_mut(ui, &mut state.open_field_chr_set_state);
        });

        ui.list(
            "All ChrSets",
            self.chr_sets.iter_mut().filter_map(|entry| entry.as_mut()),
            |ui, i, chr_set| {
                ui.header(format!("ChrSet {i}"), || {
                    let state = state.chr_set_states.get(i);
                    chr_set.render_debug_mut(ui, state);
                });
            },
        );

        ui.header_opt("Main player", self.main_player.as_mut(), |p| {
            let state = state.chr_ins_states.get(p.field_ins_handle);
            p.render_debug_mut(ui, state);
        });

        ui.nested("SummonBuddyManager", &self.summon_buddy_manager);

        ui.list(
            "NetChrSetSync",
            self.net_chr_sync
                .net_chr_set_sync
                .iter()
                .filter_map(|s| s.as_ref()),
            |ui, i, entry| ui.nested(format!("NetChrSetSync {i}"), entry),
        );

        ui.header("Debug Character Creator", || {
            ui.display_copiable(
                "Last Created Character",
                format!("{:x?}", self.debug_chr_creator.last_created_chr),
            );
        });

        // We can't use .list here because it relies on entries being stable across frames
        // and these are constantly changing, making it hard to keep track of which collapsing header is closed or open.
        ui.header("ChrInses by distance", || {
            self.chr_inses_by_distance
                .items_mut()
                .iter_mut()
                .for_each(|entry| {
                    let distance = entry.distance;
                    let chr_ins = unsafe { entry.chr_ins.as_mut() };
                    let label = format!("ChrIns {}", chr_ins.field_ins_handle);
                    let _id = ui.push_id(&label);
                    ui.header(&label, || {
                        ui.display("Distance", distance);
                        chr_ins.render_debug_mut(
                            ui,
                            state.chr_ins_states.get(chr_ins.field_ins_handle),
                        );
                    });
                });
        });

        state.chr_ins_states.remove_unread();
    }
}

impl DebugDisplay for NetChrSetSync {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Character capacity", self.capacity);

        ui.list(
            "Placement Updates",
            self.update_flags().iter(),
            |ui, i, flags| {
                let placement = &self.placement_updates()[i];
                ui.header(format!("Index {i}"), || {
                    ui.display("Has Placement Update", flags.has_placement_update());
                    ui.text(format!(
                        "Position: ({},{},{})",
                        placement.position.0, placement.position.1, placement.position.2
                    ));
                    ui.text(format!(
                        "Orientation: ({},{},{})",
                        placement.rotation.0, placement.rotation.1, placement.rotation.2
                    ));
                });
            },
        );

        ui.list(
            "Health Updates",
            self.update_flags().iter(),
            |ui, i, flags| {
                let health = &self.health_updates()[i];
                ui.header(format!("Index {i}"), || {
                    ui.display("Has Health Update", flags.has_health_update());
                    ui.display("Current HP", health.current_hp);
                    ui.display("Damage Taken", health.damage_taken);
                });
            },
        );
    }
}

#[derive(Default)]
pub struct ChrSetState {
    chr_ins_states: StateMap<FieldInsHandle, ChrInsState>,
}

impl<T> StatefulDebugDisplay for ChrSet<T>
where
    T: Subclass<ChrIns>,
{
    type State = ChrSetState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        ui.display("Character capacity", self.capacity);

        ui.list("Characters", self.characters(), |ui, _i, chr_ins| {
            let chr_ins = chr_ins.superclass_mut();
            ui.header(
                format!(
                    "c{:0>4} - {} FieldInsSelector({}, {})",
                    chr_ins.character_id,
                    chr_ins.field_ins_handle.block_id,
                    chr_ins.field_ins_handle.selector.container(),
                    chr_ins.field_ins_handle.selector.index()
                ),
                || {
                    let state = state.chr_ins_states.get(chr_ins.field_ins_handle);
                    chr_ins.superclass_mut().render_debug_mut(ui, state);
                },
            );
        });

        ui.header("Character event ID mapping", || {
            ui.table(
                "character-event-id-mapping",
                [
                    TableColumnSetup::new("Event ID"),
                    TableColumnSetup::new("Field Ins Handle"),
                ],
                self.entity_id_mapping.iter(),
                |ui, _i, e| {
                    ui.table_next_column();
                    ui.text(e.entity_id.to_string());

                    ui.table_next_column();
                    let chr_ins = unsafe { e.chr_set_entry.as_ref().chr_ins.as_ref() };
                    ui.text(format!("{}", unsafe {
                        &chr_ins.unwrap().as_ref().superclass().field_ins_handle
                    }));
                },
            );
        });

        ui.header("Group mapping", || {
            ui.table(
                "group-mapping",
                [
                    TableColumnSetup::new("Group"),
                    TableColumnSetup::new("Field Ins Handle"),
                ],
                self.group_id_mapping.iter(),
                |ui, _i, e| {
                    ui.table_next_column();
                    ui.text(e.group_id.to_string());

                    ui.table_next_column();
                    let chr_ins = unsafe { e.chr_set_entry.as_ref().chr_ins.as_ref() };
                    ui.text(format!("{}", unsafe {
                        &chr_ins.unwrap().as_ref().superclass().field_ins_handle
                    }));
                },
            );
        });
    }
}

impl DebugDisplay for SummonBuddyWarpEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Handle", self.handle);
        ui.debug("Warp stage", self.warp_stage);
        ui.display("Target position", self.target_position);
        ui.debug("Target rotation", self.q_target_rotation);
        ui.text(format!("Flags: {:032b}", self.flags));
        ui.display("Time ray blocked", self.time_ray_blocked);
        ui.display("Time path stacked", self.time_path_stacked);
    }
}

impl DebugDisplay for SummonBuddyWarpManager {
    fn render_debug(&self, ui: &Ui) {
        ui.list("Warp Entries", self.entries.iter(), |ui, index, entry| {
            ui.nested(format!("Warp Entry {index}"), entry);
        });
    }
}

impl DebugDisplay for SummonBuddyGroupEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Buddy param ID", self.buddy_param_id);
        ui.display("Has mount", self.has_mount);
        ui.display("Buddy stone param ID", self.buddy_stone_param_id);
        ui.display("Doping SpEffect ID", self.doping_sp_effect_id);
        ui.display("Dopping level SpEffect ID", self.dopping_level_sp_effect_id);
        ui.display("Spawn animation ID", self.spawn_animation);
        ui.display("Warp requested", self.warp_requested);
        ui.display("Disappear requested", self.disappear_requested);
        ui.display("Disappear delay sec", self.disappear_delay_sec);
        ui.display("Has spawn point", self.has_spawn_point);
        ui.display("Disable PC target share", self.disable_pc_target_share);
        ui.display("Follow type", self.follow_type);
        ui.display("Is remote", self.is_remote);
        ui.display("Has Mogh's Great Rune buff", self.has_mogh_great_rune_buff);
    }
}

impl DebugDisplay for SummonBuddyManager {
    fn render_debug(&self, ui: &Ui) {
        ui.display(
            "Request summon SpEffect ID",
            self.request_summon_speffect_id,
        );
        ui.display("Active summon SpEffect ID", self.active_summon_speffect_id);
        ui.display("Disappear requested", self.disappear_requested);
        ui.display("Buddy stone entity ID", self.buddy_stone_entity_id);
        ui.display(
            "Active summon buddy stone entity ID",
            self.active_summmon_buddy_stone_entity_id,
        );
        ui.display("Buddy disappear delay sec", self.buddy_disappear_delay_sec);
        ui.display("Item use cooldown timer", self.item_use_cooldown_timer);
        ui.display("Spawn rotation", self.spawn_rotation);
        ui.display("Player has alive summon", self.player_has_alive_summon);
        ui.display(
            "Is within activation range",
            self.is_within_activation_range,
        );
        ui.display("Is within warn range", self.is_within_warn_range);
        ui.display("Last buddy slot", self.last_buddy_slot);
        ui.display(
            "Debug buddy stone param ID",
            self.debug_buddy_stone_param_id,
        );
        ui.display(
            "Requested summon buddy goods ID",
            self.requested_summon_goods_id,
        );
        ui.display("Active summon buddy goods ID", self.active_summon_goods_id);

        ui.header("Spawn Origin", || {
            ui.display("X", self.spawn_origin.0);
            ui.display("Y", self.spawn_origin.1);
            ui.display("Z", self.spawn_origin.2);
            ui.display("W", self.spawn_origin.3);
        });

        ui.list("Groups", self.groups.iter(), |ui, _i, group| {
            ui.header(format!("Group {}", group.owner_event_id), || {
                ui.list("Entries", group.entries.iter(), |ui, index, v| {
                    ui.nested(format!("Entry {index}"), v);
                });
            });
        });

        ui.list(
            "Eliminate Target Entries",
            self.eliminate_target_entries.iter(),
            |ui, index, entry| {
                ui.header(format!("Entry {index}"), || {
                    ui.display("Buddy field ins handle", entry.buddy_field_ins_handle);
                    ui.display(
                        "Buddy stone param ID",
                        entry.target_calc.buddy_stone_param_id,
                    );
                    ui.display(
                        "Target event entity ID",
                        entry.target_calc.target_event_entity_id,
                    );
                    ui.display("Target in range", entry.target_calc.target_in_range);
                    ui.display("Range check counter", entry.target_calc.range_check_counter);
                });
            },
        );

        ui.nested("Warp Manager", &self.warp_manager);
    }
}
