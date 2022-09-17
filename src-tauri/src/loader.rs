/*
    loader.rs
    ELF loader that interacts with frontend
*/

use lib::state::{ RAMState, RegistersState, OptionsState };
use lib::memory::{self, Memory};
use log::trace;
use log::error;
use normpath::PathExt;
use object::Endianness;
use object::read::elf::FileHeader;
use object::read::elf::ProgramHeader;
use object::elf;
use std::path:: Path;
use tauri::{AppHandle, Manager};

use crate::interface;

#[tauri::command]
pub async fn cmd_get_elf(memory_state: RAMState<'_>, options_state: OptionsState<'_>) -> Result<memory::ELFPayload, memory::ELFPayload> {
    trace!("cmd_get_memory: checking if ELF has been loaded...");
    
    let memory_lock = memory_state.lock().await;
    let options_lock = options_state.lock().await;
    
    if memory_lock.loaded {
        trace!("cmd_get_memory: ELF has already been loaded. passing to frontend...");
        return Ok(memory::ELFPayload {
            checksum: memory_lock.checksum,
            loaded: true,
            error: "".into(),
            filename: String::clone(&options_lock.elf_file)
        })
    } else {
        trace!("cmd_get_memory: ELF has not been loaded.");
        return Err(memory::ELFPayload::default())
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
    // scope block to drop locks immediately
    {
        let app_ram_state: RAMState = app_handle.state();
        let mut ram_lock = app_ram_state.lock().await;

        let app_registers_state: RegistersState = app_handle.state();
        let mut registers_lock = app_registers_state.lock().await;

        // clear memory
        ram_lock.clear();
        registers_lock.clear();
        
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
        let path_str = path_absolute.as_path().to_string_lossy().to_string();

        trace!("load_elf: opening {}...", path_str);
        let bin_data_result = std::fs::read(path_absolute);

        match bin_data_result {
            Ok(bin_data) => {
                // load elf file
                // TODO: refactor to separate function

                let elf_object = match elf::FileHeader32::<Endianness>::parse(&*bin_data) {
                    Ok(header) => { header }
                    Err(_) => {
                        // TODO: two occurences of this, refactor to separate function
                        error!("load_elf: invalid ELF header");
                        ram_lock.loaded = false;
                        app_handle.emit_all("invalid_elf", {}).unwrap();
                        return ()
                    }
                };

                let endianness = elf_object.endian().unwrap();

                // get the entry point and load it into r15 (program counter)
                let e_entry = elf_object.e_entry.get(endianness);
                registers_lock.set_pc(e_entry);
                trace!("load_elf: {}e_entry", e_entry);

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
                        ram_lock.write_byte(paddr + i, segment_data[i as usize] as u8);
                    }
                }

                // update state
                ram_lock.checksum = ram_lock.calculate_checksum();
                ram_lock.endianness = endianness;
                ram_lock.loaded = true;
            }
            Err(e) => {
                // TODO: two occurences of this, refactor to separate function
                error!("load_elf: error loading ELF: {}", e);
                ram_lock.loaded = false;
                app_handle.emit_all("invalid_elf", {}).unwrap();
                return ()
            }
        }

        // notify the frontend when an ELF binary is successfully loaded
        app_handle.emit_all("elf_load", memory::ELFPayload {
            checksum: ram_lock.checksum,
            loaded: ram_lock.loaded,
            error: error.clone(),
            filename: String::clone(&path_str)
        }).unwrap();
    }

    // TODO: move to crate::interface with global elf state object?
    interface::emit_payloads(app_handle.clone()).await;
}