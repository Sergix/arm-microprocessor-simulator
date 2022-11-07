# TODO

## Phase 4

### Docs

- [report](https://protect.bju.edu/cps/courses/armsim-project/exec2/report.html)

### Tests

- Phase 3 unit tests, no tests for Phase 4

### CPU/Registers

- swap in banked registers when the simulator switches CPU modes
- test IRQ flag after fetch-decode-execute to process exception
  - only if CPSR interrupt flag is not disabled

### Instructions  

- all data opcode s-bit updates (technically not needed by what the instructions say, dont prioritize)
- conditional data instructions (-> CMP, TEQ, etc.)
- CPSR conditions in CPU::execute
- LDRH/STRH LSH codes (-> LDRD, LDREX, etc.)
- SWI
  - 0x0  -- putchar
  - 0x11 -- halt
  - 0x6a -- readline; prompt in terminal window and wait for user input to read up to [r2] bytes, then write input to the address of r1
- B, BL, **BX**
  - have the imm displayed in the disassembly window be the address being jumped *to*, not the offset encoded
- MOVS, MSR, MRS

### GUI

- terminal window -- any chars written to address 0x100000 should immediately display in the terminal, CPU needs to inform terminal UI component that it needs to update
- Processor Mode toolbar note ("System", "IRQ", etc.)
- CPSR I bit to Flags window
- vertical resizing of the window affects panels

### Logs

- trace logging should output the actual system mode
- flag for logging exceptions or just normal (SYS mode) instructions