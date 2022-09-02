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
    options::OPTIONS_STATE.set(RwLock::new(options::Options::default()));

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            // setup state used by app handler here since Builder::setup runs before calls to Builder::manage

            // of type memory::MemoryState
            app.manage(Mutex::new(memory::Memory::default()));

            match app.get_cli_matches() {
                Ok(matches) => {
                    trace!("{:?}", matches);

                    let mut opts = options::Options::default();
                    opts.Parse(matches);
                    options::OPTIONS_STATE.set(RwLock::new(opts));

                }
                Err(e) => {
                    trace!("{}", e)
                }
            }

            // DEBUG

            let mut mem_test: memory::Memory = memory::Memory::default();
            mem_test.memory_array[0] = 25;
            mem_test.memory_array[1] = 0xFC;
            mem_test.memory_array[2] = 0xAD;
            mem_test.memory_array[3] = 0x1E;
            mem_test.memory_array[4] = 0xFC;

            // trace!("{}", mem_test.ReadWord(0));
            // trace!("{}", mem_test.TestFlag(0, 3));
            // trace!("{}", mem_test.TestFlag(0, 5));
            // trace!("{}", mem_test.TestFlag(1, 3));
            // trace!("{}", mem_test.TestFlag(1, 11));

            // END DEBUG

            // https://doc.rust-lang.org/std/sync/struct.RwLock.html
            let opts = options::OPTIONS_STATE.get().blocking_read();
            
            // create RAM using memsize
            let handle = app.app_handle();

            let memory_state: memory::MemoryState = handle.state();
            let mut memory_lock = memory_state.blocking_lock();

            memory_lock.size = opts.memory_size;
            memory_lock.memory_array.resize(opts.memory_size, 0);
            
            trace!("OPTIONS: {}bytes, {}", opts.memory_size, opts.elf_file);
            trace!("RAM Details: {}bytes, {}actual", memory_lock.size, memory_lock.memory_array.len());

            // free lock
            drop(memory_lock);

            // if a cmd-line argument file was passed
            if !opts.elf_file.is_empty() {
                spawn(async move {
                     loader::load_elf(String::clone(&opts.elf_file), handle).await;
                });
            }

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
