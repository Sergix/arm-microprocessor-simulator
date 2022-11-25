use lib::memory::Word;
use lib::state::{CPUState, CPUThreadWatcherState};
use log::trace;

#[tauri::command]
pub async fn cmd_terminal_input_interrupt(last_char: Word, cpu_state: CPUState<'_>) -> Result<(), ()> {
    trace!("cmd_terminal_input_interrupt: user terminal input, setting CPU IRQ flag...");
    
    trace!("cmd_terminal_input_interrupt: attempting to lock state...");
    let cpu_lock = &mut cpu_state.lock().await;
    trace!("cmd_terminal_input_interrupt: obtained state lock");
    cpu_lock.set_irq();
    cpu_lock.set_last_char(((last_char & 0xff) as u8) as char);
    
    Ok(())
}

#[tauri::command]
pub async fn cmd_terminal_prompt_input(prompt_input: String, cpu_thread_watcher_state: CPUThreadWatcherState<'_>) -> Result<(), ()> {
    trace!("cmd_terminal_prompt_input: prompt completed, notifying CPU...");
    
    trace!("cmd_terminal_prompt_input: attempting to lock state...");
    let cpu_thread_watcher_lock = &mut cpu_thread_watcher_state.lock().await;
    trace!("cmd_terminal_prompt_input: obtained state lock");
    cpu_thread_watcher_lock.set_prompt_input(prompt_input.clone());

    Ok(())
}
