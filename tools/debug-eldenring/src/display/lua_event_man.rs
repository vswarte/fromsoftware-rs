use debug::UiExt;
use eldenring::cs::{
    CSDeathRestartEvent, CSLuaEventCondition, CSLuaEventManImp, CSLuaEventMsgExec_Func,
    CSLuaEventMsgExec_String, CSLuaEventMsgMap, CSLuaEventObserver, CSLuaEventProxy,
    CSLuaEventScriptImitation, EventMsgExecListEntry, LuaEventControlFlags, LuaEventId,
};

use fromsoftware_shared::{Program, Superclass, vftable_classname};
use hudhook::imgui::{TableColumnSetup, Ui};

use crate::display::DisplayUiExt;

use super::DebugDisplay;

impl DebugDisplay for CSLuaEventManImp {
    fn render_debug(&self, ui: &Ui) {
        ui.nested("Lua Event Observer", &self.lua_event_observer);
        ui.nested("Lua Event Proxy", &self.lua_event_proxy);
        ui.nested_opt(
            "Lua Event Script Imitation",
            self.lua_event_script_imitation.as_ref(),
        );
    }
}

impl DebugDisplay for CSLuaEventProxy {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Is net message", self.is_net_message);
        ui.display("Disable event networking", self.disable_event_networking);
        ui.display("Is repeat message", self.is_repeat_message);
        ui.display("Is load-wait", self.is_load_wait);
        ui.display("Is lobby-state-client", self.is_lobby_state_client);
        ui.debug("Summon param type", self.summon_param_type);
        ui.nested("Control flags", self.control_flags);
        ui.nested("Event message map", &self.lua_event_msg_map);
    }
}

impl DebugDisplay for CSLuaEventObserver {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            "Observed conditions",
            self.lua_event_observees.iter(),
            |ui, _i, cond| {
                let class_name =
                    vftable_classname(&Program::current(), *cond.vftable as *const _ as usize)
                        .unwrap_or("Unknown class".to_string());
                let header_text =
                    if let Ok(lua_event_id) = LuaEventId::try_from(cond.condition_id as u32) {
                        format!(
                            "Condition {} ({:?}) [{}]",
                            cond.condition_id, lua_event_id, class_name
                        )
                    } else {
                        format!("Condition {} [{}]", cond.condition_id, class_name)
                    };
                ui.header(&header_text, || {
                    cond.render_debug(ui);
                });
            },
        );

        ui.list(
            "Bonfire observees",
            self.bonfire_event_observees.iter(),
            |ui, _i, cond| {
                ui.header(format!("Bonfire condition {}", cond.condition_id), || {
                    cond.render_debug(ui);
                });
            },
        );
    }
}

impl DebugDisplay for CSLuaEventCondition {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Condition id", self.condition_id);
        if let Ok(lua_event_id) = LuaEventId::try_from(self.condition_id as u32) {
            ui.same_line();
            ui.text(format!(" (LuaEventId: {:?})", lua_event_id));
        }
        ui.display("Event group", self.event_group);
        ui.display("Stop execution", self.stop_execution);
        ui.display("Is deleted", self.is_deleted);
        ui.debug("Execute repetition", self.execute_repetition);
    }
}

impl DebugDisplay for CSLuaEventMsgMap {
    fn render_debug(&self, ui: &Ui) {
        ui.list(
            "EventMsgExec entries",
            self.event_msg_exec_list.iter(),
            |ui, _i, entry| {
                ui.header(format!("Group {}", entry.event_group), || {
                    entry.render_debug(ui);
                });
            },
        );

        ui.list(
            "Deferred EventMsgExec entries",
            self.deferred_event_exec_list.iter(),
            |ui, _i, entry| {
                ui.header(format!("Group {}", entry.event_group), || {
                    entry.render_debug(ui);
                });
            },
        );
    }
}

impl DebugDisplay for EventMsgExecListEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Event group", self.event_group);
        ui.display("Arg1", self.arg1);
        if let Ok(lua_event_id) = LuaEventId::try_from(self.arg1) {
            ui.same_line();
            ui.text(format!(" (LuaEventId: {:?})", lua_event_id));
        }
        ui.display("Arg2", self.arg2);
        ui.display("Arg3", self.arg3);
        ui.debug("Repetition", self.repetition);
        ui.display("Is net message", self.is_net_message);
        ui.display("Is repeat message", self.is_repeat_message);
        ui.display("Is deleted", self.is_deleted);

        if let Some(subclass) = self
            .lua_event_msg_exec
            .as_subclass::<CSLuaEventMsgExec_Func>()
        {
            ui.header("Executor: Function", || {
                subclass.render_debug(ui);
            });
        } else if let Some(subclass) = self
            .lua_event_msg_exec
            .as_subclass::<CSLuaEventMsgExec_String>()
        {
            ui.header("Executor: Script string", || {
                subclass.render_debug(ui);
            });
        } else {
            ui.text("Executor (unknown type)");
        }
    }
}

impl DebugDisplay for CSLuaEventMsgExec_Func {
    fn render_debug(&self, ui: &Ui) {
        ui.text("CSLuaEventMsgExec_Func");
    }
}

impl DebugDisplay for CSLuaEventMsgExec_String {
    fn render_debug(&self, ui: &Ui) {
        ui.text("CSLuaEventMsgExec_String");
        ui.display("Event message", &self.event_msg);
    }
}

impl DebugDisplay for CSLuaEventScriptImitation {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Clear boss id", self.clear_boss_id);
        ui.display("Is kill host", self.is_kill_host);
        ui.debug("Death state", self.death_state);
        ui.display("Is death-penalty-skip", self.is_death_penalty_skip);
        ui.display("Should reset world", self.should_reset_world);
        ui.display("Should reset character", self.should_reset_character);
        ui.display("World reset delay", self.world_reset_delay);
        ui.display(
            "Bonfire animation id offset",
            self.bonfire_animation_id_offset,
        );
        ui.display("Bonfire entity id", self.bonfire_entity_id);
        ui.nested("CSDeathRestartEvent", &self.death_restart_event);
    }
}

impl DebugDisplay for CSDeathRestartEvent {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Full screen message id", self.full_screen_message_id);
        ui.display("Character event id", self.character_event_id);
    }
}

impl DebugDisplay for LuaEventControlFlags {
    fn render_debug(&self, ui: &Ui) {
        ui.table(
            "lua-event-control-flags",
            [
                TableColumnSetup::new("Flag"),
                TableColumnSetup::new("Value"),
            ],
            [
                ("Pause reload events", self.pause_reload_events()),
                ("Pause self death events", self.pause_self_death_event()),
                ("Pause disconnect events", self.pause_disconnect_event()),
                ("Was alive at block clear", self.was_alive_at_block_clear()),
                ("Red hunt active", self.red_hunt_active()),
                (
                    "Bonfire loop begin requested",
                    self.bonfire_loop_begin_requested(),
                ),
                ("Bonfire end pending", self.bonfire_end_pending()),
                (
                    "Bonfire sitting loop active",
                    self.bonfire_sitting_loop_active(),
                ),
                (
                    "Bonfire stand up in progress",
                    self.bonfire_stand_up_in_progress(),
                ),
                ("Pause player leave event", self.pause_player_leave_event()),
                ("Notified of block clear", self.notified_of_block_clear()),
                ("Return title requested", self.return_title_requested()),
                ("Arena local player dead", self.arena_local_player_dead()),
                ("Arena death restart flag", self.arena_death_restart_flag()),
                (
                    "Arena death restart kickout",
                    self.arena_death_restart_kickout(),
                ),
                (
                    "Arena death restart pending",
                    self.arena_death_restart_pending(),
                ),
                ("Ceremony restart pending", self.ceremony_restart_pending()),
            ],
            |ui, _i, (name, value)| {
                ui.table_next_column();
                ui.text(name);
                ui.table_next_column();
                ui.text(value.to_string());
            },
        );
    }
}
