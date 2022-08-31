use state::Storage;
use tauri::async_runtime::RwLock;

// https://docs.rs/state/latest/state/#readwrite-singleton
pub static OPTIONS_STATE: Storage<RwLock<Options>> = Storage::new();

pub struct Options {
    pub(crate) memory_size: usize,
    pub(crate) elf_file: String
}

impl Options {
    pub fn Parse(&self) {
        // parse the command line
        // validates the options
        // stores the results
    }
}