use std::sync::{Arc, Mutex};

use crate::keymap::Keymap;

#[tauri::command]
fn send_keymap(keymap: Arc<Mutex<Keymap>>) -> Keymap {
    let borrowed_keymap = match keymap.lock() {
        Ok(keymap) => keymap,
        Err(err) => panic!("Could acquire lock on keymap, error: {}", err.to_string())
    };
    borrowed_keymap.to_owned()
}