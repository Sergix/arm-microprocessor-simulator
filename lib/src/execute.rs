use tokio::sync::MutexGuard;

use crate::{memory::{Word, Registers, RAM}, instruction::{Instruction, TInstruction}, cpu::CPU};

// each method accesses/modifies app state as necessary;
// however, this renders the code difficult to test with the current Tauri APIs
// since the Tauri State API cannot be mocked (yet)

pub fn instr_data_imm(_ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, _cpu_lock: &mut MutexGuard<'_, CPU>, instr: Instruction) -> Word {
    // move imm -> rd
    registers_lock.set_reg_register(instr.get_rd().unwrap(), instr.get_imm().unwrap());
    0
}