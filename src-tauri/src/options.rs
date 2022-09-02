use crate::memory;

use state::Storage;
use tauri::{async_runtime::RwLock, api::cli::Matches};

// https://docs.rs/state/latest/state/#readwrite-singleton
pub static OPTIONS_STATE: Storage<RwLock<Options>> = Storage::new();

pub struct Options {
    pub(crate) memory_size: usize,
    pub(crate) elf_file: String
}

impl Options {
    #[allow(non_snake_case)]
    pub fn Parse(&mut self, matches: Matches) {
        // matches { args, subcommand }
        // args HashMap<String, ArgData>, ArgData { value, occurances }
        
        self.memory_size = match matches.args.get("mem") {
            Some(arg) => {
                match arg.value.as_u64() {
                    Some(u) => {
                        // TODO: verify mem-size is <= 1MB
                        u as usize
                    }
                    None => {
                        // panic!("ERROR: --mem option value incompatible")
                        memory::DEFAULT_MEMORY_SIZE
                    }
                }
            }
            None => {
                memory::DEFAULT_MEMORY_SIZE // default memory size
            }
        };

        self.elf_file = String::from(match matches.args.get("elf-file") {
            Some(arg) => {
                arg.value.to_string()
            }
            None => {
                String::new()
            }
        })
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            memory_size: memory::DEFAULT_MEMORY_SIZE,
            elf_file: String::new()
        }
    }
}