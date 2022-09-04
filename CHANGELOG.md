# ARMSim Project Changelog

## Details

Each *week* contains the days the project was worked on, and each day contains three sections:

1. Progress — what was accomplished, and how long it took
2. Sources (optional) — important references
3. Tests Passed (optional) — any significant tests that were passed

If a project checkpoint was completed, a note (**Checkpoint Reached: [Details]**) is to be added at the top of the day's details.

## Week 1

### 8-26-22

#### Progress [2.5hrs]

- Setup Tauri and Rust environments on Windows 10 system to provide backend and frontend servers for application
- Setup SolidJS and modified project directory to build with Vite

#### Sources
- [Setting up a Tauri application](https://tauri.app/v1/guides/getting-started/setup/html-css-js)
- [Setting up a SolidJS application](https://docs.solidjs.com/tutorials/getting-started-with-solid/installing-solid)
- [Tauri + SolidJS example](https://github.com/lukethacoder/tauri-solid-example)

### 8-27-22

#### Progress [1.5hrs]

- Added frontend and backend logging using Rust and Tauri plugins
- Researched Rust unit testing interfaces

#### Sources
- [Tauri logging plugin](https://github.com/tauri-apps/tauri-plugin-log)
- [Rust logging crate](https://docs.rs/log/latest/log/)
- [Unit testing library examples](https://doc.rust-lang.org/rust-by-example/testing/unit_testing.html)

## Week 2

### 8-28-22 [4hrs]

#### Progress

- Scaffolded more of the project's classes
- Worked on implementing global state management using Rust's multithreading Arc and Mutex structs
- Debugged difficulties with command-line arguments

#### Sources

- [Tauri state management example](https://github.com/tauri-apps/tauri/blob/dev/examples/state/main.rs)
- [Rust CLAP (Command-Line Argument Parser)](https://docs.rs/clap/latest/clap/_derive/)

### 8-29-22 [2.5hrs]

#### Progress

- Debugged more difficulties with command-line arguments
- Debugged issues with accessing global application state and building the application

#### Sources

- [Discussion thread on managing Tauri state with threads](https://github.com/tauri-apps/tauri/discussions/3059)

### 8-30-22 [2.5hrs]

#### Progress

- More working with Rust's thread management model and the Tauri application state
- Added Tauri's built-in state management to access and manage the global Memory class

#### Sources

- [Tauri State Manager](https://docs.rs/tauri/1.0.0-beta.0/tauri/trait.Manager.html)

### 8-31-22 [3.5hrs]

#### Progress

- Added Tauri commands and events to communicate state between frontend and backend
- Added MemoryGrid view to get Memory data from backend

#### Sources

- [Tauri Command interface (frontend -> backend)](https://tauri.app/v1/guides/features/command)
- [Tauri Event interface (frontend <-> backend)](https://tauri.app/v1/guides/features/events/)

### 9-1-22 [5.25hrs]

#### Progress

- Added frontend view reactivity models for loading ELF files and displaying the checksum and memory contents to the window
- Added memory class methods

#### Sources

- [SolidJS array reactivity](https://www.solidjs.com/docs/latest#indexarray)

### 9-2-22 [2.25hrs]

#### Progress

- Updated ELF file reader to parse segments and binary data and properly load to RAM
- Fixed Memory methods to Read/Write in both big- and little-endian

#### Sources

- [Rust object (ELF) library](https://docs.rs/object/0.20.0/object/read/elf/)

#### Tests Passed

- Checksum validation for `test1.exe`, `test2.exe`, `test3.exe`

### 9-3-22 [7.5hrs]

**Checkpoint Reached: Checkpoint 1: Loader**

#### Progress

- Refactored core logic into separate `cargo` workspace `/lib`
- Added comprehensive unit testing for `Memory` class
- Added project documentation to README.md
- Debugged command line arguments in release binary
- Fixed command line arguments not passing properly from different environments
- Fixed file paths not resolving on Windows, added `normpath` crate for `path::normalize()`
- Refactored frontend UI to have prop-passing for `MemoryGrid`

#### Sources

- [https://stackoverflow.com/a/49856591](Trimming strings for different characters)
- [`normpath` crate](https://crates.io/crates/normpath)
- [Tauri file logging](https://github.com/tauri-apps/tauri-plugin-log/blob/dev/src/lib.rs#L100)

#### Tests Passed

- All tests in `/lib/src/memory.rs`:`Memory`