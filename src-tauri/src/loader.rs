use crate::ram::RAM;
use log::info;

pub fn calculate_checksum(mem: &[i32]) -> i32 {
    let mut checksum: i32 = 0;

    for address in 0..mem.len() {
        checksum += mem[address] ^ (address as i32);
    }

    return checksum;
}

pub fn load_elf(filename: String, memory: &RAM) {
    info!("Opening {}...", filename);

    // open file

    // read sections

    // load into memory
    
}