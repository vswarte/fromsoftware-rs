use hudhook::imgui::{TableColumnSetup, Ui};

use debug::UiExt;
use eldenring::cs::{
    AnnounceNotification, BackScreenData, CSChrMenuFlags, CSMenuManImp, CSPlayerMenuCtrl,
    CSPopupMenu, FeSystemAnnounceView, LoadingScreenData, MenuString, SystemAnnounceViewModelState,
    UIState,
};
use eldenring::dlkr::DLAllocator;
use eldenring::dltx::DLString;

use crate::display::{DebugDisplay, DisplayUiExt, StatefulDebugDisplay};

#[derive(Default)]
pub struct MenuManState {
    new_announcement: String,
    new_popup_message: String,
}

impl StatefulDebugDisplay for CSMenuManImp {
    type State = MenuManState;

    fn render_debug_mut(&mut self, ui: &Ui, state: &mut Self::State) {
        ui.display("Disable Mouse Cursor", self.disable_mouse_cursor);
        ui.display("Disable Save Menu", self.disable_save_menu);

        ui.nested("Player Menu Ctrl", &self.player_menu_ctrl);
        ui.nested("Back Screen Data", &self.back_screen_data);
        ui.nested("Loading Screen Data", &self.loading_screen_data);

        ui.header("System Announce View Model", || {
            ui.nested_mut(
                "View",
                unsafe { self.system_announce_view_model.view.as_mut() },
                &mut (),
            );

            ui.header("Message Queue", || {
                for (i, notification) in self
                    .system_announce_view_model
                    .notifications
                    .iter_mut()
                    .enumerate()
                {
                    ui.header(format!("Notification {i}"), || {
                        ui.checkbox("Is Active", &mut notification.is_active);
                        ui.display("Message", &notification.message);
                    });
                }
            });
            if ui.button("Remove Front") {
                self.system_announce_view_model.notifications.pop_front();
            }
            ui.same_line();
            if ui.button("Clear Queue") {
                self.system_announce_view_model.notifications.clear();
            }

            ui.input_text("", &mut state.new_announcement).build();
            ui.same_line();
            if ui.button("Push Announcement")
                && let Ok(text) = DLString::from_str(
                    &state.new_announcement,
                    DLAllocator::runtime_heap_allocator(),
                )
            {
                self.system_announce_view_model
                    .notifications
                    .push_back(AnnounceNotification {
                        is_active: true,
                        message: MenuString {
                            static_string: std::ptr::null(),
                            allocated_string: text,
                        },
                    });
                state.new_announcement.clear();
            }
        });

        if let Some(mut popup_menu) = self.popup_menu {
            let popup_menu = unsafe { popup_menu.as_mut() };
            ui.header("Popup Menu", || {
                popup_menu.render_debug(ui);

                ui.input_text("", &mut state.new_popup_message).build();
                ui.same_line();
                if ui.button("Push Popup Message")
                    && let Ok(text) = DLString::from_str(
                        &state.new_popup_message,
                        DLAllocator::runtime_heap_allocator(),
                    )
                {
                    popup_menu.popup_messages.push_back(MenuString {
                        static_string: std::ptr::null(),
                        allocated_string: text,
                    });
                    state.new_popup_message.clear();
                }
            });
        } else {
            ui.text("Popup Menu: None");
        }

        ui.header("UI States", || {
            ui.table(
                "UI States",
                [
                    TableColumnSetup::new("Entry"),
                    TableColumnSetup::new("Created"),
                    TableColumnSetup::new("Visible"),
                ],
                self.ui_states.iter().enumerate(),
                |ui, _, (entry, state): (usize, &UIState)| {
                    ui.table_next_column();
                    ui.text(entry.to_string());

                    ui.table_next_column();
                    ui.text(state.created().to_string());

                    ui.table_next_column();
                    ui.text(state.visible().to_string());
                },
            );
        });
    }
}

impl DebugDisplay for CSPlayerMenuCtrl {
    fn render_debug(&self, ui: &Ui) {
        ui.debug("Selected Goods Item", self.selected_goods_item);
        ui.debug("Selected Magic Item", self.selected_magic_item);
        ui.nested("Chr Menu Flags", &self.chr_menu_flags);
    }
}

impl DebugDisplay for CSChrMenuFlags {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Pause Menu State", self.flags.pause_menu_state());
    }
}

impl DebugDisplay for CSPopupMenu {
    fn render_debug(&self, ui: &Ui) {
        ui.display("Current Talk ID", self.current_talk_id);
        ui.display("Show Failed To Save", self.show_failed_to_save);

        ui.list(
            "Popup Messages",
            self.popup_messages.iter(),
            |ui, i, message| ui.display(format!("Message {i}"), message),
        );
    }
}

impl DebugDisplay for BackScreenData {
    fn render_debug(&self, _ui: &Ui) {}
}

impl DebugDisplay for LoadingScreenData {
    fn render_debug(&self, _ui: &Ui) {}
}

impl StatefulDebugDisplay for FeSystemAnnounceView {
    type State = ();

    fn render_debug_mut(&mut self, ui: &Ui, _state: &mut Self::State) {
        ui.checkbox("Is Active", &mut self.is_active);
        ui.checkbox("Is Visible", &mut self.is_visible);

        const PLAY_STATES: [SystemAnnounceViewModelState; 13] = [
            SystemAnnounceViewModelState::Idle,
            SystemAnnounceViewModelState::Load,
            SystemAnnounceViewModelState::FadeIn,
            SystemAnnounceViewModelState::ScrollReset,
            SystemAnnounceViewModelState::BufferWait,
            SystemAnnounceViewModelState::NoScrollWait,
            SystemAnnounceViewModelState::Scrolling,
            SystemAnnounceViewModelState::PostScrollBuffer,
            SystemAnnounceViewModelState::PostScrollBufferWait,
            SystemAnnounceViewModelState::RepeatCheck,
            SystemAnnounceViewModelState::HidePlaying,
            SystemAnnounceViewModelState::FadeOut,
            SystemAnnounceViewModelState::Dequeue,
        ];
        let mut play_state_index = self.announce_play_state as usize;
        if ui.combo("Play State", &mut play_state_index, &PLAY_STATES, |state| {
            format!("{state:?}").into()
        }) {
            self.announce_play_state = PLAY_STATES[play_state_index];
        }

        ui.nested_mut(
            "Active Announcement",
            &mut self.active_announcement,
            &mut (),
        );

        ui.header("Scroll", || {
            ui.checkbox("Needs Scroll", &mut self.needs_scroll);
            ui.input_int("Scroll Offset", &mut self.scroll_offset)
                .build();
            ui.input_int("Scroll Distance", &mut self.scroll_distance)
                .build();
            ui.input_float(
                "Scroll Buffer Timer",
                &mut self.system_announce_scroll_buffer_timer,
            )
            .build();
            ui.input_scalar("Scroll Count", &mut self.system_announce_scroll_count)
                .build();
        });
    }
}

impl StatefulDebugDisplay for AnnounceNotification {
    type State = ();

    fn render_debug_mut(&mut self, ui: &Ui, _state: &mut Self::State) {
        ui.checkbox("Is Active", &mut self.is_active);
        ui.display("Message", &self.message);
    }
}
