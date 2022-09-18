use lib::{state::{CPUState, CPUThreadWatcherState, RegistersState, RAMState}, memory::{AddressSize, RegistersPayload, RAMPayload, FlagsPayload }};
use log::{trace};
use tauri::{AppHandle, Manager};
use crate::{memory_cmd::chunk_memory, disassembly_cmd::build_disassembly_payload};

pub async fn emit_payloads (app_handle: AppHandle) {

    app_handle.emit_all("disassembly_update", build_disassembly_payload(app_handle.clone()).await).unwrap();

    // scoped block to ensure locks are dropped
    {
        trace!("emit_payloads: attempting to lock state...");

        let registers_state: RegistersState = app_handle.state();
        let registers_lock = &mut registers_state.lock().await;
        let ram_state: RAMState = app_handle.state();
        let ram_lock = &mut ram_state.lock().await;

        trace!("emit_payloads: obtained state locks");

        app_handle.emit_all("registers_update", RegistersPayload {
            register_array: registers_lock.get_all()
        }).unwrap();

        app_handle.emit_all("flags_update", FlagsPayload {
            n: registers_lock.get_n_flag(),
            z: registers_lock.get_z_flag(),
            c: registers_lock.get_c_flag(),
            v: registers_lock.get_v_flag()
        }).unwrap();

        // emit last since it's the most expensive
        app_handle.emit_all("ram_update", RAMPayload {
            checksum: ram_lock.checksum,
            memory_array: chunk_memory(ram_lock.memory_array.clone(), 0)
        }).unwrap();
    }
}

#[tauri::command]
pub async fn cmd_run(app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_run: running CPU...");

    let cpu_state: CPUState = app_handle.state();
    (&mut cpu_state.lock().await).run(app_handle.clone()).await;

    trace!("cmd_run: sending payload to frontend...");
    emit_payloads(app_handle.clone()).await;

    Ok(())
}

#[tauri::command]
pub async fn cmd_step(app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_step: stepping into CPU...");

    let cpu_state: CPUState = app_handle.state();
    (&mut cpu_state.lock().await).step(app_handle.clone()).await;

    trace!("cmd_step: CPU step finished, sending payload to frontend...");
    emit_payloads(app_handle.clone()).await;
    
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
pub async fn cmd_reset(filename: String, app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_reset: clearing memory and reloading binary...");
    crate::loader_cmd::load_elf(filename, app_handle.clone()).await;
    emit_payloads(app_handle.clone()).await;
    Ok(())
}

#[tauri::command]
pub async fn cmd_add_breakpoint(address: AddressSize, cpu_state: CPUState<'_>) -> Result<(), ()> {
    trace!("cmd_add_breakpoint: adding breakpoint {}...", address);

    (&mut cpu_state.lock().await).add_breakpoint(address);

    Ok(())
}

#[tauri::command]
pub async fn cmd_remove_breakpoint(address: AddressSize, cpu_state: CPUState<'_>) -> Result<(), ()> {
    trace!("cmd_remove_breakpoint: removing breakpoint {}...", address);

    (&mut cpu_state.lock().await).remove_breakpoint(address);

    Ok(())
}

#[tauri::command]
pub async fn cmd_toggle_breakpoint(address: AddressSize, cpu_state: CPUState<'_>, app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_toggle_breakpoint: running CPU...");

    // scoped block to ensure locks are dropped
    {
        let cpu_lock = &mut cpu_state.lock().await;
        if cpu_lock.is_breakpoint(&address) {
            cpu_lock.remove_breakpoint(address);
        } else {
            cpu_lock.add_breakpoint(address);
        }
    }

    // update disassembly window
    app_handle.emit_all("disassembly_update", build_disassembly_payload(app_handle.clone()).await).unwrap();

    Ok(())
}