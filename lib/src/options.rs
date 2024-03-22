use tauri::{api::cli::Matches};
use log::{trace, error};

pub struct Options {
    pub memory_size: Option<usize>,
    pub elf_file: Option<String>,
    pub exec: bool,
    pub traceall: bool
}

impl Options {
    pub fn parse(&mut self, matches: Matches) {
        // matches { args, subcommand }
        // args HashMap<String, ArgData>, ArgData { value, occurances }
        
        match matches.args.get("help") {
            Some(arg) => {
                println!("{}", arg.value.to_string());
                std::process::exit(0)
            }
            _ => { }
        }

        for (name, arg) in matches.args {
            // skip empty arguments
            if arg.occurrences == 0 {
                continue
            }
            
            if name == "mem" {
                trace!("parse: mem {}", arg.value.to_string());
                let arg_value_normalized = String::from(arg.value.to_string().trim_matches(&['"', '\''] as &[_]));

                self.memory_size = Some(match arg_value_normalized.parse::<usize>() {
                    Ok(u) => {
                        if u > 1024000 {
                            error!("parse: --mem must be <= 1MB (1024000b)");
                            std::process::exit(1)
                        }

                        trace!("parse: mem {}", u);
                        u as usize
                    }
                    Err(_) => {
                        panic!("parse: --mem option value incompatible {}", arg.value);
                    }
                });
            } else if name == "elf-file" {
                trace!("parse: elf_file {}", arg.value.to_string());
                
                // remove chars possibly passed by shell
                // https://stackoverflow.com/a/49856591
                self.elf_file = Some(String::from(arg.value.to_string().trim_matches(&['"', '\'', ' '] as &[_])));
            } else if name == "exec" {
                trace!("parse: exec {}", arg.value.to_string());

                self.exec = true;
            } else if name == "traceall" {
                trace!("parse: traceall enabled");
                self.traceall = true;
            }
        }
    }
}

impl Default for Options {
    fn default() -> Self {
        Options {
            memory_size: None,
            elf_file: None,
            exec: false,
            traceall: false
        }
    }
}