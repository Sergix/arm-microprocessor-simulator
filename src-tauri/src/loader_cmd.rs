/*
    loader.rs
    ELF loader that interacts with frontend
*/

use lib::state::{ RAMState, RegistersState, OptionsState };
use lib::memory::{ Memory, Word, Register };
use lib::elf::{ ELFPayload }; 
use log::trace;
use log::error;
use normpath::PathExt;
use object::Endianness;
use std::path:: Path;
use tauri::{AppHandle, Manager};

use crate::interface_cmd;

#[tauri::command]
pub async fn cmd_get_elf(memory_state: RAMState<'_>, options_state: OptionsState<'_>) -> Result<ELFPayload, ELFPayload> {
    trace!("cmd_get_memory: checking if ELF has been loaded...");
    
    let memory_lock = memory_state.lock().await;
    let options_lock = options_state.lock().await;
    
    if memory_lock.loaded {
        trace!("cmd_get_memory: ELF has already been loaded. passing to frontend...");
        return Ok(ELFPayload {
            loaded: true,
            error: "".into(),
            filename: String::clone(&options_lock.elf_file)
        })
    } else {
        trace!("cmd_get_memory: ELF has not been loaded.");
        return Err(ELFPayload::default())
    }
}

#[tauri::command]
pub async fn cmd_load_elf(filename: String, app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_load_elf: attempting to load ELF binary: {}", filename);

    load_elf(filename.clone(), app_handle).await;

    Ok(())
}

pub async fn load_elf(filename: String, app_handle: AppHandle) {
    let error: String = "".into();
    let ram_state: RAMState = app_handle.state();
    let registers_state: RegistersState = app_handle.state();

    // clear memory and immediately drop locks
    (ram_state.lock().await).clear();
    (registers_state.lock().await).clear();
        
    // resolve path
    // https://crates.io/crates/normpath
    let path = Path::new(&filename);
    let path_absolute = match path.normalize() {
        Ok(p) => { p }
        Err(e) => {
            error!("load_elf: could not normalize path: {}", e);
            return
        }
    };
        
    let path_str = path_absolute.as_path().to_string_lossy().to_string();
    trace!("load_elf: opening {}...", path_str);
        
    // open and read file
    let elf_result = lib::elf::read_elf_file(path_absolute, app_handle.clone()).await;
    let elf_data: (Word, Endianness);
    match elf_result {
        Ok(res) => { elf_data = res; }
        Err(e) => {
            error!("load_elf: error loading ELF: {}", e.to_string());
            (ram_state.lock().await).loaded = false;
            app_handle.emit_all("invalid_elf", {}).unwrap();
            return ()
        }
    }

    // update state, drop lock immediately
    {
        let ram_lock = &mut ram_state.lock().await;
        let registers_lock = &mut registers_state.lock().await;
        ram_lock.checksum = ram_lock.calculate_checksum();
        ram_lock.loaded = true;
        ram_lock.endianness = elf_data.1;
        registers_lock.set_pc(elf_data.0);
        registers_lock.set_reg_register(Register::r13, 0x7000);

        // notify the frontend that an ELF binary is successfully loaded
        app_handle.emit_all("elf_load", ELFPayload {
            loaded: ram_lock.loaded,
            error: error.clone(),
            filename: String::clone(&path_str)
        }).unwrap();
    }

    interface_cmd::emit_payloads(app_handle.clone()).await;
}