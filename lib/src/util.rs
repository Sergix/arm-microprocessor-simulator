use crate::{memory::Word, instruction::{Instruction, TInstruction}, cpu_enum::{InstrType}};

pub fn get_bit(w: Word, bit: Word) -> Word {
    if bit > 31 {
        panic!("get_bit: invalid bit range");
    }

    (w >> bit) & 1
}

pub fn test_bit(w: Word, bit: Word) -> bool {
    get_bit(w, bit) != 0
}

pub fn word_lsb_to_bool(w: Word) -> bool {
    w & 1 != 0
}

pub fn is_write_instr(instr: Instruction) -> bool {
    match instr.get_type() {
        InstrType::LDRSTRShiftRegPre |
        InstrType::LDRSTRShiftRegPost |
        InstrType::LDRSTRRegPre |
        InstrType::LDRSTRRegPost |
        InstrType::LDRSTRImmPre |
        InstrType::LDRSTRImmPost |
        InstrType::LDRHSTRHImmPre |
        InstrType::LDRHSTRHImmPost |
        InstrType::LDRHSTRHRegPre |
        InstrType::LDRHSTRHRegPost => {
            if !instr.get_ldr_str().unwrap() {
                return true
            } else {
                return false
            }
        },
        _ => false
    }
}