use std::io::{Read, Seek, SeekFrom, Write};

use ilhook::{x64::*, *};
use pelite::pe64::Pe;
use shared::{Program, ext::*};

use crate::dlio::*;
use crate::rva;
use crate::sprj::*;

/// A magic header string that we write into save data that's modified using
/// [on_save] so we can tell whether it was modified by our custom code.
const HEADER: &str = "fromsoftware-rs";

/// An enum of different circumstances in which a save file can be loaded.
pub enum OnLoadType {
    /// The fake save file for the main menu is loading. This happens when the
    /// game starts (after the first button press), and again each time the
    /// player quits their game.
    ///
    /// The `on_save` callback is not run for the main menu save, so this
    /// never has modded data associated with it.
    MainMenu,

    /// A non-menu save file with data written by the `on_save` callback is
    /// loading. This contains the written data.
    SavedData(Vec<u8>),

    /// A non-menu save file without data written by `on_save` is loading. This
    /// could be because `on_save` returned `None`, or because this is a vanilla
    /// save file that was written without hooking the save information.
    NoSavedData,
}

/// A hook created by [on_save_load]. When this is dropped, the hook will be
/// unregistered.
pub struct SaveLoadHook<'a> {
    _save: ClosureHookPoint<'a>,
    _load: ClosureHookPoint<'a>,
}

/// Registers callbacks to add data to the DS3 save file.
///
/// ## Callbacks
///
/// The `on_save` callback is called whenever DS3 saves (other than for the
/// partial save file used to save main menu settings). The callback returns a
/// byte vector which provides arbitrary data to write to the save file, with
/// the caveat that the entire save file can't exceed 2GB. It may also return
/// `None`, in which case the vanilla save data will be unchanged.
///
/// The `on_load` callback is called whenever DS3 loads a save file (*including*
/// the partial file used to save main menu settings). It takes an [OnLoadType]
/// which provides information about the file, including the data saved by
/// `on_save` if it was provided for the current file.
///
/// ## Safety
///
/// This is subject to all the standard [ilhook safety concerns].
///
/// [ilhook safety concerns]: https://docs.rs/ilhook/latest/ilhook/x64/struct.Hooker.html#method.hook
pub unsafe fn on_save_load<'a, OnSave, OnLoad>(
    on_save: OnSave,
    on_load: OnLoad,
) -> Result<SaveLoadHook<'a>, HookError>
where
    OnSave: (Fn() -> Option<Vec<u8>>) + Send + Sync + 'a,
    OnLoad: Fn(OnLoadType) + Send + Sync + 'a,
{
    unsafe {
        Ok(SaveLoadHook {
            _save: hook_save(on_save)?,
            _load: hook_load(on_load)?,
        })
    }
}

/// Registers the save hook for [on_save_load].
unsafe fn hook_save<'a, OnSave>(callback: OnSave) -> Result<ClosureHookPoint<'a>, HookError>
where
    OnSave: (Fn() -> Option<Vec<u8>>) + Send + Sync + 'a,
{
    let callback = move |reg: *mut Registers, original| {
        let original: extern "win64" fn(&EquipGameData, &mut DLMemoryOutputStream) -> usize =
            unsafe { std::mem::transmute(original) };
        // Safety: We trust that DS3 gives us valid pointers.
        let this = unsafe { &*((*reg).rcx as *const EquipGameData) };
        let stream = unsafe { &mut *((*reg).rdx as *mut DLMemoryOutputStream) };

        // Never write custom save data for the main menu.
        if !this.is_main_menu()
            && let Some(result) = callback()
        {
            // Add a small header indicating that fromsoftware-rs modified
            // this save file, so that we know which save files to run
            // [on_load] for.
            write!(stream, "{}", HEADER).unwrap();
            if stream.write_delimited(result.as_ref()).unwrap() != result.len() + 4 {
                return 1;
            }
        }

        original(this, stream)
    };

    let va = Program::current()
        .rva_to_va(rva::get().equip_game_data_serialize)
        .expect("Call target for equip_game_data_serialize was not in exe");
    unsafe {
        hook_closure_retn(
            va as usize,
            callback,
            CallbackOption::None,
            HookFlags::empty(),
        )
    }
}

/// Registers the load hook for [on_save_load].
unsafe fn hook_load<'a, OnLoad>(callback: OnLoad) -> Result<ClosureHookPoint<'a>, HookError>
where
    OnLoad: Fn(OnLoadType) + Send + Sync + 'a,
{
    let callback = move |reg: *mut Registers, original| {
        let original: extern "win64" fn(&mut EquipGameData, &mut DLMemoryInputStream) -> usize =
            unsafe { std::mem::transmute(original) };
        // Safety: We trust that DS3 gives us valid pointers.
        let this = unsafe { &mut *((*reg).rcx as *mut EquipGameData) };
        let stream = unsafe { &mut *((*reg).rdx as *mut DLMemoryInputStream) };

        let mut header = [0; HEADER.len()];
        let before_header = stream.stream_position().unwrap();
        let has_saved_data =
            stream.read(&mut header).unwrap() == HEADER.len() && header == HEADER.as_bytes();
        if has_saved_data {
            let data = stream.read_delimited().unwrap();
            callback(OnLoadType::SavedData(data));
        } else {
            stream.seek(SeekFrom::Start(before_header)).unwrap();
        }

        if original(this, stream) == 0 {
            return 0;
        }

        if !has_saved_data {
            callback(if this.is_main_menu() {
                OnLoadType::MainMenu
            } else {
                OnLoadType::NoSavedData
            });
        }

        1
    };

    let va = Program::current()
        .rva_to_va(rva::get().equip_game_data_deserialize)
        .expect("Call target for equip_game_data_deserialize was not in exe");
    unsafe {
        hook_closure_retn(
            va as usize,
            callback,
            CallbackOption::None,
            HookFlags::empty(),
        )
    }
}
