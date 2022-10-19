use std::fmt;
use std::future::Future;

use tauri::AppHandle;
use tokio::sync::MutexGuard;

use crate::cpu::CPU;
use crate::execute;
use crate::memory::{Byte, AddressSize, Word, Register, HalfWord, RAM, Registers};
use crate::cpu_enum::{Condition, ShiftType, DataOpcode, LSMCode, AddressingMode, LSH, InstrType};

// This is the parent trait that contains all the default implementations
// for each instruction; all the getters and setters are implemented directly
// for the Instruction struct 
pub trait TInstruction {
    fn new(_type: InstrType) -> Self;

    fn decode(&self);
    fn encode(&self);

    fn set_execute(&mut self, f: fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, &mut MutexGuard<'_, CPU>, Instruction) -> Word);
    fn get_execute(&self) -> fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, &mut MutexGuard<'_, CPU>, Instruction) -> Word;

    fn get_name(&self) -> String;
    // fn get_instr(&self) -> Word;

    fn get_type(&self) -> InstrType;
    fn set_type(&mut self, _type: InstrType);

    fn get_data_opcode(&self) -> Option<DataOpcode>;
    fn set_data_opcode(&mut self, code: Word);

    fn get_s_bit(&self) -> Option<bool>;
    fn set_s_bit(&mut self, bit: Word);

    fn get_shift_type(&self) -> Option<ShiftType>;
    fn set_shift_type(&mut self, sh: Word);

    fn get_imm_shift(&self) -> Option<Byte>;
    fn set_imm_shift(&mut self, imm: Word);

    fn get_imm(&self) -> Option<Word>;
    fn set_imm(&mut self, rotate: Word, imm: Word);

    // TODO:
    // - get/set writeback
    // - get/set addressing mode
    // - get/set swi
    // - get/set l bit
    // - get/set LSH
    // - get/set add/sub
    // - get/set byte/word
    // - get/set ldr/str
    // - store constant

    // shift value in immediate_shift field by shift amount
    fn shift_value(&mut self, shift_amount: Byte, shift_type: ShiftType);

    fn get_rd(&self) -> Option<Register>;
    fn set_rd(&mut self, rd: Word);
    fn get_rn(&self) -> Option<Register>;
    fn set_rn(&mut self, rn: Word);
    fn get_rm(&self) -> Option<Register>;
    fn set_rm(&mut self, rm: Word);

    // NZCV flag
    fn set_condition(&mut self, cond: Word);
    fn get_condition(&self) -> Condition;
}

// the Instruction struct will hold flags for all possible Instruction types
// to simplify how instructions are passed around the CPU; it increases the size
// of the Instruction struct, but most of the data in the struct are just
// Options with minimal data

#[derive(Clone, Copy)]
pub struct Instruction {
    _type: InstrType,
    execute: Option<fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, &mut MutexGuard<'_, CPU>, Instruction) -> Word>,
    rn: Option<Register>,
    rd: Option<Register>,
    rm: Option<Register>,
    condition: Condition,
    data_opcode: Option<DataOpcode>,
    s_bit: Option<bool>,
    shift_type: Option<ShiftType>,
    imm_shift: Option<Byte>,
    imm: Option<Word>
}

impl TInstruction for Instruction {
    fn new(_type: InstrType) -> Self {
        Instruction {
            _type: _type,
            execute: None,
            rn: None,
            rd: None,
            rm: None,
            condition: Condition::AL,
            data_opcode: None,
            s_bit: None,
            shift_type: None,
            imm_shift: None,
            imm: None
        }
    }

    fn get_type(&self) -> InstrType {
        self._type
    }

    fn set_type(&mut self, _type: InstrType) {
        self._type = _type;
    }

    fn decode(&self) {
        todo!()
    }

    fn encode(&self) {
        todo!()
    }
    
    fn get_name(&self) -> String {
        todo!()
    }

    fn shift_value(&mut self, shift_amount: Byte, shift_type: ShiftType) {
        let value = match shift_type {
            ShiftType::LSL => todo!(),
            ShiftType::LSR => todo!(),
            ShiftType::ASR => todo!(),
            ShiftType::ROR => todo!(),
        };
    }

    fn get_rd(&self) -> Option<Register> {
        self.rd
    }

    fn set_rd(&mut self, rd: Word) {
        let rd: Register = num::FromPrimitive::from_u32(rd).unwrap();
        self.rd = Some(rd);
    }

    fn get_rn(&self) -> Option<Register> {
        self.rn
    }

    fn set_rn(&mut self, rn: Word) {
        let rn: Register = num::FromPrimitive::from_u32(rn).unwrap();
        self.rn = Some(rn);
    }

    fn get_rm(&self) -> Option<Register> {
        self.rm
    }

    fn set_rm(&mut self, rm: Word) {
        let rm: Register = num::FromPrimitive::from_u32(rm).unwrap();
        self.rm = Some(rm);
    }

    fn set_condition(&mut self, cond: Word) {
        let condition: Condition = num::FromPrimitive::from_u32(cond).unwrap();
        self.condition = condition;
    }

    fn get_condition(&self) -> Condition {
        self.condition
    }

    fn get_data_opcode(&self) -> Option<DataOpcode> {
        self.data_opcode
    }

    fn set_data_opcode(&mut self, code: Word) {
        let opcode: DataOpcode = num::FromPrimitive::from_u32(code).unwrap();
        self.data_opcode = Some(opcode);
    }

    fn get_s_bit(&self) -> Option<bool> {
        self.s_bit
    }

    fn set_s_bit(&mut self, bit: Word) {
        match bit & 0x1 {
            0 => self.s_bit = Some(false),
            1 => self.s_bit = Some(true),
            _ => self.s_bit = None
        }
    }

    fn get_shift_type(&self) -> Option<ShiftType> {
        self.shift_type
    }

    fn set_shift_type(&mut self, sh: Word) {
        let shift: ShiftType = num::FromPrimitive::from_u32(sh).unwrap();
        self.shift_type = Some(shift);
    }

    fn get_imm_shift(&self) -> Option<Byte> {
        self.imm_shift
    }

    fn set_imm_shift(&mut self, imm: Word) {
        self.imm_shift = Some((imm & 0xff) as Byte);
    }

    fn get_imm(&self) -> Option<Word> {
        self.imm
    }

    fn set_imm(&mut self, rotate: Word, imm: Word) {
        let immediate_to_rotate: Word = (imm & 0xff) as Word;
        let rotate_amount = ((rotate as Byte) * 2) as Word;
        self.imm = Some(immediate_to_rotate.rotate_right(rotate_amount));
    }

    fn set_execute(&mut self, f: fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, &mut MutexGuard<'_, CPU>, Instruction) -> Word) {
        self.execute = Some(f);
    }

    fn get_execute(&self) -> fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, &mut MutexGuard<'_, CPU>, Instruction) -> Word {
        self.execute.unwrap()
    }
}

// formatted output for the instructions
// used for disassembly
impl fmt::Display for Instruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.get_type() {
            InstrType::DataImm => {

                // for each instruction, produce the opcode
                // and all information associated with it
                // through specific matches that write directly
                // to the output stream

                match self.get_data_opcode().unwrap() {
                    DataOpcode::MOV => fmt.write_str("mov")?,
                    _ => fmt.write_str("nop")?
                };

                match self.get_s_bit().unwrap() {
                    true => fmt.write_str("s")?,
                    false => ()
                }

                fmt.write_str(" ")?;
                fmt.write_str(self.get_rd().unwrap().to_string().as_str())?;
                fmt.write_str(", #")?;
                fmt.write_str(self.get_imm().unwrap_or(0).to_string().as_str())?;
            },
            // TODO: all other types
            _ => {}
        };

        Ok(())
    }
}

/*
Individual type constructs
*/

pub trait TTypeData: TInstruction {
    fn get_opcode(&self) -> DataOpcode;
    fn get_s_bit(&self) -> bool;
    fn get_shift_type(&self) -> ShiftType;
}

/*
Individual instruction factories
yay java

Each factory takes the bit patterns from the bitmatcher in the CPU decode step
and produces an Instruction struct with all the necessary methods attached
*/

pub fn instr_data_reg_imm(condition: Word, opcode: Word, s_bit: Word, rn: Word, rd: Word, imm: Word, shift_type: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::DataRegImm);
    instr.set_condition(condition);
    instr.set_data_opcode(opcode);
    instr.set_s_bit(s_bit);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm_shift(imm);
    instr.set_shift_type(shift_type);
    instr.set_rm(rm);

    instr
}

pub fn instr_data_imm(condition: Word, opcode: Word, s_bit: Word, rn: Word, rd: Word, rotate: Word, imm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::DataImm);
    instr.set_condition(condition);
    instr.set_data_opcode(opcode);
    instr.set_s_bit(s_bit);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm(rotate, imm);

    // attach the appopriate execute function (could also be done inline)
    instr.set_execute(execute::instr_data_imm);

    instr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble() {
        let instr = instr_data_imm(0b1110, 0b1101, 0b0, 0b0000, 0b0010, 0b0000, 0b110000);
        assert_eq!("mov r2, #48", instr.to_string())
    }
}