use tokio::sync::MutexGuard;

use crate::execute;
use crate::memory::{Byte, Word, Register, RAM, Registers, Memory};
use crate::cpu_enum::{Condition, ShiftType, DataOpcode, InstrType, LSH, LDMCode};

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

    fn get_l_bit(&self) -> Option<bool>;
    fn set_l_bit(&mut self, bit: Word);

    fn get_offset(&self) -> Option<i32>;
    fn set_offset(&mut self, offset: i32);

    fn get_lsh(&self) -> Option<LSH>;
    fn set_lsh(&mut self, lsh: Word);
    
    fn get_add_sub(&self) -> Option<bool>;
    fn set_add_sub(&mut self, op: bool);

    fn get_byte_word(&self) -> Option<bool>;
    fn set_byte_word(&mut self, op: bool);

    fn get_ldr_str(&self) -> Option<bool>;
    fn set_ldr_str(&mut self, op: bool);

    fn get_ldm(&self) -> Option<LDMCode>;
    fn set_ldm(&mut self, ldm: Word);

    fn get_reg_list(&self) -> Option<Word>;
    fn set_reg_list(&mut self, reg_list: Word);

    fn get_last_char(&self) -> Option<char>;
    fn set_last_char(&mut self, last_char: char);

    // shift value in immediate_shift field by shift amount
    // https://developer.arm.com/documentation/dui0489/i/arm-and-thumb-instructions/operand2-as-a-register-with-optional-shift?lang=en
    // ARM Manual A5.1
    fn shift_value_by_imm(rm: Word, shift_imm: Word, shift_type: ShiftType, c_flag: Byte) -> (Word, Word) {
        let rm_msb = (rm >> 31) & 1;
        match shift_type {
            ShiftType::LSL => {
                if shift_imm == 0 {
                    (rm, c_flag as Word)
                } else {
                    (rm << shift_imm, (rm >> (32 - shift_imm)) & 1)
                }
            },
            ShiftType::LSR => {
                if shift_imm == 0 {
                    (0, rm_msb)
                } else {
                    (rm >> shift_imm, (rm >> (shift_imm - 1)) & 1)
                }
            },
            ShiftType::ASR => {
                if shift_imm == 0 {
                    if (rm >> 31) & 1 == 0 {
                        (0, rm_msb)
                    } else {
                        (0xffffffff, rm_msb)
                    }
                } else {
                    // cast to i32 to force ASR in Rust
                    (((rm as i32) >> shift_imm) as Word, (rm >> (shift_imm - 1)) & 1)
                }
            },
            ShiftType::ROR => {
                if shift_imm == 0 {
                    ((c_flag << 31) as Word | (rm >> 1), rm & 1)
                } else {
                    (rm.rotate_right(shift_imm), (rm >> (shift_imm - 1)) & 1)
                }
            }
        }
    }

    fn shift_value_by_reg(rm: Word, rs: Word, shift_type: ShiftType, c_flag: Byte) -> (Word, Word) {
        let rs_lsb = rs & 0xff;
        let rs_lsn = rs & 0xf;
        let rm_msb = (rm >> 31) & 1;

        match shift_type {
            ShiftType::LSL => {
                if rs_lsb == 0 {
                    (rm, c_flag as Word)
                } else if rs_lsb < 32 {
                    (rm << rs_lsb, (rm >> (32 - rs_lsb)) & 1)
                } else if rs_lsb == 32 {
                    (0, rm & 1)
                } else {
                    (0, 0)
                }
            },
            ShiftType::LSR => {
                if rs_lsb == 0 {
                    (rm, c_flag as Word)
                } else if rs_lsb < 32 {
                    (rm >> rs_lsb, (rm >> (rs_lsb - 1)) & 1)
                } else if rs_lsb == 32 {
                    (0, rm_msb)
                } else {
                    (0, 0)
                }
            },
            ShiftType::ASR => {
                if rs_lsb == 0 {
                    (rm, c_flag as Word)
                } else if rs_lsb < 32 {
                    // cast to i32 to force ASR in Rust
                    (((rm as i32) >> rs_lsb) as Word, (rm >> (rs_lsb - 1)) & 1)
                } else {
                    if rm_msb == 0 {
                        (0, rm_msb)
                    } else {
                        (0xffffffff, rm_msb)
                    }
                }
            },
            ShiftType::ROR => {
                if rs_lsb == 0 {
                    (rm, c_flag as Word)
                } else if rs_lsn == 0 {
                    (rm, rm_msb)
                } else {
                    (rm.rotate_right(rs), (rm >> (rs_lsn - 1)) & 1)
                }
            }
        }
    }

    // tuple: (shifter_operand, shifter_carry_out)
    // A5.1.3
    fn rotate_value(rotate_imm: Byte, immed_8: Byte, c_flag: Byte) -> (Word, Word) {
        let immediate_to_rotate: Word = (immed_8 & 0xff) as Word;
        let rotate_amount: Word = ((rotate_imm as Byte) * 2) as Word;
        let shifter_operand = immediate_to_rotate.rotate_right(rotate_amount);
        if rotate_imm == 0 {
            (shifter_operand, c_flag as Word)
        } else {
            (shifter_operand, (shifter_operand >> 31) & 1)
        }
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
    ldr_str: Option<bool>,
    l_bit: Option<bool>,
    offset: Option<i32>,
    lsh: Option<LSH>,
    ldm: Option<LDMCode>,
    reg_list: Option<Word>,
    last_char: Option<char>
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
            ldr_str: None,
            l_bit: None,
            offset: None,
            lsh: None,
            ldm: None,
            reg_list: None,
            last_char: None
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

    fn get_l_bit(&self) -> Option<bool> {
        self.l_bit
    }

    fn set_l_bit(&mut self, bit: Word) {
        match bit & 0x1 {
            0 => self.l_bit = Some(false),
            1 => self.l_bit = Some(true),
            _ => self.l_bit = None
        }
    }

    fn get_offset(&self) -> Option<i32> {
        self.offset
    }

    fn set_offset(&mut self, offset: i32) {
        self.offset = Some(offset)
    }

    fn get_lsh(&self) -> Option<LSH> {
        self.lsh
    }

    fn set_lsh(&mut self, lsh: Word) {
        let lsh_code: LSH = num::FromPrimitive::from_u32(lsh).unwrap();
        self.lsh = Some(lsh_code);
    }

    fn get_ldm(&self) -> Option<LDMCode> {
        self.ldm
    }

    fn set_ldm(&mut self, ldm: Word) {
        let ldm_code: LDMCode = num::FromPrimitive::from_u32(ldm).unwrap();
        self.ldm = Some(ldm_code);
    }

    fn get_reg_list(&self) -> Option<Word> {
        self.reg_list
    }

    fn set_reg_list(&mut self, reg_list: Word) {
        self.reg_list = Some(reg_list);
    }

    fn get_last_char(&self) -> Option<char> {
        self.last_char
    }

    fn set_last_char(&mut self, last_char: char) {
        self.last_char = Some(last_char);
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
    instr.set_execute(execute::instr_data_imm);

    instr
}

pub fn instr_ldrstr_shifted_reg_pre(condition: Word, add_sub: Word, byte_word: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, imm: Word, shift_type: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRSTRShiftRegPre);
    instr.set_condition(condition); 
    instr.set_addr_mode(true);
    // TODO: move logic for calls like this to the interior of the setter?
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_byte_word((byte_word & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm_shift(imm);
    instr.set_shift_type(shift_type);
    instr.set_rm(rm);
    instr.set_execute(execute::instr_ldrstr_shifted_reg_pre);

    instr
}

pub fn instr_ldrstr_shifted_reg_post(condition: Word, add_sub: Word, byte_word: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, imm: Word, shift_type: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRSTRShiftRegPost);
    instr.set_condition(condition); 
    instr.set_addr_mode(false);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_byte_word((byte_word & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm_shift(imm);
    instr.set_shift_type(shift_type);
    instr.set_rm(rm);
    instr.set_execute(execute::instr_ldrstr_shifted_reg_post);

    instr
}

pub fn instr_ldrstr_reg_pre(condition: Word, add_sub: Word, byte_word: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRHSTRHRegPre);
    instr.set_condition(condition); 
    instr.set_addr_mode(true);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_byte_word((byte_word & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_rm(rm);
    instr.set_execute(execute::instr_ldrstr_reg_pre);

    instr
}

pub fn instr_ldrstr_reg_post(condition: Word, add_sub: Word, byte_word: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRHSTRHRegPost);
    instr.set_condition(condition); 
    instr.set_addr_mode(false);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_byte_word((byte_word & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_rm(rm);
    instr.set_execute(execute::instr_ldrstr_reg_post);

    instr
}

pub fn instr_ldrstr_imm_pre(condition: Word, add_sub: Word, byte_word: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, imm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRSTRImmPre);
    instr.set_condition(condition); 
    instr.set_addr_mode(true);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_byte_word((byte_word & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm_shift(imm);
    instr.set_execute(execute::instr_ldrstr_imm_pre);

    instr
}

pub fn instr_ldrstr_imm_post(condition: Word, add_sub: Word, byte_word: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, imm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRSTRImmPost);
    instr.set_condition(condition); 
    instr.set_addr_mode(false);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_byte_word((byte_word & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm_shift(imm);
    instr.set_execute(execute::instr_ldrstr_imm_post);

    instr
}

pub fn instr_ldrhstrh_imm_pre(condition: Word, add_sub: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, high_bits: Word, lsh: Word, low_bits: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRHSTRHImmPre);
    instr.set_condition(condition); 
    instr.set_addr_mode(true);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_lsh(lsh);
    instr.set_imm(((high_bits as Byte) << 4) | (low_bits as Byte));
    instr.set_execute(execute::instr_ldrhstrh_imm_pre);

    instr
}

pub fn instr_ldrhstrh_imm_post(condition: Word, add_sub: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, high_bits: Word, lsh: Word, low_bits: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRHSTRHImmPost);
    instr.set_condition(condition); 
    instr.set_addr_mode(false);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_lsh(lsh);
    instr.set_imm(((high_bits as Byte) << 4) | (low_bits as Byte));
    instr.set_execute(execute::instr_ldrhstrh_imm_post);

    instr
}

pub fn instr_ldrhstrh_reg_pre(condition: Word, add_sub: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, lsh: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRHSTRHRegPre);
    instr.set_condition(condition); 
    instr.set_addr_mode(true);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_lsh(lsh);
    instr.set_rm(rm);
    instr.set_execute(execute::instr_ldrhstrh_reg_pre);

    instr
}

pub fn instr_ldrhstrh_reg_post(condition: Word, add_sub: Word, writeback: Word, ldr_str: Word, rn: Word, rd: Word, lsh: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDRHSTRHRegPost);
    instr.set_condition(condition); 
    instr.set_addr_mode(false);
    instr.set_add_sub((add_sub & 0x1) != 0);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_lsh(lsh);
    instr.set_rm(rm);
    instr.set_execute(execute::instr_ldrhstrh_reg_post);

    instr
}

pub fn instr_b(condition: Word, l_bit: Word, offset: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::B);
    instr.set_condition(condition); 
    instr.set_l_bit(l_bit);
    instr.set_offset(((offset as i32) << 2) + 8);
    instr.set_execute(execute::instr_b);

    instr
}

pub fn instr_bx(condition: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::BX);
    instr.set_condition(condition); 
    instr.set_rm(rm);
    instr.set_execute(execute::instr_bx);

    instr
}

pub fn instr_ldmstm(condition: Word, ldm_code: Word, s_bit: Word, writeback: Word, ldr_str: Word, rn: Word, reg_list: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::LDMSTM);
    instr.set_condition(condition); 
    instr.set_ldm(ldm_code);
    instr.set_s_bit(s_bit);
    instr.set_writeback((writeback & 0x1) != 0);
    instr.set_ldr_str((ldr_str & 0x1) != 0);
    instr.set_rn(rn);
    instr.set_reg_list(reg_list);
    instr.set_execute(execute::instr_ldmstm);

    instr
}

pub fn instr_mul(condition: Word, s_bit: Word, rd: Word, rs: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::MUL);
    instr.set_condition(condition);
    instr.set_s_bit(s_bit);
    instr.set_rd(rd);
    instr.set_rs(rs);
    instr.set_rm(rm);
    instr.set_execute(execute::instr_mul);

    instr
}

pub fn instr_swi(condition: Word, swi: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::SWI);
    instr.set_condition(condition);
    instr.set_swi(swi);
    instr.set_execute(execute::instr_swi);

    instr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disassemble_instr_data_imm() {
        let instr = instr_data_imm(0b1110, 0b1101, 0b0, 0b0000, 0b0010, 0b0000, 0b110000);
        assert_eq!("mov r2, #48", instr.to_string())
    }

    #[test]
    fn test_shift_value() {
        todo!()
    }

    #[test]
    fn test_rotate_value() {
        todo!();
    }

}