use lib::{state::{CPUState}, memory::AddressSize};
use log::trace;
use tauri::{async_runtime::spawn, AppHandle, Manager};

#[tauri::command]
pub async fn cmd_run(app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_run: running CPU...");

    // TODO: save handle and join thread when user clicks "stop"
    // need to spawn thread here to stop on-demand later
    let handle = spawn(async move {
        let cpu_state: CPUState = app_handle.state();
        let cpu_lock = cpu_state.lock().await;

        cpu_lock.run(app_handle.clone()).await;
    });

    // TODO: emit to listeners (register_update, ram_update) for once cpu has finished running or hit breakpoint

    Ok(())
}

#[tauri::command]
pub async fn cmd_step(app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_step: stepping into CPU...");

    let cpu_state: CPUState = app_handle.state();
    let cpu_lock = cpu_state.lock().await;

    cpu_lock.step(app_handle.clone()).await;
    
    Ok(())
}

#[tauri::command]
pub async fn cmd_stop(app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_stop: stopping CPU...");

    Ok(())
}

#[tauri::command]
pub async fn cmd_add_breakpoint(address: AddressSize, cpu_state: CPUState<'_>) -> Result<(), ()> {
    trace!("cmd_add_breakpoint: running CPU...");

    let cpu_lock = &mut cpu_state.lock().await;
    cpu_lock.add_breakpoint(address);

    Ok(())
}