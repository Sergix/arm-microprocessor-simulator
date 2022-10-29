use lib::memory::{RAMPayload, AddressSize, Byte, Memory};
use lib::state::RAMState;
use log::{trace, error};
use tauri::{AppHandle, Manager};

const MEMORY_ROW_SIZE: usize = 16;

// here because it's for the UI logic, not in the core logic
pub fn chunk_memory(mut payload_memory_array: Vec<Byte>, mut offset: AddressSize) -> Vec<Vec<Byte>> {
    if offset >= payload_memory_array.len() as u32 {
        error!("chunk_memory: offset must be within bounds; defaulting to 0");
        offset = 0;
    }
    
    trace!("chunk_memory: chunking memory table...");

    let mut memory_array: Vec<Vec<Byte>> = vec![vec![0; 0]; 0];
    
    let payload_memory_array_size = payload_memory_array.len();
    let first_row_size = offset % (MEMORY_ROW_SIZE as u32);
    let full_row_count = (((payload_memory_array_size - first_row_size as usize) / MEMORY_ROW_SIZE) as f32).floor() as usize;
    let last_row_size = payload_memory_array_size - ((MEMORY_ROW_SIZE * full_row_count) + first_row_size as usize);
    
    if first_row_size > 0 {
        memory_array.push(payload_memory_array.splice(0..(first_row_size as usize), []).collect());
    }

    if payload_memory_array_size < MEMORY_ROW_SIZE {
        trace!("chunk_memory: nothing to chunk, exiting early");
        return memory_array
    }

    while payload_memory_array.len() > last_row_size {
        memory_array.push(payload_memory_array.splice(0..MEMORY_ROW_SIZE, []).collect());
    }

    if last_row_size > 0 {
        memory_array.push(payload_memory_array.splice(0..last_row_size, []).collect());
    }

    trace!("chunk: finished chunking");
    return memory_array
}

#[tauri::command]
pub async fn cmd_get_ram(ram_state: RAMState<'_>) -> Result<RAMPayload, ()> {
    trace!("cmd_get_ram: grabbing RAM...");
    
    let ram_lock = ram_state.lock().await;
    
    Ok(RAMPayload {
        checksum: ram_lock.get_checksum(),
        memory_array: chunk_memory(ram_lock.memory_array.clone(), ram_lock.display_offset)
    })
}

#[tauri::command]
pub async fn cmd_set_offset(offset: AddressSize, app_handle: AppHandle) -> Result<(), String> {
    let ram_state: RAMState = app_handle.state();
    let ram_lock = &mut ram_state.lock().await;

    ram_lock.display_offset = offset;

    app_handle.emit_all("ram_update", RAMPayload {
        checksum: ram_lock.checksum,
        memory_array: chunk_memory(ram_lock.memory_array.clone(), ram_lock.display_offset)
    }).unwrap();

    Ok(())
}