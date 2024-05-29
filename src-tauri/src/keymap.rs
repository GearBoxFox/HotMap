pub mod keymap {
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use serde::{Serialize, Deserialize};
    use rdev::Key;
    use tauri::api::path;
    use tauri::Config;
    use num_traits::FromPrimitive;

    use crate::programmable_keys::programmable_keys::ProgrammableKeys;

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
    pub enum MacroAction {
        Print(String),
        Tap(Key),
        Press(Key),
        Release(Key),
        Delay(i32),
        None
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
    pub enum MacroType {
        Once,
        Toggle,
        Repeat(i32)
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
    pub struct MacroKey {
        programmable_key: ProgrammableKeys,
        macro_type: MacroType,
        actions: Vec<MacroAction>
    }

    #[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
    pub struct Keymap {
        map_name: String,
        button_count: i32,
        buttons: Vec<MacroKey>
    }

    impl Keymap {
        /// Create a new blank keymap with a certain number of buttons
        pub fn new(name: String, count: i32) -> Keymap {
            let mut blank_buttons: Vec<MacroKey> = Vec::new();

            // Fill a blank vec of macros, incrementing for each new button
            for i in 0..count {
                blank_buttons.push(
                    MacroKey {
                        programmable_key: FromPrimitive::from_i32(i).unwrap(),
                        macro_type: MacroType::Once,
                        actions: vec![MacroAction::None]
                    }
                )
            }

            Keymap{
                map_name: name,
                button_count: count,
                buttons: blank_buttons
            }
        }

        /// Load a keymap file into a Keymap struct, returning an error
        /// if no file is found.
        pub fn load_from_file(keymap_name: String, app_config: &Config) -> Result<Keymap, io::Error> {
            // load keymap json file from appdata directory
            let mut keymap_path = path::app_data_dir(app_config).unwrap();
            keymap_path.push("/keymaps/".to_owned() + &*keymap_name + ".json");

            let file_result = File::open(keymap_path);

            // check if file exists, if not return an error
            match file_result {
                Ok(_) => {
                    let mut keymap_json = String::new();
                    file_result.unwrap().read_to_string(&mut keymap_json).expect("Failed to read keymap file!");
                    let keymap: Keymap = serde_json::from_str(&keymap_json).unwrap();
                    Ok(keymap)
                }
                Err(_) => {
                    eprintln!("Failed to find keymap file!");
                    Err(file_result.err().unwrap())
                }
            }
        }
    }
}
