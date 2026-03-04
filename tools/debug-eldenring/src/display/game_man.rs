use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{GameMan, PartyMemberInfo, PartyMemberInfoEntry};

use crate::display::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for GameMan {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Warp Requested", self.warp_requested);
        ui.display("Save Slot", self.save_slot);
        ui.display("Save State", self.save_state);
        ui.display("Save Requested", self.save_requested);
        ui.display("New Game Plus Requested", self.new_game_plus_requested);
        ui.display("Is In Online Mode", self.is_in_online_mode);
        ui.display("Server Connection Enabled", self.server_connection_enabled);
        ui.display("Is Inactive", self.is_inactive);

        ui.header("Map / Warp", || {
            ui.display("Initial Area Entity ID", self.initial_area_entity_id);
            ui.display("Load Target Block ID", self.load_target_block_id);
            ui.nested("Last Load Position", self.last_load_position);
            ui.nested("Last Load Orientation", self.last_load_orientation);
            ui.display(
                "Requested Save Slot Load Index",
                self.requested_save_slot_load_index,
            );
            ui.display("Disable Map Enter Anim", self.disable_map_enter_anim);
            ui.display("Simple Loading Screen", self.simple_loading_screen);
            ui.display(
                "Sub Area Name Popup Message ID",
                self.sub_area_name_popup_message_id,
            );
            ui.display("Entry File List ID", self.entryfilelist_id);
            ui.display("Target Ceremony", self.target_ceremony);
            ui.display(
                "Ceremony Entry Point Entity ID",
                self.ceremony_entry_point_entity_id,
            );
            ui.display(
                "Item Replenish From Chest Requested",
                self.item_replanish_from_chest_requested,
            );
            ui.display(
                "Item Restore After QM Requested",
                self.item_restore_after_qmrequested,
            );
        });

        ui.header("Camera", || {
            ui.display("Normal Camera Param ID", self.normal_camera_param_id);
            ui.display("Locked Camera Param ID", self.locked_camera_param_id);
            ui.display("Talk ESD Camera Param ID", self.talk_esd_camera_param_id);
            ui.display("Lock On Camera Param ID", self.lock_on_camera_param_id);
            ui.display(
                "Camera Follow Dummy Poly ID",
                self.camera_follow_dummy_poly_id,
            );
            ui.display(
                "Camera Chr Lock On Param ID",
                self.camera_chr_lock_on_param_id,
            );
            ui.display(
                "Camera Zoom Target Dist Mult",
                self.camera_zoom_target_dist_mult,
            );
            ui.display("Cam Override Lerp Factor", self.cam_override_lerp_factor);
            ui.display(
                "Cam Zoom Interpolated Progress",
                self.cam_zoom_interpolated_progress,
            );
            ui.display(
                "Cam Timed Override Duration",
                self.cam_timed_override_duration,
            );
            ui.display(
                "Cam Zoom Override Lerp Factor",
                self.cam_zoom_override_lerp_factor,
            );
            ui.display(
                "Cam Zoom Reset Previous Distance",
                self.cam_zoom_reset_previous_distance,
            );
            ui.display(
                "Cam Override Check Collisions",
                self.cam_override_check_collisions,
            );

            ui.header("Force Camera Direction", || {
                ui.debug("Rotation Method", self.force_cam_rotation_method);
                ui.display("Vertical Angle (rad)", self.force_cam_vertical_angle_rad);
                ui.display(
                    "Horizontal Angle (rad)",
                    self.force_cam_horizontal_angle_rad,
                );
                ui.display(
                    "Interpolation Progress",
                    self.force_cam_interpolation_progress,
                );
                ui.display("Vertical Enabled", self.force_cam_vertical_enabled);
                ui.display("Horizontal Enabled", self.force_cam_horizontal_enabled);
                ui.display("First Execution", self.force_cam_first_execution);
            });
        });

        ui.header("Multiplayer", || {
            ui.debug("Event World Type", self.event_world_type);
            ui.debug("Multiplay Role", self.multiplay_role);
            ui.debug("Summon Param Type", self.summon_param_type);
            ui.display("Has Password", self.has_password);
            ui.display("Character Name Is Empty", self.character_name_is_empty);
            ui.display(
                "Multiplay Join Block Position",
                self.multiplay_join_block_pos,
            );
            ui.nested(
                "Multiplay Join Orientation",
                self.multiplay_join_orientation,
            );

            ui.header("Stay In Multiplay Area", || {
                ui.nested("Saved Position", self.stay_in_multiplay_area_saved_position);
                ui.nested("Saved Rotation", self.stay_in_multiplay_area_saved_rotation);
                ui.display("Saved Block ID", self.stay_in_multiplay_area_saved_block_id);
            });

            ui.nested("Party Member Info", &self.party_member_info);
        });
    }
}

impl DebugDisplay for PartyMemberInfo {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Player Counts", || {
            ui.display("Friendly Phantom Count", self.friendly_phantom_count);
            ui.display("Hostile Phantom Count", self.hostile_phantom_count);
            ui.display(
                "In World Online Player Count",
                self.in_world_online_player_count,
            );
            ui.display("In World Players Count", self.in_world_players_count);
            ui.display("Non NPC Player Count", self.non_npc_player_count);
            ui.display("All Players Count", self.all_players_count);
            ui.display(
                "Session Online Player Count",
                self.session_online_player_count,
            );
        });

        ui.header("Party Members", || {
            for (i, member) in self.party_members.iter().enumerate() {
                ui.header(format!("Member {i}"), || {
                    member.render_debug(ui);
                });
            }
        });

        ui.header("NPC Host Entities", || {
            ui.table(
                "party-member-info-npc-host-entities",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Field Ins Handle"),
                ],
                self.npc_host_entities[..self.npc_host_entity_count as usize].iter(),
                |ui, index, handle| {
                    ui.table_next_column();
                    ui.text(index.to_string());

                    ui.table_next_column();
                    ui.text(handle.to_string());
                },
            );
        });

        ui.header("Pseudo Multiplayer", || {
            ui.debug("Ceremony State", self.pseudo_mp_ceremony_state);
            ui.display("Host Entity ID", self.pseudo_mp_host_entity_id);
            ui.display("Event Flag", self.pseudo_mp_event_flag);
            ui.display(
                "Event Text For Map ID",
                self.pseudo_mp_event_text_for_map_id,
            );
            ui.debug("Summon Param Type", self.summon_param_type);
            ui.display("Network Msg NPC ID", self.pseudo_mp_network_msg_npc_id);
            ui.display("Role Param Override", self.pseudo_mp_role_param_override);
            ui.display(
                "Role Param Override Host",
                self.pseudo_mp_role_param_override_host,
            );
            ui.display(
                "Role Param Override Guest",
                self.pseudo_mp_role_param_override_guest,
            );
            ui.debug("Role Host", self.pseudo_mp_role_host);
            ui.debug("Role Guest", self.pseudo_mp_role_guest);
        });

        ui.display("Needs Update", self.needs_update);
        ui.display("NPC Leave Requested", self.npc_leave_requested);
    }
}

impl DebugDisplay for PartyMemberInfoEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Field Ins Handle", self.field_ins_handle);
        ui.debug("Member Type", self.member_type);
        ui.debug("State", self.state);
        ui.display("Apply Multiplayer Rules", self.apply_multiplayer_rules);
        ui.display(
            "Disconnect Request Delta Time",
            self.disconnect_request_delta_time,
        );

        ui.header("NPC Info", || {
            ui.debug("NPC Chr Type", self.npc_chr_type);
            ui.debug("NPC Multiplay Role", self.npc_multiplay_role);
            ui.display("NPC Name FMG ID", self.npc_name_fmg_id);
            ui.display("NPC Invasion Event Flag", self.npc_invasion_event_flag);
            ui.display("NPC Return Event Flag ID", self.npc_return_event_flag_id);
        });
    }
}
