#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod loader;
mod ram;
mod options;

use log::info;
use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};
// use clap::Parser;

/// https://docs.rs/clap/latest/clap/_derive/
// #[derive(Parser)]
// #[clap(author, version, about, long_about = None)]
// struct Args {
//     /// The pattern to look for
//     #[clap(long, value_parser, help = "a number specifying the number of bytes in the simulated RAM")]
//     mem: Option<usize>,

//     /// The path to the file to read
//     #[clap(required = true, last = true, parse(from_os_str), help = "the name of a file in ELF format")]
//     elf_file: std::path::PathBuf,
// }

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

    // read command line args
    // let args = Args::parse();

    // default memory size
    let mut memsize: usize = 32768;

    // command line arguments options
    let mut opts: options::Options;

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .setup(|app| {
            match app.get_cli_matches() {
            // `matches` here is a Struct with { args, subcommand }.
            // `args` is `HashMap<String, ArgData>` where `ArgData` is a struct with { value, occurances }.
            // `subcommand` is `Option<Box<SubcommandMatches>>` where `SubcommandMatches` is a struct with { name, matches }.
            Ok(matches) => {
                println!("{:?}", matches);

                // TODO: create Options class with matches to process
                opts = options::Options {
                    memory_size: memsize,
                    elf_file: String::from("Hello")
                }
            }
            Err(_) => {}
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

    // TODO: check Options class
    // if --mem, set memsize to that value
    // if elf_file,
        // TODO: move below to separate function
        // create RAM using memsize
        let mut memory: ram::RAM = ram::RAM {
            size: memsize,
            memory_array: vec![0; memsize]
        };

        // TODO: load the ELF file into RAM
        loader::load_elf(opts.elf_file, &memory);

        // TODO: display contents to frontend

        // TODO: compute checksum
        memory.CalculateChecksum();

}
