use tokio::sync::MutexGuard;

use crate::{memory::{Word, Registers, RAM, Byte, Memory}, instruction::{Instruction, TInstruction}, cpu_enum::{DataOpcode, ShiftType}};

// this method matches all the data operations with their appropriate operation
// the caller is expected to resolve the operand2 ahead of time; this function
// does the minimal amount of computation possible while modifying the app state
fn data_match_opcode(registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, rn: Word, shifter_operand: Word) {
    // temp values so register_lock doesn't have to be moved
    let c_flag = registers_lock.get_c_flag();

    // rd := (result)
    registers_lock.set_reg_register(
        instr.get_rd().unwrap(),
        match instr.get_data_opcode().unwrap() {
            DataOpcode::AND => rn & shifter_operand,
            DataOpcode::EOR => rn ^ shifter_operand,
            DataOpcode::SUB => rn.wrapping_sub(shifter_operand),
            DataOpcode::RSB => shifter_operand.wrapping_sub(rn),
            DataOpcode::ADD => rn.wrapping_add(shifter_operand),
            DataOpcode::ADC => rn.wrapping_add(shifter_operand + (if c_flag { 1 } else { 0 })),
            DataOpcode::SBC => rn.wrapping_sub(shifter_operand - (if !c_flag { 1 } else { 0 })),
            DataOpcode::RSC => shifter_operand.wrapping_sub(rn - (if !c_flag { 1 } else { 0 })),
            DataOpcode::TST => todo!(),
            DataOpcode::TEQ => todo!(),
            DataOpcode::CMP => todo!(),
            DataOpcode::CMN => todo!(),
            DataOpcode::ORR => rn | shifter_operand,
            DataOpcode::MOV => shifter_operand,
            DataOpcode::BIC => rn & !shifter_operand,
            DataOpcode::MVN => !shifter_operand,
    })
}

fn update_flags(registers_lock: &mut MutexGuard<'_, Registers>, op1: Word, op2: Word, result: Word) {
    registers_lock.clear_nzcv();

    if (result >> 30) == 1 {
        registers_lock.set_n_flag(true);
    }
    if result == 0 {
        registers_lock.set_z_flag(true);
    }
    // TODO: Carry flag
    if (op1 >> 30) == (op2 >> 30) && (op1 >> 30) != (result >> 30) && (op2 >> 30) != (result >> 30) {
        registers_lock.set_v_flag(true);
    }
}

// each method accesses/modifies app state as necessary;
// however, this renders the code difficult to test with the current Tauri APIs
// since the Tauri State API cannot be mocked (yet)

// each of these methods are mapped to the Instruction::execute method
// and called from CPU::execute

pub fn instr_data_reg_imm(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    // shift_value rm by imm_shift
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
    // shift_value rm by imm_shift
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

    // TODO: update flags (move to CPU::execute?)

    // return result
    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

// p.463
pub fn instr_ldrstr_shifted_reg(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());
    let index = Instruction::shift_value(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        instr.get_imm_shift().unwrap(),
        instr.get_shift_type().unwrap());
    let address = match instr.get_add_sub().unwrap() {
        true  /* add */ => rn + index,
        false /* sub */ => rn - index
    };

    match instr.get_ldr_str().unwrap() {
        true  /* ldr */ => {
            // TODO: what is CP15_reg_Ubit in manual?
            let data = ram_lock.read_word(address);
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data);
        },
        false /* str */ => {
            ram_lock.write_word(address, rd);
        }
    }

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_ldrstr_imm(ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    let rn = registers_lock.get_reg_register(instr.get_rn().unwrap());
    let rd = registers_lock.get_reg_register(instr.get_rd().unwrap());
    let offset_12 = instr.get_imm_shift().unwrap();
    let address = match instr.get_add_sub().unwrap() {
        true  /* add */ => rn + offset_12,
        false /* sub */ => rn - offset_12
    };

    match instr.get_ldr_str().unwrap() {
        true  /* ldr */ => {
            let data = ram_lock.read_word(address);
            registers_lock.set_reg_register(instr.get_rd().unwrap(), data);
        },
        false /* str */ => {
            ram_lock.write_word(address, rd);
        }
    }

    registers_lock.get_reg_register(instr.get_rd().unwrap())
}