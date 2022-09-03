use crate::memory;
use tauri::{api::cli::Matches};
use log::trace;

pub struct Options {
    pub memory_size: usize,
    pub elf_file: String
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
                        trace!("Parse: mem {}", u);
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
                // skip over non-string values in case a falsy value is passed
                if !arg.value.is_string() {
                    return ()
                }

                trace!("Parse: elf_file {}", arg.value.to_string());
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