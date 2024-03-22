# Draft Design Notes

https://developer.arm.com/documentation/ddi0165/b/ch01s01s01

how to construct interp. for rust binary

## Base

### Enums
- registers (each mapped to int when calling get_register) (https://stackoverflow.com/questions/28028854/how-do-i-match-enum-values-with-an-integer)
- addressing mode (pre-index, pre-index writeback, post-index)
- LSM codes
- shift type
- LDRH/STRH SH bit flag
- condition code (each mapped to nibble)
- DataOpcode

### Bitmatching
- use bitmatch to map bits to individual instruction struct and pass rest of bits for instruction to interpret (flags, etc.) (https://docs.rs/bitmatch/latest/bitmatch/)
map: 
- NCZV flags (28-31)
- type bits (27-25)
- bits 7 & 4
- bits 5 & 6

## Instruction trait
- text form function stub ("mov", "ldr", etc.)
- action function stub (tying in to MemoryState, RegisterState, CPUState, etc.)
- enum for condition codes
- Rd, Rn, Rm

## Type traits

- rely on the Instruction trait also being present (https://phaiax.github.io/mdBook/rustbook/ch19-03-advanced-traits.html#supertraits-to-use-one-traits-functionality-within-another-trait)

### Data
- S bit flag
- DataOpcode

### Branch
- L bit flag

### LSM
- LSM code (PU bits 24-23)
- writeback bit flag
- L/S bit flag
- S bit flag
- array of registers

### LDR/STR
- PUBWL bit flags
- shift type
- function to load/store depending on L bit
- store constant

#### LDRH/STRH
- SH enum

### SWI
- swi number

### Multiply
- S bit flag

## Instruction structs

- data register immediate shift
- data register register shift
- data immediate
  - get rotate
- LSM
- LDR/STR shifted register offset
- LDR/STR register offset
- LDR/STR immediate offset
- LDRH/STRH register offset
  - SH bit flags
- LDRH/STRH immediate offset
  - SH bit flags

## Struct/trait purposes

- CPU: fetch/decode/execute
- Memory: inherited trait; reading/writing, setting flags, clearing bits
  - Registers: setting/getting r0-15, CPSR register
  - RAM: all memory
- Instruction: decoding/encoding and storing individual instruction data
  - DataProcessing
    - instr_data_reg_imm_shift
    - instr_data_reg_reg_shift
    - instr_data_imm
  - Branch
    - instr_b
  - LSM
    - instr_lsm
  - LoadStore
    - instr_ldrstr_shifted_reg_offset
      - get immediate shift
    - instr_ldrstr_reg_offset
    - instr_ldrstr_imm_offset
      - get shift
    - instr_ldrhstrh_reg_offset
      - SH bit flags
    - instr_ldrhstrh_imm_offset
      - get immediate offset ((high nibble << 4) | low nibble)
      - SH bit flags
  - SWI
    - instr_swi
  - Multiply
    - instr_multiply