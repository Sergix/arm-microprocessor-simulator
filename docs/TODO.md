# TODO

- [?] unsure of requirements
- [-] partially complete
- [x] fully complete

## Phase 4

### Docs

- [-] [report](https://protect.bju.edu/cps/courses/armsim-project/exec2/report.html)
  - [ ] Features
    - If you implemented instructions or addressing modes that were not required by the specifications, highlight those here
    - In order to receive extra credit for them, you must provide a small extras.s test file that tests your extra addressing modes / instructions, along with a trace log with relevant portions highlighted, and a detailed discussion of how the log demonstrates that your instructions are correctly executed
  - [-] User Guide: A comprehensive guide to the features of your simulator, including command line arguments as well as GUI capabilities. Discuss how to create exe's that can be used with your simulator. Include screen shots.
  - [-] Software Architecture
    - UML diagram showing the key classes in your model and the relationships between them
    - describe the significant classes and their relationships
    - mention any use of threads/timers
    - model-view separation
    - third-party libraries
    - design patterns
    - terminal I/O
  - [-] Bug Report
    - trace file comparisons for sim1 and sim2
    - For test files for which there is no official trace, indicate whether the simulator produced the expected output, and if not, why not
    - list significant omissions as well as known issues
  - [ ] Appendices
    - Project journal
    - git log
    - subtotal hours for each phase
    - total number of hours for the entire project

### CPU/Registers

- [x] swap in banked registers when the simulator switches CPU modes
  - extend register array to include enough for each mode, and when reading/writing registers for R13/R14/CPSR ensure it writes to the correct register
- [x] add CPU IRQ flag and test after each fetch-decode-execute to process exception
  - [x] only if CPSR interrupt flag is not disabled
  - [x] set CPSR I bit
  - [x] clear the CPU IRQ flag
  - [x] handle the interrupt
- [x] on keyboard event, set CPU IRQ flag
- [x] when attempt to read from 0x100001, get the last stored char from frontend
  - CPU injects last read char into each instruction so it can be accessed without threads
- [x] when attempt to write to 0x100000, send last char to terminal window
- [x] when resetting the CPU, set Supervisor mode, check for non-zero number in first byte of RAM and set PC to 0

### Instructions  

- [x] all data opcode s-bit updates (technically not needed by what the instructions say, dont prioritize)
- [x] conditional data instructions (-> CMP, TEQ, etc.)
- [x] CPSR conditions in CPU::execute
- [-] LDRH/STRH LSH codes (-> LDRD, LDREX, etc.)
  - No doubleword implementations
- [x] SWI
  - [x] 0x0  -- putchar
  - [x] 0x11 -- halt
  - [x] 0x6a -- readline; prompt in terminal window and wait for user input to read up to [r2] bytes, then write input to the address of r1
- [x] B, BL, **BX**
  - [x] have the imm displayed in the disassembly window be the address being jumped *to*, not the offset encoded
- [x] MOVS, MSR, MRS

### GUI

- [x] terminal window -- any chars written to address 0x100000 should immediately display in the terminal, CPU needs to inform terminal UI component that it needs to update
- [x] terminal window -- char input should emit to CPU
- [x] Processor Mode toolbar note ("System", "IRQ", etc.)
- [x] CPSR I bit to Flags window
- [x] vertical resizing of the window affects panels

### Logs

- [x] trace logging should output the actual system mode
- [-] flag for logging exceptions or just normal (SYS mode) instructions
  - [?] suppress during reset handler