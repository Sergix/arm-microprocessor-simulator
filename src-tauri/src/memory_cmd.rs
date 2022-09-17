use lib::memory::{RAMPayload, Memory};
use lib::state::RAMState;
use log::trace;

#[tauri::command]
pub async fn cmd_get_ram(ram_state: RAMState<'_>) -> Result<RAMPayload, ()> {
    trace!("cmd_get_ram: grabbing RAM...");
    
    let mut ram_lock = ram_state.lock().await;
    
    Ok(RAMPayload {
        memory_array: ram_lock.get_memory_array().clone()
    })
}