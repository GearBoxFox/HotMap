// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};
use std::{thread, time};

use rdev::Key;
use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray, SystemTrayEvent};

use crate::keymap::{Keymap, MacroAction, MacroKey, MacroType};
use crate::programmable_keys::ProgrammableKeys;
use crate::tauri_commands::{add_button, save_keymap, send_keymap};

mod keymap;
mod programmable_keys;
mod tauri_commands;

const QUEUE_CHECKING_DELAY: time::Duration = time::Duration::from_millis(20);

#[cfg(target_os = "linux")]
mod linux_listener;

#[cfg(target_os = "windows")]
mod windows_listener;

fn main() {
    // Create dummy keymap
    let keymap: Keymap = Keymap {
        map_name: "Test Map".to_string(),
        button_count: 1,
        buttons: vec![MacroKey {
            programmable_key: ProgrammableKeys::MACRO1,
            macro_type: MacroType::Once,
            actions: vec![
                MacroAction::None,
                MacroAction::Delay(1),
                MacroAction::Tap(Key::Backspace),
                MacroAction::Press(Key::KeyA),
                MacroAction::Release(Key::KeyA),
                MacroAction::Print("Hello, world!".to_string()),
            ],
        }],
    };

    let keymap_arc: Arc<Mutex<Keymap>> = Arc::new(Mutex::new(keymap.clone()));

    // Handle keyboard presses
    let programmable_keys_vec: Vec<ProgrammableKeys> = Vec::new();
    let programmable_keys_arc = Arc::new(Mutex::new(programmable_keys_vec));

    let queue = programmable_keys_arc.clone();
    let keymap_clone = keymap_arc.clone();
    thread::spawn(move || {
        println!("started handler thread");
        loop {
            thread::sleep(QUEUE_CHECKING_DELAY);

            let retrieved_key = match queue.lock() {
                Ok(mut borrowed_queue) => borrowed_queue.pop(),
                Err(e) => {
                    eprintln!("Error locking queue: {:?}", e);
                    None
                }
            };

            match retrieved_key {
                Some(key) => {
                    println!("Handling a keypress");
                    ProgrammableKeys::process_keys(key, &keymap_clone);
                }
                None => {}
            }
        }
    });

    thread::spawn(move || {
        #[cfg(target_os = "linux")]
        linux_listener::linux_start(&programmable_keys_arc);

        #[cfg(target_os = "windows")]
        windows_listener::windows_start(&programmable_keys_arc);
    });

    // Create tauri app
    let show = CustomMenuItem::new("show".to_string(), "Show");
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let hide = CustomMenuItem::new("hide".to_string(), "Hide");
    let tray_menu = SystemTrayMenu::new()
        .add_item(quit)
        .add_native_item(SystemTrayMenuItem::Separator)
        .add_item(show)
        .add_item(hide);
    let tray = SystemTray::new().with_menu(tray_menu);

    tauri::Builder::default()
        .manage(keymap_arc)
        .system_tray(tray)
        .on_system_tray_event(|app, event| match event {
            SystemTrayEvent::MenuItemClick { id, .. } => match id.as_str() {
                "quit" => {
                    std::process::exit(0);
                }
                "show" => {
                    let window = app.get_window("main").unwrap();
                    window.show().unwrap();
                }
                "hide" => {
                    let window = app.get_window("main").unwrap();
                    window.hide().unwrap();
                }
                _ => {}
            },
            _ => {}
        })
        .setup(|app| {
            // Event listeners for reloading the keymap on frontend
            let id = app.listen_global("reload-keymap", |event| {
                println!("got event-name with payload {:?}", event.payload());
            });

            app.emit_all("load-keymap", "").unwrap();

            Ok(())
        })
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .invoke_handler(tauri::generate_handler![
            send_keymap,
            add_button,
            save_keymap
        ])
        .build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|_app_handle, event| match event {
            tauri::RunEvent::Ready {} => _app_handle.emit_all("load-keymap", "").unwrap(),
            tauri::RunEvent::ExitRequested { api, .. } => {
                api.prevent_exit();
            }
            _ => {}
        });
}
