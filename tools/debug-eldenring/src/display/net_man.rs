use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{
    BreakInAreaList, BreakInData, BreakInManager, BreakInTarget, CSBattleRoyalContext,
    CSNetBloodMessageDb, CSNetBloodMessageDbItem, CSNetMan, CSQuickMatchContext,
    CSQuickMatchingCtrl, QuickMatchBattleSessiontData, QuickmatchManager,
    QuickmatchManagerDebugSettings, QuickmatchParticipant, QuickmatchSpawnData,
};

use super::{DebugDisplay, DisplayUiExt};

impl DebugDisplay for CSNetMan {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("NAT Type", self.nat_type);
        ui.debug("Session NAT Type", self.session_nat_type);
        ui.display("Disable Multiplay", self.disable_multiplay);
        ui.display("Low FPS Penalty", self.low_fps_penalty);
        ui.display("Server Connection Lost", self.server_connection_lost);
        ui.display("Block ID", self.block_id);
        ui.display("Play Region ID", self.play_region_id);
        ui.display("Debug Group Password", self.debug_group_password);

        ui.separator();
        ui.nested("Blood Messages", &self.blood_message_db);
        ui.nested("Break In", &self.breakin_manager);
        ui.nested("Quickmatch", &self.quickmatch_manager);
    }
}

impl DebugDisplay for BreakInManager {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Multiplay Type", self.multiplay_type);

        ui.list("Break In Targets", self.targets.iter(), |ui, _i, item| {
            item.render_debug(ui);
        });

        ui.nested("Break In Data", &self.data);
        ui.debug("Error Code", self.error_code);
        ui.nested("Break In Areas", &self.areas);
        ui.debug("Invasion Search State", self.invasion_search_state);

        ui.debug(
            "Prev Invasion Search State",
            self.last_update_invasion_search_state,
        );

        ui.debug("Attempt Interval Timer", self.attempt_interval_timer.time);
        ui.display("Time Out Timer", self.time_out_timer.time);
        ui.display("Is Yellow Monk", self.is_yellow_costume_region);
        ui.display("Is Multi Region", self.is_multi_region);
    }
}

impl DebugDisplay for BreakInTarget {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Player ID", self.player_id);
        let steam_id_str = self.external_id.to_str().unwrap_or("Invalid");
        ui.debug_copiable("Steam ID", steam_id_str);
        ui.display("Play Region", self.play_region)
    }
}

impl DebugDisplay for BreakInAreaList {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Count", self.count);
        ui.header("Areas", || {
            ui.table(
                "breakin-areas-list",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Area"),
                ],
                self.areas.iter(),
                |ui, i, e| {
                    ui.table_next_column();
                    ui.text(format!("{i}"));

                    ui.table_next_column();
                    ui.text(format!("{e}"));
                },
            );
        });
    }
}

impl DebugDisplay for BreakInData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Block ID", self.block_id);
        ui.nested("Block Position", self.block_pos);
        ui.display("Entry File List Id", self.entryfilelist_id);
        ui.debug("Summon Param Type", self.summon_param_type);
        ui.debug("Multi Play Role", self.multiplay_role);
        ui.display("Has Password", self.has_password);
    }
}

impl DebugDisplay for CSNetBloodMessageDb {
    fn render_debug(&self, ui: &Ui) {
        ui.header("Entries", || {
            render_message_table(self.entries.iter().map(|f| f.as_ref()), ui);
        });

        ui.header("Created message data", || {
            self.created_data
                .iter()
                .for_each(|f| ui.text(format!("{f} {f:x}")));
        });

        ui.header("Discovered messages", || {
            render_message_table(
                self.discovered_messages.iter().map(|f| f.as_ref().as_ref()),
                ui,
            );
        });
    }
}

fn render_message_table<'a>(messages: impl Iterator<Item = &'a CSNetBloodMessageDbItem>, ui: &Ui) {
    ui.table(
        "cs-net-man-blood-messages-entries",
        [
            TableColumnSetup::new("Message ID"),
            TableColumnSetup::new("Map ID"),
            TableColumnSetup::new("Placement (x, y, z, angle)"),
            TableColumnSetup::new("Template 1"),
            TableColumnSetup::new("Part 1"),
            TableColumnSetup::new("Infix"),
            TableColumnSetup::new("Template 2"),
            TableColumnSetup::new("Part 2"),
            TableColumnSetup::new("Gesture"),
        ],
        messages,
        |ui, _i, message| {
            ui.table_next_column();
            ui.text(format!("{:x}", message.message_id));

            ui.table_next_column();
            ui.text(message.block_id.to_string());

            ui.table_next_column();
            ui.text(format!(
                "{}, {}, {}, {}",
                message.position_x, message.position_y, message.position_z, message.angle,
            ));

            ui.table_next_column();
            ui.text(message.template1.to_string());

            ui.table_next_column();
            ui.text(message.part1.to_string());

            ui.table_next_column();
            ui.text(message.infix.to_string());

            ui.table_next_column();
            ui.text(message.template2.to_string());

            ui.table_next_column();
            ui.text(message.part2.to_string());

            ui.table_next_column();
            ui.text(message.gesture_param.to_string());
        },
    );
}

impl DebugDisplay for QuickmatchManager {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("CSQuickMatchingCtrl", &self.quickmatching_ctrl);
        ui.nested("CSBattleRoyalContext", &self.battle_royal_context);
        ui.nested_opt(
            "Active CSBattleRoyalContext",
            self.active_battle_royal_context
                .map(|ptr| unsafe { ptr.as_ref() }),
        );

        ui.separator();
        ui.display("Skip Leave Multiplay Log", self.skip_leave_multiplay_log);
        ui.nested("Battle Session Data", &self.battle_session_data);
        ui.display("My Team Eliminations", self.my_team_eliminations);
        ui.display("Other Team Eliminations", self.other_team_eliminations);
        ui.display("Results Computed", self.results_computed);

        ui.separator();
        ui.display("Character ID", self.character_id);
        ui.display("United Combat Rank", self.quickmatch_united_combat_rank);
        ui.display("Duel Rank", self.quickmatch_duel_rank);
        ui.display("Spirit Ashes Rank", self.quickmatch_spirit_ashes_rank);
        ui.display("United Combat Points", self.quickmatch_united_combat_points);
        ui.display("Duel Points", self.quickmatch_duel_points);
        ui.display("Spirit Ashes Points", self.quickmatch_spirit_ashes_points);

        ui.separator();
        ui.nested("Debug Settings", &self.debug_settings);
    }
}

impl DebugDisplay for QuickMatchBattleSessiontData {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Result", self.result);
        ui.display("Elimination Count", self.elimination_count);
    }
}

impl DebugDisplay for QuickmatchManagerDebugSettings {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Settings", self.settings);
        ui.debug("Venue", self.venue);
        ui.debug("Desired Team", self.desired_team);
        ui.display_copiable("Password", &self.password);
    }
}

impl DebugDisplay for CSBattleRoyalContext {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("Quickmatch Context", &self.quickmatch_context);

        ui.separator();
        ui.display("Max players", self.match_player_count);
        ui.debug("Settings", self.setting);
        ui.display("Current players", self.current_player_count);
        ui.debug("Venue", self.venue);
        ui.display_copiable("Password", &self.password);
        ui.display("Is Fixed Map", self.is_fixed_map);
        ui.display("Is Any Format", self.is_any_format);
        ui.display("Session NAT Type Override", self.session_nat_type_override);
    }
}

impl DebugDisplay for QuickmatchSpawnData {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Block ID", self.block_id);
        ui.nested("Block Position", self.block_position);
        ui.debug("Role", self.role);
    }
}

impl DebugDisplay for CSQuickMatchContext {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Match settings", self.match_settings);
        ui.debug("Match map (arena)", self.match_map);
        ui.nested("Spawn Data", &self.spawn_data);

        ui.header("Arena List", || {
            ui.table(
                "quickmatch-context-arena-list",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("Arena"),
                ],
                self.arena_list.iter(),
                |ui, i, arena| {
                    ui.table_next_column();
                    ui.text(format!("{i}"));
                    ui.table_next_column();
                    ui.text(format!("{arena:?}"));
                },
            );
        });

        ui.display(
            "Join Candidate Stack Count",
            self.join_candidate_stack.len(),
        );
        ui.display(
            "Allied Password Opponent Candidates Count",
            self.allied_password_opponent_candidates.len(),
        );

        ui.list("Participants", self.participants.iter(), |ui, _i, item| {
            item.render_debug(ui);
        });

        ui.separator();
        ui.display("Lobby Search Timed Out", self.lobby_search_timed_out);
        ui.debug("Error State", self.error_state);
        ui.debug("Result Submition State", self.result_submition_state);
        ui.debug("Venue", self.venue);
    }
}

impl DebugDisplay for QuickmatchParticipant {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Player ID", self.player_id);
        let external_id_str = self.external_id.to_str().unwrap_or("Invalid");
        ui.debug_copiable("External ID", external_id_str);
        ui.display("Character ID", self.character_id);
        ui.debug("Ready State", self.ready_state);
        ui.display(
            "Ready State Kickout Timer",
            self.ready_state_kickout_timer.time,
        );
        ui.debug("Team", self.team);
        ui.nested_opt(
            "Player Game Data",
            self.player_game_data.map(|ptr| unsafe { ptr.as_ref() }),
        );
    }
}

impl DebugDisplay for CSQuickMatchingCtrl {
    fn render_debug(&self, ui: &Ui) {
        ui.debug_copiable("Match state", self.stepper.current_state);

        ui.separator();
        ui.display(
            "Guest Research Retry Timer",
            self.guest_research_retry_timer.time,
        );
        ui.display("Can Edit Settings", self.can_edit_settings);
        ui.display("All Participants Ready", self.all_participants_ready);
        ui.display("Move Map Requested", self.move_map_requested);
        ui.display("Local Move Map Ready", self.local_move_map_ready);
        ui.display("Received World Flag Sync", self.received_world_flag_sync);
        ui.display(
            "Checked Session State This Update",
            self.checked_session_state_this_update,
        );
        ui.display("Join Retry Pending", self.join_retry_pending);
        ui.display("Success Full End", self.success_full_end);
        ui.display("Should Start Join", self.should_start_join);
        ui.display("Recompute Lead Requested", self.recompute_lead_requested);
        ui.display("Sent Move Map Ready", self.sent_move_map_ready);
        ui.display("Send Ranking Result", self.send_ranking_result);
        ui.display("Death Unregister", self.death_unregister);
        ui.display("Pause Guest Invites", self.pause_guest_invites);
        ui.display("Sent Desired Team Packet", self.sent_desired_team_packet);
        ui.display(
            "Received Allied Password Team Stagger Packet",
            self.received_allied_password_team_stagger_packet,
        );

        ui.separator();
        ui.display_copiable("Enemy Team Password", &self.enemy_team_password);
        ui.display("Ally Team Eliminations", self.ally_team_elimination_count);
        ui.display("Enemy Team Eliminations", self.enemy_team_elimination_count);

        ui.separator();
        ui.display(
            "Host Registration Update Timer",
            self.host_registration_update_timer.time,
        );
        ui.display(
            "Summon Message Interval Timer",
            self.summon_message_interval_timer.time,
        );
        ui.display("Move Map Timeout Timer", self.move_map_timeout_timer.time);
        ui.display(
            "Wait Session Timeout Timer",
            self.wait_session_timeout_timer.time,
        );
        ui.display(
            "Allied Password Assembly Timeout Time",
            self.allied_password_assembly_timeout_time.time,
        );
        ui.display(
            "Allied Password Team Stagger Timer",
            self.allied_password_team_stagger_timer.time,
        );

        ui.separator();
        ui.display(
            "Allied Password Joined Count Prev",
            self.allied_password_joined_count_prev,
        );
        ui.display(
            "Allied Password Joined Count",
            self.allied_password_joined_count,
        );
        ui.display(
            "Enemy Password Joined Count",
            self.enemy_password_joined_count,
        );
        ui.display("Active State Elapsed Time", self.active_state_elapsed_time);

        let join_target_host_external_id_str = self
            .join_target_host_external_id
            .to_str()
            .unwrap_or("Invalid");
        ui.debug_copiable(
            "Join Target Host External ID",
            join_target_host_external_id_str,
        );

        ui.display(
            "Pause Accepting Join Requests",
            self.pause_accepting_join_requests,
        );
        ui.display("Sent World Enter Packet", self.sent_world_enter_packet);
    }
}
