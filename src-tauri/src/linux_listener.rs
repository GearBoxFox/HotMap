use std::fs::{File, OpenOptions};
use std::os::unix::{
    fs::OpenOptionsExt,
    io::{FromRawFd, IntoRawFd, RawFd},
};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::{thread, time};

use input::event::keyboard::KeyboardEventTrait;
use input::event::KeyboardEvent;
use input::{Event, Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};

use crate::programmable_keys::ProgrammableKeys;

const LIBINPUT_FETCH_DELAY: time::Duration = time::Duration::from_millis(20);

struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<RawFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into_raw_fd())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: RawFd) {
        unsafe {
            File::from_raw_fd(fd);
        }
    }
}

fn watch_events(input: Libinput, queue: &Arc<Mutex<Vec<ProgrammableKeys>>>) {
    loop {
        let mut borrowed_input: Libinput = input.clone();
        match borrowed_input.dispatch() {
            Ok(_) => {
                for event in borrowed_input {
                    if let Event::Keyboard(KeyboardEvent::Key(event)) = event {
                        if event.key_state() == input::event::keyboard::KeyState::Pressed {
                            let prog_key = ProgrammableKeys::from_u32(event.key());
                            match prog_key {
                                ProgrammableKeys::MACROUNKNOWN => {
                                    eprintln!("MACROUNKOWN PRESSED");
                                }
                                _ => match queue.lock() {
                                    Ok(mut borrowed_queue) => {
                                        borrowed_queue.push(prog_key);
                                    }
                                    Err(e) => {
                                        eprintln!("Error locking queue: {:?}", e);
                                    }
                                },
                            }
                        }
                    }
                }
            }
            Err(err) => {
                eprintln!("Failed to dispatch libinput: {}", err);
            }
        }
        // sleep here so it doesn't eat up all the CPU
        thread::sleep(LIBINPUT_FETCH_DELAY);
    }
}

pub fn linux_start(queue: &Arc<Mutex<Vec<ProgrammableKeys>>>) {
    let mut input = Libinput::new_with_udev(Interface);
    println!("Created input device!");

    match input.udev_assign_seat("seat0") {
        Ok(_) => {
            watch_events(input, &queue);
        }
        Err(_) => eprintln!("Failed to assign seat"),
    }
}
