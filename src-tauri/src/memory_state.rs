/*
    memory_state.rs
    State type for global Memory state managed by Tauri
*/

use lib::memory;
use tauri::{ State, async_runtime::Mutex };

pub(crate) type MemoryState<'a> = State<'a, Mutex<memory::Memory>>;