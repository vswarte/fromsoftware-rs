use std::str::FromStr;

use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{
    CSChrBehaviorDataModule, CSChrLadderModule, CSChrModelParamModifierModule, CSChrPhysicsModule,
    CSChrRideModule, CSChrTimeActModule, CSPairAnimNode, CSRideNode, ChrAsm, ChrAsmEquipEntries,
    ChrAsmEquipment, ChrAsmSlot, ChrIns, ChrInsExt, ChrInsModuleContainer, ChrInsSubclassMut,
    ChrPhysicsMaterialInfo, EquipGameData, EquipInventoryData, EquipItemData, EquipMagicData,
    ItemReplenishStateTracker, PlayerGameData, PlayerIns,
};
use fromsoftware_shared::NonEmptyIteratorExt;

use crate::display::{DebugDisplay, DisplayUiExt, StatefulDebugDisplay};

#[derive(Default)]
pub struct ChrInsState {
    new_speffect: String,
}

impl StatefulDebugDisplay for PlayerIns {
    type State = ChrInsState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        chr_ins_common_debug(&mut self.chr_ins, ui, state);

        ui.nested("ChrAsm", &self.chr_asm);
        ui.nested("PlayerGameData", &self.player_game_data);
        ui.nested(
            "Session Player Entry",
            self.session_manager_player_entry.as_ref(),
        );
        ui.display(
            "Invincibility timer",
            self.invincibility_timer_for_net_player,
        );
        ui.display("Locked on enemy", self.locked_on_enemy);
        ui.display("Block position", self.block_position);
    }
}

impl DebugDisplay for ChrAsm {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("ChrAsmEquipment", &self.equipment);
        ui.header("GaitemHandles", || {
            ui.table(
                "chr-asm-gaitem-handles",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Slot"),
                    TableColumnSetup::new("Gaitem Handle"),
                ],
                self.gaitem_handles.iter(),
                |ui, i, e| {
                    ui.table_next_column();
                    ui.text(format!("{i}"));

                    ui.table_next_column();
                    match ChrAsmSlot::from_index(i as u32) {
                        Ok(slot) => ui.text(format!("{slot:?}")),
                        Err(err) => ui.text(err.to_string()),
                    }

                    ui.table_next_column();
                    ui.text(e.to_string());
                },
            );
        });

        ui.header("Param IDs", || {
            ui.table(
                "chr-asm-param-ids",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Slot"),
                    TableColumnSetup::new("Param ID"),
                ],
                self.equipment_param_ids.iter(),
                |ui, i, e| {
                    ui.table_next_column();
                    ui.text(format!("{i}"));

                    ui.table_next_column();
                    match ChrAsmSlot::from_index(i as u32) {
                        Ok(slot) => ui.text(format!("{slot:?}")),
                        Err(err) => ui.text(err.to_string()),
                    }

                    ui.table_next_column();
                    ui.text(e.to_string());
                },
            );
        });
    }
}

impl DebugDisplay for ChrAsmEquipment {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Arm style", self.arm_style);
        ui.debug(
            "Left-hand weapon slot",
            self.selected_slots.left_weapon_slot,
        );
        ui.debug(
            "Right-hand weapon slot",
            self.selected_slots.right_weapon_slot,
        );
        ui.debug("Left-hand arrow slot", self.selected_slots.left_arrow_slot);
        ui.debug(
            "Right-hand arrow slot",
            self.selected_slots.right_arrow_slot,
        );
        ui.debug("Left-hand bolt slot", self.selected_slots.left_bolt_slot);
        ui.debug("Right-hand bolt slot", self.selected_slots.right_bolt_slot);
    }
}

impl DebugDisplay for ChrAsmEquipEntries {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Primary Left weapon", self.weapon_primary_left.param_id());
        ui.debug("Primary Right weapon", self.weapon_primary_right.param_id());
        ui.debug(
            "Secondary Left weapon",
            self.weapon_secondary_left.param_id(),
        );
        ui.debug(
            "Secondary Right weapon",
            self.weapon_secondary_right.param_id(),
        );
        ui.debug("Tertiary Left weapon", self.weapon_tertiary_left.param_id());
        ui.debug(
            "Tertiary Right weapon",
            self.weapon_tertiary_right.param_id(),
        );

        ui.debug("Primary Left arrow", self.arrow_primary.param_id());
        ui.debug("Primary Left bolt", self.bolt_primary.param_id());
        ui.debug("Secondary Left arrow", self.arrow_secondary.param_id());
        ui.debug("Secondary Left bolt", self.bolt_secondary.param_id());
        ui.debug("Tertiary Left arrow", self.arrow_tertiary.param_id());
        ui.debug("Tertiary Left bolt", self.bolt_tertiary.param_id());

        ui.debug("Protector Head", self.protector_head.param_id());
        ui.debug("Protector Chest", self.protector_chest.param_id());
        ui.debug("Protector Hands", self.protector_hands.param_id());
        ui.debug("Protector Legs", self.protector_legs.param_id());

        ui.list("Accessories", self.accessories.iter(), |ui, i, item| {
            ui.text(format!("{}: {:?}", i, item));
        });

        ui.text(format!("Covenant: {:?}", self.covenant.param_id()));

        ui.list("Quick Items", self.quick_tems.iter(), |ui, index, item| {
            ui.text(format!("{}: {:?}", index, item));
        });

        ui.list("Pouch", self.pouch.iter(), |ui, i, item| {
            ui.text(format!("{}: {:?}", i, item));
        });
    }
}

impl DebugDisplay for PlayerGameData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Player ID", self.player_id);
        ui.debug(
            "Furlcalling Finger Active",
            self.furlcalling_finger_remedy_active,
        );
        ui.debug("Rune Arc Active", self.rune_arc_active);
        ui.debug("White Ring Active", self.white_ring_active);
        ui.debug("Blue Ring Active", self.blue_ring_active);

        ui.debug("Character Event ID", self.character_event_id);
        ui.debug("Character Type", self.chr_type);
        ui.debug("Multiplay Role", self.multiplay_role);

        ui.nested("EquipGameData", &self.equipment);
        ui.nested_opt("Storage Box EquipInventoryData", self.storage.as_ref());
    }
}

impl DebugDisplay for EquipGameData {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("EquipInventoryData", &self.equip_inventory_data);
        ui.nested("EquipMagicData", &self.equip_magic_data);
        ui.nested("EquipItemData", &self.equip_item_data);
        ui.nested_opt(
            "Item Replenish State Tracker",
            self.item_replenish_state_tracker.as_ref(),
        );
    }
}

impl DebugDisplay for ItemReplenishStateTracker {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "item-replenish-state-tracker-entries",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("Item ID"),
                TableColumnSetup::new("Auto Replenish"),
            ],
            self.entries().iter(),
            |ui, index, item| {
                ui.table_next_column();
                ui.text(index.to_string());

                ui.table_next_column();
                ui.text(format!("{:?}", item.item_id));

                ui.table_next_column();
                ui.text(item.auto_replenish.to_string());
            },
        );
        ui.text(format!("Count: {}", self.count));
    }
}

impl DebugDisplay for EquipMagicData {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Selected slot: {}", self.selected_slot));

        ui.header("EquipDataItem", || {
            ui.table(
                "equip-magic-data-entries",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Param ID"),
                    TableColumnSetup::new("Charges"),
                ],
                self.entries.iter(),
                |ui, index, item| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(item.param_id.to_string());

                    ui.table_next_column();
                    ui.text(item.charges.to_string());
                },
            );
        });
    }
}

impl DebugDisplay for EquipItemData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Selected quick slot", self.selected_quick_slot);

        ui.header("Quick slots", || {
            ui.table(
                "equip-item-data-quick-slots",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Inventory Index"),
                ],
                self.quick_slots.iter(),
                |ui, index, item| {
                    ui.table_next_column();
                    ui.text(index.to_string());
                    ui.align_text_to_frame_padding();

                    ui.table_next_column();
                    ui.text(item.gaitem_handle.to_string());

                    ui.table_next_column();
                    ui.text(item.index.to_string());
                },
            );
        });

        ui.header("Pouch slots", || {
            ui.table(
                "equip-item-data-pouch-slots",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Inventory Index"),
                ],
                self.pouch_slots.iter(),
                |ui, index, item| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(item.gaitem_handle.to_string());

                    ui.table_next_column();
                    ui.text(item.index.to_string());
                },
            );
        });

        ui.text(format!(
            "Greatrune: {}, index: {}",
            self.great_rune.gaitem_handle, self.great_rune.index
        ));

        ui.nested("Equipment Entries", &self.equip_entries);
        ui.display("Selected Quick Slot", self.selected_quick_slot);
    }
}

impl DebugDisplay for EquipInventoryData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Total item entry count", self.total_item_entry_count);

        let normal_items = self
            .items_data
            .normal_entries()
            .iter()
            .non_empty()
            .collect::<Vec<_>>();
        let label = format!(
            "Normal Items ({}/{})",
            normal_items.len(),
            self.items_data.normal_items_capacity
        );
        ui.header(&label, || {
            ui.table(
                "equip-inventory-data-normal-items",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                    TableColumnSetup::new("Display ID"),
                    TableColumnSetup::new("Is New"),
                ],
                normal_items.iter(),
                |ui, index, item| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(item.gaitem_handle.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", item.item_id));

                    ui.table_next_column();
                    ui.text(item.quantity.to_string());

                    ui.table_next_column();
                    ui.text(item.sort_id.to_string());

                    ui.table_next_column();
                    ui.text(item.is_new.to_string());
                },
            );
        });

        let key_items = self
            .items_data
            .key_entries()
            .iter()
            .non_empty()
            .collect::<Vec<_>>();
        let label = format!(
            "Key Items ({}/{})",
            key_items.len(),
            self.items_data.key_items_capacity
        );
        ui.header(&label, || {
            ui.table(
                "equip-inventory-data-key-items",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                    TableColumnSetup::new("Display ID"),
                    TableColumnSetup::new("Is New"),
                ],
                key_items.iter(),
                |ui, index, item| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(item.gaitem_handle.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", item.item_id));

                    ui.table_next_column();
                    ui.text(item.quantity.to_string());

                    ui.table_next_column();
                    ui.text(item.sort_id.to_string());

                    ui.table_next_column();
                    ui.text(item.is_new.to_string());
                },
            );
        });

        let multiplay_key_items = self
            .items_data
            .multiplay_key_entries()
            .iter()
            .non_empty()
            .collect::<Vec<_>>();
        let label = format!(
            "Multiplay Key Items ({}/{})",
            multiplay_key_items.len(),
            self.items_data.multiplay_key_items_capacity
        );
        ui.header(&label, || {
            ui.table(
                "equip-inventory-data-multiplay-key-items",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Gaitem Handle"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Quantity"),
                    TableColumnSetup::new("Display ID"),
                ],
                multiplay_key_items.iter(),
                |ui, index, item| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(item.gaitem_handle.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", item.item_id));

                    ui.table_next_column();
                    ui.text(item.quantity.to_string());

                    ui.table_next_column();
                    ui.text(item.sort_id.to_string());
                },
            );
        });
    }
}

impl StatefulDebugDisplay for ChrIns {
    type State = ChrInsState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        match ChrInsSubclassMut::from(self) {
            ChrInsSubclassMut::PlayerIns(player) => player.render_debug_mut(ui, state),
            mut chr_ins => chr_ins_common_debug(chr_ins.superclass_mut(), ui, state),
        }
    }
}

fn chr_ins_common_debug(chr_ins: &mut ChrIns, ui: &Ui, state: &mut ChrInsState) {
    ui.display("Team", chr_ins.team_type);
    ui.debug("Chr Type", chr_ins.chr_type);
    ui.display("Field Ins Handle", chr_ins.field_ins_handle);
    ui.display("P2P Entity Handle", &chr_ins.p2p_entity_handle);

    ui.display("Block ID", chr_ins.block_id);
    ui.display("Block ID Override", chr_ins.block_id_override);
    ui.display("Block ID Origin", chr_ins.block_origin);
    ui.display("Block ID Origin Override", chr_ins.block_origin_override);

    ui.nested("Chunk Position", chr_ins.chunk_position);
    ui.nested("Initial Position", chr_ins.initial_position);
    ui.nested("Initial Orientation", chr_ins.initial_orientation_euler);

    ui.display("Last hit by", chr_ins.last_hit_by);
    ui.debug("TAE use item", chr_ins.tae_queued_use_item);

    ui.header("Special Effect", || {
        ui.input_text("", &mut state.new_speffect).build();
        ui.same_line();
        let id = i32::from_str(&state.new_speffect);
        ui.disabled(id.is_err(), || {
            if ui.button("Apply") {
                chr_ins.apply_speffect(id.unwrap(), false);
            }
        });

        let mut remove = None;
        ui.table(
            "chr-ins-special-effects",
            [
                TableColumnSetup::new("ID"),
                TableColumnSetup::new("Timer"),
                TableColumnSetup::new("Removal timer"),
                TableColumnSetup::new("Duration"),
                TableColumnSetup::new("Interval Timer"),
                TableColumnSetup::new(""),
            ],
            chr_ins.special_effect.entries(),
            |ui, _i, entry| {
                ui.table_next_column();
                ui.text(format!("{}", entry.param_id));

                ui.table_next_column();
                ui.text(format!("{}", entry.interval_timer));

                ui.table_next_column();
                ui.text(format!("{}", entry.removal_timer));

                ui.table_next_column();
                ui.text(format!("{}", entry.duration));

                ui.table_next_column();
                ui.text(format!("{}", entry.interval_timer));

                ui.table_next_column();
                if ui.button("Remove") {
                    // We can't directly call `chr_ins.remove_speffect` here
                    // because `chr_ins` is already borrowed for the iteration.
                    remove = Some(entry.param_id);
                }
            },
        );

        if let Some(speffect) = remove {
            chr_ins.remove_speffect(speffect);
        }
    });

    ui.nested("Modules", &chr_ins.module_container);
}

impl DebugDisplay for ChrInsModuleContainer {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("Physics", &self.physics);
        ui.nested("Behavior Data", &self.behavior_data);
        ui.nested("Model param modifier", &self.model_param_modifier);
        ui.nested("Ladder", &self.ladder);
        ui.nested("Time Act", &self.time_act);
        ui.nested("Ride", &self.ride);
    }
}

impl DebugDisplay for CSChrLadderModule {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Ladder handle: {:?}", self.ladder_handle));
        ui.text(format!("State: {:?}", self.state));
        ui.text(format!("Top: {:?}", self.top));
        ui.text(format!("Bottom: {:?}", self.bottom));
    }
}

impl DebugDisplay for CSChrPhysicsModule {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Position", self.position);
        ui.display("Orientation", self.orientation);
        ui.nested("Physics material", unsafe {
            self.slide_info.material_info.as_ref()
        });
    }
}

impl DebugDisplay for CSChrBehaviorDataModule {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Has twist modifier", self.has_twist_modifier);
        ui.display("Fixed rotation direction", self.fixed_rotation_direction);
        ui.display("Min twist rank", self.min_twist_rank);
        ui.display("HKS root motion multiplier", self.hks_root_motion_mult);
        ui.display("Turn speed", self.turn_speed);
        ui.display(
            "HKS animation speed multiplier",
            self.hks_animation_speed_multiplier,
        );

        ui.header("Twist modifiers", || {
            ui.table(
                "behavior-data-twist-modifiers",
                [
                    TableColumnSetup::new("ID"),
                    TableColumnSetup::new("Target"),
                    TableColumnSetup::new("Rank"),
                    TableColumnSetup::new("Limits (U/D/L/R)"),
                    TableColumnSetup::new("Minimums (U/D/L/R)"),
                ],
                self.twist_modifiers.iter(),
                |ui, _i, modifier| {
                    ui.table_next_column();
                    ui.text(modifier.modifier_id.to_string());

                    ui.table_next_column();
                    ui.text(modifier.target_type.to_string());

                    ui.table_next_column();
                    ui.text(modifier.rank.to_string());

                    ui.table_next_column();
                    ui.text(format!(
                        "{:.2}/{:.2}/{:.2}/{:.2}",
                        modifier.up_limit_angle,
                        modifier.down_limit_angle,
                        modifier.left_limit_angle,
                        modifier.right_limit_angle
                    ));

                    ui.table_next_column();
                    ui.text(format!(
                        "{:.2}/{:.2}/{:.2}/{:.2}",
                        modifier.up_minimum_angle,
                        modifier.down_minimum_angle,
                        modifier.left_minimum_angle,
                        modifier.right_minimum_angle
                    ));
                },
            );
        });
    }
}

impl DebugDisplay for ChrPhysicsMaterialInfo {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Ground normal vector", self.normal_vector);
    }
}

impl DebugDisplay for CSChrModelParamModifierModule {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "chr-ins-model-param-modifier",
            [TableColumnSetup::new("Name")],
            self.modifiers.items().iter(),
            |ui, _i, modifier| {
                ui.table_next_column();
                ui.text(unsafe { modifier.name.to_string() }.unwrap());
            },
        );
    }
}

impl DebugDisplay for CSChrTimeActModule {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "chr-ins-time-act-module",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("Anim ID"),
                TableColumnSetup::new("Play Time"),
                TableColumnSetup::new("Length"),
            ],
            self.anim_queue.iter(),
            |ui, index, entry| {
                ui.table_next_column();
                ui.text(index.to_string());

                ui.table_next_column();
                ui.text(entry.anim_id.to_string());

                ui.table_next_column();
                ui.text(entry.play_time.to_string());

                ui.table_next_column();
                ui.text(entry.anim_length.to_string());
            },
        );
        ui.display("Read IDX", self.read_idx);
        ui.display("Write IDX", self.write_idx);
        ui.header("Current Anim Info", || {
            let current_anim_info = &self.anim_queue[self.read_idx as usize];
            ui.display("Anim ID", current_anim_info.anim_id);
            ui.display("Play Time", current_anim_info.play_time);
            ui.display("Anim Length", current_anim_info.anim_length);
        });
    }
}

impl DebugDisplay for CSChrRideModule {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("CSRideNode", &self.ride_node);
        ui.debug("Last mounted", self.last_mounted);
        ui.display("Has ride param", self.has_ride_param);
        ui.display("Is ridden character", self.is_ride_character);
        ui.display("Mount rotation", self.mount_data.rotation);
        ui.display("Mount position", self.mount_data.mount_position);
        ui.display("Mount velocity", self.mount_data.velocity);
        ui.display("Attack direction", self.mount_data.attack_direction);
        ui.display(
            "Attack received damage type",
            self.mount_data.received_damage_type,
        );
        ui.display("Mount health", self.mount_data.mount_health);
        ui.display("Fall height", self.mount_data.fall_height);
        ui.display(
            "Is touching solid ground",
            self.mount_data.is_touching_solid_ground,
        );
        ui.display("Is falling", self.mount_data.is_falling);
        ui.display("Is sliding", self.mount_data.is_sliding);
        ui.display("Is mounting", self.is_mounting);
        ui.display("Is mounted", self.is_mounted);
    }
}

impl DebugDisplay for CSPairAnimNode {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Counter party", self.counter_party);
        ui.display("Start position", self.start_position);
        ui.display("Start rotation", self.start_rotation);
    }
}

impl DebugDisplay for CSRideNode {
    fn render_debug(&self, ui: &Ui) {
        self.pair_anim_node.render_debug(ui);
        ui.display("Ride state", self.ride_state);
        ui.display("Ride param ID", self.ride_param_id);
        ui.display("Camera mount control", self.camera_mount_control);
    }
}
