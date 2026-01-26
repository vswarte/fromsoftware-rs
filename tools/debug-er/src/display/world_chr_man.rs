use eldenring::cs::{
    ChrIns, ChrSet, NetChrSetSync, OpenFieldChrSet, SummonBuddyGroupEntry, SummonBuddyManager,
    SummonBuddyWarpEntry, SummonBuddyWarpManager, WorldChrMan,
};
use hudhook::imgui::{TableColumnSetup, Ui};

use fromsoftware_shared::Subclass;

use super::{DebugDisplay, UiExt};

impl DebugDisplay for WorldChrMan {
    fn render_debug(&self, ui: &Ui) {
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

        ui.header("Player ChrSet", || {
            self.player_chr_set.render_debug(ui);
        });

        ui.header("Ghost ChrSet", || {
            self.ghost_chr_set.render_debug(ui);
        });

        ui.header("SummonBuddy ChrSet", || {
            self.summon_buddy_chr_set.render_debug(ui);
        });

        ui.header("Debug ChrSet", || {
            self.debug_chr_set.render_debug(ui);
        });

        ui.header("OpenField ChrSet", || {
            self.open_field_chr_set.render_debug(ui);
        });

        ui.list(
            "All ChrSets",
            self.chr_sets.iter().filter_map(|entry| entry.as_ref()),
            |ui, i, chr_set| {
                ui.header(&format!("ChrSet {i}"), || {
                    chr_set.render_debug(ui);
                });
            },
        );

        match self.main_player.as_ref() {
            Some(p) => {
                ui.header("Main player", || {
                    p.render_debug(ui);
                });
            }
            None => ui.text("No Main player instance"),
        }

        ui.header("SummonBuddyManager", || {
            self.summon_buddy_manager.render_debug(ui);
        });

        ui.list(
            "NetChrSetSync",
            self.net_chr_sync
                .net_chr_set_sync
                .iter()
                .filter_map(|s| s.as_ref()),
            |ui, i, entry| {
                ui.header(&format!("NetChrSetSync {i}"), || {
                    entry.render_debug(ui);
                });
            },
        );

        ui.header("Debug Character Creator", || {
            ui.input_text(
                "Last Created Character",
                &mut format!("{:x?}", self.debug_chr_creator.last_created_chr),
            )
            .read_only(true)
            .build();
        });

        // We can't use .list here because it relies on entries being stable across frames
        // and these are constantly changing, making it hard to keep track of which collapsing header is closed or open.
        ui.header("ChrInses by distance", || {
            self.chr_inses_by_distance.items().iter().for_each(|entry| {
                let distance = entry.distance;
                let chr_ins = unsafe { entry.chr_ins.as_ref() };
                let label = format!("ChrIns {}", chr_ins.field_ins_handle);
                let _id = ui.push_id(&label);
                ui.header(&label, || {
                    ui.text(format!("Distance: {}", distance));
                    chr_ins.render_debug(ui);
                });
            });
        });
    }
}

impl DebugDisplay for NetChrSetSync {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        ui.list(
            "Placement Updates",
            self.update_flags().iter(),
            |ui, i, flags| {
                let placement = &self.placement_updates()[i];
                ui.header(&format!("Index {i}"), || {
                    ui.text(format!(
                        "Has Placement Update: {}",
                        flags.has_placement_update()
                    ));
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
                ui.header(&format!("Index {i}"), || {
                    ui.text(format!("Has Health Update: {}", flags.has_health_update()));
                    ui.text(format!("Current HP: {}", health.current_hp));
                    ui.text(format!("Damage Taken: {}", health.damage_taken));
                });
            },
        );
    }
}

impl<T> DebugDisplay for ChrSet<T>
where
    T: Subclass<ChrIns>,
{
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Character capacity: {}", self.capacity));

        ui.list("Characters", self.characters(), |ui, _i, chr_ins| {
            let chr_ins = chr_ins.superclass();
            ui.header(
                &format!(
                    "c{:0>4} - {} FieldInsSelector({}, {})",
                    chr_ins.character_id,
                    chr_ins.field_ins_handle.block_id,
                    chr_ins.field_ins_handle.selector.container(),
                    chr_ins.field_ins_handle.selector.index()
                ),
                || {
                    chr_ins.superclass().render_debug(ui);
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

impl DebugDisplay for OpenFieldChrSet {
    fn render_debug(&self, ui: &Ui) {
        self.base.render_debug(ui)
    }
}

impl DebugDisplay for SummonBuddyWarpEntry {
    fn render_debug(&self, ui: &Ui) {
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
    fn render_debug(&self, ui: &Ui) {
        ui.list("Warp Entries", self.entries.iter(), |ui, index, entry| {
            ui.header(&format!("Warp Entry {index}"), || {
                entry.render_debug(ui);
            });
        });
    }
}

impl DebugDisplay for SummonBuddyGroupEntry {
    fn render_debug(&self, ui: &Ui) {
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
    fn render_debug(&self, ui: &Ui) {
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

        ui.header("Spawn Origin", || {
            ui.text(format!("X: {}", self.spawn_origin.0));
            ui.text(format!("Y: {}", self.spawn_origin.1));
            ui.text(format!("Z: {}", self.spawn_origin.2));
            ui.text(format!("W: {}", self.spawn_origin.3));
        });

        ui.list("Groups", self.groups.iter(), |ui, _i, group| {
            ui.header(&format!("Group {}", group.owner_event_id), || {
                ui.list("Entries", group.entries.iter(), |ui, index, v| {
                    ui.header(&format!("Entry {index}"), || {
                        v.render_debug(ui);
                    });
                });
            });
        });

        ui.list(
            "Eliminate Target Entries",
            self.eliminate_target_entries.iter(),
            |ui, index, entry| {
                ui.header(&format!("Entry {index}"), || {
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
                });
            },
        );

        ui.header("Warp Manager", || {
            self.warp_manager.render_debug(ui);
        });
    }
}
