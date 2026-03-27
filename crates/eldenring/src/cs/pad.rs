use std::ops::{Deref, DerefMut};

use shared::{Subclass, Superclass};

use crate::fd4::{FD4BasePad, InputType, InputTypeGroup};

/// Represents the base CSPad class.
/// The game has various CSPad instances, all with a different RTTI name.
///
/// Make use of [FD4PadManager] to obtain the desired [CSPad] instance, a.e `CSInGamePad_UserInput1` to poll inputs during gameplay.
///
/// Example usage:
/// ```
/// if let Ok(in_game_pad) = unsafe { FD4PadManager::instance() }
///    .ok()
///    .and_then(|pad_man| pad_man.get_in_game_pad()
/// {
///    let is_jump_pressed = in_game_pad.poll_input(UserInputKey::Jump);
/// }
/// ```
#[repr(C)]
#[derive(Superclass)]
#[superclass(children(CSInGamePad, CSMenuViewerPad))]
pub struct CSPad {
    base: FD4BasePad,
}

impl CSPad {

    pub fn poll_digital_input(&self, input: i32) -> bool {
        if !self.allow_polling {
            return false;
        }

        if let Some(pair) = self.unused_input_map.iter().find(|pair| pair.key == input)
            && pair.value
        {
            return true;
        }

        let Some(input_type_group) = self.get_input_type_group(input) else {
            return false;
        };

        input_type_group.iter().any(|(mapped_input, input_type)| {
            if !self.check_input(mapped_input) {
                return false;
            }

            let key_assign = unsafe { self.key_assign.as_ref() };
            let Some(virtual_input_index) = key_assign.get_virtual_input_index(mapped_input) else {
                return false;
            };

            match input_type {
                InputType::AreKeysDown => self.index_digital_input(virtual_input_index),
                InputType::AreKeysUp => !self.index_digital_input(virtual_input_index),
                InputType::IsStickMoving => self.index_analog_input(virtual_input_index) != 0.0,
            }
        })
    }

    pub fn poll_analog_input(&self, input: i32) -> f32 {
        if !self.allow_polling {
            return 0.0;
        }

        let Some(input_type_group) = self.get_input_type_group(input) else {
            return 0.0;
        };

        input_type_group
            .iter()
            .find_map(|(mapped_input, input_type)| {
                let key_assign = unsafe { self.key_assign.as_ref() };
                if input_type == InputType::IsStickMoving
                    && self.check_input(mapped_input)
                    && let Some(virtual_input_index) =
                        key_assign.get_virtual_input_index(mapped_input)
                {
                    let movement = self.index_analog_input(virtual_input_index);
                    Some(movement)
                } else {
                    None
                }
            })
            .unwrap_or_default()
    }

    fn get_input_type_group(&self, input: i32) -> Option<InputTypeGroup> {
        let input_type_group_tree = unsafe { self.input_type_group.as_ref() };
        input_type_group_tree
            .iter()
            .find(|pair| pair.key == input)
            .map(|pair| pair.value)
    }

    fn check_input(&self, mapped_input: i32) -> bool {
        let input_code_check_tree = unsafe { self.input_code_check.as_ref() };
        if let Some(check) = input_code_check_tree
            .iter()
            .find(|pair| pair.key == mapped_input)
            .map(|pair| &pair.value)
        {
            check.state_1 && !check.state_2
        } else {
            false
        }
    }

    pub fn index_digital_input(&self, virtual_input_index: i32) -> bool {
        let multi_device = unsafe { self.input_devices.as_ref() };

        let user_input_device = unsafe {
            &multi_device
                .virtual_multi_device
                .as_ref()
                .device
        };

        if user_input_device.get_virtual_digital_state(virtual_input_index as usize) {
            return true;
        }

        let key_assign = unsafe { self.key_assign.as_ref() };
        if let Some(mut fallback_index) = key_assign
            .unk78_index_map
            .iter()
            .find(|pair| pair.key == virtual_input_index)
            .filter(|pair| pair.value != -1)
            .map(|pair| pair.value)
        {
            fallback_index -= 1000;
            if 0 <= fallback_index && fallback_index <= 80 {
                return multi_device.unk78.bitset_fallback[fallback_index as usize];
            }
        }

        false
    }

    pub fn index_analog_input(&self, virtual_input_index: i32) -> f32 {
        let user_input_device = unsafe {
            &self
                .input_devices
                .as_ref()
                .virtual_multi_device
                .as_ref()
                .device
        };

        user_input_device.get_virtual_analog_state(virtual_input_index as usize)
    }
}

impl Deref for CSPad {
    type Target = FD4BasePad;

    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

impl DerefMut for CSPad {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

/// Derived class on top of the [CSPad].
///
/// The vftable will be changed and it's gonna use different associated functions for input polling.
///
/// Used for polling inputs during the `InGameStep`.
#[repr(C)]
#[derive(Subclass)]
pub struct CSInGamePad {
    base: CSPad,
}

/// Derived class on top of the [CSPad].
///
/// The vftable will be changed and it's gonna use different associated functions for input polling.
#[repr(C)]
#[derive(Subclass)]
pub struct CSMenuViewerPad {
    base: CSPad,
}

/// Enum to poll key's with [CSInGameUserInput].
///
/// Source of name: Debug string in `CSInGamePad_UserInput1::vftable`.
#[repr(i32)]
pub enum UserInputKey {
    /// Horizontal movement of the mouse.
    /// (Hold)
    MouseMovementX = 4,
    /// Vertical movement of the mouse.
    /// (Hold)
    MouseMovementY = 5,

    /// Movement Control
    /// Press while moving to walk
    /// (Hold)
    MovementControl = 6,

    /// Attack (RH & Two-Handed Armament)
    /// Normal attack with your right-hand armament
    Attack = 7,
    /// Strong Attack (RH & 2H Armament)
    /// Strong attack with your right-hand armament. Hold to charge the attack.
    StrongAttack = 8,
    /// Guard (LH Armament)
    /// Guard with your left-hand armament
    /// Attacks with left-hand armament when the equipped item doesn't guard.
    Guard = 9,
    /// Skill
    /// Preform a skill
    Skill = 10,
    /// Event Action (Examine, Open, etc.)
    /// Perform various actions
    /// (Hold)
    EventAction = 11,
    /// Backstep, Dodge Roll, Dash
    /// Press while standing still to backstep.
    /// Press while moving for a dodge roll, or hold to dash.
    /// (Hold)
    Backstep = 12,
    /// Backstep, Dodge Roll, Dash
    /// Press while standing still to backstep.
    /// Press while moving for a dodge roll, or hold to dash.
    /// (Tap)
    BackstepTapped = 13,
    /// Jump
    /// (Hold)
    Jump = 14,
    /// Use item
    /// Use an item
    UseItem = 15,
    /// Switch Sorcery/Incantation
    /// Change your selected sorcery or incantation
    /// (Hold)
    SwitchSpell = 16,
    /// Switch Right-hand Armament
    /// Change your right-hand armament
    SwitchRightHandArmament = 17,
    /// Switch left-Hand Armament
    /// Change your left-hand armament
    SwitchleftHandArmament = 18,
    /// Switch Item
    /// Change your Quick Item
    SwitchItem = 19,
    /// Reset Camera, Lock-On/Remove Target
    /// Reset the camera's position.
    /// Turn lock-on fixation on or off during lock-on.
    /// (Hold)
    ResetCamera = 20,
    /// Crouch / Stand Up
    /// Switch between standing and crouching,
    /// (Hold)
    Crouch = 21,

    /// Switch Sorcery/Incantation
    /// Change your selected sorcery or incantation
    /// (Hold)
    SwitchSpell2 = 24,
    /// Switch Item
    /// Change your Quick Item
    /// (Hold)
    SwitchItem2 = 25,
    /// Reset Camera, Lock-On/Remove Target
    /// Reset the camera's position.
    /// Turn lock-on fixation on or off during lock-on.
    /// (Tap)
    ResetCameraTapped = 26,
    /// Event Action (Examine, Open, etc.)
    /// Perform various actions
    /// (Hold)
    EventActionPouch = 27,

    /// Map
    /// Display the map menu
    Map = 300,

    /// Move forwards
    /// (Hold)
    MoveForwards = 417,
    /// Move backwards
    /// (hold)
    MoveBackwards = 418,
    /// Move left
    /// (hold)
    MoveLeft = 419,
    /// Move Right
    /// (hold)
    MoveRight = 420,

    /// Move Camera / Change Target (Up)
    /// Move the camera up.
    /// Change target upwards during lock-on.
    /// Doesn't poll Mouse movement, use `MouseMovementY` for that.
    MoveCameraUp = 424,
    /// Move Camera / Change Target (Down)
    /// Move the camera down.
    /// Change target downwards during lock-on.
    /// Doesn't poll Mouse movement, use `MouseMovementY` for that.
    MoveCameraDown = 425,
    /// Move Camera / Change Target (Left)
    /// Move the camera left.
    /// Change target to the left during lock-on.
    /// Doesn't poll Mouse movement, use `MouseMovementX` for that.
    MoveCameraLeft = 426,
    /// Move Camera / Change Target (Right)
    /// Move the camera right.
    /// Change target to the right during lock-on.
    /// Doesn't poll Mouse movement, use `MouseMovementX` for that.
    MoveCameraRight = 427,
}

impl From<UserInputKey> for i32 {
    fn from(value: UserInputKey) -> Self {
        value as i32
    }
}
