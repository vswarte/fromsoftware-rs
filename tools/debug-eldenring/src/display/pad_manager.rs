use std::ops::Deref;

use debug::UiExt;
use eldenring::{
    cs::{CSKeyAssign, CSPad},
    dluid::{
        DLUserInputDevice, DLUserInputDeviceImpl, DLVirtualAnalogKeyInfo, DLVirtualInputData,
        DummyDevice, DynamicBitset, KeyboardDevice, MouseDevice, PadDevice, VirtualMultiDevice,
    },
    fd4::{
        FD4PadDevice, FD4PadDevice0x78, FD4PadManager, InputTypeGroup, PadEntry,
        WindowCursorContext,
    },
};
use hudhook::imgui::TableColumnSetup;

use crate::display::DebugDisplay;

impl DebugDisplay for FD4PadManager {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("FD4PadDeviceList", || {
            for (index, pad_device_ptr) in self.pad_device_list.iter().enumerate() {
                ui.header(format!("FD4PadDevice [{}]", index), || {
                    let pad_device = unsafe { pad_device_ptr.as_ref() };
                    pad_device.render_debug(ui);
                });
            }
        });
        ui.header("CSPadEntryMapList", || {
            for (index, ptr) in self.pad_entry_map_list.iter().enumerate() {
                ui.header(format!("CSPadEntryMap [{}]", index), || {
                    let tree = unsafe { ptr.as_ref() };
                    for (index, pair) in tree.iter().enumerate() {
                        ui.header(format!("Pair [{}]", index), || {
                            ui.text(format!("Key: {:#X}", pair.key));
                            ui.header("CSPad", || {
                                let cs_pad = unsafe { pair.value.entry.as_ref() };
                                cs_pad.render_debug(ui);
                            });
                        });
                    }
                });
            }
        });
        ui.header("CSKeyAssignMapList", || {
            for (index, ptr) in self.key_assign_map_list.iter().enumerate() {
                ui.header(format!("CSKeyAssignMap [{}]", index), || {
                    let tree = unsafe { ptr.as_ref() };
                    for (index, pair) in tree.iter().enumerate() {
                        ui.header(format!("Pair [{}]", index), || {
                            ui.text(format!("Key: {:#X}", pair.key));
                            ui.header("CSKeyAssign", || {
                                let key_assign = unsafe { pair.value.as_ref() };
                                key_assign.render_debug(ui);
                            });
                        });
                    }
                });
            }
        });

        ui.text(format!(
            "exit_foreground_signaled: {}",
            self.exit_foreground_signaled
        ));
        ui.text(format!(
            "is_back_ground_window: {}",
            self.is_back_ground_window
        ));
    }
}

impl DebugDisplay for FD4PadDevice {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("VirtualMultiDevice", || {
            let device = unsafe { self.virtual_multi_device.as_ref() };
            device.render_debug(ui);
        });
        ui.header("PadDeviceList", || {
            for (index, pad_device_ptr) in self.pad_devices.iter().enumerate() {
                ui.header(format!("PadDevice [{}]", index), || {
                    let pad_device = unsafe { pad_device_ptr.as_ref() };
                    pad_device.render_debug(ui);
                });
            }
        });
        ui.header("MouseDevice", || {
            let device = unsafe { self.mouse_device.as_ref() };
            device.render_debug(ui);
        });
        ui.header("KeyboardDevice", || {
            let device = unsafe { self.keyboard_device.as_ref() };
            device.render_debug(ui);
        });
        ui.header("FD4PadDevice0x78", || {
            self.unk78.render_debug(ui);
        });
    }
}

impl DebugDisplay for FD4PadDevice0x78 {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("VirtualMultiDevice", || {
            let device = unsafe { self.virtual_multi_device.as_ref() };
            device.render_debug(ui);
        });
        ui.header("bitset_fallback", || {
            let items = self.bitset_fallback;
            let len = items.len();
            ui.table(
                "bitset_fallback_TABLE",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("+ 0"),
                    TableColumnSetup::new("+ 1"),
                    TableColumnSetup::new("+ 2"),
                    TableColumnSetup::new("+ 3"),
                ],
                (0..len).step_by(4),
                |ui: &hudhook::imgui::Ui, _, index: usize| {
                    ui.table_next_column();
                    ui.text(format!("{}", index));
                    for offset in 0..4 {
                        ui.table_next_column();
                        let idx = index + offset;
                        if idx < items.len() {
                            ui.text(format!("{}", items[idx]));
                        }
                    }
                },
            );
        });
    }
}

impl DebugDisplay for DLUserInputDevice {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("DLVirtualInputData", || {
            self.virtual_input_data.render_debug(ui);
        });
    }
}

impl DebugDisplay for DLUserInputDeviceImpl {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("DLUserInputDevice", || {
            self.deref().render_debug(ui);
        });
        ui.header("DLPlainLightMutex", || {
            ui.text(format!("{:#?}", self.mutex.critical_section));
        });
        ui.header("analog_positive_axis", || {
            self.analog_positive_axis.render_debug(ui);
        });
        ui.header("analog_negative_axis", || {
            self.analog_negative_axis.render_debug(ui);
        });
        ui.header("DLVirtualInputData", || {
            self.initial_virtual_input_data.render_debug(ui);
        });
    }
}

impl DebugDisplay for DLVirtualAnalogKeyInfo<f32> {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.text(format!("Items: {}", self.vector.len()));
        let items = self.vector.items();
        let len = items.len();
        ui.table(
            "DLVirtualAnalogKeyInfo_TABLE",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("+ 0"),
                TableColumnSetup::new("+ 1"),
                TableColumnSetup::new("+ 2"),
                TableColumnSetup::new("+ 3"),
            ],
            (0..len).step_by(4),
            |ui: &hudhook::imgui::Ui, _, index: usize| {
                ui.table_next_column();
                ui.text(format!("{}", index));
                for offset in 0..4 {
                    ui.table_next_column();
                    let idx = index + offset;
                    if idx < items.len() {
                        ui.text(format!("{}", items[idx]));
                    }
                }
            },
        );
    }
}

impl DebugDisplay for DynamicBitset {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        let items = self.as_slice();
        ui.table(
            "DYNAMIC_BITSET_TABLE",
            [
                TableColumnSetup::new("Index"),
                TableColumnSetup::new("Bits"),
            ],
            items.iter(),
            |ui: &hudhook::imgui::Ui, index: usize, bits: &u32| {
                ui.table_next_column();
                ui.text(format!("{}", index));
                ui.table_next_column();
                ui.text(format!("{:032b}", bits));
            },
        );
    }
}

impl DebugDisplay for DLVirtualInputData {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("virtual_analog_key_info", || {
            self.analog_key_info.render_debug(ui);
        });
        ui.header("virtual_digital_key_info", || {
            self.dynamic_bitset.render_debug(ui);
        });
    }
}

impl DebugDisplay for VirtualMultiDevice {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("DLUserInputDeviceImpl", || {
            self.deref().render_debug(ui);
        });
    }
}

impl DebugDisplay for DummyDevice {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("DLUserInputDeviceImpl", || {
            self.deref().render_debug(ui);
        });
    }
}

impl DebugDisplay for PadDevice {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("DLUserInputDeviceImpl", || {
            self.deref().render_debug(ui);
        });
        ui.header("XINPUT_STATE", || {
            ui.text(format!("dwUserIndex: {}", self.dw_user_index));
            ui.text(format!("{:#?}", self.w_buttons));
            ui.text(format!("Thumb LX: {}", self.s_thumb_lx));
            ui.text(format!("Thumb LY: {}", self.s_thumb_ly));
            ui.text(format!("Thumb RX: {}", self.s_thumb_rx));
            ui.text(format!("Thumb RY: {}", self.s_thumb_ry));
            ui.text(format!("Left Trigger: {}", self.b_left_trigger));
            ui.text(format!("Right Trigger: {}", self.b_right_trigger));
        });
    }
}

impl DebugDisplay for KeyboardDevice {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("DLUserInputDeviceImpl", || {
            self.deref().render_debug(ui);
        });
        ui.header("DIRECTINPUT_KEYBOARD", || {
            let step_size = 4;
            let len = self.di_keyboard_state.len();
            ui.table(
                "DIRECTINPUT_KEYBOARD_TABLE",
                [
                    TableColumnSetup::new("Index"),
                    TableColumnSetup::new("+ 0"),
                    TableColumnSetup::new("+ 1"),
                    TableColumnSetup::new("+ 2"),
                    TableColumnSetup::new("+ 3"),
                ],
                (0..len).step_by(step_size),
                |ui: &hudhook::imgui::Ui, _, index: usize| {
                    ui.table_next_column();
                    ui.text(format!("{}", index));
                    for offset in 0..4 {
                        ui.table_next_column();
                        let key = index + offset;
                        if key < len {
                            let state = self.is_key_pressed(key);
                            ui.text(format!("{}", state));
                        }
                    }
                },
            );
        });
    }
}

impl DebugDisplay for MouseDevice {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("DLUserInputDeviceImpl", || {
            self.deref().render_debug(ui);
        });
        ui.header("DIRECTINPUT_MOUSE", || {
            ui.text(format!("lx: {}", self.di_mouse_state.lx));
            ui.text(format!("ly: {}", self.di_mouse_state.ly));
            ui.text(format!("lz: {}", self.di_mouse_state.lz));
            self.di_mouse_state
                .buttons
                .iter()
                .enumerate()
                .for_each(|(index, button)| {
                    let state = *button & 0x80 != 0;
                    ui.text(format!("Button [{}]: {}", index, state));
                });
        });
    }
}

impl DebugDisplay for PadEntry {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.text(format!("enable_use: {}", self.enable_use));
        ui.header("CSPad", || {
            let cs_pad = unsafe { self.entry.as_ref() };
            cs_pad.render_debug(ui);
        });
    }
}

impl DebugDisplay for CSPad {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("FD4PadDevice", || {
            let pad_device = unsafe { self.pad_device.as_ref() };
            pad_device.render_debug(ui);
        });

        let pad_name: String = unsafe {
            let mut len = 0;
            let mut ptr = self.pad_name;

            if !ptr.is_null() {
                while *ptr != 0 && len < 64 {
                    len += 1;
                    ptr = ptr.add(1);
                }
            }

            if len == 0 {
                String::from("No pad name")
            } else {
                let slice = std::slice::from_raw_parts(self.pad_name, len);
                String::from_utf16_lossy(slice)
            }
        };

        ui.text(format!("pad_name: {}", pad_name));
        ui.text(format!("allow_polling: {}", self.allow_polling));

        ui.header("CSKeyAssign", || {
            let key_assign = unsafe { self.key_assign.as_ref() };
            key_assign.render_debug(ui);
        });

        ui.header("InputTypeGroupMap", || {
            let tree = unsafe { self.input_type_group.as_ref() };
            ui.table(
                "InputTypeGroupMap_TABLE",
                [
                    TableColumnSetup::new("Key"),
                    TableColumnSetup::new("InputTypeGroup"),
                ],
                tree.iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    pair.value.render_debug(ui);
                },
            );
        });

        ui.header("InputCodeCheck", || {
            let tree = unsafe { self.input_code_check.as_ref() };
            ui.table(
                "InputCodeCheck_TABLE",
                [
                    TableColumnSetup::new("Key"),
                    TableColumnSetup::new("State 1"),
                    TableColumnSetup::new("State 2"),
                ],
                tree.iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.value.state_1));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.value.state_2));
                },
            );
        });

        ui.text(format!(
            "Empty DLString<UTF16>: {}",
            self.empty_str
                .to_str()
                .unwrap_or("Invalid string".to_string())
        ));

        ui.header("WindowCursorContext", || {
            let context = unsafe { self.window_cursor_context.as_ref() };
            context.render_debug(ui);
        });

        ui.header("UnusedInputMap", || {
            let tree = &self.unused_input_map;
            ui.table(
                "UnusedInputMap_TABLE",
                [TableColumnSetup::new("Key"), TableColumnSetup::new("State")],
                tree.iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    ui.text(format!("{}", pair.value));
                },
            );
        });
    }
}

impl DebugDisplay for InputTypeGroup {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.table(
            "InputTypeGroup_TABLE",
            [
                TableColumnSetup::new("Mapped Input"),
                TableColumnSetup::new("Input Type"),
            ],
            self.iter(),
            |ui, _, (mapped_input, input_type)| {
                ui.table_next_column();
                ui.text(format!("{}", mapped_input));
                ui.table_next_column();
                ui.text(format!("{:?}", input_type));
            },
        );
    }
}

impl DebugDisplay for WindowCursorContext {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.text(format!("Window handle: {:#}", self.window_handle));
        ui.text(format!("cursor_x: {}", self.cursor_x));
        ui.text(format!("cursor_y: {}", self.cursor_y));
    }
}

impl DebugDisplay for CSKeyAssign {
    fn render_debug(&self, ui: &hudhook::imgui::Ui) {
        ui.header("VirtualInputDataIndexMap", || {
            let tree = unsafe { self.virtual_input_data_index_map.as_ref() };
            ui.table(
                "VirtualInputDataIndexMap_TABLE",
                [
                    TableColumnSetup::new("Mapped Input"),
                    TableColumnSetup::new("Input Data Index"),
                ],
                tree.iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    ui.text(format!("{:?}", pair.value));
                },
            );
        });
        ui.header("unk78IndexMap", || {
            let tree = unsafe { self.virtual_input_data_index_map.as_ref() };
            ui.table(
                "unk78IndexMap_TABLE",
                [
                    TableColumnSetup::new("Mapped Input"),
                    TableColumnSetup::new("Unk78 Index"),
                ],
                tree.iter(),
                |ui, _, pair| {
                    ui.table_next_column();
                    ui.text(format!("{}", pair.key));
                    ui.table_next_column();
                    ui.text(format!("{:?}", pair.value));
                },
            );
        });
    }
}
