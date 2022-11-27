# Report

**Name:** Peyton McGinnis  
**Course:** CpS 310  
**Submission date:** 28 November 2022  
**Hours spent this phase:** 35

## Table of Contents

1. Introduction
2. Features
3. Software Prerequisites
4. Build and Tests
5. Configuration
6. User Guide
7. Software Architecture
8. Bug Report
9. Appendices

## Introduction

ARMSim is a GUI debugger for ELF-binary applications compiled for ARM32. This application can load an ELF binary, disassemble the binary, and show the flags, memory, and stack as the application runs with inline run/step debugging.

This report details all information pertaining the project's features, how to use the program, and how to compile and configure the program. This report also includes instructions for compiling your own program to run on the simulator.

## Features

#### C-Level
- ELF loader displays correct checksums for valid ELF files
- Simulated RAM with checksums
- `--mem`, `--exec`, and `<elf-file>` command line options are supported and validated
- Register viewer (r0..15)
- Flags display
- Stacks panel
- Internal CPU simulator
- Automatic execution through `--exec` option

#### B-Level
- Scrollable memory grid
  - Navigates to any given address and properly formats the table
- Comprehensive unit tests
  - All `lib::memory` logic
  - A few for disassembly, decoding, and `instruction` building
- Multithreaded debugger controls -- Run, Step, Stop, Reset
- All hotkeys implemented
- Optional trace logs output to `trace.log` in local directory

#### A-Level
- Logging framework implemented using `tauri-plugin-log`, however enabling/disabling logging to shell in Debug mode or changing the default logfile destination are currently not supported. (More information in the Configuration section)
- Polished GUI
- Resizable window
- Breakpoints
  - Add and toggle breakpoints in the disassembly window
- Quality draft and detailed design
- All required instructions implemented
- Disassembly table with accurate assembly for all instructions
- Correct trace logs for all tests, with trace logging for all system modes
- Interrupt processing
- All processor modes: `SYS`, `SVC`, `IRQ`
- Banked register swapping and CPU modes for SYS, SVC, and IRQ modes
- Memory-mapped keyboard and display device I/O handling
- Terminal window interaction with real-time output and interactive interrupt prompts
  - SWI instructions
  - Memory-mapped keyboard and display devices
- Processor mode notes in toolbar
- SWI I/O handlers (putchar, halt, readline) with processor mode switching

#### Extra Credit

**LDRH/STRH**  
*Tests: `\tests\pmcgi795_halfword_no_io.c`, `\tests\pmcgi795_halfword_no_io.lst`*  
*Trace log: `\tests\logs\pmcgi795_halfword_no_io_trace.log`*

LDRH and STRH modes are implemented in the program. The trace log shows the following line and its translated assembly to show that it properly functions as the instruction correctly loads the value `1` into the register `r2`:

`1048:	e1d220b2 	ldrh	r2, [r2, #2]`  
`000019 00001048 1FFFDB3B 0000 SYS 0=00000000 1=00000000 2=00000001 3=00000006 4=00000000 5=00000000 6=00000000 7=00000000 8=00000000 9=00000000 10=00000000 11=00006FFC 12=00000000 13=00006FF0 14=00000000`

**Data Move S-versions**  
*No details or tests given*  

#### Not implemented features

- LDRH/STRH doublewords
- LDRH/STRH LSH code disassembly
- Log suppression for reset handler
- Logging configuration

## Software Prerequisites

#### OS Platforms

Windows 10/11, macOS, Debian, Arch, Fedora, openSUSE

#### Software

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

#### Building

1. Install the necessary software noted above for your platform.
2. `git clone https://github.com/bjucps310/cps310-simulator-Sergix`
3. `yarn install` at the root level of the project directory to install the necessary `npm` packages.
4. `yarn tauri build` to build the project.
    - To enable logging output to your shell when running the application, run `yarn tauri build --debug`.

The release target binary is exported to `/src-tauri/target/release` along with the platform-specific installer package files. The debug target binary is similarly in `/src-tauri/target/release`.

#### Development

To run the built-in development environment with hot module reloading (HMR), run `yarn tauri dev`.

#### Testing

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

![ARMsim](/img/armsim-running.png)

`armsim.exe [--mem <memory-size>] [--traceall] [--exec] <elf-file>`

To launch the application from the command-line, navigate to the directory containing the program executable and run `armsim.exe elf_file.bin`. By default, this loads `elf_file.bin` into a 32K block of simulated RAM and opens a window on your desktop with a scrollable memory grid. The initial window has a button titled **Load ELF**. Once you click this button, it will open up a file selection dialog where you can select your ELF binary and it will automatically load into the window.

To specify the amount of simulated RAM, simply pass in the `--mem <memory_size>` option: `armsim.exe --mem 33768 elf_file.bin`

The `--exec` option automatically begins executing the executable oonce it finishes loading and enables trace logging (see *Trace Logs* below). The `<elf-file>` option must also be specified.

The `--traceall` option enables trace logging for *all* system modes: `SYS`, `SVC`, `IRQ`. By default, trace logs only log `SYS` mode steps.

#### Debugging Controls

Once a binary is loaded, you can use the **Run** button in the toolbar to begin executing the application. The binary will run on a separate thread and continue until:
1. A HLT (`0x0`) instruction or HLT SWI instruction is reached
2. The **Stop** button is pressed
3. A breakpoint is hit

You can also use the **Step** button to step to the next instruction.

Using the **Add Breakpoint** function, you can manually add a breakpoint at a given address.

Press **Reset** to reset the display, memory, and registers, but keep all breakpoints intact.

#### Trace Logs

The **Trace** function is used to output a log of all CPU steps to `./trace.log` to inspect all register information after the result of each instruction cycle. The format for each entry is:  
`step_number program_counter checksum nzcv mode r0 r1 r2 r3 r4 r5 r6 r7 r8 r9 r10 r11 r12 r13 r14 `

The **Trace** button in the UI will be *green* when trace logging is active for the currently loaded executable. The trace log will appear in the directory from which the application was executed.

![ARMsim](/img/trace-button.png)

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

![Memory Panel](/img/memory-panel.png)

#### Flags Panel

When one of the NZCV flags is active, the flag's icon will be green.

![Flags Panel](/img/flags-panel.png)

#### Registers Panel

This panel displays all the registers from r0...r15 for the currently loaded register bank.

![Registers Panel](/img/registers-panel.png)

#### Stack Panel

This panel displays memory locations close to the stack pointer: 3 above and 3 below. The stack pointer address is highlighted in the table.

![Stack Panel](/img/stack-panel.png)

#### Terminal Panel

The terminal panel can be used by programs that execute interrupt instructions to read and write to an output device. The following functionality is enabled for programs:
1. SWI `0x0`: output the character to the terminal
2. SWI `0x6a`: prompt the user for a string input
3. `0x100000`: write calls to this address result in writing the character to the terminal
4. `0x100001`: read calls to this address result in reading the last-pressed character from the terminal

![Terminal Panel](/img/terminal-panel.png)

#### Disassembly Panel

This panel displays a table with instructions surrounding the currently executing instruction and each instruction's ARM assembly representation.

By clicking the icon in the *BP* (BreakPoint) column, you can also toggle a breakpoint for that specific address.

![Disassembly  Panel](/img/disassembly-panel.png)

#### Writing Programs

It is best and easiest to use C or assembly to write compatible programs.

To compile a progam that is compatible with the simulator, you will need the following tools (tool are only for Windows). Copy each executable from the locations in each package below into the folder that has your programs.

1. [gcc-arm-win32-toolset](https://developer.arm.com/downloads/-/gnu-rm)
	- \arm-none-eabi\bin\as.exe
	- \bin\arm-none-eabi-gcc.exe
	- \bin\arm-none-eabi-ld.exe
	- \bin\arm-none-eabi-objdump.exe
2. [WinLibs GCC from MingGW for Win32 (without LLVM/Clang/LLD/LLDB)](https://winlibs.com/)
	- \mingw32\libexec\gcc\i686-w64-mingw32\cc1.exe

In addition, save the linker script in Code Listing 1 at the end of this document into a file called `linker.ld`.

Then, you will need to execute the following commands for your source file, for example `program.c`:
```
arm-none-eabi-gcc.exe -c program.c -o program.o -nostdlib -fno-builtin -nostartfiles -nodefaultlibs  -mcpu=arm7tdmi
arm-none-eabi-ld -T linker.ld -n -e main -o program.exe program.o 
```

Then, you can load `program.exe` into the simulator.

## Software Architecture

This application uses a combination of Rust with Tauri for the backend and the Tauri interface with SolidJS for the frontend.

#### UML Diagram

<embed src="/docs/DRAFT-diagram.drawio.pdf" width="500" height="375"></embed>

#### Class Relationships

The codebase is split into three main packages, `/lib`, `/src-tauri`, and `/src`:
- `/lib` (cargo crate): logic that doesn't interact with the interface
    This logic may interact with Tauri's application state, but does not interact with any UI logic
- `/src-tauri` (npm package): Tauri commands, state, events, and other interface logic that sets up the frontend and responds to UI events
- `/src` (cargo crate): frontend and UI, SolidJS, driven by Tauri server

Execution begins in `/src-tauri/src/main.rs`, where Tauri sets up the web view backend. The frontend interacts with the backend by invoking events.

The primary backend classes are the following:
- `Memory`: all abstract methods and implementations for reading and writing data
  - `RAM`: primary singleton for memory
  - `Registers`: singleton for all program registers
- `CPU`: all fetch, decode, execute steps that uses `Instruction` classes for processing
- `CPUThreadWatcher`: watches the CPU thread state to manage events (IRQ interrupts, start/stop, etc.) from the frontend as the CPU is running
- `Instruction`: CPU stores each decoded Word from the program as an `Instruction` class that also contains a reference to its appropriate execute method
- `Options`: program command-line option parsing and storage
- `Trace`: singleton tracefile instance called by CPU for saving trace logs

#### Threading

The application uses a somewhat-complex threading model because of Rust's strict enforcement of variable lifetimes. Global state singletons (`/lib/src/state.rs`) are managed by Tauri and can be accessed by any function in the application with access to the global `app_handle` object. This `app_handle` is passed around to functions and functions can access the state mutexes through this instance, but each function has to ensure that its locks are freed as soon as possible.

Because the `CPU` class will likely be continuously running as the program executes, the `CPUThreadWatcher` class (as mentioned before) maintains the CPU's thread state while the CPU runs and intercepts events from the frontend.

#### Model-View Separation

*[Tauri Event Documentation](https://tauri.app/v1/guides/features/events/)*  

Both the Rust Tauri backend and the frontend run on separate threads by nature. Events are sent to the frontend via `emit` calls on the "app handler", and events are sent to the backend via `invoke` calls. The backend listens to events via commands, and the frontend listens to events via `listen`ers.

#### Third-Party Libraries

- [Tauri](https://tauri.app/): primary application and webview driver
- [normpath](https://crates.io/crates/normpath): normalize user-specified executable paths
- [object](https://crates.io/crates/object): efficiently read ELF binaries
- [bitmatch](https://crates.io/crates/bitmatch): pattern matching for bit sequences used in decoding instructions
- [num](https://crates.io/crates/num): map enums to values

#### Design Patterns

Most of the signals sent throughout the program use some form of flagging technique. No special observer or event patterns are defined since Tauri intrinsically provides a complete event-based interface.

#### Terminal I/O

Terminal I/O is accomplished differently for how the data is either read or written:
1. SWI `putchar` (`0x0`)
   - set all the conditions defined by the ARM Manual (A2.6.4)
   - read `r0`
   - emit update event to frontend, frontend captures and adds character to terminal
2. SWI `readline` (`0x6a`)
   - set all the conditions defined by the ARM Manual (A2.6.4)
   - get arguments from `r1` and `r2`
   - emit a prompt event to frontend, frontend opens prompt input
   - loop
   - once user enters prompt, send a signal to the backend `CPUThreadWatcher`
   - exit loop
   - store input in memory
3. Memory-mapped display device (`0x100000`)
   - after decode and before execute, check if the instruction intends to write to the address
   - if so, emit update event to frontend, frontend captures and adds character to terminal
   - continue executing instruction as normal
   - instruction execute method ignores write address
4. Memory-mapped keyboard device (`0x100001`)
   - after decode and before execute, inject the user's last-pressed key into the instruction class
   - execute the instruction
   - if the instruction intends to read from the address, the instruction replaces the value with the last-pressed key value

## Bug Report

#### Sim1 Tracefile Comparisons

- `btest.exe`: trace logs are identical
- `ctest.exe`: trace logs are identical
- `ldmstm.exe`: works as expected

#### Sim2 Tracefile Comparisons

- `branch.exe`: trace logs are identical
- `cmp.exe`: trace logs are identical
- `locals_no_io.exe`: trace logs are identical
- `mersenne_no_io.exe`: trace logs are identical
- `quicksort_no_io.exe`: trace logs are identical
- `countdown.exe`: works as expected
- `iodemo.exe`: works as expected
- `mersenne.exe`: works as expected
- `quicksort.exe`: works as expected
- `simpleiodemo.exe`: works as expected
- `syscalldemo.exe`: works as expected

#### General

- Most ELF headers are currently not validated in the program except for the magic number, so they will cause errors in the console but the exceptions are caught.
- Although rarely, the program may hang when running a program using the `--traceall` and `--exec` options.

## Appendices

#### [Project Journal](CHANGELOG.md)

#### [Git log](https://github.com/bjucps310/cps310-simulator-Sergix/commits/master)

#### Project Hours

**Subtotal hours for each phase**

1. Phase 1: 31.25h
2. Phase 2: 22.15h
3. Phase 3: 33.25h
4. Phase 4: 35h

**Total number of hours for the entire project**: 120.9h

## Academic Integrity Statement

By affixing my signature below, I certify that the accompanying work represents my own intellectual effort. Furthermore, I have received no outside help other than what is documented below.

*Peyton McGinnis*

| Date | Name | Nature of Help | Time Spent | 
|------|------|----------------|------------|
|------|------|----------------|------------|


## Appendices

#### Code Listing 1

```
SECTIONS
{
    . = 0x001000;

    .text ALIGN (0x04) :
    {
        *(.text)
        *(.rodata)
        *(.data)
    } 

    .bss :
    {
        sbss = .;
        *(COMMON)
        *(.bss)
        ebss = .;
    }
}
```