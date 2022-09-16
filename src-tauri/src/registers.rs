use lib::memory::{RegistersPayload, Word};
use crate::memory_state::RegistersState;
use log::trace;

#[tauri::command]
pub async fn cmd_get_registers(registers_state: RegistersState<'_>) -> Result<RegistersPayload, ()> {
    trace!("cmd_get_registers: grabbing register r0..r15...");
    
    let mut registers_lock = registers_state.lock().await;
    let mut regs: Vec<Word> = vec![0; 16];
    for i in 0..15 {
        regs[i] = registers_lock.get_as_word(i);
    }
    
    Ok(RegistersPayload {
        register_array: regs
    })
}