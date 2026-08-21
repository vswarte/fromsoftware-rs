use crate::{
    DLVector,
    dlkr::DLPlainLightMutex,
    dluid::{
        DLUserInputDeviceImpl, DummyDevice, KeyboardDevice, MouseDevice, PadDevice,
        VirtualMultiDevice,
    },
};
use pelite::pe::Pe;
use shared::{FromStatic, InstanceError, Program, Subclass, Superclass, UnknownStruct};
use std::{borrow::Cow, ptr::NonNull};
use windows::Win32::Foundation::HMODULE;

#[repr(C)]
pub struct DLUserInputManagerImplBase {
    vftable: *const (),
    pub mutex: DLPlainLightMutex,
    allocator: *const (),
}

#[repr(C)]
pub struct DLUserInputManagerImpl {
    pub base: DLUserInputManagerImplBase,
    /// IDirectInput8W instance.
    pub direct_input_8_interface: *const (),
    unk48: DLVector<UnknownStruct<0x10>>,
    unk68: DLVector<UnknownStruct<0x14>>,
    pub user_input_devices: UserInputDeviceVector,
    /// Copied over from FD4PadManager
    pub window_handle: isize,
    pub dummy_device: DummyDevice,
    pub is_co_initialized: bool,
    unk889: bool,
    unk88a: bool,
    /// Disables `IDirectInput8::EnumDevices` from being called.
    pub use_lib_sce_pad: bool,
    unk88c: bool,
    pub is_game_window_focused: bool,
    /// Ext.UserInput.CooperativeLevel.SetForeGround.Pad
    pub set_foreground_pad: bool,
    /// Ext.UserInput.CooperativeLevel.SetForeGround.Keyboard
    pub set_foreground_keyboard: bool,
    /// Ext.UserInput.CooperativeLevel.SetForeGround.Mouse
    pub set_foreground_mouse: bool,
    /// DLVector containing unchecked ScePadHandle's.
    /// It pushes sce_pad_open result i32's to the DLVector.
    pub sce_pad_handles: DLVector<ScePadHandle>,
    pub lib_sce_pad_x64: HMODULE,
    sce_pad_init: *const (),
    sce_pad_open: *const (),
    sce_pad_close: *const (),
    sce_pad_read_state: *const (),
    sce_pad_reset_orientation: *const (),
    sce_pad_set_angular_velocity_deadband_state: *const (),
    sce_pad_set_tilt_correction_state: *const (),
    sce_pad_set_vibration: *const (),
    sce_pad_get_controller_information: *const (),
}
type UserInputDeviceVector = DLVector<NonNull<DLUserInputDeviceImpl>>;

impl DLUserInputManagerImpl {
    fn get_user_input_device<T: Subclass<DLUserInputDeviceImpl>>(&self) -> Option<&T> {
        self.user_input_devices.iter().find_map(|ptr| {
            let device = unsafe { ptr.as_ref() };
            device.as_subclass::<T>()
        })
    }
    pub fn get_keyboard_device(&self) -> Option<&KeyboardDevice> {
        self.get_user_input_device::<KeyboardDevice>()
    }
    pub fn get_mouse_device(&self) -> Option<&MouseDevice> {
        self.get_user_input_device::<MouseDevice>()
    }
    pub fn get_virtual_multi_device(&self) -> Option<&VirtualMultiDevice> {
        self.get_user_input_device::<VirtualMultiDevice>()
    }

    fn get_user_input_device_mut<T: Subclass<DLUserInputDeviceImpl>>(&mut self) -> Option<&mut T> {
        self.user_input_devices.iter_mut().find_map(|ptr| {
            let device = unsafe { ptr.as_mut() };
            device.as_subclass_mut::<T>()
        })
    }
    pub fn get_keyboard_device_mut(&mut self) -> Option<&mut KeyboardDevice> {
        self.get_user_input_device_mut::<KeyboardDevice>()
    }
    pub fn get_mouse_device_mut(&mut self) -> Option<&mut MouseDevice> {
        self.get_user_input_device_mut::<MouseDevice>()
    }
    pub fn get_virtual_multi_device_mut(&mut self) -> Option<&mut VirtualMultiDevice> {
        self.get_user_input_device_mut::<VirtualMultiDevice>()
    }

    pub fn get_pad_device(&self, user_index: i32) -> Option<&PadDevice> {
        self.user_input_devices.iter().find_map(|ptr| {
            let device = unsafe { ptr.as_ref() };
            if let Some(pad) = device.as_subclass::<PadDevice>()
                && pad.dw_user_index == user_index
            {
                Some(pad)
            } else {
                None
            }
        })
    }
    pub fn get_pad_device_mut(&mut self, user_index: i32) -> Option<&mut PadDevice> {
        self.user_input_devices.iter_mut().find_map(|ptr| {
            let device = unsafe { ptr.as_mut() };
            if let Some(pad) = device.as_subclass_mut::<PadDevice>()
                && pad.dw_user_index == user_index
            {
                Some(pad)
            } else {
                None
            }
        })
    }
}

#[repr(C)]
pub struct ScePadHandle(i32);
impl ScePadHandle {
    pub fn is_valid(&self) -> bool {
        self.0 >= 0
    }
}

impl From<ScePadHandle> for i32 {
    fn from(value: ScePadHandle) -> Self {
        value.0
    }
}

impl From<i32> for ScePadHandle {
    fn from(value: i32) -> Self {
        Self(value)
    }
}

type ObtainDLUserInputmanager = extern "C" fn() -> *mut DLUserInputManagerImpl;
impl FromStatic for DLUserInputManagerImpl {
    fn name() -> Cow<'static, str> {
        Cow::Borrowed("DLUserInputManagerImpl")
    }

    fn instance_ptr() -> shared::InstanceResult<*mut Self> {
        unsafe {
            let rva = crate::rva::get().obtain_dl_user_input_manager;
            let src = Program::current()
                .rva_to_va(rva)
                .map_err(|_| InstanceError::NotFound(Self::name()))?;

            let obtain_dl_user_input_manager =
                std::mem::transmute::<u64, ObtainDLUserInputmanager>(src);

            let result = obtain_dl_user_input_manager();

            if result.is_null() {
                Err(InstanceError::Null(Self::name()))
            } else {
                Ok(result)
            }
        }
    }

    unsafe fn instance() -> shared::InstanceResult<&'static Self> {
        let ptr = Self::instance_ptr()?;
        unsafe { ptr.as_ref().ok_or(InstanceError::Null(Self::name())) }
    }

    unsafe fn instance_mut() -> shared::InstanceResult<&'static mut Self> {
        let ptr = Self::instance_ptr()?;
        unsafe { ptr.as_mut().ok_or(InstanceError::Null(Self::name())) }
    }
}
