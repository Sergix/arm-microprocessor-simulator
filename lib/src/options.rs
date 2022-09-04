use crate::memory;
use tauri::{api::cli::Matches};
use log::{trace, error};

pub struct Options {
    pub memory_size: usize,
    pub elf_file: String
}

impl Options {
    pub fn parse(&mut self, matches: Matches) {
        // matches { args, subcommand }
        // args HashMap<String, ArgData>, ArgData { value, occurances }
        
        self.memory_size = match matches.args.get("mem") {
            Some(arg) => {
                // automatically passes a falsy value if option not present, so skip
                if arg.occurrences == 0 {
                    memory::DEFAULT_MEMORY_SIZE
                } else {
                    let arg_value_normalized = String::from(arg.value.to_string().trim_matches(&['"', '\''] as &[_]));

                    match arg_value_normalized.parse::<usize>() {
                        Ok(u) => {
                            if u > 1024000 {
                                error!("parse: --mem must be <= 1MB (1024000b)");
                                std::process::exit(1)
                            }

                            trace!("parse: mem {}", u);
                            u as usize
                        }
                        Err(_) => {
                            error!("parse: --mem option value incompatible {}", arg.value);
                            std::process::exit(1)
                        }
                    }
                }
            }
            None => {
                memory::DEFAULT_MEMORY_SIZE
            }
        };

        self.elf_file = String::from(match matches.args.get("elf-file") {
            Some(arg) => {
                // skip over non-string values in case a falsy value is passed
                if arg.occurrences == 0 {
                    error!("parse: <elf_file> missing");

                    // don't automatically exit if debugging
                    if cfg!(debug_assertions) {
                        String::from("")
                    } else {
                        std::process::exit(1)
                    }
                } else {
                    trace!("parse: elf_file {}", arg.value.to_string());

                    // remove chars possibly passed by shell
                    // https://stackoverflow.com/a/49856591
                    String::from(arg.value.to_string().trim_matches(&['"', '\'', ' '] as &[_]))
                }
            }
            None => {
                error!("parse: <elf_file> missing");
                std::process::exit(1)
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