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

- [Trimming strings for different characters](https://stackoverflow.com/a/49856591)
- [`normpath` crate](https://crates.io/crates/normpath)
- [Tauri file logging](https://github.com/tauri-apps/tauri-plugin-log/blob/dev/src/lib.rs#L100)

#### Tests Passed

- All tests in `/lib/src/memory.rs`:`Memory`

## Week 4

### 9-14-22

#### Progress [2.15hrs]

- Added state management for registers/flags

#### Sources

- [ARM CPSR Flags](https://developer.arm.com/documentation/ddi0406/b/System-Level-Architecture/The-System-Level-Programmers--Model/ARM-processor-modes-and-core-registers/Program-Status-Registers--PSRs-)

### 9-15-22

#### Progress [8.5hrs]

- Updated memory grid logic to rechunk on updates and properly format based on the address offset given by the user
- Added Flags and Registers panels
- Payloads now send for each individual panel

### 9-16-22

#### Progress [5hrs]

- Added CPU class
- Added configurable breakpoints
- Hotkeys implemented
- Multhreaded implementation of CPU running
- Fixed a lot of bugs related to thread locking
- Added interface controls
- Reconfigured how payloads are sent to the frontend

#### Sources

- [Does a mutex's lock drop immediately?](https://www.reddit.com/r/rust/comments/ws61hk/does_a_mutexs_lock_free_automatically_if_not/)
- [Hotkeys.js](https://www.npmjs.com/package/hotkeys-js)
- [Accessing self reference in a thread](https://stackoverflow.com/questions/54971024/accessing-a-method-of-self-inside-a-thread-in-rust)

### 9-17-22

**Checkpoint Reached: Checkpoint 2: GUI**

#### Progress [6.5hrs]

- Added disassembly panel and logic with mocked assembly instructions
- Added breakpoint toggling
- Added tests for CPU class
- Tests for Registers class
- Updated CHANGELOG and README

#### Sources

- [Remove an item from an array in Rust](https://stackoverflow.com/a/26243276)
- [Tauri Rust API mocking](https://github.com/tauri-apps/tauri/pull/4752)

## Week 5

### 9-18-22

#### Progress [1hrs]

- Breakpoints in disassembly window are now clickable to toggle
- Overlay for memory panel to notify user that the table is still chunking memory

### 10-1-22

#### Progress [0.25hrs]

- Work on draft UML with instruction decoding design

## Week 6

### 10-8-22

#### Progress [2.5hrs]

- Finish draft UML design and upload with document detailing outline

## Week 7

### 10-13-22

#### Progress [2hrs]

- Detailed design progress
- Add bitmatch library for decoding instructions
- Add CPU enums in `lib::cpu_enum` with easy converting with primitives using `num::FromPrimitive`

#### Sources

- [bitmatch library](https://docs.rs/bitmatch/latest/bitmatch/)
- [num::FromPrimitive](https://docs.rs/num/0.1.29/num/traits/trait.FromPrimitive.html)

### 10-15-22

#### Progress [2hrs]

- Detailed design progress
- Add basic structs with traits for instruction categories
- Implement factories for a couple basic data instructions
- String conversion for enums

## Week 8

### 10-18-22

#### Progress [3hrs]

- Refactored instruction implementation to use a single `Instruction` struct rather than subtyping
- Added more CPU enum string conversion
- Implement disassembly and test for `mov` instruction
- Make instruction execute pass function pointer for CPU to easily get execute method
  
### 10-19-22

#### Progress [2.25hrs]

- Outlne more program architecture
- Match data opcodes for better data instruction execution
- Add disassembly for more data instruction types

## Week 9

### 10-27-22

#### Progress [2.25hrs]

- Add some data processing and LDR/STR factories and execution implementations
- Refactoring more of the `Instruction` class

### 10-28-22

#### Progress [4hrs]

- Add all data processing, LDR/STR, LDRH/STRH, branch, LDR/STM, and MUL instructions
- Split different executes based on ldr/str bits
- Add helper methods to `Instruction` class

#### Sources

- The ARM Manual

### 10-29-22

#### Progress [14hrs] (yes, actually)

- Disassembly for all the previous instructions listed and their categories
- Trace logs
- `--exec` option
- `Mode` enum for switching CPU modes along with methods to easily switch the CPSR mode bits
- Halt on SWI instruction
- Barrel shifters
- Memory now recomputes checksum when writing to memory
- Trace toggle button and hotkey in UI toolbar
- UI overlay that shows when the user presses a hotkey
- Update UML detailed diagram
- Fixed PC to properly point to two instructions (8 bytes) ahead
- Update README documentation

#### Sources

- [Rust std::fs::File](https://doc.rust-lang.org/std/fs/struct.File.html)
- [Rust bit shifting operators](https://doc.rust-lang.org/reference/expressions/operator-expr.html#arithmetic-and-logical-binary-operators)
- [Barrel shifters](https://www.davespace.co.uk/arm/introduction-to-arm/barrel-shifter.html)