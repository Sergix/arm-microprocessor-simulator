use lib::cpu::CPUPayload;
use lib::state::CPUState;
use log::trace;

#[tauri::command]
pub async fn cmd_get_trace(cpu_state: CPUState<'_>) -> Result<CPUPayload, ()> {
    trace!("cmd_get_trace: cpu trace state...");
    
    let cpu_lock = cpu_state.lock().await;
    
    Ok(CPUPayload {
        trace: cpu_lock.get_trace()
    })
}