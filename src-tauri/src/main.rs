// #![cfg_attr(
//     all(not(debug_assertions), target_os = "windows"),
//     windows_subsystem = "windows"
// )]

mod loader;
mod options_state;
mod memory_state;

use lib::memory;
use lib::options;
use log::trace;
use tauri::{async_runtime::{Mutex, spawn}, Manager};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};

fn main() {
    // logging interface setup
    // logs to stdout and WebView console simultaneously when called from frontend
    let targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];
    let colors = ColoredLevelConfig::default();

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            
            // setup state used by app handler here since Builder::setup runs before calls to Builder::manage
            app.manage(Mutex::new(memory::Memory::default()));
            app.manage(Mutex::new(options::Options::default()));

            match app.get_cli_matches() {
                Ok(matches) => {
                    trace!("{:?}", matches);

                    let handle = app.app_handle();
                    let opts: options_state::OptionsState = handle.state();
                    let mut opts_lock = opts.blocking_lock();
                    opts_lock.Parse(matches);
                    drop(opts_lock);

                }
                Err(e) => {
                    trace!("{}", e)
                }
            }
            
            let handle = app.app_handle();
            
            let opts: options_state::OptionsState = handle.state();
            let opts_lock = opts.blocking_lock();

            let memory: memory_state::MemoryState = handle.state();
            let mut memory_lock = memory.blocking_lock();
            
            // create RAM using memsize
            memory_lock.size = opts_lock.memory_size;
            memory_lock.memory_array.resize(opts_lock.memory_size, 0);
            
            // debug information
            trace!("OPTIONS: {}bytes, {}", opts_lock.memory_size, opts_lock.elf_file);
            trace!("RAM Details: {}bytes, {}actual", memory_lock.size, memory_lock.memory_array.len());

            // copy here to pass to loader after locks are freed
            let elf_file: String = String::clone(&opts_lock.elf_file);

            // free locks
            drop(memory_lock);
            drop(opts_lock);
            
            // if a cmd-line argument file was passed
            if !elf_file.is_empty() {
                spawn(async move {
                     loader::load_elf(String::clone(&elf_file), handle).await;
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
