# README

**Name:** Peyton McGinnis  
**Course:** CpS 310  
**Submission date:** 29 October 2022  
**Hours spent this phase:** 32.25

## Overview

ARMSim is a GUI debugger for ELF-binary applications compiled for ARM32. This application can load an ELF binary, disassemble the binary, and show the flags, memory, and stack as the application runs with inline run/step debugging.

## Features

### Phase 1 nd GUI

- `--mem`, `--exec`, and `<elf-file>` command line options are supported and validated
- Logging framework implemented using `tauri-plugin-log`, however enabling/disabling logging to shell in Debug mode or changing the default logfile destination are currently not supported. (More information in the Configuration section)
- Scrollable memory grid
  - Navigates to any given address and properly formats the table
- ELF file loader in GUI
- Simulated RAM with checksums
- Unit tests
  - All `lib::memory` logic
  - A few for disassembly, decoding, and `instruction` building
- Disassembly table (with accurate assembly)
- Register viewer (r0..15)
- Flags display

### Phase 3

- Internal CPU simulator
- Resizable window
- Add and toggle breakpoints in the disassembly window
- All hotkeys implemented
- Multithreaded debugger controls -- Run, Step, Stop, Reset
- All required instructions implemented
- Optional trace logs output to `trace.log` in local directory
  - Correct trace for C- and B-level tests
- Automatic execution through `--exec` option

### Phase 4

- Banked register swapping and CPU modes for SYS, SVC, and IRQ modes
- Trace logging for all system modes
- SWI handlers (putchar, halt, readline)
- Memory-mapped keyboard and display device handling
- All required instructions implemented
- Terminal window interaction with real-time output and interactive itnerrupt prompts
- Processor mode notes in toolbar
- Active hotkey notification in toolbar

### Not required, but implemented features

**LDRH/STRH**  
*Tests: `\tests\sergix_halfword_no_io.c`, `\tests\sergix_halfword_no_io.lst`*  
*Trace log: `\tests\sergix_halfword_no_io_trace.log`*

LDRH and STRH modes are implemented in the program. The trace log shows the following line and its translated assembly to show that it properly functions as the instruction correctly loads the value `1` into the register `r2`:

`1048:	e1d220b2 	ldrh	r2, [r2, #2]`  
`000019 00001048 1FFFDB3B 0000 SYS 0=00000000 1=00000000 2=00000001 3=00000006 4=00000000 5=00000000 6=00000000 7=00000000 8=00000000 9=00000000 10=00000000 11=00006FFC 12=00000000 13=00006FF0 14=00000000`

### Not implemented features

- LDRH/STRH doublewords
- LDRH/STRH LSH code disassembly
- Log suppression for reset handler

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

To run the tests, run `cd lib` then `cargo test`.

Testing is implemented for the Memory trait and for some of the CPU. Some of the CPU is untestable as core logic because it's tightly integrated with the threading model and state model of the internal API. [The Tauri project is currently pushing for mocking these models for testing in the next version.](https://github.com/tauri-apps/tauri/pull/4752)

## Configuration

Currently, logging configuration is not supported.

The Debug target binary (`--debug` mode) logs output to the shell, the WebView developer tools, and to a logfile. In normal release mode, the program only logs output to a logfile.

[The default logfile destinations are the following](https://github.com/tauri-apps/tauri-plugin-log/blob/dev/src/lib.rs#L100):
- Linux: `{configDir}/com.sergix.dev` (Example: `/home/alice/.config/com.sergix.dev`)
- macOS: `{homeDir}/Library/Logs/com.sergix.dev` (Example: `/Users/Alice/Library/Logs/com.sergix.dev`)
- Windows: `{configDir}/com.sergix.dev` (`C:\Users\Alice\AppData\Roaming\com.sergix.dev`)

## User Guide

`armsim.exe [--mem <memory-size>] [--traceall] [--exec] <elf-file>`

To launch the application from the command-line, navigate to the directory containing the program executable and run `armsim.exe elf_file.bin`. By default, this loads `elf_file.bin` into a 32K block of simulated RAM and opens a window on your desktop with a scrollable memory grid. The initial window has a button titled **Load ELF**. Once you click this button, it will open up a file selection dialog where you can select your ELF binary and it will automatically load into the window.

To specify the amount of simulated RAM, simply pass in the `--mem <memory_size>` option: `armsim.exe --mem 33768 elf_file.bin`

The `--exec` option automatically begins executing the executable oonce it finishes loading and enables trace logging (see *Trace Logs* below). The `<elf-file>` option must also be specified.

The `--traceall` option enables trace logging for *all* system modes: `SYS`, `SVC`, `IRQ`. By default, trace logs only log `SYS` mode steps.

#### Debugging Controls

Once a binary is loaded, you can use the **Run** button in the toolbar to begin executing the application. The binary will run on a separate thread and continue until:
1. A HLT (0x0) instruction is reached
2. The **Stop** button is pressed
3. A breakpoint is hit

You can also use the **Step** button to step to the next instruction.

Using the **Add Breakpoint** function, you can manually add a breakpoint at a given address.

Press **Reset** to reset the display, memory, and registers, but keep all breakpoints intact.

#### Trace Logs

The **Trace** function is used to output a log of all CPU steps to `./trace.log` to inspect all register information after the result of each instruction cycle. The format for each entry is:  
`step_number program_counter checksum nzcv mode r0 r1 r2 r3 r4 r5 r6 r7 r8 r9 r10 r11 r12 r13 r14 `

The **Trace** button in the UI will be *green* when trace logging is active for the currently loaded executable.

#### Hotkeys

1. Load File: Ctrl-O
2. Run: F5
3. Single-step: F10
4. Stop execution: Ctrl-Q
5. Reset: Ctrl-R
6. Toggle Breakpoint: Ctrl-B
7. Trace: Ctrl-T

#### Memory Panel

In the memory panel, you can enter a hex address in the *Address* input and press **GO** to navigate to that address in the table.

#### Flags Panel

When one of the NZCV flags is active, the flag's icon will be green.

## Instruction Implementation

- `AND`, `EOR`, `SUB`, `RSB`, `ADD`, `SBC`, `RSC`, `ORR`, `MOV`, `BIC`, `MVN`
  - register with immediate shift, register with register shift, immediate
  - all barrel shifters (except RXX)
- `LDR`, `STR`
  - pre-index, pre-index writeback, post-index
  - unsigned byte, word
  - shifted register offset, register offset, immediate offset
- `LDRH`, `STRH`: 
  - LSH shifts not implemented
  - pre-index, pre-index writeback, post-index
  - register offset, immediate offset 
- `B`, `BL`
- `LDM`, `STM`
  - with and without writeback
  - All LSM codes (increment after, decrement before, ...)
- `MUL`
- `SWI`
  - sets CPU mode, jumps, and halts

## Bug Report

### Sim1 Tracefile Comparisons


### Sim2 Tracefile Comparisons

- Most ELF headers are currently not validated in the program except for the magic number, so they will cause errors in the console but the exceptions are caught.
- Loading some IO-based programs after running an IO-based program can cause the program to hang due to thread locks
- Some IO-based programs (such as quicksort) will hang when running in "exec" mode with trace logs

## [Project Journal](CHANGELOG.md)

## Academic Integrity Statement

By affixing my signature below, I certify that the accompanying work represents my own intellectual effort. Furthermore, I have received no outside help other than what is documented below.

*Peyton McGinnis*

| Date | Name | Nature of Help | Time Spent | 