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
mod stack_cmd;

use lib::memory;
use lib::options;
use lib::cpu;
use lib::state::CPUState;
use lib::state::OptionsState;
use lib::state::RAMState;
use lib::state::TraceFileState;
use lib::trace;
use log::trace;
use tauri::{async_runtime::{Mutex, spawn}, Manager};
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};

fn main() {
    // logging interface setup
    // logs to stdout and WebView console simultaneously when called from frontend
    #[cfg(debug_assertions)]
    let log_targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];

    // disable other logs to fix thread errors and speed up the runtime in release mode
    #[cfg(not(debug_assertions))]
    let log_targets = [LogTarget::LogDir];
    
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
            let opts_elf_file;
            
            // drop locks immediately
            {
                let opts: OptionsState = handle.state();
                let cpu: CPUState = handle.state();
                let memory: RAMState = handle.state();
                let trace: TraceFileState = handle.state();
                
                let opts_lock = opts.blocking_lock();
                let mut cpu_lock = cpu.blocking_lock();
                let mut memory_lock = memory.blocking_lock();
                let mut trace_lock = trace.blocking_lock();
                
                // copy here to pass to loader after locks are freed
                opts_elf_file = opts_lock.elf_file.clone().unwrap_or("".to_string());

                // enable CPU step tracing if --exec is provided and an elf-file is provided
                if opts_lock.exec && opts_lock.elf_file.is_some() { cpu_lock.toggle_trace(); }

                // enable traceall if option enabled
                if opts_lock.traceall { trace_lock.set_traceall(); }
                
                // create RAM using memsize
                let opts_memsize = match opts_lock.memory_size {
                    Some(size) => size,
                    None => memory::DEFAULT_MEMORY_SIZE,
                };
                memory_lock.size = opts_memsize;
                memory_lock.memory_array.resize(opts_memsize, 0);
                
                // debug information
                trace!("OPTIONS: {}bytes, {}", opts_memsize, opts_elf_file);
                trace!("RAM Details: {}bytes, {}actual", opts_memsize, memory_lock.memory_array.len());

            }
            
            // if a cmd-line argument file was passed
            if !opts_elf_file.is_empty() {
                spawn(async move {
                     loader_cmd::load_elf(opts_elf_file.clone(), handle).await;
                });
            }
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            stack_cmd::cmd_get_stack,
            terminal_cmd::cmd_terminal_input_interrupt,
            terminal_cmd::cmd_terminal_prompt_input,
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
              .targets(log_targets)
              .build(),
          )
        .run(context)
        .expect("error while running tauri application");

}
