// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[macro_use]
extern crate num_derive;

use std::{thread, time};
use std::io::Error;
use std::sync::{Arc, Mutex};

use tauri::{CustomMenuItem, SystemTrayMenu, SystemTrayMenuItem};
use tauri::{Manager, SystemTray, SystemTrayEvent};

use crate::keymap::Keymap;
use crate::programmable_keys::ProgrammableKeys;

mod keymap;
mod programmable_keys;

const QUEUE_CHECKING_DELAY: time::Duration = time::Duration::from_millis(20);

#[cfg(target_os = "linux")]
mod linux_listener;

#[cfg(target_os = "windows")]
mod windows_listener;
mod tauri_commands;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), Error> {
    // Create dummy keymap
    let keymap: Arc<Mutex<Keymap>> = Arc::new(Mutex::new(Keymap::new(String::from("Test"), 1)));

    Keymap::load_from_file(String::from("testkeymap"), &mut keymap.clone())
        .expect("Failed to save keymap!");

    // Handle keyboard presses
    let programmable_keys_vec: Vec<ProgrammableKeys> = Vec::new();
    let programmable_keys_arc = Arc::new(Mutex::new(programmable_keys_vec));

    let queue = programmable_keys_arc.clone();
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
                    ProgrammableKeys::process_keys(key, &keymap);
                }
                None => {}
            }
        }
    });

    tokio::spawn(async move {
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
        .on_window_event(|event| match event.event() {
            tauri::WindowEvent::CloseRequested { api, .. } => {
                event.window().hide().unwrap();
                api.prevent_close();
            }
            _ => {}
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
