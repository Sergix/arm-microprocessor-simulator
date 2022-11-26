use lib::memory::{Word, Byte};
use lib::state::{CPUThreadWatcherState};
use log::trace;

#[tauri::command]
pub async fn cmd_terminal_input_interrupt(last_char: Word, cpu_state: CPUThreadWatcherState<'_>) -> Result<(), ()> {
    trace!("cmd_terminal_input_interrupt: user terminal input, setting CPUThreadWatcher IRQ flag...");
    
    trace!("cmd_terminal_input_interrupt: attempting to lock state...");
    let cpu_thread_watcher_lock = &mut cpu_state.lock().await;
    trace!("cmd_terminal_input_interrupt: obtained state lock");
    cpu_thread_watcher_lock.set_irq_flag();
    cpu_thread_watcher_lock.set_irq_last_char(((last_char & 0xff) as Byte) as char);
    
    Ok(())
}

#[tauri::command]
pub async fn cmd_terminal_prompt_input(prompt_input: String, cpu_thread_watcher_state: CPUThreadWatcherState<'_>) -> Result<(), ()> {
    trace!("cmd_terminal_prompt_input: prompt completed, notifying CPU...");
    
    trace!("cmd_terminal_prompt_input: attempting to lock state...");
    (&mut cpu_thread_watcher_state.lock().await).set_prompt_input(prompt_input.clone());
    trace!("cmd_terminal_prompt_input: obtained state lock");

    Ok(())
}
