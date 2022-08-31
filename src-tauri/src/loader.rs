use crate::memory::{self, Memory};

use log::trace;
use tauri::{AppHandle, Manager};
use std::{fs::File, io::Read};

pub fn calculate_checksum(mem: &[i32]) -> i32 {
    let mut checksum: i32 = 0;

    for address in 0..mem.len() {
        checksum += mem[address] ^ (address as i32);
    }

    return checksum;
}

#[tauri::command]
pub async fn cmd_get_memory(app_handle: AppHandle, memory_state: memory::MemoryState<'_>) -> Result<memory::MemoryPayload, memory::MemoryPayload> {
    trace!("Loader: checeking if ELF has been loaded...");
    
    let memory_lock = memory_state.lock().await;
    
    if memory_lock.loaded {
        trace!("Loader: ELF has already been loaded. Passing to frontend...");
        return Ok(memory::MemoryPayload {
            loaded: true,
            memory_array: memory_lock.memory_array.clone()
        })
    } else {
        trace!("Loader: ELF has not been loaded.");
        return Err(memory::MemoryPayload {
            loaded: false,
            memory_array: vec![0; 0]
        })
    }
}

#[tauri::command]
pub async fn cmd_load_elf(filename: &str, app_handle: AppHandle, memory: memory::MemoryState<'_>) -> Result<(), ()> {
    // load elf file, await
    // when completed,
    // if successful
        // send event back to frontend with loaded payload and checksum
    // if error
        // send event back with error payload

    // verify checksums
    trace!("Attempting to load ELF binary: {}", filename);
    let result = calculate_checksum(&[0x01, 0x82, 0x03, 0x84]);
    trace!("Checksum: {}", result);

    Ok(())
}

pub async fn load_elf(filename: String, app_handle: AppHandle) {
    // get state from app handler
    // https://discord.com/channels/616186924390023171/1012276284430229576/1012403646295707738
    // https://github.com/tauri-apps/tauri/discussions/1336#discussioncomment-1936523

    trace!("Loader: Opening {}...", filename);
    
    let state: memory::MemoryState = app_handle.state();
    let mut memory_lock = state.lock().await;

    // open file
    let mut f = File::open(filename).unwrap();
    match f.read(&mut memory_lock.memory_array) {
        Ok(_) => {
            // TODO: check if all bytes from file were read (parameter is # of bytes read)
            trace!("Loader: Loaded ELF");

            memory_lock.loaded = true;
            app_handle.emit_all("elf_load", memory::MemoryPayload { loaded: memory_lock.loaded, memory_array: memory_lock.memory_array.clone() }).unwrap();
        }
        Err(e) => {
            trace!("Loader: Error loading ELF: {}", e);
        }
    }
}