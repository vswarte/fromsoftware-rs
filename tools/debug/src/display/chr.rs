use eldenring::cs::{
    CSChrModelParamModifierModule, CSChrPhysicsModule, CSChrTimeActModule, ChrAsm,
    ChrAsmEquipEntries, ChrAsmEquipment, ChrAsmSlot, ChrIns, ChrInsModuleContainer, ChrInsSubclass,
    EquipGameData, EquipInventoryData, EquipItemData, EquipMagicData, ItemReplenishStateTracker,
    PlayerGameData, PlayerIns,
};
use fromsoftware_shared::NonEmptyIteratorExt;
use hudhook::imgui::{TableColumnSetup, Ui};

use super::{DebugDisplay, UiExt};

impl DebugDisplay for PlayerIns {
    fn render_debug(&self, ui: &Ui) {
        chr_ins_common_debug(&self.chr_ins, ui);

        ui.header("ChrAsm", || {
            self.chr_asm.render_debug(ui);
        });

        ui.header("PlayerGameData", || {
            self.player_game_data.render_debug(ui);
        });

        ui.header("Session Player Entry", || {
            self.session_manager_player_entry.as_ref().render_debug(ui);
        });

        ui.text(format!(
            "Invincibility timer: {}",
            self.invincibility_timer_for_net_player
        ));
        ui.text(format!("Locked on enemy: {}", self.locked_on_enemy));
        ui.text(format!("Block position: {}", self.block_position));
    }
}

impl DebugDisplay for ChrAsm {
    fn render_debug(&self, ui: &Ui) {
        ui.header("ChrAsmEquipment", || {
            self.equipment.render_debug(ui);
        });
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
                        Err(err) => {
                            ui.text(err.to_string());
                        }
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
                        Err(err) => {
                            ui.text(err.to_string());
                        }
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
        ui.text(format!("Arm style: {:?}", self.arm_style));
        ui.text(format!(
            "Left-hand weapon slot: {:?}",
            self.selected_slots.left_weapon_slot
        ));
        ui.text(format!(
            "Right-hand weapon slot: {:?}",
            self.selected_slots.right_weapon_slot
        ));
        ui.text(format!(
            "Left-hand arrow slot: {:?}",
            self.selected_slots.left_arrow_slot
        ));
        ui.text(format!(
            "Right-hand arrow slot: {:?}",
            self.selected_slots.right_arrow_slot
        ));
        ui.text(format!(
            "Left-hand bolt slot: {:?}",
            self.selected_slots.left_bolt_slot
        ));
        ui.text(format!(
            "Right-hand bolt slot: {:?}",
            self.selected_slots.right_bolt_slot
        ));
    }
}

impl DebugDisplay for ChrAsmEquipEntries {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!(
            "Primary Left weapon: {:?}",
            self.weapon_primary_left.param_id()
        ));
        ui.text(format!(
            "Primary Right weapon: {:?}",
            self.weapon_primary_right.param_id()
        ));
        ui.text(format!(
            "Secondary Left weapon: {:?}",
            self.weapon_secondary_left.param_id()
        ));
        ui.text(format!(
            "Secondary Right weapon: {:?}",
            self.weapon_secondary_right.param_id()
        ));
        ui.text(format!(
            "Tertiary Left weapon: {:?}",
            self.weapon_tertiary_left.param_id()
        ));
        ui.text(format!(
            "Tertiary Right weapon: {:?}",
            self.weapon_tertiary_right.param_id()
        ));

        ui.text(format!(
            "Primary Left arrow: {:?}",
            self.arrow_primary.param_id()
        ));
        ui.text(format!(
            "Primary Left bolt: {:?}",
            self.bolt_primary.param_id()
        ));
        ui.text(format!(
            "Secondary Left arrow: {:?}",
            self.arrow_secondary.param_id()
        ));
        ui.text(format!(
            "Secondary Left bolt: {:?}",
            self.bolt_secondary.param_id()
        ));
        ui.text(format!(
            "Tertiary Left arrow: {:?}",
            self.arrow_tertiary.param_id()
        ));
        ui.text(format!(
            "Tertiary Left bolt: {:?}",
            self.bolt_tertiary.param_id()
        ));

        ui.text(format!(
            "Protector Head: {:?}",
            self.protector_head.param_id()
        ));
        ui.text(format!(
            "Protector Chest: {:?}",
            self.protector_chest.param_id()
        ));
        ui.text(format!(
            "Protector Hands: {:?}",
            self.protector_hands.param_id()
        ));
        ui.text(format!(
            "Protector Legs: {:?}",
            self.protector_legs.param_id()
        ));

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
        ui.text(format!("Player ID: {}", self.player_id));
        ui.text(format!(
            "Furlcalling Finger Active: {:?}",
            self.furlcalling_finger_remedy_active
        ));
        ui.text(format!("Rune Arc Active: {:?}", self.rune_arc_active));
        ui.text(format!("White Ring Active: {:?}", self.white_ring_active));
        ui.text(format!("Blue Ring Active: {:?}", self.blue_ring_active));

        ui.text(format!("Character Event ID: {:?}", self.character_event_id));
        ui.text(format!("Character Type: {:?}", self.chr_type));
        ui.text(format!("Multiplay Role: {:?}", self.multiplay_role));

        ui.header("EquipGameData", || {
            self.equipment.render_debug(ui);
        });

        ui.header("Storage Box EquipInventoryData", || {
            self.storage.render_debug(ui);
        });
    }
}

impl DebugDisplay for EquipGameData {
    fn render_debug(&self, ui: &Ui) {
        ui.header("EquipInventoryData", || {
            self.equip_inventory_data.render_debug(ui);
        });

        ui.header("EquipMagicData", || {
            self.equip_magic_data.render_debug(ui);
        });

        ui.header("EquipItemData", || {
            self.equip_item_data.render_debug(ui);
        });

        ui.header("Item Replenish State Tracker", || {
            ui.table(
                "equip-game-data-item-replenish-state-tracker",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Item ID"),
                    TableColumnSetup::new("Auto Replenish"),
                ],
                self.item_replenish_state_tracker.entries().iter(),
                |ui, index, item| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", item.item_id));

                    ui.table_next_column();
                    ui.text(item.auto_replenish.to_string());
                },
            );
        });
        self.item_replenish_state_tracker.render_debug(ui);
    }
}

impl DebugDisplay for ItemReplenishStateTracker {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Item Replenish State Entries", || {
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
        });
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
        ui.text(format!("Selected quick slot: {}", self.selected_quick_slot));

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

        ui.header("Equipment Entries", || {
            self.equip_entries.render_debug(ui);
        });

        ui.text(format!("Selected Quick Slot: {}", self.selected_quick_slot));
    }
}

impl DebugDisplay for EquipInventoryData {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!(
            "Total item entry count: {}",
            self.total_item_entry_count
        ));

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

impl DebugDisplay for ChrIns {
    fn render_debug(&self, ui: &Ui) {
        match ChrInsSubclass::from(self) {
            ChrInsSubclass::PlayerIns(player) => player.render_debug(ui),
            _ => chr_ins_common_debug(self, ui),
        }
    }
}

fn chr_ins_common_debug(chr_ins: &ChrIns, ui: &Ui) {
    ui.text(format!("Team: {}", chr_ins.team_type));
    ui.text(format!("Chr Type: {:?}", chr_ins.chr_type));
    ui.text(format!("Field Ins Handle: {}", chr_ins.field_ins_handle));
    ui.text(format!("P2P Entity Handle: {}", chr_ins.p2p_entity_handle));

    ui.text(format!("Block ID: {}", chr_ins.block_id));
    ui.text(format!("Block ID Override: {}", chr_ins.block_id_override));
    ui.text(format!("Block ID Origin: {}", chr_ins.block_origin));
    ui.text(format!(
        "Block ID Origin Override: {}",
        chr_ins.block_origin_override
    ));

    ui.header("Chunk Position", || {
        chr_ins.chunk_position.render_debug(ui);
    });

    ui.header("Initial Position", || {
        chr_ins.initial_position.render_debug(ui);
    });
    ui.header("Initial Orientation", || {
        chr_ins.initial_orientation_euler.render_debug(ui);
    });

    ui.text(format!("Last hit by: {}", chr_ins.last_hit_by));
    ui.text(format!("TAE use item: {:?}", chr_ins.tae_queued_use_item));

    ui.header("Special Effect", || {
        ui.table(
            "chr-ins-special-effects",
            [
                TableColumnSetup::new("ID"),
                TableColumnSetup::new("Timer"),
                TableColumnSetup::new("Removal timer"),
                TableColumnSetup::new("Duration"),
                TableColumnSetup::new("Interval Timer"),
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
            },
        );
    });

    ui.header("Modules", || {
        chr_ins.module_container.render_debug(ui);
    });
}

impl DebugDisplay for ChrInsModuleContainer {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Physics", || {
            self.physics.render_debug(ui);
        });

        ui.header("Model param modifier", || {
            self.model_param_modifier.render_debug(ui);
        });

        ui.header("Time Act", || {
            self.time_act.render_debug(ui);
        });
    }
}

impl DebugDisplay for CSChrPhysicsModule {
    fn render_debug(&self, ui: &Ui) {
        ui.text(format!("Position: {}", self.position));
        ui.text(format!("Orientation: {}", self.orientation));
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
        ui.text(format!("Read IDX: {}", self.read_idx));
        ui.text(format!("Write IDX: {}", self.write_idx));
        ui.header("Current Anim Info", || {
            let current_anim_info = &self.anim_queue[self.read_idx as usize];
            ui.text(format!("Anim ID: {}", current_anim_info.anim_id));
            ui.text(format!("Play Time: {}", current_anim_info.play_time));
            ui.text(format!("Anim Length: {}", current_anim_info.anim_length));
        });
    }
}
