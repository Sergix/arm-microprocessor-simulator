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

pub async fn build_disassembly_payload (app_handle: AppHandle) -> DisassemblyPayload {
    trace!("emit_dissassembly_payload: attempting to lock state...");

    let cpu_state: CPUState = app_handle.state();
    let cpu_lock = &mut cpu_state.lock().await;
    let registers_state: RegistersState = app_handle.state();
    let registers_lock = &mut registers_state.lock().await;
    let ram_state: RAMState = app_handle.state();
    let ram_lock = &mut ram_state.lock().await;

    trace!("emit_dissassembly_payload: obtained state locks");

    let mut disassembly_instructions: Vec<DisassemblyInstruction> = Vec::new();

    // get 7 instructions: 3 before pc + pc + 3 after pc (each instruction is 4 bytes)
    let pc = registers_lock.get_pc();
    let mut address = pc - 12;
    loop {
        let instr_raw = ram_lock.read_word(address);
        let instr_str = cpu_lock.decode(instr_raw).to_string();
        let breakpoint_set = cpu_lock.is_breakpoint(&address);
        disassembly_instructions.push((breakpoint_set, address, instr_raw, instr_str));
        
        address += 4; // word is 4 bytes
        if address > pc + 12 { break }
    }

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