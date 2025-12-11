use std::collections::{HashMap, hash_map::Entry};
use std::sync::{LazyLock, Mutex};
use std::time::{Duration, Instant};

use windows::Win32::UI::Input::KeyboardAndMouse;

const DEBOUNCE_TIMEOUT: Duration = Duration::from_millis(250);

type DebounceMap = HashMap<i32, Instant>;
static DEBOUNCE_MAP: LazyLock<Mutex<DebounceMap>> = LazyLock::new(Default::default);

pub fn is_key_pressed(key: i32) -> bool {
    if unsafe { KeyboardAndMouse::GetKeyState(key) } < 0 {
        let now = Instant::now();

        match DEBOUNCE_MAP.lock().unwrap().entry(key) {
            Entry::Occupied(mut o) => {
                if o.get().elapsed() > DEBOUNCE_TIMEOUT {
                    o.insert(now);
                    return true;
                } else {
                    return false;
                }
            }
            Entry::Vacant(v) => {
                v.insert(now);
                return true;
            }
        }
    }

    false
}
