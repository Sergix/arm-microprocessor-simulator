use tokio::sync::MutexGuard;

use crate::{memory::{Word, Registers, RAM, Byte, Memory, Register, HalfWord}, instruction::{Instruction, TInstruction}, cpu_enum::{DataOpcode, ShiftType, LDMCode}};

// this method matches all the data operations with their appropriate operation
// the caller is expected to resolve the operand2 ahead of time; this function
// does the minimal amount of computation possible while modifying the app state
fn data_match_opcode(registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, rn: Word, shifter_operand: Word) {
    // temp values so register_lock doesn't have to be moved
    let c_flag = registers_lock.get_c_flag();
    let mut alu_out = 0;

    let value: Option<Word> = match instr.get_data_opcode().unwrap() {
        DataOpcode::AND => Some(rn & shifter_operand),
        DataOpcode::EOR => Some(rn ^ shifter_operand),
        DataOpcode::SUB => Some(rn.wrapping_sub(shifter_operand)),
        DataOpcode::RSB => Some(shifter_operand.wrapping_sub(rn)),
        DataOpcode::ADD => Some(rn.wrapping_add(shifter_operand)),
        DataOpcode::ADC => Some(rn.wrapping_add(shifter_operand + (if c_flag { 1 } else { 0 }))),
        DataOpcode::SBC => Some(rn.wrapping_sub(shifter_operand - (if !c_flag { 1 } else { 0 }))),
        DataOpcode::RSC => Some(shifter_operand.wrapping_sub(rn - (if !c_flag { 1 } else { 0 }))),
        DataOpcode::ORR => Some(rn | shifter_operand),
        DataOpcode::MOV => Some(shifter_operand),
        DataOpcode::BIC => Some(rn & !shifter_operand),
        DataOpcode::MVN => Some(!shifter_operand),
        _ => None
    };

    if value.is_some() {
        // rd := (result)
        registers_lock.set_reg_register(instr.get_rd().unwrap(), value.unwrap());
    } else {
        match instr.get_data_opcode().unwrap() {
            DataOpcode::TST => alu_out = rn & shifter_operand,
            DataOpcode::TEQ => alu_out = rn ^ shifter_operand,
            DataOpcode::CMP => alu_out = rn - shifter_operand,
            DataOpcode::CMN => alu_out = rn + shifter_operand,
            _ => ()
        }
    }

    // TODO: update CPSR flags if s-bit enabled
    if instr.get_s_bit().unwrap() {
        // match DataOpcode...
    }
}

fn ldr_str_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, offset: Word) {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());

    let address = match instr.get_add_sub().unwrap() {
        true  /* add */ => rn + offset,
        false /* sub */ => rn - offset
    };

    match instr.get_ldr_str().unwrap() {
        true  /* ldr */ => {
            let data = match instr.get_byte_word().unwrap() {
                true  => ram_lock.read_byte(address) as Word,
                false => ram_lock.read_word(address)
            };
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data);
        },
        false /* str */ => {
            match instr.get_byte_word().unwrap() {
                true  => ram_lock.write_byte(address, rd as Byte),
                false => ram_lock.write_word(address, rd)
            };
        }
    }
    
    if instr.get_writeback().unwrap() {
        registers_lock.set_reg_register(instr.get_rn().unwrap(), address);
    }
}

fn ldr_str_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, offset: Word) {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());

    let address = rn;

    match instr.get_ldr_str().unwrap() {
        true  /* ldr */ => {
            let data = match instr.get_byte_word().unwrap() {
                true  => ram_lock.read_byte(address) as Word,
                false => ram_lock.read_word(address)
            };
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data);
        },
        false /* str */ => {
            match instr.get_byte_word().unwrap() {
                true  => ram_lock.write_byte(address, rd as Byte),
                false => ram_lock.write_word(address, rd)
            };
        }
    }

    // writeback
    registers_lock.set_reg_register(
        instr.get_rn().unwrap(),
        match instr.get_add_sub().unwrap() {
            true  /* add */ => rn + offset,
            false /* sub */ => rn - offset
        }
    );
}

// TODO: match with LSH code
fn ldrh_strh_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, offset: Word) {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());
    
    let address = match instr.get_add_sub().unwrap() {
        true  /* add */ => rn + offset,
        false /* sub */ => rn - offset
    };
    
    match instr.get_ldr_str().unwrap() {
        true  /* ldr */ => {
            let data = ram_lock.read_half_word(address);
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data as Word);
        },
        false /* str */ => {
            ram_lock.write_half_word(address, rd as HalfWord);
        }
    }
    
    if instr.get_writeback().unwrap() {
        registers_lock.set_reg_register(instr.get_rn().unwrap(), address);
    }
}

// TODO: match with LSH code
fn ldrh_strh_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, offset: Word) {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());

    let address = rn;

    match instr.get_ldr_str().unwrap() {
        true  /* ldr */ => {
            let data = ram_lock.read_half_word(address);
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data as Word);
        },
        false /* str */ => {
            ram_lock.write_half_word(address, rd as HalfWord);
        }
    }

    // writeback
    registers_lock.set_reg_register(
        instr.get_rn().unwrap(),
        match instr.get_add_sub().unwrap() {
            true  /* add */ => rn + offset,
            false /* sub */ => rn - offset
        }
    );
}

// each method accesses/modifies app state as necessary;
// however, this renders the code difficult to test with the current Tauri APIs
// since the Tauri State API cannot be mocked (yet)

// each of these methods are mapped to the Instruction::execute method
// and called from CPU::execute

pub fn instr_data_reg_imm(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let shifter_operand = Instruction::shift_value(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        instr.get_imm_shift().unwrap(),
        instr.get_shift_type().unwrap());

    data_match_opcode(registers_lock, instr, rn, shifter_operand);

    // return result
    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_data_reg_reg(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let shifter_operand = Instruction::shift_value(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        registers_lock.get_reg_register(instr.get_rs().unwrap()) & 0xff, // grab the LSB
        instr.get_shift_type().unwrap());

    data_match_opcode(registers_lock, instr, rn, shifter_operand);

    // return result
    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_data_imm(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rn =  registers_lock.get_reg_register(instr.get_rn().unwrap());
    let shifter_operand =  Instruction::rotate_value(
        instr.get_rotate().unwrap(),
        instr.get_imm().unwrap());

    data_match_opcode(registers_lock, instr, rn, shifter_operand);

    // return result
    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

// p.463
pub fn instr_ldrstr_shifted_reg_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let index = Instruction::shift_value(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        instr.get_imm_shift().unwrap(),
        instr.get_shift_type().unwrap());

    ldr_str_pre(ram_lock, registers_lock, instr, index);

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrstr_shifted_reg_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let index = Instruction::shift_value(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        instr.get_imm_shift().unwrap(),
        instr.get_shift_type().unwrap());

    ldr_str_post(ram_lock, registers_lock, instr, index);

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrstr_reg_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());
    ldr_str_post(ram_lock, registers_lock, instr, rm);
    
    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrstr_reg_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());
    ldr_str_post(ram_lock, registers_lock, instr, rm);

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrstr_imm_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let offset_12 = instr.get_imm_shift().unwrap();

    ldr_str_pre(ram_lock, registers_lock, instr, offset_12);

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrstr_imm_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let offset_12 = instr.get_imm_shift().unwrap();

    ldr_str_post(ram_lock, registers_lock, instr, offset_12);

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrhstrh_imm_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let offset_8 = instr.get_imm().unwrap() as Word;

    ldrh_strh_pre(ram_lock, registers_lock, instr, offset_8);

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrhstrh_imm_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let offset_8 = instr.get_imm().unwrap() as Word;

    ldrh_strh_post(ram_lock, registers_lock, instr, offset_8);

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrhstrh_reg_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());

    ldrh_strh_pre(ram_lock, registers_lock, instr, rm);

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrhstrh_reg_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());

    ldrh_strh_post(ram_lock, registers_lock, instr, rm);

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_branch(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    if instr.get_l_bit().unwrap() {
        let address_after_branch = registers_lock.get_pc() + 4;
        registers_lock.set_reg_register(Register::r14, address_after_branch);
    }

    let target_address: Word = ((registers_lock.get_pc() as i32) + instr.get_offset().unwrap()) as Word;
    registers_lock.set_pc(target_address);

    registers_lock.get_pc()
}

fn ldm_ldr(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, start_address: Word) -> Word {
    let mut address = start_address;
    
    for ri in 0..14 {
        if (instr.get_reg_list().unwrap() >> ri) & 0x1 == 1 {
            registers_lock.set_register(ri, ram_lock.read_word(address));
            address += 4;
        }
    }

    // if bit 15 n the register list is set
    if instr.get_reg_list().unwrap() >> 15 & 0x1 == 1 {
        let value = ram_lock.read_word(address);
        registers_lock.set_pc(value & 0xFFFFFFFC);
        address += 4; 
    }

    address
}

fn ldm_str(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, start_address: Word) -> Word {
    let mut address = start_address;

    for ri in 0..15 {
        if (instr.get_reg_list().unwrap() >> ri) & 0x1 == 1 {
            ram_lock.write_word(address, registers_lock.get_register(ri));
            address += 4;
        }
    }

    address
}

// p.187
pub fn instr_ldmstm(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let number_of_set_bits_in_reg_list = instr.get_reg_list().unwrap().count_ones();

    // p.483
    let start_address = match instr.get_ldm().unwrap() {
        LDMCode::DecAfter => rn - (number_of_set_bits_in_reg_list * 4) + 4,
        LDMCode::IncAfter => rn,
        LDMCode::DecBefore => rn - (number_of_set_bits_in_reg_list * 4),
        LDMCode::IncBefore => rn + 4,
    };
    let end_address = match instr.get_ldm().unwrap() {
        LDMCode::DecAfter => rn,
        LDMCode::IncAfter => rn + (number_of_set_bits_in_reg_list * 4) - 4,
        LDMCode::DecBefore => rn - 4,
        LDMCode::IncBefore => rn + (number_of_set_bits_in_reg_list * 4),
    };

    if instr.get_writeback().unwrap() {
        let value = match instr.get_ldm().unwrap() {
            LDMCode::DecAfter => rn - (number_of_set_bits_in_reg_list * 4),
            LDMCode::IncAfter => rn + (number_of_set_bits_in_reg_list * 4),
            LDMCode::DecBefore => rn - (number_of_set_bits_in_reg_list * 4),
            LDMCode::IncBefore => rn + (number_of_set_bits_in_reg_list * 4),
        };
        registers_lock.set_reg_register(instr.get_rn().unwrap(), value);
    }

    let address = match instr.get_ldr_str().unwrap() {
        true => ldm_ldr(ram_lock, registers_lock, instr, start_address),
        false => ldm_str(ram_lock, registers_lock, instr, start_address),
    };

    assert_eq!(end_address, address - 4);

    0
}

pub fn instr_mul(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rs = registers_lock.get_reg_register(instr.get_rs().unwrap());
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());
    registers_lock.set_reg_register(instr.get_rd().unwrap(), (rm * rs) as Word);

    if instr.get_s_bit().unwrap() {
        let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());
        registers_lock.set_n_flag(rd >> 31 != 0);
        registers_lock.set_z_flag(rd == 0);
    }

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}