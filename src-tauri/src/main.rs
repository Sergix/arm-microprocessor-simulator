#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod loader;
mod ram;
mod options;

use log::info;
use tauri::{async_runtime::{RwLock, Mutex}, Manager, State};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};

#[tauri::command]
fn load_elf(filename: &str) {
    // read into RAM and pass reference back

    // verify checksums
    info!("Attempting to load ELF binary: {}", filename);
    let result = loader::calculate_checksum(&[0x01, 0x82, 0x03, 0x84]);
    info!("Checksum: {}", result);
}

fn main() {
    // logging interface setup
    // logs to stdout and WebView console simultaneously when called from frontend
    let targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];
    let colors = ColoredLevelConfig::default();

    // global state defaults
    options::OPTIONS_STATE.set(RwLock::new(options::Options { memory_size: 32768, elf_file: String::new() }));

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            // setup state used by app handler here since Builder::setup runs before calls to Builder::manage
            app.manage(Mutex::new(ram::Memory::default()));

            match app.get_cli_matches() {
                // `matches` here is a Struct with { args, subcommand }.
                // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurances }
                Ok(matches) => {
                    info!("{:?}", matches);

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
                                        32768
                                    }
                                }
                            }
                            None => {
                                32768 // default memory size
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
                Err(E) => {
                    info!("{}", E)
                }
            }

            // https://doc.rust-lang.org/std/sync/struct.RwLock.html
            let opts = options::OPTIONS_STATE.get().blocking_read();

            
            // create RAM using memsize
            let handle = app.app_handle();
            // let state: State<Mutex<ram::Memory>> = handle.state();
            let state: State<Mutex<ram::Memory>> = handle.state();
            let mut memory = state.blocking_lock();
            // let memory = lock.
            memory.size = opts.memory_size;
            memory.memory_array.resize(opts.memory_size, 0);

            info!("OPTIONS: {}bytes, {}", opts.memory_size, opts.elf_file);
            info!("RAM Details: {}bytes, {}", memory.size, memory.memory_array.len());

            // if a cmd-line argument file was passed
            if !opts.elf_file.is_empty() {
                // load the ELF file into RAM
                loader::load_elf(String::clone(&opts.elf_file));

                // TODO: compute checksum
                // memory.CalculateChecksum();
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![load_elf])
        .plugin(
            LoggerBuilder::new()
              .with_colors(colors)
              .targets(targets)
              .build(),
          )
        .run(context)
        .expect("error while running tauri application");

}
