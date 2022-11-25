# TODO

- [-] partially complete
- [x] fully complete

## Phase 4

### Docs

- [report](https://protect.bju.edu/cps/courses/armsim-project/exec2/report.html)

### CPU/Registers

- [x] swap in banked registers when the simulator switches CPU modes
  - extend register array to include enough for each mode, and when reading/writing registers for R13/R14/CPSR ensure it writes to the correct register
- [x] add CPU IRQ flag and test after each fetch-decode-execute to process exception
  - [x] only if CPSR interrupt flag is not disabled
  - [x] set CPSR I bit
  - [x] clear the CPU IRQ flag
  - [x] handle the interrupt
- [x] on keyboard event, set CPU IRQ flag
- [0] when attempt to read from 0x100001, get the last stored char from frontend
  - CPU injects last read char into each instruction so it can be accessed without threads
- [-] when attempt to write to 0x100000, send last char to terminal window
- [x] when resetting the CPU, set Supervisor mode, check for non-zero number in first byte of RAM and set PC to 0

### Instructions  

- [-] all data opcode s-bit updates (technically not needed by what the instructions say, dont prioritize)
- [-] conditional data instructions (-> CMP, TEQ, etc.)
- [x] CPSR conditions in CPU::execute
- [ ] LDRH/STRH LSH codes (-> LDRD, LDREX, etc.)
- [ ] SWI
  - [ ] 0x0  -- putchar
  - [ ] 0x11 -- halt
  - [ ] 0x6a -- readline; prompt in terminal window and wait for user input to read up to [r2] bytes, then write input to the address of r1
- [x] B, BL, **BX**
  - [x] have the imm displayed in the disassembly window be the address being jumped *to*, not the offset encoded
- [x] MOVS, MSR, MRS

### GUI

- [ ] terminal window -- any chars written to address 0x100000 should immediately display in the terminal, CPU needs to inform terminal UI component that it needs to update
- [-] terminal window -- char input should emit to CPU
- [x] Processor Mode toolbar note ("System", "IRQ", etc.)
- [x] CPSR I bit to Flags window
- [ ] vertical resizing of the window affects panels

### Logs

- [x] trace logging should output the actual system mode
- [-] flag for logging exceptions or just normal (SYS mode) instructions
  - [ ] suppress during reset handler