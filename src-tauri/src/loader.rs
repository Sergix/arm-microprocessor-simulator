/*
    loader.rs
    ELF loader that interacts with frontend
*/

use crate::memory_state::{ MemoryState };
use crate::options_state::{ OptionsState };

use lib::memory;
use log::trace;
use log::error;
use normpath::PathExt;
use object::Endianness;
use object::read::elf::FileHeader;
use object::read::elf::ProgramHeader;
use object::elf;
use std::path:: Path;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn cmd_get_memory(memory_state: MemoryState<'_>, options_state: OptionsState<'_>) -> Result<memory::MemoryPayload, memory::MemoryPayload> {
    trace!("cmd_get_memory: checking if ELF has been loaded...");
    
    let memory_lock = memory_state.lock().await;
    let options_lock = options_state.lock().await;
    
    if memory_lock.loaded {
        trace!("cmd_get_memory: ELF has already been loaded. Passing to frontend...");
        return Ok(memory::MemoryPayload {
            checksum: memory_lock.checksum,
            loaded: true,
            memory_array: memory_lock.memory_array.clone(),
            error: "".into(),
            filename: String::clone(&options_lock.elf_file)
        })
    } else {
        trace!("cmd_get_memory: ELF has not been loaded.");
        return Err(memory::MemoryPayload::default())
    }
}

#[tauri::command]
pub async fn cmd_load_elf(filename: String, app_handle: AppHandle) -> Result<(), ()> {
    trace!("cmd_load_elf: attempting to load ELF binary: {}", filename);
    
    // load elf file, await
    // automatically emits
    load_elf(filename.clone(), app_handle).await;

    Ok(())
}

pub async fn load_elf(filename: String, app_handle: AppHandle) {
    let error: String = "".into();
    
    // get global state from app handler
    // https://discord.com/channels/616186924390023171/1012276284430229576/1012403646295707738
    // https://github.com/tauri-apps/tauri/discussions/1336#discussioncomment-1936523
    let app_memory_state: MemoryState = app_handle.state();
    let mut memory_lock = app_memory_state.lock().await;
    let memory_size: usize = memory_lock.size;

    let app_options_state: OptionsState = app_handle.state();
    let mut options_lock = app_options_state.lock().await;

    // clear memory
    memory_lock.memory_array.clear();
    memory_lock.memory_array.resize(memory_size, 0);

    // resolve path
    // https://crates.io/crates/normpath
    let path = Path::new(&filename);
    let path_absolute = match path.normalize() {
        Ok(p) => { p }
        Err(e) => {
            error!("load_elf: could not normalize path: {}", e);
            return
        }
    };
    
    // open and read file
    // TODO: dont update global options; this is only here so that the frontend is properly updated from cmd_get_memory payload
    let path_str = path_absolute.as_path().to_string_lossy().to_string();
    options_lock.elf_file = String::clone(&path_str);

    trace!("load_elf: opening {}...", path_str);
    let bin_data_result = std::fs::read(path_absolute);

    match bin_data_result {
        Ok(bin_data) => {
            // load elf file
            let elf_object = match elf::FileHeader32::<Endianness>::parse(&*bin_data) {
                Ok(header) => { header }
                Err(_) => {
                    // TODO: two occurences of this, refactor to separate function
                    error!("load_elf: invalid ELF header");
                    memory_lock.loaded = false;
                    app_handle.emit_all("invalid_elf", {}).unwrap();
                    return ()
                }
            };

            let endianness = elf_object.endian().unwrap();

            // loop over program header segments (e_phnum)
            trace!("load_elf: {} segments", elf_object.e_phnum(endianness));
            for segment in elf_object.program_headers(endianness, &*bin_data).unwrap() {
                // header offsets
                let offset = segment.p_offset(endianness);

                // get size of segment (p_memsz)
                let memsz = segment.p_memsz(endianness);

                // load into specified address in RAM (p_paddr)
                let paddr = segment.p_paddr(endianness);

                trace!("load_elf: segment {}memsz {}offset {}paddr", memsz, offset, paddr);

                // write segment data to memory starting at paddr
                let segment_data = segment.data(endianness, &*bin_data).unwrap();
                for i in 0..(memsz - 1) {
                    memory_lock.write_byte(paddr + i, segment_data[i as usize] as u8);
                }
            }

            // update state
            memory_lock.checksum = memory_lock.calculate_checksum();
            memory_lock.endianness = endianness;
            memory_lock.loaded = true;
        }
        Err(e) => {
            // TODO: two occurences of this, refactor to separate function
            error!("load_elf: error loading ELF: {}", e);
            memory_lock.loaded = false;
            app_handle.emit_all("invalid_elf", {}).unwrap();
            return ()
        }
    }

    // notify the frontend when an ELF binary is successfully loaded
    app_handle.emit_all("elf_load", memory::MemoryPayload {
        checksum: memory_lock.checksum,
        loaded: memory_lock.loaded,
        memory_array: memory_lock.memory_array.clone(),
        error: error.clone(),
        filename: String::clone(&path_str)
    }).unwrap();
}