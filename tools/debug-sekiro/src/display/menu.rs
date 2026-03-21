use hudhook::imgui::*;

use debug::UiExt;
use sekiro::{app_menu::*, sprj::*};

use super::{DebugDisplay, DisplayUiExt, StatefulDebugDisplay};

impl StatefulDebugDisplay for MenuMan {
    type State = ();

    fn render_debug_mut(&mut self, ui: &Ui, _state: &mut Self::State) {
        ui.checkbox("Hide all menus", &mut self.hide_all_menus);

        ui.debug("Menu mode", self.is_menu_mode());

        ui.debug("Move map step number", self.move_map_step_number);
        ui.debug("Move map step update time", self.move_map_step_update_time);

        ui.list("Action Spots", self.action_spots(), |ui, _, spot| {
            spot.render_debug(ui)
        });

        ui.debug("Target site position", self.target_site_position);
        ui.debug("Target site visible", self.target_site_visible);
        ui.debug("Target site force visible", self.target_site_force_visible);
        ui.debug("VFX data position", self.vfx_data_position);
        ui.debug("VFX data visible", self.vfx_data_visible);
        ui.debug("Last talk ID", self.last_talk_id);
        ui.list("Enemy Gauges", self.enemy_gauges(), |ui, _, gauge| {
            gauge.render_debug(ui)
        });

        ui.header("Miniboss Gauge", || {
            ui.debug("Name ID", self.miniboss_gauge_name_id);
            ui.debug("Name ID 2", self.miniboss_gauge_name_id_2);
            ui.debug("My damage", self.miniboss_gauge_my_damage);
            ui.debug("Net damage", self.miniboss_gauge_net_damage);
        });

        ui.header("Boss Gauge", || {
            ui.debug("Name ID", self.boss_gauge_name_id);
            ui.debug("Name ID 2", self.boss_gauge_name_id_2);
            ui.debug("My damage", self.boss_gauge_my_damage);
            ui.debug("Net damage", self.boss_gauge_net_damage);
        });

        ui.header("Likely unused", || {
            ui.checkbox("Draw layout on cursor", &mut self.draw_layout_on_cursor);
            ui.checkbox("Draw layout only last", &mut self.draw_layout_only_last);
            ui.checkbox(
                "Draw layout display position",
                &mut self.draw_layout_display_position,
            );
            ui.checkbox(
                "Draw layout only register",
                &mut self.draw_layout_only_register,
            );
            ui.checkbox("Show action button", &mut self.show_action_button);
            ui.checkbox("Hide FE", &mut self.hide_fe);

            ui.debug("Caption flag", self.caption_flag());
            ui.debug("Selected inventory ID", self.selected_inventory_id());
            ui.debug("Drop ret inventory ID", self.drop_ret_inventory_id());
            ui.debug("Selected inventory slot", self.selected_inventory_slot());
            ui.debug("Inventory page state", self.inventory_page_state());
            ui.debug(
                "Selected sort inventory ID",
                self.selected_sort_inventory_id(),
            );

            ui.debug("Talk message ID", self.talk_message_id);
            ui.debug("Enable talk icon", self.enable_talk_icon);
            ui.debug("Talk shop message ID", self.talk_shop_message_id);
            ui.debug(
                "Map action data invisible time",
                self.map_action_data_invisible_time,
            );

            ui.header("Drop", || {
                ui.debug("Equip type", self.drop_equip_type);
                ui.debug("Equip ID", self.drop_equip_id);
                ui.debug("Durability", self.drop_durability);
                ui.debug("Quantity", self.drop_quantity);
            });

            ui.list(
                "Floating PC Gauges",
                &mut self.floating_pc_gauges,
                |ui, i, gauge| {
                    ui.header(format!("Gauge #{i}"), || {
                        gauge.render_debug_mut(ui, &mut ())
                    });
                },
            );

            ui.nested("Menu Info Data", &self.menu_info_data);

            ui.debug("Shop lineup start ID", self.shop_lineup_start_id);
            ui.debug("Shop lineup end ID", self.shop_lineup_end_id);
            ui.checkbox("Enable shop test", &mut self.enable_shop_test);

            ui.header("Select Equip Info", || {
                ui.debug("Type", self.select_equip_info_type);
                ui.debug("ID", self.select_equip_info_id);
                ui.debug("Durability", self.select_equip_info_durability);
            });

            ui.header("Last Selected", || {
                ui.debug("Top menu item", self.last_selected_top_menu_item);
                ui.debug("Inventory tab", self.last_selected_inventory_tab);
                ui.debug(
                    "Tab scroll position",
                    self.last_selected_tab_scroll_position,
                );
                ui.debug("Inventory item", self.last_selected_inventory_item);
                ui.debug(
                    "Inventory item scroll position",
                    self.last_selected_inventory_item_scroll_position,
                );
                ui.debug("Message tab", self.last_selected_message_tab);
                ui.debug("Message item", self.last_selected_message_item);
                ui.debug(
                    "Message item scroll position",
                    self.last_selected_message_item_scroll_position,
                );
                ui.debug("Message edit mode", self.last_selected_message_edit_mode);
                ui.debug("Option tab", self.last_selected_option_tab);
                ui.debug("Option item", self.last_selected_option_item);
            });

            ui.header("Condition Message IDs", || {
                ui.debug("All", self.condition_message_ids);
                ui.debug("Current", self.current_condition_message_id);
                ui.debug("Next", self.current_condition_message_id);
            });

            ui.debug(
                "Start tab animation play speed",
                self.start_tab_animation_play_speed,
            );

            ui.debug("Current player pad index", self.current_player_pad_index);
            ui.debug("Desired player pad index", self.desired_player_pad_index);
        });

        ui.header("Flags", || {
            ui.table(
                "menu-man-flags",
                [TableColumnSetup::new("ID"), TableColumnSetup::new("Value")],
                self.flags.iter(),
                |ui, i, value| {
                    ui.table_next_column();
                    ui.text(format!("{i}"));

                    ui.table_next_column();
                    ui.text(format!("{value:x}"));
                },
            );
        });
    }
}

impl DebugDisplay for MenuActionSpot {
    fn render_debug(&self, ui: &Ui) {
        ui.header(format!("Action Spot {}", self.unique_id), || {
            ui.debug("Distance", self.distance);
            ui.debug("Max distance", self.max_distance);
            ui.debug("Score", self.score);
            ui.debug("Position", self.position);
            ui.debug("Text ID", self.text_id);
            ui.debug("Time since vanish", self.time_since_vanish);
        });
    }
}

impl StatefulDebugDisplay for FloatingPcGauge {
    type State = ();

    fn render_debug_mut(&mut self, ui: &Ui, _state: &mut Self::State) {
        ui.debug("Position", self.position);
        ui.debug("Visible", self.is_visible);
        ui.debug("Shielded", self.is_shielded);
        ui.debug("Display permanently", self.display_permanently);
        ui.debug("Damage value", self.damage_value);
        ui.debug("Damage display timer", self.damage_value_display_timer);
        ui.checkbox("Force visible", &mut self.force_visible);
    }
}

impl DebugDisplay for MenuInfoData {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Current stack index", self.current_stack_index);
        ui.list("Entries", &self.entries, |ui, i, e| {
            ui.nested(format!("#{i}"), e)
        });
    }
}

impl DebugDisplay for MenuInfoDataEntry {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("System message id", self.system_message_id);
        ui.debug("Is in use", self.is_in_use);
        ui.debug("Use any pad menu", self.use_any_pad_menu);
        ui.debug("Button type", self.button_type);
        ui.debug("User ID", self.user_id);
    }
}

impl DebugDisplay for EnemyGauge {
    fn render_debug(&self, ui: &Ui) {
        ui.header(format!("Enemy Gauge {}", self.handle), || {
            ui.debug("Draw position", self.draw_position);
            ui.debug("Is lock", self.is_lock);
            ui.debug("Is visible", self.is_visible);
            ui.debug("Damage value", self.damage_value);
            ui.debug("Time since vanish", self.time_since_vanish);
            ui.debug("Time since appear", self.time_since_appear);
            ui.debug("Time since hit", self.time_since_hit);
        });
    }
}

impl DebugDisplay for NewMenuSystem {
    fn render_debug(&self, ui: &Ui) {
        ui.list("Windows", self.windows.iter(), |ui, i, window| {
            ui.pointer(format!("{i}"), window)
        });
    }
}
