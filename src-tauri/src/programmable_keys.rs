use std::thread;
use std::time::Duration;

use rdev::{EventType, Key, simulate};
use serde::{Deserialize, Serialize};

use crate::keymap::{MacroAction, MacroKey};

static DELAY: Duration = Duration::from_millis(20);

/// Sends a raw keypress
fn send(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(_) => {
            eprintln!("Could not simulate event type {:?}!", event_type);
        }
    }

    thread::sleep(DELAY);
}

/// Sends all the key presses needed to send a string
fn send_string(string: String) {
    for char in string.chars() {
        if char.is_uppercase() {
            send(&EventType::KeyPress(Key::ShiftLeft));
        }

        send(&EventType::KeyPress(char_to_key_event(
            char.to_lowercase().next().unwrap(),
        )));
        send(&EventType::KeyRelease(char_to_key_event(
            char.to_lowercase().next().unwrap(),
        )));

        if char.is_uppercase() {
            send(&EventType::KeyRelease(Key::ShiftLeft));
        }
    }
}

/// Converts a char into a rdev KeyEvent
fn char_to_key_event(char: char) -> Key {
    match char {
        'a' => Key::KeyA,
        'b' => Key::KeyB,
        'c' => Key::KeyC,
        'd' => Key::KeyD,
        'e' => Key::KeyE,
        'f' => Key::KeyF,
        'g' => Key::KeyG,
        'h' => Key::KeyH,
        'i' => Key::KeyI,
        'j' => Key::KeyJ,
        'k' => Key::KeyK,
        'l' => Key::KeyL,
        'm' => Key::KeyM,
        'n' => Key::KeyN,
        'o' => Key::KeyO,
        'p' => Key::KeyP,
        'q' => Key::KeyQ,
        'r' => Key::KeyR,
        's' => Key::KeyS,
        't' => Key::KeyT,
        'u' => Key::KeyU,
        'v' => Key::KeyV,
        'w' => Key::KeyW,
        'x' => Key::KeyX,
        'y' => Key::KeyY,
        'z' => Key::KeyZ,
        _ => Key::Unknown(char.to_digit(10).unwrap()),
    }
}

/// handles all the actions bound to a macro key
fn handle_macro_key(macro_key: MacroKey) {
    for action in macro_key.actions {
        match action {
            MacroAction::Print(string) => send_string(string),
            MacroAction::Tap(key) => {
                send(&EventType::KeyPress(key));
                send(&EventType::KeyRelease(key));
            }
            MacroAction::Press(key) => send(&EventType::KeyPress(key)),
            MacroAction::Release(key) => send(&EventType::KeyRelease(key)),
            MacroAction::Delay(ms) => thread::sleep(Duration::from_millis(ms)),
            MacroAction::None => {}
        }
    }
}

// https://docs.qmk.fm/#/feature_programmable_button
#[cfg(target_os = "linux")]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq, FromPrimitive)]
pub enum ProgrammableKeys {
    MACROUNKNOWN = 0,
    MACRO1 = 656,
    MACRO2 = 657,
    MACRO3 = 658,
    MACRO4 = 659,
    MACRO5 = 660,
    MACRO6 = 661,
    MACRO7 = 662,
    MACRO8 = 663,
    MACRO9 = 664,
    MACRO10 = 665,
    MACRO11 = 666,
    MACRO12 = 667,
    MACRO13 = 668,
    MACRO14 = 669,
    MACRO15 = 670,
    MACRO16 = 671,
    MACRO17 = 672,
    MACRO18 = 673,
    MACRO19 = 674,
    MACRO20 = 675,
    MACRO21 = 676,
    MACRO22 = 677,
    MACRO23 = 678,
    MACRO24 = 679,
    MACRO25 = 680,
    MACRO26 = 681,
    MACRO27 = 682,
    MACRO28 = 683,
    MACRO29 = 684,
    MACRO30 = 685,
    MACRO31 = 686,
    MACRO32 = 687,
}

#[cfg(target_os = "windows")]
#[derive(Debug, Serialize, Deserialize, FromPrimitive)]
pub enum ProgrammableKeys {
    MACROUNKOWN = 0,
    MACRO1 = 261,
    MACRO2 = 517,
    MACRO3 = 1029,
    MACRO4 = 2053,
    MACRO5 = 4101,
    MACRO6 = 8197,
    MACRO7 = 16389,
    MACRO8 = 32773,
    MACRO9 = 65541,
    MACRO10 = 131077,
    MACRO11 = 262149,
    MACRO12 = 200,
    MACRO13 = 201,
    MACRO14 = 202,
    MACRO15 = 203,
    MACRO16 = 204,
    MACRO17 = 205,
    MACRO18 = 206,
    MACRO19 = 207,
    MACRO20 = 134217733,
    MACRO21 = 208,
    MACRO22 = 209,
    MACRO23 = 210,
    MACRO24 = 211,
    MACRO25 = 212,
    MACRO26 = 213,
    MACRO27 = 214,
    MACRO28 = 215,
    MACRO29 = 216,
    MACRO30 = 217,
    MACRO31 = 218,
    MACRO32 = 219,
}

impl ProgrammableKeys {
    #[cfg(target_os = "linux")]
    pub fn from_u32(value: u32) -> ProgrammableKeys {
        match value {
            656 => ProgrammableKeys::MACRO1,
            657 => ProgrammableKeys::MACRO2,
            658 => ProgrammableKeys::MACRO3,
            659 => ProgrammableKeys::MACRO4,
            660 => ProgrammableKeys::MACRO5,
            661 => ProgrammableKeys::MACRO6,
            662 => ProgrammableKeys::MACRO7,
            663 => ProgrammableKeys::MACRO8,
            664 => ProgrammableKeys::MACRO9,
            665 => ProgrammableKeys::MACRO10,
            666 => ProgrammableKeys::MACRO11,
            667 => ProgrammableKeys::MACRO12,
            668 => ProgrammableKeys::MACRO13,
            669 => ProgrammableKeys::MACRO14,
            670 => ProgrammableKeys::MACRO15,
            671 => ProgrammableKeys::MACRO16,
            672 => ProgrammableKeys::MACRO17,
            673 => ProgrammableKeys::MACRO18,
            674 => ProgrammableKeys::MACRO19,
            675 => ProgrammableKeys::MACRO20,
            676 => ProgrammableKeys::MACRO21,
            677 => ProgrammableKeys::MACRO22,
            678 => ProgrammableKeys::MACRO23,
            679 => ProgrammableKeys::MACRO24,
            680 => ProgrammableKeys::MACRO25,
            681 => ProgrammableKeys::MACRO26,
            682 => ProgrammableKeys::MACRO27,
            683 => ProgrammableKeys::MACRO28,
            684 => ProgrammableKeys::MACRO29,
            685 => ProgrammableKeys::MACRO30,
            686 => ProgrammableKeys::MACRO31,
            687 => ProgrammableKeys::MACRO32,
            _ => ProgrammableKeys::MACROUNKNOWN,
        }
    }

    #[cfg(target_os = "windows")]
    pub fn from_u32(value: u32) -> ProgrammableKeys {
        match value {
            261 => ProgrammableKeys::MACRO1,
            517 => ProgrammableKeys::MACRO2,
            1029 => ProgrammableKeys::MACRO3,
            2053 => ProgrammableKeys::MACRO4,
            4101 => ProgrammableKeys::MACRO5,
            8197 => ProgrammableKeys::MACRO6,
            16389 => ProgrammableKeys::MACRO7,
            32773 => ProgrammableKeys::MACRO8,
            65541 => ProgrammableKeys::MACRO9,
            131077 => ProgrammableKeys::MACRO10,
            262149 => ProgrammableKeys::MACRO11,
            524293 => ProgrammableKeys::MACRO12,
            1048581 => ProgrammableKeys::MACRO13,
            2097157 => ProgrammableKeys::MACRO14,
            4194309 => ProgrammableKeys::MACRO15,
            8388613 => ProgrammableKeys::MACRO16,
            16777221 => ProgrammableKeys::MACRO17,
            33554437 => ProgrammableKeys::MACRO18,
            207 => ProgrammableKeys::MACRO19,
            134217733 => ProgrammableKeys::MACRO20,
            208 => ProgrammableKeys::MACRO21,
            209 => ProgrammableKeys::MACRO22,
            210 => ProgrammableKeys::MACRO23,
            211 => ProgrammableKeys::MACRO24,
            212 => ProgrammableKeys::MACRO25,
            213 => ProgrammableKeys::MACRO26,
            214 => ProgrammableKeys::MACRO27,
            215 => ProgrammableKeys::MACRO28,
            216 => ProgrammableKeys::MACRO29,
            217 => ProgrammableKeys::MACRO30,
            218 => ProgrammableKeys::MACRO31,
            219 => ProgrammableKeys::MACRO32,
            _ => ProgrammableKeys::MACROUNKNOWN,
        }
    }

    pub async fn process_keys(key: ProgrammableKeys) {
        match key {
            ProgrammableKeys::MACRO1 => {
                println!("MACRO1 PRESSED");
            }
            ProgrammableKeys::MACRO2 => {
                println!("MACRO2 PRESSED");
            }
            ProgrammableKeys::MACRO3 => {
                println!("MACRO3 PRESSED");
            }
            ProgrammableKeys::MACRO4 => {
                println!("MACRO4 PRESSED");
            }
            ProgrammableKeys::MACRO5 => {
                println!("MACRO5 PRESSED");
            }
            ProgrammableKeys::MACRO6 => {
                println!("MACRO6 PRESSED");
            }
            ProgrammableKeys::MACRO7 => {
                println!("MACRO7 PRESSED");
            }
            ProgrammableKeys::MACRO8 => {
                println!("MACRO8 PRESSED");
            }
            ProgrammableKeys::MACRO9 => {
                println!("MACRO9 PRESSED");
            }
            ProgrammableKeys::MACRO10 => {
                println!("MACRO10 PRESSED");
            }
            ProgrammableKeys::MACRO11 => {
                println!("MACRO11 PRESSED");
            }
            ProgrammableKeys::MACRO12 => {
                println!("MACRO12 PRESSED");
            }
            ProgrammableKeys::MACRO13 => {
                println!("MACRO13 PRESSED");
            }
            ProgrammableKeys::MACRO14 => {
                println!("MACRO14 PRESSED");
            }
            ProgrammableKeys::MACRO15 => {
                println!("MACRO15 PRESSED");
            }
            ProgrammableKeys::MACRO16 => {
                println!("MACRO16 PRESSED");
            }
            ProgrammableKeys::MACRO17 => {
                println!("MACRO17 PRESSED");
            }
            ProgrammableKeys::MACRO18 => {
                println!("MACRO18 PRESSED");
            }
            ProgrammableKeys::MACRO19 => {
                println!("MACRO19 PRESSED");
            }
            ProgrammableKeys::MACRO20 => {
                println!("MACRO20 PRESSED");
            }
            ProgrammableKeys::MACRO21 => {
                println!("MACRO21 PRESSED");
            }
            ProgrammableKeys::MACRO22 => {
                println!("MACRO22 PRESSED");
            }
            ProgrammableKeys::MACRO23 => {
                println!("MACRO23 PRESSED");
            }
            ProgrammableKeys::MACRO24 => {
                println!("MACRO24 PRESSED");
            }
            ProgrammableKeys::MACRO25 => {
                println!("MACRO25 PRESSED");
            }
            ProgrammableKeys::MACRO26 => {
                println!("MACRO26 PRESSED");
            }
            ProgrammableKeys::MACRO27 => {
                println!("MACRO27 PRESSED");
            }
            ProgrammableKeys::MACRO28 => {
                println!("MACRO28 PRESSED");
            }
            ProgrammableKeys::MACRO29 => {
                println!("MACRO29 PRESSED");
            }
            ProgrammableKeys::MACRO30 => {
                println!("MACRO30 PRESSED");
            }
            ProgrammableKeys::MACRO31 => {
                println!("MACRO31 PRESSED");
            }
            ProgrammableKeys::MACRO32 => {
                println!("MACRO32 PRESSED");
            }
            _ => {
                println!("MACROUNKOWN PRESSED");
            }
        }
    }
}
