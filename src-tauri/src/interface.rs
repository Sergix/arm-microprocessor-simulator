use lib::{state::{CPUState, CPUThreadWatcherState}, memory::AddressSize};
use log::trace;
use tauri::{AppHandle, Manager};

pub fn emit_payloads (app_handle: AppHandle) {
    
}

#[tauri::command]
pub async fn cmd_run(app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_run: running CPU...");

    let cpu_state: CPUState = app_handle.state();
    let cpu_lock = &mut cpu_state.lock().await;
    cpu_lock.run(app_handle.clone()).await;

    // TODO: emit to listeners (register_update, ram_update) for once cpu has finished running or hit breakpoint
    trace!("cmd_run: sending payload to frontend...");
    emit_payloads(app_handle.clone());

    Ok(())
}

#[tauri::command]
pub async fn cmd_step(app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_step: stepping into CPU...");

    let cpu_state: CPUState = app_handle.state();
    let cpu_lock = &mut cpu_state.lock().await;

    cpu_lock.step(app_handle.clone()).await;

    // TODO: emit to listeners
    trace!("cmd_step: CPU step finished, sending payload to frontend...");
    
    Ok(())
}

#[tauri::command]
pub async fn cmd_stop(app_handle: AppHandle) -> Result<bool, ()> {
    trace!("cmd_stop: stopping CPU thread...");
    
    let cpu_thread_state: CPUThreadWatcherState = app_handle.state();
    cpu_thread_state.lock().await.set_running(false);

    Ok(true)
}

#[tauri::command]
pub async fn cmd_add_breakpoint(address: AddressSize, cpu_state: CPUState<'_>) -> Result<(), ()> {
    trace!("cmd_add_breakpoint: running CPU...");

    let cpu_lock = &mut cpu_state.lock().await;
    cpu_lock.add_breakpoint(address);

    Ok(())
}