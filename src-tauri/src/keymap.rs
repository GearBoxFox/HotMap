use std::fs::{File, OpenOptions};
use std::io;
use std::io::{ErrorKind, Read, Write};
use std::ops::Add;
use std::sync::{Arc, Mutex};

use num_traits::FromPrimitive;
use rdev::Key;
use serde::{Deserialize, Serialize};
use tauri::api::path;
use tauri::Config;

use crate::programmable_keys::ProgrammableKeys;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum MacroAction {
    Print(String),
    Tap(Key),
    Press(Key),
    Release(Key),
    Delay(u64),
    None,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum MacroType {
    Once,
    Toggle,
    Repeat(i32),
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct MacroKey {
    pub programmable_key: ProgrammableKeys,
    pub macro_type: MacroType,
    pub actions: Vec<MacroAction>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct Keymap {
    pub(crate) map_name: String,
    pub(crate) button_count: i32,
    pub buttons: Vec<MacroKey>,
}

impl Keymap {
    /// Create a new blank keymap with a certain number of buttons
    pub fn new(name: String, count: i32) -> Keymap {
        let mut blank_buttons: Vec<MacroKey> = Vec::new();

        // Fill a blank vec of macros, incrementing for each new button
        for i in 0..count {
            blank_buttons.push(MacroKey {
                programmable_key: FromPrimitive::from_i32(i).unwrap(),
                macro_type: MacroType::Once,
                actions: vec![MacroAction::None],
            })
        }

        Keymap {
            map_name: name,
            button_count: count,
            buttons: blank_buttons,
        }
    }

    /// Load a keymap json file into a Keymap struct, returning an error
    /// if no file is found.
    pub fn load_from_file(keymap_name: String, app_config: &Config) -> Result<Keymap, io::Error> {
        // load keymap json file from appdata directory
        let mut keymap_path = path::app_data_dir(app_config).unwrap();
        keymap_path.push("/keymaps/".to_owned() + &*keymap_name + ".json");

        // check if file exists, if not return an error
        let mut keymap_file = match File::open(keymap_path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("Failed to find keymap file!");
                return Err(err);
            }
        };

        let mut keymap_json = String::new();
        keymap_file
            .read_to_string(&mut keymap_json)
            .expect("Failed to read keymap file!");
        let keymap: Keymap = serde_json::from_str(&keymap_json).unwrap();
        Ok(keymap)
    }

    /// Saves a Keymap struct to a json file
    pub fn save_to_file(
        keymap_arc: Arc<Mutex<Keymap>>,
        app_config: &Config,
    ) -> Result<(), io::Error> {
        // establish a lock on the keymap while reading
        let keymap = match keymap_arc.lock() {
            Ok(keymap) => keymap,
            Err(err) => {
                eprintln!("Failed to establish lock on keymap file to save: {}", err);
                return Err(io::Error::new(ErrorKind::PermissionDenied, err.to_string()));
            }
        };

        // create the path to keymap json file in the appdata directory
        let mut keymap_path = path::local_data_dir().unwrap();
        let binding = keymap.map_name.to_string().add(".json");
        let file_name = binding.as_str();
        keymap_path.extend(["hotmap", "keymaps"]);

        if !keymap_path.exists() {
            std::fs::create_dir_all(&keymap_path)?;
        }

        keymap_path.push(file_name);

        let mut keymap_file = File::create(keymap_path)?;

        match keymap_file.write_all(
            serde_json::to_string(&keymap.clone())
                .expect("Failed to parse keymap file!")
                .as_bytes(),
        ) {
            Ok(_) => Ok(()),
            Err(err) => {
                eprintln!("Failed to write data to keymap file!");
                Err(err)
            }
        }
    }
}
