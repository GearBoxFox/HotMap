use std::sync::{Arc, Mutex};

use crate::keymap::{Keymap, MacroKey};

#[tauri::command]
pub fn send_keymap(state: tauri::State<Arc<Mutex<Keymap>>>) -> Keymap {
    let keymap_clone = match state.lock() {
        Ok(keymap) => keymap,
        Err(_) => {
            panic!("Failed to acquire keymap lock")
        }
    };

    Keymap {
        map_name: keymap_clone.map_name.clone(),
        buttons: keymap_clone.buttons.clone(),
        button_count: keymap_clone.button_count,
    }
}

#[tauri::command]
pub fn add_button(button: MacroKey, state: tauri::State<Arc<Mutex<Keymap>>>) {
    let mut keymap_clone = match state.lock() {
        Ok(keymap) => keymap,
        Err(_) => {
            panic!("Failed to acquire keymap lock")
        }
    };

    keymap_clone.buttons.push(button);
}
