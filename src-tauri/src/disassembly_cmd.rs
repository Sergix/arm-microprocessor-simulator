use lib::instruction::TInstruction;
use lib::memory::{Word, Memory};
use lib::state::{RegistersState, CPUState, RAMState};
use log::trace;
use tauri::{AppHandle, Manager};

pub type DisassemblyInstruction = (bool, Word, Word, String);

#[derive(Clone, serde::Serialize)]
pub struct DisassemblyPayload {
    pc: Word,
	instructions: Vec<DisassemblyInstruction>
}

pub async fn build_disassembly_payload(app_handle: AppHandle) -> DisassemblyPayload {
    trace!("build_dissassembly_payload: attempting to lock state...");

    let cpu_state: CPUState = app_handle.state();
    let cpu_lock = &mut cpu_state.lock().await;
    let registers_state: RegistersState = app_handle.state();
    let registers_lock = &mut registers_state.lock().await;
    let ram_state: RAMState = app_handle.state();
    let ram_lock = &mut ram_state.lock().await;

    trace!("build_dissassembly_payload: obtained state locks");

    let mut disassembly_instructions: Vec<DisassemblyInstruction> = Vec::new();

    // get 7 instructions: 3 before pc + pc + 3 after pc (each instruction is 4 bytes)
    let pc = registers_lock.get_pc_current_address();
    let mut address = pc.checked_sub(12).unwrap_or(0);
    loop {
        let instr_raw = ram_lock.read_word(address);
        let mut instr = cpu_lock.decode(instr_raw);
        instr.set_pc_address(address + 8);
        let instr_str = instr.to_string();
        let breakpoint_set = cpu_lock.is_breakpoint(&address);
        disassembly_instructions.push((breakpoint_set, address, instr_raw, instr_str));
        
        address += 4; // word is 4 bytes
        if address as usize >= ram_lock.get_size() { break }
        if address > pc + 12 { break }
    }

    trace!("build_dissassembly_payload: finished");

    DisassemblyPayload {
        pc,
        instructions: disassembly_instructions.clone()
    }
}

#[tauri::command]
pub async fn cmd_get_disassembly(app_handle: AppHandle) -> Result<DisassemblyPayload, ()> {
    trace!("cmd_get_disassembly: grabbing disassembly...");
    
    Ok(build_disassembly_payload(app_handle.clone()).await)
}