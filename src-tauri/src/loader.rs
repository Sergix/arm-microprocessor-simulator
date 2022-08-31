use crate::ram::RAM;

use log::info;
use std::{fs::File, sync::Mutex};

pub fn calculate_checksum(mem: &[i32]) -> i32 {
    let mut checksum: i32 = 0;

    for address in 0..mem.len() {
        checksum += mem[address] ^ (address as i32);
    }

    return checksum;
}

pub fn load_elf(filename: String, /* app */) {
    // get state from app handler
    // https://discord.com/channels/616186924390023171/1012276284430229576/1012403646295707738
    // https://github.com/tauri-apps/tauri/discussions/1336#discussioncomment-1936523

    info!("Opening {}...", filename);

    // info!("{:?}", RAM_STATE.memory_array.get(1));

    // open file
    let mut f = File::open(filename).unwrap();
    
    // read bytes into memory

    // read sections
    
}