use lib::memory::{RegistersPayload};
use lib::state::RegistersState;
use log::trace;

#[tauri::command]
pub async fn cmd_get_registers(registers_state: RegistersState<'_>) -> Result<RegistersPayload, ()> {
    trace!("cmd_get_registers: grabbing register r0..r15...");
    
    let mut registers_lock = registers_state.lock().await;
    
    Ok(RegistersPayload {
        register_array: registers_lock.get_all()
    })
}