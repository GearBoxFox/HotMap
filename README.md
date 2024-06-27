# HotMap
_______
Hotmap is an intuitive desktop app designed to take advantage of 
the [QMK Programmable Button](https://docs.qmk.fm/features/programmable_button) keycodes,
allowing for on-the-fly updating of keybindings without having to recompile or deploy firmware.

## Features
____
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
___
1. Install the latest release for your platform in the Releases page
2. Add Programmable Button to your QMK firmware
   3. Buttons must be a range starting at 1, without skipping any
4. Add buttons to match your keyboard
5. Configure the macro for each button

> [!WARNING]  
> There is a known issue with the `Print` action where you cannot use punctuation. A fix is in the works

## Building
______
### Requirements
- Rust and Cargo
- Node.js and NPM

1. Clone the latest version of the repository from `https://www.github.com/GearBoxFox/HotMap.git`
2. Run `npm install`
3. Download the tauri cli tool `cargo install tauri-cli`
4. Run the development build `cargo tauri dev`
5. Run the production build `cargo tauri build`
