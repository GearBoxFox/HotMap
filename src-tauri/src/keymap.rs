use std::fs::{File, OpenOptions};
use std::io;
use std::io::{Error, Read, Write};
use std::ops::Add;

use serde::{Deserialize, Serialize};
use tauri::api::path;

use crate::programmable_keys::ProgrammableKeys;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub enum Key {
    Alt,
    Backspace,
    CapsLock,
    ControlLeft,
    ControlRight,
    Delete,
    DownArrow,
    End,
    Escape,
    F1,
    F10,
    F11,
    F12,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    Home,
    LeftArrow,
    // also known as "windows", "super", and "command"
    MetaLeft,
    // also known as "windows", "super", and "command"
    MetaRight,
    PageDown,
    PageUp,
    Return,
    RightArrow,
    ShiftLeft,
    ShiftRight,
    Space,
    Tab,
    UpArrow,
    PrintScreen,
    ScrollLock,
    Pause,
    NumLock,
    BackQuote,
    Num1,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num0,
    Minus,
    Equal,
    KeyQ,
    KeyW,
    KeyE,
    KeyR,
    KeyT,
    KeyY,
    KeyU,
    KeyI,
    KeyO,
    KeyP,
    LeftBracket,
    RightBracket,
    KeyA,
    KeyS,
    KeyD,
    KeyF,
    KeyG,
    KeyH,
    KeyJ,
    KeyK,
    KeyL,
    SemiColon,
    Quote,
    BackSlash,
    IntlBackslash,
    KeyZ,
    KeyX,
    KeyC,
    KeyV,
    KeyB,
    KeyN,
    KeyM,
    Comma,
    Dot,
    Slash,
    Insert,
    KpPlus,
    KpMultiply,
    Unknown(u32),
}

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
        for i in 1..count + 1 {
            blank_buttons.push(MacroKey {
                programmable_key: ProgrammableKeys::get_from_index(i),
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
    pub fn load_from_file(keymap_name: String) -> Result<Keymap, Error> {
        // create the path to keymap json file in the appdata directory
        let mut keymap_path = path::local_data_dir().unwrap();
        let binding = keymap_name.to_string().add(".json");
        let file_name = binding.as_str();
        keymap_path.extend(["hotmap", "keymaps"]);
        keymap_path.push(file_name);

        if !keymap_path.exists() {
            println!("Didn't find existing keymap file");
            return Ok(Keymap::new("keymap".to_string(), 1));
        }

        // check if file exists, if not return an error
        let mut keymap_file = match OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(keymap_path)
        {
            Ok(file) => file,
            Err(err) => {
                return Err(err);
            }
        };

        let mut keymap_json = String::new();
        keymap_file
            .read_to_string(&mut keymap_json)
            .expect("Failed to read keymap file!");

        // copy values from json file into the used keymap
        let temp: Keymap = serde_json::from_str(&keymap_json).unwrap();

        println!("Keymap file exists: {:?}", temp);

        Ok(temp)
    }

    /// Saves a Keymap struct to a json file
    pub fn save_to_file(keymap: Keymap) -> Result<(), io::Error> {
        // establish a lock on the keymap while reading
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
