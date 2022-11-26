use lib::memory::{Word, Memory, SignedWord};
use lib::state::{RegistersState, RAMState};
use log::trace;
use tauri::{AppHandle, Manager};

pub type StackAddress = (Word, Word);

#[derive(Clone, serde::Serialize)]
pub struct StackPayload {
    sp: Word,
	addresses: Vec<StackAddress>
}

pub async fn build_stack_payload(app_handle: AppHandle) -> StackPayload {
    trace!("build_stack_payload: attempting to lock state...");

    let registers_state: RegistersState = app_handle.state();
    let registers_lock = &mut registers_state.lock().await;
    let ram_state: RAMState = app_handle.state();
    let ram_lock = &mut ram_state.lock().await;

    trace!("build_stack_payload: obtained state locks");

    let mut stack_addresses: Vec<StackAddress> = Vec::new();

    let sp = registers_lock.get_sp();

    // get up to 4 addresses after sp (4 * 4bytes)
    let mut current_address: SignedWord = sp.checked_add(4 * 4).unwrap_or(0) as SignedWord;

    // check to ensure address doesn't go past memory array
    if current_address as usize >= ram_lock.get_size() { current_address = (ram_lock.get_size() as SignedWord) - 4; }
    
    // get up to 4 addresses before sp (4 * 4bytes)

    let bottom_address = sp.checked_sub(4 * 4).unwrap_or(0);
    loop {
        trace!("build_stack_payload: {}ca {}ba", current_address, bottom_address);

        let address = current_address as Word;
        let value = ram_lock.read_word(current_address as Word);
        stack_addresses.push((address, value));
        
        current_address = current_address - 4; // word is 4 bytes
        if current_address < bottom_address as SignedWord { break }
    }

    trace!("build_stack_payload: finished");

    StackPayload {
        sp: sp,
        addresses: stack_addresses.clone()
    }
}

#[tauri::command]
pub async fn cmd_get_stack(app_handle: AppHandle) -> Result<StackPayload, ()> {
    trace!("cmd_get_stack: grabbing stack...");
    
    Ok(build_stack_payload(app_handle.clone()).await)
}