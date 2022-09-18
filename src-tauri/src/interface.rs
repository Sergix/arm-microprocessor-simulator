use lib::{state::{CPUState, CPUThreadWatcherState, RegistersState, RAMState}, memory::{AddressSize, RegistersPayload, RAMPayload, FlagsPayload, Word, Memory}};
use log::trace;
use tauri::{AppHandle, Manager};

type DisassemblyInstruction = (bool, Word, Word, String);

#[derive(Clone, serde::Serialize)]
struct DisassemblyPayload {
    pc: Word,
	instructions: Vec<DisassemblyInstruction>
}

pub async fn emit_disassembly_payload (app_handle: AppHandle) {
    // TODO: remove in later phases
    const MOCKED_ASM: [&str; 14] = [
        "push	{fp, lr}",
        "add	fp, sp, #4",
        "sub	sp, sp, #8",
        "mov	r3, #10",
        "str	r3, [fp, #-8]",
        "mov	r3, #0",
        "str	r3, [fp, #-12]",
        "bne	174 <mystart+0x3c>",
        "ldr	r2, [fp, #-12]",
        "ldr	r3, [fp, #-8]",
        "add	r3, r2, r3",
        "str	r3, [fp, #-12]",
        "ldr	r3, [fp, #-8]",
        "sub	r3, r3, #1"
    ];

    trace!("emit_dissassembly_payload: attempting to lock state...");

    let cpu_state: CPUState = app_handle.state();
    let cpu_lock = &mut cpu_state.lock().await;
    let registers_state: RegistersState = app_handle.state();
    let registers_lock = &mut registers_state.lock().await;
    let ram_state: RAMState = app_handle.state();
    let ram_lock = &mut ram_state.lock().await;

    trace!("emit_dissassembly_payload: obtained state locks");

    let mut disassembly_instructions: Vec<DisassemblyInstruction> = Vec::new();

    // get 7 instructions: 3 before pc + pc + 3 after pc (each instruction is 4 bytes)
    let pc = registers_lock.get_pc();
    let mut address = pc - 12;
    loop {
        let r_str = MOCKED_ASM.get(((pc + address) as usize) % MOCKED_ASM.len()).unwrap().to_string();
        let breakpoint_set = cpu_lock.is_breakpoint(&address);
        disassembly_instructions.push((breakpoint_set, address, ram_lock.read_word(address), r_str));
        address += 4; // word is 4 bytes
        if address > pc + 12 { break }
    }
    app_handle.emit_all("disassembly_update", DisassemblyPayload {
        pc,
        instructions: disassembly_instructions
    }).unwrap();
}

pub async fn emit_payloads (app_handle: AppHandle) {
    
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
        app_handle.emit_all("ram_update", RAMPayload {
            checksum: ram_lock.checksum,
            memory_array: ram_lock.memory_array.clone()
        }).unwrap();
        app_handle.emit_all("flags_update", FlagsPayload {
            n: registers_lock.get_n_flag(),
            z: registers_lock.get_z_flag(),
            c: registers_lock.get_c_flag(),
            v: registers_lock.get_v_flag()
        }).unwrap();
    }

    emit_disassembly_payload(app_handle.clone()).await;
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
    crate::loader::load_elf(filename, app_handle.clone()).await;
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
    emit_disassembly_payload(app_handle.clone()).await;

    Ok(())
}