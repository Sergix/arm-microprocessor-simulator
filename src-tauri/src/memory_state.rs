/*
    memory_state.rs
    State type for global Memory and Register state managed by Tauri
*/

use lib::memory;
use tauri::{ State, async_runtime::Mutex };

pub(crate) type RAMState<'a> = State<'a, Mutex<memory::RAM>>;
pub(crate) type RegistersState<'a> = State<'a, Mutex<memory::Registers>>;