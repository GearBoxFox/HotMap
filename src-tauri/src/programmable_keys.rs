use std::cmp::PartialEq;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use enigo::{Enigo, Keyboard};
use serde::{Deserialize, Serialize};

use crate::keymap::{Key, Keymap, MacroAction, MacroKey};

/// handles all the actions bound to a macro key
fn handle_macro_key(macro_key: MacroKey, mut simulator: Enigo) {
    for action in macro_key.actions {
        match action {
            MacroAction::Print(string) => simulator.text(&*string).unwrap(),
            MacroAction::Tap(key) => simulator
                .key(match_key_to_enigo(key), enigo::Direction::Click)
                .unwrap(),
            MacroAction::Press(key) => simulator
                .key(match_key_to_enigo(key), enigo::Direction::Press)
                .unwrap(),
            MacroAction::Release(key) => simulator
                .key(match_key_to_enigo(key), enigo::Direction::Release)
                .unwrap(),
            MacroAction::Delay(ms) => thread::sleep(Duration::from_millis(ms)),
            MacroAction::None => {}
        }
    }
}

fn match_key_to_enigo(key: Key) -> enigo::Key {
    match key {
        Key::Alt => enigo::Key::Alt,
        Key::Backspace => enigo::Key::Backspace,
        Key::CapsLock => enigo::Key::CapsLock,
        Key::ControlLeft => enigo::Key::LControl,
        Key::ControlRight => enigo::Key::RControl,
        Key::Delete => enigo::Key::Delete,
        Key::DownArrow => enigo::Key::DownArrow,
        Key::End => enigo::Key::End,
        Key::Escape => enigo::Key::Escape,
        Key::F1 => enigo::Key::F1,
        Key::F10 => enigo::Key::F10,
        Key::F11 => enigo::Key::F11,
        Key::F12 => enigo::Key::F12,
        Key::F2 => enigo::Key::F2,
        Key::F3 => enigo::Key::F3,
        Key::F4 => enigo::Key::F4,
        Key::F5 => enigo::Key::F5,
        Key::F6 => enigo::Key::F6,
        Key::F7 => enigo::Key::F7,
        Key::F8 => enigo::Key::F8,
        Key::F9 => enigo::Key::F9,
        Key::Home => enigo::Key::Home,
        Key::LeftArrow => enigo::Key::LeftArrow,
        Key::MetaLeft => enigo::Key::Meta,
        Key::MetaRight => enigo::Key::Meta,
        Key::PageDown => enigo::Key::PageDown,
        Key::PageUp => enigo::Key::PageUp,
        Key::Return => enigo::Key::Return,
        Key::RightArrow => enigo::Key::RightArrow,
        Key::ShiftLeft => enigo::Key::LShift,
        Key::ShiftRight => enigo::Key::RShift,
        Key::Space => enigo::Key::Space,
        Key::Tab => enigo::Key::Tab,
        Key::UpArrow => enigo::Key::UpArrow,
        Key::PrintScreen => enigo::Key::Print,
        Key::Pause => enigo::Key::Pause,
        Key::NumLock => enigo::Key::Numlock,
        Key::BackQuote => enigo::Key::Unicode('\''),
        Key::Num1 => enigo::Key::Unicode('1'),
        Key::Num2 => enigo::Key::Unicode('2'),
        Key::Num3 => enigo::Key::Unicode('3'),
        Key::Num4 => enigo::Key::Unicode('4'),
        Key::Num5 => enigo::Key::Unicode('5'),
        Key::Num6 => enigo::Key::Unicode('6'),
        Key::Num7 => enigo::Key::Unicode('7'),
        Key::Num8 => enigo::Key::Unicode('8'),
        Key::Num9 => enigo::Key::Unicode('9'),
        Key::Num0 => enigo::Key::Unicode('0'),
        Key::Minus => enigo::Key::Unicode('-'),
        Key::Equal => enigo::Key::Unicode('='),
        Key::KeyQ => enigo::Key::Unicode('q'),
        Key::KeyW => enigo::Key::Unicode('w'),
        Key::KeyE => enigo::Key::Unicode('e'),
        Key::KeyR => enigo::Key::Unicode('r'),
        Key::KeyT => enigo::Key::Unicode('t'),
        Key::KeyY => enigo::Key::Unicode('y'),
        Key::KeyU => enigo::Key::Unicode('u'),
        Key::KeyI => enigo::Key::Unicode('i'),
        Key::KeyO => enigo::Key::Unicode('o'),
        Key::KeyP => enigo::Key::Unicode('p'),
        Key::LeftBracket => enigo::Key::Unicode('['),
        Key::RightBracket => enigo::Key::Unicode(']'),
        Key::KeyA => enigo::Key::Unicode('a'),
        Key::KeyS => enigo::Key::Unicode('s'),
        Key::KeyD => enigo::Key::Unicode('d'),
        Key::KeyF => enigo::Key::Unicode('f'),
        Key::KeyG => enigo::Key::Unicode('g'),
        Key::KeyH => enigo::Key::Unicode('h'),
        Key::KeyJ => enigo::Key::Unicode('j'),
        Key::KeyK => enigo::Key::Unicode('k'),
        Key::KeyL => enigo::Key::Unicode('l'),
        Key::SemiColon => enigo::Key::Unicode(';'),
        Key::Quote => enigo::Key::Unicode('\''),
        Key::BackSlash => enigo::Key::Unicode('\\'),
        Key::IntlBackslash => enigo::Key::Unicode('\\'),
        Key::KeyZ => enigo::Key::Unicode('z'),
        Key::KeyX => enigo::Key::Unicode('x'),
        Key::KeyC => enigo::Key::Unicode('c'),
        Key::KeyV => enigo::Key::Unicode('v'),
        Key::KeyB => enigo::Key::Unicode('b'),
        Key::KeyN => enigo::Key::Unicode('n'),
        Key::KeyM => enigo::Key::Unicode('m'),
        Key::Comma => enigo::Key::Unicode(','),
        Key::Dot => enigo::Key::Unicode('.'),
        Key::Slash => enigo::Key::Unicode('/'),
        Key::Insert => enigo::Key::Insert,
        Key::KpPlus => enigo::Key::Unicode('+'),
        Key::KpMultiply => enigo::Key::Unicode('*'),
        Key::Unknown(num) => enigo::Key::Other(num.into()),
    }
}

// https://docs.qmk.fm/#/feature_programmable_button
#[cfg(target_os = "linux")]
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
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
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub enum ProgrammableKeys {
    MACROUNKNOWN = 0,
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

    pub fn get_from_index(index: i32) -> ProgrammableKeys {
        match index {
            1 => ProgrammableKeys::MACRO1,
            2 => ProgrammableKeys::MACRO2,
            3 => ProgrammableKeys::MACRO3,
            4 => ProgrammableKeys::MACRO4,
            5 => ProgrammableKeys::MACRO5,
            6 => ProgrammableKeys::MACRO6,
            7 => ProgrammableKeys::MACRO7,
            8 => ProgrammableKeys::MACRO8,
            9 => ProgrammableKeys::MACRO9,
            10 => ProgrammableKeys::MACRO10,
            11 => ProgrammableKeys::MACRO11,
            12 => ProgrammableKeys::MACRO12,
            13 => ProgrammableKeys::MACRO13,
            14 => ProgrammableKeys::MACRO14,
            15 => ProgrammableKeys::MACRO15,
            16 => ProgrammableKeys::MACRO16,
            17 => ProgrammableKeys::MACRO17,
            18 => ProgrammableKeys::MACRO18,
            19 => ProgrammableKeys::MACRO19,
            20 => ProgrammableKeys::MACRO20,
            21 => ProgrammableKeys::MACRO21,
            22 => ProgrammableKeys::MACRO22,
            23 => ProgrammableKeys::MACRO23,
            24 => ProgrammableKeys::MACRO24,
            25 => ProgrammableKeys::MACRO25,
            26 => ProgrammableKeys::MACRO26,
            27 => ProgrammableKeys::MACRO27,
            28 => ProgrammableKeys::MACRO28,
            29 => ProgrammableKeys::MACRO29,
            30 => ProgrammableKeys::MACRO30,
            31 => ProgrammableKeys::MACRO31,
            32 => ProgrammableKeys::MACRO32,
            _ => ProgrammableKeys::MACROUNKNOWN,
        }
    }

    pub fn process_keys(key: ProgrammableKeys, keymap_arc: &Arc<Mutex<Keymap>>, simulator: Enigo) {
        let borrowed_map = match keymap_arc.lock() {
            Ok(keymap) => Some(keymap),
            Err(err) => {
                eprintln!("Error retrieving keymap lock: {}", err);
                None
            }
        };

        match borrowed_map {
            Some(keymap) => {
                let matching_key = match keymap
                    .buttons
                    .clone()
                    .into_iter()
                    .find(|k| k.programmable_key == key)
                {
                    None => return,
                    Some(key) => key,
                };

                handle_macro_key(matching_key, simulator);
            }
            None => (),
        }
    }
}
