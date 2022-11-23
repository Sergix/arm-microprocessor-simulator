/*
    main.rs
    Entry point for Tauri application
*/

#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod loader_cmd;
mod registers_cmd;
mod flags_cmd;
mod interface_cmd;
mod memory_cmd;
mod disassembly_cmd;
mod cpu_cmd;
mod terminal_cmd;

use lib::memory;
use lib::memory::Byte;
use lib::options;
use lib::cpu;
use lib::state::CPUState;
use lib::state::OptionsState;
use lib::state::RAMState;
use lib::trace;
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
            app.manage(Mutex::new(memory::Registers::default()));
            app.manage(Mutex::new(memory::RAM::default()));
            app.manage(Mutex::new(options::Options::default()));
            app.manage(Mutex::new(cpu::CPU::default()));
            app.manage(Mutex::new(cpu::CPUThreadWatcher::default()));
            app.manage(Mutex::new(trace::TraceFile::default()));

            match app.get_cli_matches() {
                Ok(matches) => {
                    trace!("{:?}", matches);

                    let handle = app.app_handle();
                    let opts: OptionsState = handle.state();
                    let mut opts_lock = opts.blocking_lock();
                    opts_lock.parse(matches);
                    drop(opts_lock);

                }
                Err(e) => {
                    trace!("{}", e)
                }
            }
            
            let handle = app.app_handle();
            let elf_file;
            
            // drop locks immediately
            {
                let opts: OptionsState = handle.state();
                let opts_lock = opts.blocking_lock();

                let cpu: CPUState = handle.state();
                let mut cpu_lock = cpu.blocking_lock();

                let memory: RAMState = handle.state();
                let mut memory_lock = memory.blocking_lock();

                // enable CPU step tracing if --exec is provided and an elf-file is provided
                if opts_lock.exec && !opts_lock.elf_file.is_empty() { cpu_lock.toggle_trace(); }
                
                // create RAM using memsize
                memory_lock.size = opts_lock.memory_size;
                memory_lock.memory_array.resize(opts_lock.memory_size, 0);
                
                // debug information
                trace!("OPTIONS: {}bytes, {}", opts_lock.memory_size, opts_lock.elf_file);
                trace!("RAM Details: {}bytes, {}actual", memory_lock.size, memory_lock.memory_array.len());

                // copy here to pass to loader after locks are freed
                elf_file = String::clone(&opts_lock.elf_file);
            }
            
            // if a cmd-line argument file was passed
            if !elf_file.is_empty() {
                spawn(async move {
                     loader_cmd::load_elf(String::clone(&elf_file), handle).await;
                });
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            terminal_cmd::cmd_terminal_input,
            cpu_cmd::cmd_get_cpu,
            loader_cmd::cmd_get_elf,
            loader_cmd::cmd_load_elf,
            registers_cmd::cmd_get_registers,
            flags_cmd::cmd_get_flags,
            memory_cmd::cmd_get_ram,
            memory_cmd::cmd_set_offset,
            disassembly_cmd::cmd_get_disassembly,
            interface_cmd::cmd_run,
            interface_cmd::cmd_step,
            interface_cmd::cmd_stop,
            interface_cmd::cmd_reset,
            interface_cmd::cmd_add_breakpoint,
            interface_cmd::cmd_remove_breakpoint,
            interface_cmd::cmd_toggle_breakpoint,
            interface_cmd::cmd_toggle_trace
        ])
        .plugin(
            LoggerBuilder::new()
              .with_colors(colors)
              .targets(targets)
              .build(),
          )
        .register_uri_scheme_protocol("ram", |app_handle, _req| {
            let ram_state: RAMState = app_handle.state();

            let ram_buf: Vec<Byte>;
            // release lock immediately
            { 
                ram_buf = ram_state.blocking_lock().memory_array.clone();
            }
            
            // IPC socket for updating RAM when frontend requests
            // from example: https://github.com/JonasKruckenberg/pisano/blob/1fd0e722f1df70874aa0268690213fa7b37f5e66/src-tauri/src/main.rs
            tauri::http::ResponseBuilder::new()
                .header("Origin", "*")
                .mimetype("application/octet-stream")
                .header("Content-Length", ram_buf.len())
                .status(200)
                .body(ram_buf)
        })
        .run(context)
        .expect("error while running tauri application");

}
