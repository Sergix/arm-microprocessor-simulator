use crate::memory::{self};

use log::trace;
use log::error;
use object::BigEndian;
use object::Endianness;
use object::read::elf::FileHeader;
use object::read::elf::ProgramHeader;
use object::read::elf::{ ElfFile, ElfFile32 };
use object::elf;
use tauri::{AppHandle, Manager};
use std::{fs::{File}, io::Read};

#[tauri::command]
pub async fn cmd_get_memory(memory_state: memory::MemoryState<'_>) -> Result<memory::MemoryPayload, memory::MemoryPayload> {
    trace!("cmd_get_memory: checking if ELF has been loaded...");
    
    let memory_lock = memory_state.lock().await;
    
    if memory_lock.loaded {
        trace!("cmd_get_memory: ELF has already been loaded. Passing to frontend...");
        return Ok(memory::MemoryPayload {
            checksum: memory_lock.checksum,
            loaded: true,
            memory_array: memory_lock.memory_array.clone(),
            error: "".into()
        })
    } else {
        trace!("cmd_get_memory: ELF has not been loaded.");
        return Err(memory::MemoryPayload::default())
    }
}

#[tauri::command]
pub async fn cmd_load_elf(filename: String, app_handle: AppHandle, memory_state: memory::MemoryState<'_>) -> Result<(), ()> {
    trace!("cmd_load_elf: attempting to load ELF binary: {}", filename);
    // load elf file, await
    // automatically emits
    load_elf(filename.clone(), app_handle).await;

    // verify checksums
    let memory_lock = memory_state.lock().await;
    let checksum = memory_lock.CalculateChecksum();
    trace!("cmd_load_elf: checksum: {}", checksum);

    Ok(())
}

pub async fn load_elf(filename: String, app_handle: AppHandle) {
    // get state from app handler
    // https://discord.com/channels/616186924390023171/1012276284430229576/1012403646295707738
    // https://github.com/tauri-apps/tauri/discussions/1336#discussioncomment-1936523

    trace!("load_elf: opening {}...", filename);
    
    let state: memory::MemoryState = app_handle.state();
    let mut memory_lock = state.lock().await;
    let memory_size: usize = memory_lock.size;

    // clear memory
    memory_lock.memory_array.clear();
    memory_lock.memory_array.resize(memory_size, 0);

    // open and read file
    // let mut f = File::open().unwrap();
    let bin_data_result = std::fs::read(filename);
    match bin_data_result {
        Ok(bin_data) => {
            let mut error: String = "".into();

            // load elf file
            let elf_object = elf::FileHeader32::<Endianness>::parse(&*bin_data).unwrap();
            let endianness = elf_object.endian().unwrap();

            // loop over number program header segments (e_phnum)
            for segment in elf_object.program_headers(endianness, &*bin_data).unwrap() {
                // get size of segment (p_memsz)
                let memsz = segment.p_memsz(endianness);

                // load into specified address in RAM (p_paddr)
                let paddr = segment.p_paddr(endianness);

                trace!("load_elf: {}memsz {}paddr", memsz, paddr);

                // write segment data to memory starting at paddr
                let segment_data = segment.data(endianness, &*bin_data).unwrap();
                for i in 0..(memsz - 1) {
                    memory_lock.WriteByte(paddr + i, segment_data[i as usize] as u8);
                }
            }
        
            memory_lock.checksum = memory_lock.CalculateChecksum();
            memory_lock.endianness = endianness;
            memory_lock.loaded = true;

            app_handle.emit_all("elf_load", memory::MemoryPayload {
                checksum: memory_lock.checksum,
                loaded: memory_lock.loaded,
                memory_array: memory_lock.memory_array.clone(),
                error: error.clone()
            }).unwrap();
        }
        Err(e) => {
            error!("load_elf: error loading ELF: {}", e);
        }
    }
}