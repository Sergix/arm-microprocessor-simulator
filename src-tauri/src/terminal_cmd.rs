use lib::state::CPUState;
use lib::state::RegistersState;
use log::trace;

#[tauri::command]
pub async fn cmd_terminal_input_interrupt(last_char: char, cpu_state: CPUState<'_>, _registers_state: RegistersState<'_>) -> Result<(), ()> {
    trace!("cmd_terminal_input: user terminal input, setting CPU IRQ flag...");
    
    let cpu_lock = &mut cpu_state.lock().await;
    cpu_lock.set_irq();
    cpu_lock.set_last_char(last_char);
    
    Ok(())
}
