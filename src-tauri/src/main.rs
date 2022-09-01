#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod loader;
mod memory;
mod options;

use log::trace;
use tauri::{async_runtime::{RwLock, Mutex, spawn}, Manager};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};

fn main() {
    // logging interface setup
    // logs to stdout and WebView console simultaneously when called from frontend
    let targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];
    let colors = ColoredLevelConfig::default();

    // global state defaults
    options::OPTIONS_STATE.set(RwLock::new(options::Options { memory_size: memory::DEFAULT_MEMORY_SIZE, elf_file: String::new() }));

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            // setup state used by app handler here since Builder::setup runs before calls to Builder::manage

            // of type memory::MemoryState
            app.manage(Mutex::new(memory::Memory::default()));

            match app.get_cli_matches() {
                // `matches` here is a Struct with { args, subcommand }.
                // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurances }
                Ok(matches) => {
                    trace!("{:?}", matches);

                    // parse args
                    let opts = options::Options {
                        memory_size: match matches.args.get("mem") {
                            Some(arg) => {
                                match arg.value.as_u64() {
                                    Some(u) => {
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
                        },
                        elf_file: String::from(match matches.args.get("elf-file") {
                            Some(arg) => {
                                arg.value.to_string()
                            }
                            None => {
                                String::new()
                            }
                        })
                    };

                    options::OPTIONS_STATE.set(RwLock::new(opts));

                }
                Err(e) => {
                    trace!("{}", e)
                }
            }

            // https://doc.rust-lang.org/std/sync/struct.RwLock.html
            let opts = options::OPTIONS_STATE.get().blocking_read();
            
            // create RAM using memsize
            let handle = app.app_handle();

            let memory_state: memory::MemoryState = handle.state();
            let mut memory_lock = memory_state.blocking_lock();
            memory_lock.size = opts.memory_size;

            let memory_rows: usize = opts.memory_size / 16 as usize;
            memory_lock.memory_array.resize(memory_rows, [0; 16]);
            
            trace!("OPTIONS: {}bytes, {}", opts.memory_size, opts.elf_file);
            trace!("RAM Details: {}bytes, {}", memory_lock.size, memory_lock.memory_array.len());

            // free lock
            drop(memory_lock);

            // if a cmd-line argument file was passed
            // if !opts.elf_file.is_empty() {
            //     spawn(async move {
            //          loader::load_elf(String::clone(&opts.elf_file), handle).await;
            //     });

            //     // TODO: compute checksum
            //     // memory.CalculateChecksum();
            // }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            loader::cmd_load_elf,
            loader::cmd_get_memory
        ])
        .plugin(
            LoggerBuilder::new()
              .with_colors(colors)
              .targets(targets)
              .build(),
          )
        .run(context)
        .expect("error while running tauri application");

}
