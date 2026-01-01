use std::time::Duration;

use eldenring::{
    cs::{
        BlockId, CSTaskGroupIndex, CSTaskImp, EzStateInvokeError, FieldInsHandle, FieldInsSelector,
        MenuType, TalkScript, WorldChrMan,
    },
    ez_state::EzStateValue,
    fd4::FD4TaskData,
    util::system::wait_for_system_init,
};
use fromsoftware_shared::{FromStatic, program::Program, task::*};
use windows::Win32::UI::Input::KeyboardAndMouse::{GetKeyState, VIRTUAL_KEY, VK_T};

fn is_key_down(key: VIRTUAL_KEY) -> bool {
    let key_state = unsafe { GetKeyState(key.0 as i32) } as u16;
    key_state & 0x8000 != 0
}

const SHOW_SHOP_MESSAGE: i32 = 10;
const ADD_TALK_LIST_DATA: i32 = 19;
const CLEAR_TALK_LIST_DATA: i32 = 20;
const OPEN_REGULAR_SHOP: i32 = 22;
const OPEN_SELL_SHOP: i32 = 46;

const GET_TALK_LIST_MENU_RESULT: i32 = 23;
const CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN: i32 = 58;
const CHECK_SPECIFIC_PERSON_MENU_IS_OPEN: i32 = 59;

enum TalkScriptDemoState {
    Idle,
    EnterMainMenu,
    WhileMainMenu,
    WhilePurchase,
    WhileSell,
}

/// State machine that simulates a simple talk script with multiple menu options
struct TalkScriptDemo {
    talk_script: TalkScript,
    state: TalkScriptDemoState,
}

impl TalkScriptDemo {
    pub fn step(&mut self) -> Result<(), EzStateInvokeError> {
        let talk_script = &mut self.talk_script;
        self.state = match self.state {
            TalkScriptDemoState::Idle => {
                if is_key_down(VK_T) {
                    TalkScriptDemoState::EnterMainMenu
                } else {
                    TalkScriptDemoState::Idle
                }
            }
            TalkScriptDemoState::EnterMainMenu => {
                // ClearTalkListData()
                talk_script.event([EzStateValue::Int32(CLEAR_TALK_LIST_DATA)])?;

                // AddTalkListData(1, 20000010, -1) // "Purchase"
                talk_script.event([
                    EzStateValue::Int32(ADD_TALK_LIST_DATA),
                    EzStateValue::Int32(1),
                    EzStateValue::Int32(20000010),
                    EzStateValue::Int32(-1),
                ])?;

                // AddTalkListData(2, 20000011, -1) // "Sell"
                talk_script.event([
                    EzStateValue::Int32(ADD_TALK_LIST_DATA),
                    EzStateValue::Int32(2),
                    EzStateValue::Int32(20000011),
                    EzStateValue::Int32(-1),
                ])?;

                // AddTalkListData(3, 20000009, -1) // "Leave"
                talk_script.event([
                    EzStateValue::Int32(ADD_TALK_LIST_DATA),
                    EzStateValue::Int32(3),
                    EzStateValue::Int32(20000009),
                    EzStateValue::Int32(-1),
                ])?;

                // ShowShopMessage(1)
                talk_script.event([
                    EzStateValue::Int32(SHOW_SHOP_MESSAGE),
                    EzStateValue::Int32(1),
                ])?;

                TalkScriptDemoState::WhileMainMenu
            }
            TalkScriptDemoState::WhileMainMenu => {
                // not (CheckSpecificPersonMenuIsOpen(1, 0) == 1 and not CheckSpecificPersonGenericDialogIsOpen(0))
                let specific_person_menu_is_open: i32 = talk_script
                    .env([
                        EzStateValue::Int32(CHECK_SPECIFIC_PERSON_MENU_IS_OPEN),
                        EzStateValue::Int32(MenuType::TalkList as i32),
                        EzStateValue::Int32(0),
                    ])?
                    .into();

                let specific_person_generic_dialog_is_open: i32 = talk_script
                    .env([
                        EzStateValue::Int32(CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN),
                        EzStateValue::Int32(0),
                    ])?
                    .into();

                if !(specific_person_menu_is_open == 1
                    && specific_person_generic_dialog_is_open == 0)
                {
                    // GetTalkListMenuResult()
                    match talk_script
                        .env([EzStateValue::Int32(GET_TALK_LIST_MENU_RESULT)])
                        .map(|v| v.into())?
                    {
                        1 => {
                            // OpenRegularShop(100500, 100524)
                            talk_script.event([
                                EzStateValue::Int32(OPEN_REGULAR_SHOP),
                                EzStateValue::Int32(100500),
                                EzStateValue::Int32(100524),
                            ])?;

                            TalkScriptDemoState::WhilePurchase
                        }
                        2 => {
                            // OpenSellShop(-1, -1)
                            talk_script.event([
                                EzStateValue::Int32(OPEN_SELL_SHOP),
                                EzStateValue::Int32(-1),
                                EzStateValue::Int32(-1),
                            ])?;

                            TalkScriptDemoState::WhileSell
                        }
                        _ => TalkScriptDemoState::Idle,
                    }
                } else {
                    TalkScriptDemoState::WhileMainMenu
                }
            }
            TalkScriptDemoState::WhilePurchase => {
                // not (CheckSpecificPersonMenuIsOpen(5, 0) == 1 and not CheckSpecificPersonGenericDialogIsOpen(0))
                let specific_person_menu_is_open: i32 = talk_script
                    .env([
                        EzStateValue::Int32(CHECK_SPECIFIC_PERSON_MENU_IS_OPEN),
                        EzStateValue::Int32(MenuType::RegularShop as i32),
                        EzStateValue::Int32(0),
                    ])?
                    .into();

                let specific_person_generic_dialog_is_open: i32 = talk_script
                    .env([
                        EzStateValue::Int32(CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN),
                        EzStateValue::Int32(0),
                    ])?
                    .into();

                if !(specific_person_menu_is_open == 1
                    && specific_person_generic_dialog_is_open == 0)
                {
                    TalkScriptDemoState::EnterMainMenu
                } else {
                    TalkScriptDemoState::WhilePurchase
                }
            }
            TalkScriptDemoState::WhileSell => {
                // not (CheckSpecificPersonMenuIsOpen(5, 0) == 1 and not CheckSpecificPersonGenericDialogIsOpen(0))
                let specific_person_menu_is_open: i32 = talk_script
                    .env([
                        EzStateValue::Int32(CHECK_SPECIFIC_PERSON_MENU_IS_OPEN),
                        EzStateValue::Int32(MenuType::SellShop as i32),
                        EzStateValue::Int32(0),
                    ])?
                    .into();

                let specific_person_generic_dialog_is_open: i32 = talk_script
                    .env([
                        EzStateValue::Int32(CHECK_SPECIFIC_PERSON_GENERIC_DIALOG_IS_OPEN),
                        EzStateValue::Int32(0),
                    ])?
                    .into();

                if !(specific_person_menu_is_open == 1
                    && specific_person_generic_dialog_is_open == 0)
                {
                    TalkScriptDemoState::EnterMainMenu
                } else {
                    TalkScriptDemoState::WhileSell
                }
            }
        };
        Ok(())
    }
}

unsafe impl Send for TalkScriptDemo {}

#[unsafe(no_mangle)]
/// # Safety
/// This is exposed this way such that libraryloader can call it. Do not call this yourself.
pub unsafe extern "C" fn DllMain(_hmodule: u64, reason: u32) -> bool {
    // Exit early if we're not attaching a DLL
    if reason != 1 {
        return true;
    }

    std::thread::spawn(move || {
        wait_for_system_init(&Program::current(), Duration::MAX)
            .expect("Timeout waiting for system init");

        let mut demo = Box::new(TalkScriptDemo {
            talk_script: TalkScript::new(
                BlockId::none(),
                1000,
                FieldInsHandle {
                    block_id: BlockId::none(),
                    selector: FieldInsSelector(0),
                },
            ),
            state: TalkScriptDemoState::Idle,
        });

        let cs_task = unsafe { CSTaskImp::instance().unwrap() };
        cs_task.run_recurring(
            move |_: &FD4TaskData| {
                if let Ok(world_chr_man) = unsafe { WorldChrMan::instance() }
                    && let Some(ref mut main_player) = world_chr_man.main_player
                {
                    demo.talk_script.npc_talk.base.field_ins_handle =
                        main_player.chr_ins.field_ins_handle;

                    if let Err(e) = demo.step() {
                        println!("{:?}", e);
                        demo.state = TalkScriptDemoState::Idle;
                    }
                }
            },
            CSTaskGroupIndex::FrameBegin,
        );
    });

    true
}
