use std::fmt;

use tokio::sync::MutexGuard;

use crate::execute;
use crate::memory::{Byte, Word, Register, RAM, Registers};
use crate::cpu_enum::{Condition, ShiftType, DataOpcode, InstrType};

// This is the parent trait that contains all the default implementations
// for each instruction; all the getters and setters are implemented directly
// for the Instruction struct 
pub trait TInstruction {
    fn new(_type: InstrType) -> Self;

    fn decode(&self);
    fn encode(&self);

    fn set_execute(&mut self, f: fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, Instruction) -> Word);
    fn get_execute(&self) -> fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, Instruction) -> Word;

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

    fn get_imm_shift(&self) -> Option<Word>;
    fn set_imm_shift(&mut self, imm: Word);

    fn get_rotate(&self) -> Option<Byte>;
    fn set_rotate(&mut self, rotate: Byte);

    fn get_imm(&self) -> Option<Byte>;
    fn set_imm(&mut self, imm: Byte);

    fn get_writeback(&self) -> Option<bool>;
    fn set_writeback(&mut self, op: bool);

    fn get_addr_mode(&self) -> Option<bool>;
    fn set_addr_mode(&mut self, op: bool);

    fn get_swi(&self) -> Option<Word>;
    fn set_swi(&mut self, num: Word);

    // - get/set l bit
    // - get/set LSH
    
    fn get_add_sub(&self) -> Option<bool>;
    fn set_add_sub(&mut self, op: bool);

    fn get_byte_word(&self) -> Option<bool>;
    fn set_byte_word(&mut self, op: bool);

    fn get_ldr_str(&self) -> Option<bool>;
    fn set_ldr_str(&mut self, op: bool);

    // - store constant

    // shift value in immediate_shift field by shift amount
    // https://developer.arm.com/documentation/dui0489/i/arm-and-thumb-instructions/operand2-as-a-register-with-optional-shift?lang=en
    fn shift_value(value: Word, shift_amount: Word, shift_type: ShiftType) -> Word {
        match shift_type {
            ShiftType::LSL => todo!(),
            ShiftType::LSR => todo!(),
            ShiftType::ASR => todo!(),
            ShiftType::ROR => todo!(),
        };
        0
    }

    fn rotate_value(rotate: Byte, imm: Byte) -> Word {
        let immediate_to_rotate: Word = (imm & 0xff) as Word;
        let rotate_amount: Word = ((rotate as Byte) * 2) as Word;
        immediate_to_rotate.rotate_right(rotate_amount)
    }

    fn get_rd(&self) -> Option<Register>;
    fn set_rd(&mut self, rd: Word);
    fn get_rn(&self) -> Option<Register>;
    fn set_rn(&mut self, rn: Word);
    fn get_rm(&self) -> Option<Register>;
    fn set_rm(&mut self, rm: Word);
    fn get_rs(&self) -> Option<Register>;
    fn set_rs(&mut self, rs: Word);

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
    execute: Option<fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, Instruction) -> Word>,
    rn: Option<Register>,
    rd: Option<Register>,
    rm: Option<Register>,
    rs: Option<Register>,
    condition: Condition,
    data_opcode: Option<DataOpcode>,
    s_bit: Option<bool>,
    shift_type: Option<ShiftType>,
    imm_shift: Option<Word>,
    rotate: Option<Byte>,
    imm: Option<Byte>,
    writeback: Option<bool>,
    addr_mode: Option<bool>,
    swi: Option<Word>,
    add_sub: Option<bool>,
    byte_word: Option<bool>,
    ldr_str: Option<bool>
}

impl TInstruction for Instruction {
    fn new(_type: InstrType) -> Self {
        Instruction {
            _type: _type,
            execute: None,
            rn: None,
            rd: None,
            rm: None,
            rs: None,
            condition: Condition::AL,
            data_opcode: None,
            s_bit: None,
            shift_type: None,
            imm_shift: None,
            rotate: None,
            imm: None,
            writeback: None,
            addr_mode: None,
            swi: None,
            add_sub: None,
            byte_word: None,
            ldr_str: None
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

    fn get_rs(&self) -> Option<Register> {
        self.rs
    }

    fn set_rs(&mut self, rs: Word) {
        let rs: Register = num::FromPrimitive::from_u32(rs).unwrap();
        self.rs = Some(rs);
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

    fn get_imm_shift(&self) -> Option<Word> {
        self.imm_shift
    }

    fn set_imm_shift(&mut self, imm: Word) {
        self.imm_shift = Some(imm);
    }

    fn get_imm(&self) -> Option<Byte> {
        self.imm
    }

    fn set_imm(&mut self, imm: Byte) {
        self.imm = Some(imm);
    }

    fn set_execute(&mut self, f: fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, Instruction) -> Word) {
        self.execute = Some(f);
    }

    fn get_execute(&self) -> fn(&mut MutexGuard<'_, RAM>, &mut MutexGuard<'_, Registers>, Instruction) -> Word {
        self.execute.unwrap()
    }

    fn get_rotate(&self) -> Option<Byte> {
        self.rotate
    }

    fn set_rotate(&mut self, rotate: Byte) {
        self.rotate = Some(rotate)
    }

    fn get_writeback(&self) -> Option<bool> {
        self.writeback
    }

    fn set_writeback(&mut self, op: bool) {
        self.writeback = Some(op)
    }

    fn get_addr_mode(&self) -> Option<bool> {
        self.addr_mode
    }

    fn set_addr_mode(&mut self, op: bool) {
        self.addr_mode = Some(op)
    }

    fn get_swi(&self) -> Option<Word> {
        self.swi
    }

    fn set_swi(&mut self, num: Word) {
        self.swi = Some(num)
    }

    fn get_add_sub(&self) -> Option<bool> {
        self.add_sub
    }

    fn set_add_sub(&mut self, op: bool) {
        self.add_sub = Some(op)
    }

    fn get_byte_word(&self) -> Option<bool> {
        self.byte_word
    }

    fn set_byte_word(&mut self, op: bool) {
        self.byte_word = Some(op)
    }

    fn get_ldr_str(&self) -> Option<bool> {
        self.ldr_str
    }

    fn set_ldr_str(&mut self, op: bool) {
        self.ldr_str = Some(op)
    }
}

fn get_data_opcode_str(opcode: DataOpcode) -> String {
    match opcode {
        DataOpcode::MOV => "mov".to_string(),
        DataOpcode::AND => "and".to_string(),
        DataOpcode::EOR => "eor".to_string(),
        DataOpcode::SUB => "sub".to_string(),
        DataOpcode::RSB => "rsb".to_string(),
        DataOpcode::ADD => "add".to_string(),
        DataOpcode::ADC => "adc".to_string(),
        DataOpcode::SBC => "sbc".to_string(),
        DataOpcode::RSC => "rsc".to_string(),
        DataOpcode::TST => "tst".to_string(),
        DataOpcode::TEQ => "teq".to_string(),
        DataOpcode::CMP => "cmp".to_string(),
        DataOpcode::CMN => "cmn".to_string(),
        DataOpcode::ORR => "orr".to_string(),
        DataOpcode::BIC => "bic".to_string(),
        DataOpcode::MVN => "mvn".to_string(),
    }
}

fn get_s_bit_str(s_bit: bool) -> String {
    match s_bit {
        true => "s".to_string(),
        false => "".to_string()
    }
}

fn get_condition_str(condition: Condition) -> String {
    match condition {
        Condition::AL => "".to_string(),
        _ => condition.to_string()
    }
}

fn get_byte_word_str(op: bool) -> String {
    match op {
        true => "b".to_string(),
        false => "".to_string()
    }
}

fn get_ldr_str_str(op: bool) -> String {
    match op {
        true => "ldr".to_string(),
        false => "str".to_string()
    }
}

fn get_writeback_str(op: bool) -> String {
    match op {
        true => "!".to_string(),
        false => "".to_string()
    }
}

// formatted output for the instructions
// used for disassembly
impl fmt::Display for Instruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.get_type() {
            InstrType::DataImm => {
                // mov rd, #imm8

                // optional operand2
                let rn = match self.get_data_opcode().unwrap() {
                    DataOpcode::MOV | DataOpcode::MVN => "".to_string(), 
                    _ => match self.get_rn() {
                        Some(r) => format!("{},", r.to_string()),
                        None => "".to_string(),
                    },
                };

                // for each instruction, produce the opcode
                // and all information associated with it
                fmt.write_str(
                    format!(
                        "{}{}{} {}, {} #{}",
                        get_data_opcode_str(self.get_data_opcode().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_s_bit_str(self.get_s_bit().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        rn,
                        self.get_imm().unwrap_or(0).to_string()
                    ).as_str()
                )?;
            },
            InstrType::DataRegImm => {
                // TODO: ignore rn if mov or mvn

                // optional rn
                let rn = match self.get_data_opcode().unwrap() {
                    DataOpcode::MOV | DataOpcode::MVN => "".to_string(), 
                    _ => match self.get_rn() {
                        Some(r) => format!("{},", r.to_string()),
                        None => "".to_string(),
                    },
                };
                
                // optional operand2
                let operand2 = match self.get_imm_shift() {
                    Some(_) => format!("{} #{}", self.get_shift_type().unwrap().to_string(), self.get_imm_shift().unwrap().to_string()),
                    None => "".to_string(),
                };
                
                fmt.write_str(
                    format!(
                        "{}{}{} {}, {} {} {}",
                        get_data_opcode_str(self.get_data_opcode().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_s_bit_str(self.get_s_bit().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        rn,
                        self.get_rm().unwrap().to_string(),
                        operand2
                    ).as_str()
                )?;
            },
            InstrType::DataRegReg => {
                // add rd, rm, sh rs
                // optional rn
                let rn = match self.get_data_opcode().unwrap() {
                    DataOpcode::MOV | DataOpcode::MVN => "".to_string(), 
                    _ => match self.get_rn() {
                        Some(r) => format!("{},", r.to_string()),
                        None => "".to_string(),
                    }
                };
                
                // optional operand2
                let operand2 = match self.get_imm_shift() {
                    Some(_) => format!("{} {}", self.get_shift_type().unwrap().to_string(), self.get_rs().unwrap().to_string()),
                    None => "".to_string(),
                };
                

                fmt.write_str(
                    format!(
                        "{}{}{} {}, {} {}, {}",
                        get_data_opcode_str(self.get_data_opcode().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_s_bit_str(self.get_s_bit().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        rn,
                        self.get_rm().unwrap().to_string(),
                        operand2
                    ).as_str()
                )?;
            },
            InstrType::LDRSTRShiftReg => {
                // ex: ldr 

                // fmt.write_str(
                //     format!(
                //         "{}{}{} {}, {} {}, {}",
                //         get_data_opcode_str(self.get_data_opcode().unwrap()),
                //         get_condition_str(self.get_condition()),
                //         get_s_bit_str(self.get_s_bit().unwrap()),
                //         self.get_rd().unwrap().to_string(),
                //         rn,
                //         self.get_rm().unwrap().to_string(),
                //         operand2
                //     ).as_str()
                // )?;
            },
            // InstrType::LDRSTRReg => todo!(),
            InstrType::LDRSTRImm => {
                // ex: strb rd,  [rn, #8]!
                //     {}{} {},  {{}, {}}{}

                let addr_mode = match self.get_addr_mode().unwrap() {
                    true => {
                        format!(
                            "[{}, #{}]",
                            self.get_rn().unwrap().to_string(),
                            self.get_imm_shift().unwrap().to_string()
                        )
                    },
                    false => {
                        format!(
                            "{}, #{}",
                            self.get_rn().unwrap().to_string(),
                            self.get_imm_shift().unwrap().to_string()
                        )
                    }
                };

                fmt.write_str(
                    format!(
                        "{}{}{} {}, {}{}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_byte_word_str(self.get_byte_word().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        addr_mode,
                        get_writeback_str(self.get_writeback().unwrap())
                    ).as_str()
                )?;
            },
            // InstrType::LDRHSTRHImm => todo!(),
            // InstrType::LDRHSTRHReg => todo!(),
            // InstrType::LSM => todo!(),
            // InstrType::Branch => todo!(),
            // InstrType::SWI => todo!(),
            // InstrType::Multiply => todo!(),
            // InstrType::NOP => todo!()
            _ => ()
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

    instr.set_execute(execute::instr_data_reg_imm);

    instr
}

pub fn instr_data_reg_reg(condition: Word, opcode: Word, s_bit: Word, rn: Word, rd: Word, rs: Word, shift_type: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::DataRegReg);
    instr.set_condition(condition);
    instr.set_data_opcode(opcode);
    instr.set_s_bit(s_bit);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_rs(rs);
    instr.set_shift_type(shift_type);
    instr.set_rm(rm);

    instr.set_execute(execute::instr_data_reg_reg);

    instr
}

pub fn instr_data_imm(condition: Word, opcode: Word, s_bit: Word, rn: Word, rd: Word, rotate: Word, imm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::DataImm);
    instr.set_condition(condition);
    instr.set_data_opcode(opcode);
    instr.set_s_bit(s_bit);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_rotate(rotate as Byte);
    instr.set_imm(imm as Byte);

    // attach the appopriate execute function (could also be done inline)
    instr.set_execute(execute::instr_data_imm);

    instr
}

pub fn instr_ldrstr_shifted_reg(condition: Word, addr_mode: Word, add_sub: Word, byte_word: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, imm: Word, shift_type: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRSTRShiftReg);
    instr.set_condition(condition); 
    instr.set_addr_mode((addr_mode & 0x1) != 0);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_byte_word((byte_word & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm_shift(imm);
    instr.set_shift_type(shift_type);
    instr.set_rm(rm);

    // attach the appopriate execute function (could also be done inline)
    instr.set_execute(execute::instr_ldrstr_shifted_reg);

    instr
}

pub fn instr_ldrstr_imm(condition: Word, addr_mode: Word, add_sub: Word, byte_word: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, imm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRSTRImm);
    instr.set_condition(condition); 
    instr.set_addr_mode((addr_mode & 0x1) != 0);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_byte_word((byte_word & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm_shift(imm);

    // attach the appopriate execute function (could also be done inline)
    instr.set_execute(execute::instr_ldrstr_imm);

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