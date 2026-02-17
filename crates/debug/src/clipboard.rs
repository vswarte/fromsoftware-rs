use clipboard_win::*;
use hudhook::imgui::ClipboardBackend;

/// A backend that uses the Windows clipboard to implement clipboard access for
/// imgui.
pub struct WindowsClipboardBackend {}

impl ClipboardBackend for WindowsClipboardBackend {
    fn get(&mut self) -> Option<String> {
        let Ok(_c) = Clipboard::new_attempts(10) else {
            return None;
        };

        let mut result = String::new();
        formats::Unicode
            .read_clipboard(&mut result)
            .ok()
            .map(|_| result)
    }

    fn set(&mut self, value: &str) {
        let Ok(_c) = Clipboard::new_attempts(10) else {
            return;
        };

        let _ = formats::Unicode.write_clipboard(&value);
    }
}
