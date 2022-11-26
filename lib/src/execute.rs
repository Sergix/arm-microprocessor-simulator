use log::trace;
use tokio::sync::MutexGuard;

use crate::{memory::{Word, Registers, RAM, Byte, Memory, Register, HalfWord, KEYBOARD_ADDR, DISPLAY_ADDR, SignedWord}, instruction::{Instruction, TInstruction}, cpu_enum::{DataOpcode, LDMCode, InstrExecuteCondition::{NOP, HLT, SWI, self}, LSH}, util};

// this method matches all the data operations with their appropriate operation
// the caller is expected to resolve the operand2 ahead of time; this function
// does the minimal amount of computation possible while modifying the app state
fn data_match_opcode(registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, rn: Word, shifter_operand: Word, shifter_carry_out: Word) {
    // temp values so register_lock doesn't have to be moved
    let c_flag = registers_lock.get_c_flag();

    let rd: Option<Word> = match instr.get_data_opcode().unwrap() {
        DataOpcode::AND => {
            let rd = rn & shifter_operand;

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(util::word_lsb_to_bool(shifter_carry_out));
            }
            Some(rd)
        },
        DataOpcode::EOR => {
            let rd = rn ^ shifter_operand;

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(util::word_lsb_to_bool(shifter_carry_out));
            }
            Some(rd)
        },
        DataOpcode::SUB => {
            let (rd, overflow) = rn.overflowing_sub(shifter_operand);
            
            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(!(shifter_operand > rn)); // borrows if shifter_operand > rn
                registers_lock.set_z_flag(overflow);
            }
            Some(rd)
        },
        DataOpcode::RSB => {
            let (rd, overflow) = shifter_operand.overflowing_sub(rn);

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(!(rn > shifter_operand));
                registers_lock.set_v_flag(overflow);
            }
            Some(rd)
        },
        DataOpcode::ADD => {
            let (rd, overflow) = rn.overflowing_add(shifter_operand);

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(shifter_operand > rn);
                registers_lock.set_v_flag(overflow);
            }
            Some(rd)
        },
        DataOpcode::ADC => {
            let (rd, overflow) = rn.overflowing_add(shifter_operand + if c_flag { 1 } else { 0 });

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(rn > shifter_operand + if c_flag { 1 } else { 0 });
                registers_lock.set_v_flag(overflow);
            }
            Some(rd)
        },
        DataOpcode::SBC => {
            let (rd, overflow) = rn.overflowing_sub(shifter_operand - (if !c_flag { 1 } else { 0 }));

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(!((shifter_operand + if !c_flag { 1 } else { 0 }) > rn));
                registers_lock.set_v_flag(overflow);
            }
            Some(rd)
        },
        DataOpcode::RSC => {
            let (rd, overflow) = shifter_operand.overflowing_sub(rn - (if !c_flag { 1 } else { 0 }));

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(!((rn + if !c_flag { 1 } else { 0 })) > shifter_operand);
                registers_lock.set_v_flag(overflow);
            }
            Some(rd)
        },
        DataOpcode::ORR => {
            let rd = rn | shifter_operand;

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(util::word_lsb_to_bool(shifter_carry_out));
            }
            Some(rd)
        },
        DataOpcode::MOV => {
            let rd = shifter_operand;
            
            if instr.get_s_bit().unwrap() && instr.get_rd().unwrap() == Register::r15 {
                if registers_lock.current_mode_has_spsr() {
                    let spsr: Word = registers_lock.get_spsr();
                    registers_lock.set_cpsr(spsr);
                }
            } else if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(util::word_lsb_to_bool(shifter_carry_out));
            }
            Some(rd)
        },
        DataOpcode::BIC => {
            let rd = rn & !shifter_operand;

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(util::word_lsb_to_bool(shifter_carry_out));
            }
            Some(rd)
        },
        DataOpcode::MVN => {
            let rd = !shifter_operand;

            if instr.get_s_bit().unwrap() {
                registers_lock.set_c_flag(util::word_lsb_to_bool(shifter_carry_out));
            }
            Some(rd)
        },
        _ => None
    };

    if rd.is_some() {
        if instr.get_s_bit().unwrap() {
            registers_lock.set_n_flag(util::test_bit(rd.unwrap(), 31));
            registers_lock.set_z_flag(rd.unwrap() == 0);
        }
        registers_lock.set_reg_register(instr.get_rd().unwrap(), rd.unwrap());
    } else {
        match instr.get_data_opcode().unwrap() {
            DataOpcode::TST => {
                let alu_out = rn & shifter_operand;
                registers_lock.set_n_flag(util::test_bit(alu_out, 31));
                registers_lock.set_z_flag(alu_out == 0);
                registers_lock.set_c_flag(util::word_lsb_to_bool(shifter_carry_out));
            },
            DataOpcode::TEQ => {
                let alu_out = rn ^ shifter_operand;
                registers_lock.set_n_flag(util::test_bit(alu_out, 31));
                registers_lock.set_z_flag(alu_out == 0);
                registers_lock.set_c_flag(util::word_lsb_to_bool(shifter_carry_out));
            },
            DataOpcode::CMP => {
                // needs to be signed so that overflow is properly calculated
                let (alu_out, overflow) = (rn as SignedWord).overflowing_sub(shifter_operand as SignedWord);
                registers_lock.set_n_flag(util::test_bit(alu_out as Word, 31));
                registers_lock.set_z_flag(alu_out == 0);
                registers_lock.set_c_flag(!(shifter_operand > rn)); // borrows if shifter_operand > rn
                registers_lock.set_v_flag(overflow);
            },
            DataOpcode::CMN => {
                let (alu_out, overflow) = (rn as SignedWord).overflowing_add(shifter_operand as SignedWord);
                registers_lock.set_n_flag(util::test_bit(alu_out as Word, 31));
                registers_lock.set_z_flag(alu_out == 0);
                registers_lock.set_c_flag(shifter_operand > rn);
                registers_lock.set_v_flag(overflow);
            },
            _ => ()
        }
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
            let data: Word = match address {
                // map to keyboard hardware event if needed
                // cpu injects character into instruction
                KEYBOARD_ADDR => instr.get_last_char().unwrap() as Word,
                _ => {
                    match instr.get_byte_word().unwrap() {
                        true  => ram_lock.read_byte(address) as Word,
                        false => ram_lock.read_word(address)
                    }
                }
            };
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data);
        },
        false /* str */ => {
            match address {
                // if hardware display event, CPU will handle it
                DISPLAY_ADDR => (),
                _ => match instr.get_byte_word().unwrap() {
                    true  => ram_lock.write_byte(address, rd as Byte),
                    false => ram_lock.write_word(address, rd)
                }
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
            let data: Word = match address {
                // map to keyboard hardware event if needed
                // cpu injects character into instruction
                KEYBOARD_ADDR => instr.get_last_char().unwrap() as Word,
                _ => {
                    match instr.get_byte_word().unwrap() {
                        true  => ram_lock.read_byte(address) as Word,
                        false => ram_lock.read_word(address)
                    }
                }
            };
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data);
        },
        false /* str */ => {
            match address {
                // if hardware display event, CPU will handle it
                DISPLAY_ADDR => (),
                _ => match instr.get_byte_word().unwrap() {
                    true  => ram_lock.write_byte(address, rd as Byte),
                    false => ram_lock.write_word(address, rd)
                }
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

fn ldrh_strh_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, offset: Word) {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());
    
    let address = match instr.get_add_sub().unwrap() {
        true  /* add */ => rn + offset,
        false /* sub */ => rn - offset
    };
    
    match instr.get_ldr_str().unwrap() {
        true  /* ldr */ => {
            let data: Word = match address {
                // map to keyboard hardware event if needed
                // cpu injects character into instruction
                KEYBOARD_ADDR => instr.get_last_char().unwrap() as Word,
                _ => {
                    match instr.get_lsh().unwrap() {
                        LSH::LdrUHalfWord => {
                            ram_lock.read_half_word(address) as Word
                        },
                        LSH::LdrSByte => {
                            (ram_lock.read_byte(address) as i8) as Word
                        },
                        LSH::LdrSHalfWord => {
                            (ram_lock.read_half_word(address) as i16) as Word
                        },
                        _ => panic!("{}", instr.get_lsh().unwrap().to_string())
                    }
                }
            };
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data);
        },
        false /* str */ => {
            match address {
                // if hardware display event, CPU will handle it
                DISPLAY_ADDR => (),
                _ => {
                    match instr.get_lsh().unwrap() {
                        LSH::StrHalfWord => ram_lock.write_half_word(address, rd as HalfWord),
                        LSH::LdrDoubleWord => (), // NOT IMPLEMENTED
                        LSH::StrDoubleWord => (), // NOT IMPLEMENTED
                        _ => panic!("{}", instr.get_lsh().unwrap().to_string())
                    };
                }
            };
        }
    }
    
    if instr.get_writeback().unwrap() {
        registers_lock.set_reg_register(instr.get_rn().unwrap(), address);
    }
}

fn ldrh_strh_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, offset: Word) {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());

    let address = rn;

    match instr.get_ldr_str().unwrap() {
        true  /* ldr */ => {
            let data = match address {
                // map to keyboard hardware event if needed
                // cpu injects character into instruction
                KEYBOARD_ADDR => instr.get_last_char().unwrap() as Word,
                _ => {
                    match instr.get_lsh().unwrap() {
                        LSH::LdrUHalfWord => {
                            ram_lock.read_half_word(address) as Word
                        },
                        LSH::LdrSByte => {
                            (ram_lock.read_byte(address) as i8) as Word
                        },
                        LSH::LdrSHalfWord => {
                            (ram_lock.read_half_word(address) as i16) as Word
                        },
                        _ => panic!("")
                    }
                }
            };
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data);
        },
        false /* str */ => {
            match address {
                // if hardware display event, CPU will handle it
                DISPLAY_ADDR => (),
                _ => {
                    match instr.get_lsh().unwrap() {
                        LSH::StrHalfWord => ram_lock.write_half_word(address, rd as HalfWord),
                        LSH::LdrDoubleWord => (), // NOT IMPLEMENTED
                        LSH::StrDoubleWord => (), // NOT IMPLEMENTED
                        _ => ()
                    };
                }
            }
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

fn ldm(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, start_address: Word) -> Word {
    let mut address = start_address;
    
    for ri in 0..=14 {
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

fn stm(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, start_address: Word) -> Word {
    let mut address = start_address;

    for ri in 0..=15 {
        if (instr.get_reg_list().unwrap() >> ri) & 0x1 == 1 {
            ram_lock.write_word(address, registers_lock.get_register(ri));
            address += 4;
        }
    }

    address
}

// each method accesses/modifies app state as necessary;
// however, this renders the code difficult to test with the current Tauri APIs
// since the Tauri State API cannot be mocked (yet)

// each of these methods are mapped to the Instruction::execute method
// and called from CPU::execute

pub fn instr_data_reg_imm(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let (shifter_operand, shifter_carry_out) = Instruction::shift_value_by_imm(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        instr.get_imm_shift().unwrap(),
        instr.get_shift_type().unwrap(),
        registers_lock.get_c_flag() as Byte);

    data_match_opcode(registers_lock, instr, rn, shifter_operand, shifter_carry_out);

    NOP
}

pub fn instr_data_reg_reg(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let (shifter_operand, shifter_carry_out) = Instruction::shift_value_by_reg(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        registers_lock.get_reg_register(instr.get_rs().unwrap()), // grab the LSB
        instr.get_shift_type().unwrap(),
        registers_lock.get_c_flag() as Byte);

    data_match_opcode(registers_lock, instr, rn, shifter_operand, shifter_carry_out);

    NOP
}

pub fn instr_data_imm(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let rn =  registers_lock.get_reg_register(instr.get_rn().unwrap());
    let (shifter_operand, shifter_carry_out) = Instruction::rotate_value(
        instr.get_rotate().unwrap(),
        instr.get_imm().unwrap(),
        registers_lock.get_c_flag() as Byte);

    data_match_opcode(registers_lock, instr, rn, shifter_operand, shifter_carry_out);

    NOP
}

// p.463
pub fn instr_ldrstr_shifted_reg_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let index = Instruction::shift_value_by_imm(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        instr.get_imm_shift().unwrap(),
        instr.get_shift_type().unwrap(),
        registers_lock.get_c_flag() as Byte).0;

    ldr_str_pre(ram_lock, registers_lock, instr, index);

    NOP
}

pub fn instr_ldrstr_shifted_reg_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let index = Instruction::shift_value_by_imm(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        instr.get_imm_shift().unwrap(),
        instr.get_shift_type().unwrap(),
        registers_lock.get_c_flag() as Byte).0;

    ldr_str_post(ram_lock, registers_lock, instr, index);

    NOP
}

pub fn instr_ldrstr_reg_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());
    ldr_str_post(ram_lock, registers_lock, instr, rm);
    
    NOP
}

pub fn instr_ldrstr_reg_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());
    ldr_str_post(ram_lock, registers_lock, instr, rm);

    NOP
}

pub fn instr_ldrstr_imm_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let offset_12 = instr.get_imm_shift().unwrap();

    ldr_str_pre(ram_lock, registers_lock, instr, offset_12);

    NOP
}

pub fn instr_ldrstr_imm_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let offset_12 = instr.get_imm_shift().unwrap();

    ldr_str_post(ram_lock, registers_lock, instr, offset_12);

    NOP
}

pub fn instr_ldrhstrh_imm_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let offset_8 = instr.get_imm().unwrap() as Word;

    ldrh_strh_pre(ram_lock, registers_lock, instr, offset_8);

    NOP
}

pub fn instr_ldrhstrh_imm_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let offset_8 = instr.get_imm().unwrap() as Word;

    ldrh_strh_post(ram_lock, registers_lock, instr, offset_8);

    NOP
}

pub fn instr_ldrhstrh_reg_pre(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());

    ldrh_strh_pre(ram_lock, registers_lock, instr, rm);

    NOP
}

pub fn instr_ldrhstrh_reg_post(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());

    ldrh_strh_post(ram_lock, registers_lock, instr, rm);

    NOP
}

pub fn instr_b(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    if instr.get_l_bit().unwrap() {
        let address_after_branch = registers_lock.get_pc_current_address() + 4;
        registers_lock.set_reg_register(Register::r14, address_after_branch);
    }

    // increment by an extra 4 bytes because:
    // 1. CPU fetch
    // 2. CPU decode
    // 3. CPU execute branch instruction
    //   - go to target address
    // 4. CPU increments PC by 4
    // -> CPU only increments 1 instruction ahead
    // => need to increment by extra 4 bytes here to ensure CPU PC is two instructions ahead
    let target_address: Word = (instr.get_pc_address() as SignedWord + instr.get_offset().unwrap() + 4) as Word;
    trace!("instr_b: {}pc {}offset {}target", instr.get_pc_address(), instr.get_offset().unwrap(), target_address);
    registers_lock.set_pc(target_address);

    NOP
}

pub fn instr_bx(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    // TODO: why + 4?
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap()) + 4;

    registers_lock.set_t_flag(util::word_lsb_to_bool(rm));
    registers_lock.set_pc(rm & 0xFFFFFFFE);

    NOP
}

// p.187
pub fn instr_ldmstm(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let number_of_set_bits_in_reg_list = instr.get_reg_list().unwrap().count_ones();

    // A5.4.2
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
        true => ldm(ram_lock, registers_lock, instr, start_address),
        false => stm(ram_lock, registers_lock, instr, start_address),
    };

    assert_eq!(end_address, address - 4);

    NOP
}

pub fn instr_mul(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let rs = registers_lock.get_reg_register(instr.get_rs().unwrap());
    let rm = registers_lock.get_reg_register(instr.get_rm().unwrap());

    registers_lock.set_reg_register(instr.get_rd().unwrap(), (rm as u64 * rs as u64) as Word);

    if instr.get_s_bit().unwrap() {
        let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());
        registers_lock.set_n_flag(rd >> 31 != 0);
        registers_lock.set_z_flag(rd == 0);
    }

    NOP
}

// p.58, 360
pub fn instr_swi(_ram_lock: &mut MutexGuard<'_, RAM>, _registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    // actual SWI processing is done post-execute by processor in CPU::step

    if instr.get_swi().unwrap() == 0x11 {
        return HLT
    // these conditions need to be handled by the outer CPU::step context to access the app thread
    } else if instr.get_swi().unwrap() == 0x0 || instr.get_swi().unwrap() == 0x6a {
        return SWI
    }

    NOP
}

pub fn instr_mrs(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    if instr.get_gpregister().unwrap() {
        let spsr = registers_lock.get_spsr();
        registers_lock.set_reg_register(instr.get_rd().unwrap(), spsr);
    } else {
        let cpsr = registers_lock.get_cpsr();
        registers_lock.set_reg_register(instr.get_rd().unwrap(), cpsr);
    }

    NOP
}

fn instr_msr(registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, operand: Word) {
    // const UNALLOC_MASK: Word = 0x06F0FC00;
    const USER_MASK: Word    = 0xF80F0200;
    const PRIV_MASK: Word    = 0x000001DF;
    const STATE_MASK: Word   = 0x01000020;

    let cpsr = registers_lock.get_cpsr();
    let spsr = registers_lock.get_spsr();
    let mask ;

    let field_mask = instr.get_field_mask().unwrap() as Word;
    let byte_mask: Word = (if util::test_bit(field_mask, 0) { 0x000000FF } else { 0x0 }) |
                          (if util::test_bit(field_mask, 1) { 0x0000FF00 } else { 0x0 }) |
                          (if util::test_bit(field_mask, 2) { 0x00FF0000 } else { 0x0 }) |
                          (if util::test_bit(field_mask, 3) { 0xFF000000 } else { 0x0 });
    if !instr.get_gpregister().unwrap() {
        // assumed privaleged mode since working with SYS rather than USER mode
        mask = byte_mask & (USER_MASK | PRIV_MASK);
        registers_lock.set_cpsr((cpsr & !mask) | (operand & mask));
    } else {
        if registers_lock.current_mode_has_spsr() {
            mask = byte_mask & (USER_MASK | PRIV_MASK | STATE_MASK);
            registers_lock.set_spsr((spsr & !mask) | (operand & mask));
        }
    }
}

pub fn instr_msr_imm(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let operand = instr.get_imm().unwrap().rotate_right((instr.get_rotate().unwrap() * 2) as Word) as Word;
    instr_msr(registers_lock, instr, operand);
    NOP
}

pub fn instr_msr_reg(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> InstrExecuteCondition {
    let operand = registers_lock.get_reg_register(instr.get_rm().unwrap());
    instr_msr(registers_lock, instr, operand);
    NOP
}

pub fn instr_nop(_ram_lock: &mut MutexGuard<'_, RAM>, _registers_lock: &mut MutexGuard<'_, Registers>, _instr: Instruction) -> InstrExecuteCondition {
    NOP
}