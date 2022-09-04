# README

**Name:** Peyton McGinnis  
**Course:** CpS 310  
**Submission date:** 3 September 2022  
**Hours spent:** 34.5  

## Features

- `--mem` and `<elf-file>` command line options are supported and validated
- Logging framework implemented using `tauri-plugin-log`, however enabling/disabling logging to shell in Debug mode or changing the default logfile destination are currently not supported. (More information in the Configuration section)
- Scrollable memory grid
- ELF file loader in GUI
- Simulated RAM with checksums
- Complete unit tests for Memory logic

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
4. `cargo tauri build` to build the project.
    - To enable logging output to your shell when running the application, run `cargo tauri build --debug`.

The release target binary is exported to `/src-tauri/target/release` along with the platform-specific installer package files. The debug target binary is similarly in `/src-tauri/target/release`.

### Development

To run the built-in development environment with hot module reloading (HMR), run `cargo tauri dev`.

### Testing

To run the tests, run `cd lib` -> `cargo test`. Testing is only implemented for the core logic.

## Configuration

Currently, logging configuration is not supported.

The Debug target binary (`--debug` mode) logs output to the shell, the WebView developer tools, and to a logfile. In normal release mode, the program only logs output to a logfile.

[The default logfile destinations are the following](https://github.com/tauri-apps/tauri-plugin-log/blob/dev/src/lib.rs#L100):
- Linux: `{configDir}/com.sergix.dev` (Example: `/home/alice/.config/com.sergix.dev`)
- macOS: `{homeDir}/Library/Logs/com.sergix.dev` (Example: `/Users/Alice/Library/Logs/com.sergix.dev`)
- Windows: `{configDir}/com.sergix.dev` (`C:\Users\Alice\AppData\Roaming\com.sergix.dev`)

## User Guide

### Command-Line

Command format: `armsim.exe [--mem <memory-size>] <elf-file>`

To launch the application from the command-line, simply navigate to the directory containing the binary and run `armsim.exe elf_file.bin`. By default, this loads `elf_file.bin` into a 32K block of simulated RAM and opens a window on your desktop with a scrollable memory grid.

To specify the amount of simulated RAM, simply pass in the `--mem <memory_size>` option: `armsim.exe --mem 33768 elf_file.bin`

### Launch

When launching the application from your desktop environment, the initial window has a button titled **Load ELF**. Once you click this button, it will open up a file selection dialog where you can select your ELF binary and it will automatically load into the window.

## Bug Report

- Most ELF headers are currently not validated in the program except for the magic number, so they will cause errors in the console but the exceptions are caught.

## [Project Journal](CHANGELOG.md)

## Academic Integrity Statement

By affixing my signature below, I certify that the accompanying work represents my own intellectual effort. Furthermore, I have received no outside help other than what is documented below.

*Peyton McGinnis*

| Date | Name | Nature of Help | Time Spent | 