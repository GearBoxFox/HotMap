// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Mutex, Arc};
use std::time;
use std::io::Error;
use std::thread;

mod programmable_keys;
use crate::programmable_keys::programmable_keys::ProgrammableKeys;

const QUEUE_CHECKING_DELAY: time::Duration = time::Duration::from_millis(20);

#[cfg(target_os = "linux")]
mod linux_listener;

#[cfg(target_os = "windows")]
mod windows_listener;

#[tokio::main(flavor = "multi_thread", worker_threads = 1)]
async fn main() -> Result<(), Error> {
    // Handle keyboard presses
    let programmable_keys_vec: Vec<ProgrammableKeys> = Vec::new();
    let programmable_keys_arc = Arc::new(Mutex::new(programmable_keys_vec));

    let queue = programmable_keys_arc.clone();
    tokio::spawn(async move{
        loop {
            std::thread::sleep(QUEUE_CHECKING_DELAY);

            let retrieved_key = match queue.lock() {
                Ok(mut borrowed_queue) => {
                    borrowed_queue.pop()
                },
                Err(e) => {
                    eprintln!("Error locking queue: {:?}", e);
                    None
                }
            };

            match retrieved_key {
                Some(key) => {
                    ProgrammableKeys::process_keys(key).await;
                },
                None => {}
            }
        }
    });

    thread::spawn(move || {
      #[cfg(target_os = "linux")]
      linux_listener::linux_start(&programmable_keys_arc);

      #[cfg(target_os = "windows")]
      windows_listener::windows_start(&programmable_keys_arc).await;
      }
    );

    // Create tauri app
    tauri::Builder::default()
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

    Ok(())
}

