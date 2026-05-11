use crate::display::{DebugDisplay, DisplayUiExt};
use debug::UiExt;
use eldenring::cs::{
    ActionRequestQueue, ActionTimers, CSChrActionRequestModule, ChrActions, MovementRequestFlags,
    TaeCancelFlags,
};
use hudhook::imgui::{TableColumnSetup, Ui};

impl DebugDisplay for CSChrActionRequestModule {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("Action Requests", self.action_requests);
        ui.nested("Previous Action Requests", self.previous_action_requests);
        ui.nested("New Action Presses", self.new_action_presses);
        ui.nested("Released Actions", self.released_actions);
        ui.nested("Cancel Ready Actions", self.cancel_ready_actions);
        ui.nested("Queued Action Inputs", self.queued_action_inputs);
        ui.nested("Disabled Action Inputs", self.disabled_action_inputs);
        ui.nested("Possible Action Inputs", self.possible_action_inputs);
        ui.nested("Possible Action Cancels", self.possible_action_cancels);
        ui.nested(
            "Prev Possible Action Inputs",
            self.prev_possible_action_inputs,
        );
        ui.nested("Action Request Queue", &self.action_request_queue);
        ui.nested("Action Timers", &self.action_timers);
        ui.display("Movement Request Duration", self.movement_request_duration);
        ui.display("NPC Action ID", self.npc_action_id);
        ui.display("Requested Gesture", self.requested_gesture);
        ui.nested("Movement Request Flags", self.movement_request_flags);
        ui.nested("TAE Cancel Flags", self.tae_cancels);

        ui.header("Readback", || {
            ui.nested("New Presses", self.readback_new_presses);
            ui.nested("Cancel Ready", self.readback_cancel_ready);
            ui.nested("Queued Inputs", self.readback_queued_inputs);
            ui.nested("Possible Inputs", self.readback_possible_inputs);
            ui.nested("Possible Cancels", self.readback_possible_cancels);
            ui.display("NPC Action ID", self.readback_npc_action_id);
        });

        ui.display("Queue Mode Enabled", self.queue_mode_enabled);
        ui.display("Queue Index Override", self.queue_index_override);
    }
}

impl DebugDisplay for ActionRequestQueue {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Current Index", self.current_index);
        ui.header("Input Entries", || {
            ui.table(
                "action-request-queue-inputs",
                [
                    TableColumnSetup::new("State Index"),
                    TableColumnSetup::new("Actions"),
                ],
                self.input_entries.items(),
                |ui, _i, entry| {
                    ui.table_next_column();
                    ui.text(entry.state_index.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", entry.actions));
                },
            );
        });
        ui.header("Cancel Entries", || {
            ui.table(
                "action-request-queue-cancels",
                [
                    TableColumnSetup::new("State Index"),
                    TableColumnSetup::new("Actions"),
                ],
                self.cancel_entries.items(),
                |ui, _i, entry| {
                    ui.table_next_column();
                    ui.text(entry.state_index.to_string());

                    ui.table_next_column();
                    ui.text(format!("{:?}", entry.actions));
                },
            );
        });
    }
}

impl DebugDisplay for ActionTimers {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "action-timers",
            [
                TableColumnSetup::new("Action"),
                TableColumnSetup::new("Duration (s)"),
            ],
            [
                ("R1 (Main Light)", self.r1),
                ("R2 (Main Heavy)", self.r2),
                ("L1 (Off Light)", self.l1),
                ("L2 (Off Heavy)", self.l2),
                ("Action", self.action),
                ("Roll/Backstep", self.roll),
                ("Jump", self.jump),
                ("Use Item", self.use_item),
                ("Switch Spell", self.switch_spell),
                ("Change Weapon R", self.change_weapon_r),
                ("Change Weapon L", self.change_weapon_l),
                ("Change Item", self.change_item),
                ("R3 (Lock On)", self.r3),
                ("L3 (Crouch)", self.l3),
                ("Touch R", self.touch_r),
                ("Touch L", self.touch_l),
            ]
            .iter(),
            |ui, _i, (name, duration)| {
                ui.table_next_column();
                ui.text(name);

                ui.table_next_column();
                ui.text(format!("{:.3}", duration));
            },
        );
    }
}

impl DebugDisplay for ChrActions {
    fn render_debug(&self, ui: &Ui) {
        type ActionEntry = (&'static str, fn(&ChrActions) -> bool);

        const ACTIONS: &[ActionEntry] = &[
            ("R1", ChrActions::r1),
            ("R2", ChrActions::r2),
            ("L1", ChrActions::l1),
            ("L2", ChrActions::l2),
            ("Action", ChrActions::action),
            ("SP Move", ChrActions::sp_move),
            ("Jump", ChrActions::jump),
            ("Use Item", ChrActions::use_item),
            ("Switch Form", ChrActions::switch_form),
            ("Change Weapon R", ChrActions::change_weapon_r),
            ("Change Weapon L", ChrActions::change_weapon_l),
            ("Change Item", ChrActions::change_item),
            ("R3", ChrActions::r3),
            ("L3", ChrActions::l3),
            ("Touch R", ChrActions::touch_r),
            ("Touch L", ChrActions::touch_l),
            ("Backstep", ChrActions::backstep),
            ("Rolling", ChrActions::rolling),
            ("Magic R", ChrActions::magic_r),
            ("Magic L", ChrActions::magic_l),
            ("Gesture", ChrActions::gesture),
            ("Ladder Up", ChrActions::ladderup),
            ("Ladder Down", ChrActions::ladderdown),
            ("Guard", ChrActions::guard),
            ("Emergency Step", ChrActions::emergencystep),
            ("Light Kick", ChrActions::light_kick),
            ("Heavy Kick", ChrActions::heavy_kick),
            ("Change Style R", ChrActions::change_style_r),
            ("Change Style L", ChrActions::change_style_l),
            ("Ride On", ChrActions::rideon),
            ("Ride Off", ChrActions::rideoff),
            ("Buddy Disappear", ChrActions::buddy_disappear),
            ("Magic R2", ChrActions::magic_r2),
            ("Magic L2", ChrActions::magic_l2),
        ];

        const QUEUE_BITS: &[ActionEntry] = &[
            ("Movement", ChrActions::movement),
            ("Movement Prev", ChrActions::movement_prev),
            ("Slot Switch", ChrActions::slot_switch),
        ];

        ui.table(
            "chr-actions",
            [
                TableColumnSetup::new("Action"),
                TableColumnSetup::new("Active"),
            ],
            ACTIONS.iter(),
            |ui, _i, (name, getter)| {
                ui.table_next_column();
                ui.text(name);

                ui.table_next_column();
                ui.text(if getter(self) { "YES" } else { "-" });
            },
        );

        ui.header("Queue Bits", || {
            ui.table(
                "chr-actions-queue-bits",
                [
                    TableColumnSetup::new("Bit"),
                    TableColumnSetup::new("Active"),
                ],
                QUEUE_BITS.iter(),
                |ui, _i, (name, getter)| {
                    ui.table_next_column();
                    ui.text(name);

                    ui.table_next_column();
                    ui.text(if getter(self) { "YES" } else { "-" });
                },
            );
        });
    }
}

impl DebugDisplay for MovementRequestFlags {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Raw Input", self.raw_input());
        ui.display("Cancel Eligible", self.cancel_eligible());
        ui.display("Dash", self.dash());
    }
}

impl DebugDisplay for TaeCancelFlags {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Input Queue Flush", self.input_queue_flush());
        ui.display("Movement Cancel", self.movement_cancel());
        ui.display("Movement Cancel Prev", self.movement_cancel_prev());
        ui.display("Slot Switch", self.slot_switch());
        ui.display("RH Attack", self.rh_attack());
        ui.display("AI Attack Queued", self.ai_attack_queued());
        ui.display("AI Cancel Step", self.ai_cancel_step());
        ui.display("Action General", self.action_general());
        ui.display("Falling/Jump Frames", self.falling_jump_frames());
        ui.display("Cancel Disable", self.cancel_disable());
    }
}
