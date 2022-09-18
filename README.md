# README

**Name:** Peyton McGinnis  
**Course:** CpS 310  
**Submission date:** 3 September 2022  
**Hours spent this phase:** 22.15

## Overview

ARMSim is a GUI debugger for ELF-binary applications compiled for ARM32. This application can load an ELF binary, disassemble the binary, and show the flags, memory, and stack as the application runs with inline run/step debugging.

## Features

- `--mem` and `<elf-file>` command line options are supported and validated
- Logging framework implemented using `tauri-plugin-log`, however enabling/disabling logging to shell in Debug mode or changing the default logfile destination are currently not supported. (More information in the Configuration section)
- Scrollable memory grid
  - Navigates to any given address and properly formats the table
- ELF file loader in GUI
- Simulated RAM with checksums
- Complete unit tests for Memory logic
- Disassembly table (with mocked assembly)
- Register viewer (r0..15)
- Flags display
- Internal CPU simulator
- Resizable window
- Add and toggle breakpoints in the disassembly window
- All hotkeys implemented
- Multithreaded debugger controls -- Run, Step, Stop, Reset

## Prerequisites

### OS Platforms

Windows 10/11, macOS, Debian, Arch, Fedora, openSUSE

### Software

- [yarn 1.22.^](https://classic.yarnpkg.com/en/docs/install)
- [Rust + cargo](https://www.rust-lang.org/)
- [Tauri CLI](https://tauri.app/v1/guides/getting-started/setup/html-css-js#create-the-rust-project)

#### Windows-only
- [Build Tools for Visual Studio 2022](https://visualstudio.microsoft.com/visual-cpp-build-tools/)

#### macOS-only

- [CLang and macOS development dependencies](https://tauri.app/v1/guides/getting-started/prerequisites#1-clang-and-macos-development-dependencies)

#### Linux-only

- [C compiler and WebKit2GTK](https://tauri.app/v1/guides/getting-started/prerequisites#1-system-dependencies)

## Build and Tests

### Building

1. Install the necessary software noted above for your platform.
2. `git clone https://github.com/bjucps310/cps310-simulator-Sergix`
3. `yarn install` at the root level of the project directory to install the necessary `npm` packages.
4. `yarn tauri build` to build the project.
    - To enable logging output to your shell when running the application, run `yarn tauri build --debug`.

The release target binary is exported to `/src-tauri/target/release` along with the platform-specific installer package files. The debug target binary is similarly in `/src-tauri/target/release`.

### Development

To run the built-in development environment with hot module reloading (HMR), run `yarn tauri dev`.

### Testing

To run the tests, run `cd lib` -> `cargo test`.

Testing is implemented for the Memory trait and for some of the CPU. Some of the CPU is untestable as core logic because it's tightly integrated with the threading model and state model of the internal API. [The Tauri project is currently pushing for mocking these models for testing in the next version.](https://github.com/tauri-apps/tauri/pull/4752)

## Configuration

Currently, logging configuration is not supported.

The Debug target binary (`--debug` mode) logs output to the shell, the WebView developer tools, and to a logfile. In normal release mode, the program only logs output to a logfile.

[The default logfile destinations are the following](https://github.com/tauri-apps/tauri-plugin-log/blob/dev/src/lib.rs#L100):
- Linux: `{configDir}/com.sergix.dev` (Example: `/home/alice/.config/com.sergix.dev`)
- macOS: `{homeDir}/Library/Logs/com.sergix.dev` (Example: `/Users/Alice/Library/Logs/com.sergix.dev`)
- Windows: `{configDir}/com.sergix.dev` (`C:\Users\Alice\AppData\Roaming\com.sergix.dev`)

## User Guide

`armsim.exe [--mem <memory-size>] <elf-file>`

To launch the application from the command-line, navigate to the directory containing the program executable and run `armsim.exe elf_file.bin`. By default, this loads `elf_file.bin` into a 32K block of simulated RAM and opens a window on your desktop with a scrollable memory grid. The initial window has a button titled **Load ELF**. Once you click this button, it will open up a file selection dialog where you can select your ELF binary and it will automatically load into the window.

To specify the amount of simulated RAM, simply pass in the `--mem <memory_size>` option: `armsim.exe --mem 33768 elf_file.bin`

#### Debugging Controls

Once a binary is loaded, you can use the **Run** button in the toolbar to begin executing the application. The binary will run on a separate thread and continue until:
1. A HLT (0x0) instruction is reached
2. The **Stop** button is pressed
3. A breakpoint is hit

You can also use the **Step** button to step to the next instruction.

Using the **Add Breakpoint** function, you can manually add a breakpoint at a given address.

Press **Reset** to reset the display, memory, and registers, but keep all breakpoints intact.

#### Hotkeys

1. Load File: Ctrl-O
2. Run: F5
3. Single-step: F10
4. Stop execution: Ctrl-Q
5. Reset: Ctrl-R
6. Toggle Breakpoint: Ctrl-B

#### Memory Panel

In the memory panel, you can enter a hex address in the *Address* input and press **GO** to navigate to that address in the table.

#### Flags Panel

When one of the NZCV flags is active, the flag's icon will be green.

## Bug Report

- Most ELF headers are currently not validated in the program except for the magic number, so they will cause errors in the console but the exceptions are caught.
- Loading the memory grid in the display causes the app to hang since it's currently a very large computation. Working on a new implementation that uses WebSocket IPC.
- Panels are not resized when the window's height changes, only the width

## [Project Journal](CHANGELOG.md)

## Academic Integrity Statement

By affixing my signature below, I certify that the accompanying work represents my own intellectual effort. Furthermore, I have received no outside help other than what is documented below.

*Peyton McGinnis*

| Date | Name | Nature of Help | Time Spent | 