use tokio::sync::MutexGuard;

use crate::{memory::{Word, Registers, RAM, Register, Memory}, instruction::{Instruction, TInstruction}, cpu::CPU, cpu_enum::DataOpcode};

// this method matches all the data operations with their appropriate operation
// the caller is expected to resolve the operand2 ahead of time; this function
// does the minimal amount of computation possible while modifying the app state
fn instr_data_match_opcode(registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction, operand2: Word) {
    match instr.get_data_opcode().unwrap() {
        DataOpcode::AND => todo!(),
        DataOpcode::EOR => todo!(),
        DataOpcode::SUB => {
            let value = registers_lock.get_reg_register(instr.get_rn().unwrap()).wrapping_sub(operand2);
            registers_lock.set_reg_register(instr.get_rd().unwrap(), value)
        },
        DataOpcode::RSB => todo!(),
        DataOpcode::ADD => {
            let value = registers_lock.get_reg_register(instr.get_rn().unwrap()).wrapping_add(operand2);
            registers_lock.set_reg_register(instr.get_rd().unwrap(), value)
        },
        DataOpcode::ADC => todo!(),
        DataOpcode::SBC => todo!(),
        DataOpcode::RSC => todo!(),
        DataOpcode::TST => todo!(),
        DataOpcode::TEQ => todo!(),
        DataOpcode::CMP => todo!(),
        DataOpcode::CMN => todo!(),
        DataOpcode::ORR => todo!(),
        DataOpcode::MOV => registers_lock.set_reg_register(instr.get_rd().unwrap(), operand2),
        DataOpcode::BIC => todo!(),
        DataOpcode::MVN => todo!(),
    }
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
    // shift_value rm by imm_shift
    let operand2 = Instruction::shift_value(
        registers_lock.get_reg_register(instr.get_rm().unwrap()),
        instr.get_imm_shift().unwrap(),
        instr.get_shift_type().unwrap());

    // val -> rd
    instr_data_match_opcode(registers_lock, instr, operand2);

    // return result
    registers_lock.get_reg_register(instr.get_rd().unwrap())
}

pub fn instr_data_imm(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
    // imm -> rd
    instr_data_match_opcode(registers_lock, instr, instr.get_imm().unwrap());

    // return result
    registers_lock.get_reg_register(instr.get_rd().unwrap())
}