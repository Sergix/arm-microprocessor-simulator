#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

// use tauri::Manager;
mod loader;
use log::info;
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

    let context = tauri::generate_context!();
    tauri::Builder::default()
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
