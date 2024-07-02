# HotMap

Hotmap is an intuitive desktop app designed to take advantage of
the [QMK Programmable Button](https://docs.qmk.fm/features/programmable_button) keycodes,
allowing for on-the-fly updating of keybindings without having to recompile or deploy firmware.

## Features

- **32 Unique Programmable Buttons**
- **5 Unique macro actions**
    - Tap
    - Press
    - Release
    - Print
    - Delay
- **Easy to navigate UI**
- **Windows and Linux Support**
    - OsX support planned

## Usage

1. Install the latest release for your platform in the Releases page

> ![WARNING] \
> Linux users will have to make sure to run Hotmap with elevated privileges for
> the backend to run properly

2. Add Programmable Button to your QMK firmware

- Buttons must be a range starting at 1, without skipping any

3. Add buttons to match your keyboard
4. Configure the macro for each button

## Building

### Requirements

- Rust and Cargo
- Node.js and NPM

1. Clone the latest version of the repository from `https://www.github.com/GearBoxFox/HotMap.git`
2. Run `npm install`
3. Download the tauri cli tool `cargo install tauri-cli`
4. Run the development build `cargo tauri dev`
5. Run the production build `cargo tauri build`

- On linux set the environment variable `NO_STRIP=true` to build the AppImage properly
