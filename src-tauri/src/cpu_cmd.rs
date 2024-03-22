use lib::cpu::CPUPayload;
use lib::state::CPUState;
use lib::state::RegistersState;
use log::trace;

#[tauri::command]
pub async fn cmd_get_cpu(cpu_state: CPUState<'_>, registers_state: RegistersState<'_>) -> Result<CPUPayload, ()> {
    trace!("cmd_get_cpu: cpu state...");
    
    let cpu_lock = cpu_state.lock().await;
    let registers_lock = &mut registers_state.lock().await;
    
    Ok(CPUPayload {
        trace: cpu_lock.get_trace(),
        mode: registers_lock.get_cpsr_mode()
    })
}
