use lib::memory::FlagsPayload;
use lib::state::RegistersState;
use log::trace;

#[tauri::command]
pub async fn cmd_get_flags(registers_state: RegistersState<'_>) -> Result<FlagsPayload, ()> {
    trace!("cmd_get_flags: grabbing flags...");
    
    let mut registers_lock = registers_state.lock().await;
    
    Ok(FlagsPayload {
        n: registers_lock.get_n_flag(),
        z: registers_lock.get_z_flag(),
        c: registers_lock.get_c_flag(),
        v: registers_lock.get_v_flag(),
    })
}