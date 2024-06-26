use std::io::Error;

use log::trace;
use object::{elf, Endianness, read::elf::{FileHeader, ProgramHeader}};
use tauri::{AppHandle, Manager};
use normpath::{BasePathBuf};

use crate::{state::RAMState, memory::{Word, Memory}};

#[derive(Clone, serde::Serialize)]
pub struct ELFPayload {
    pub loaded: bool,
    pub error: String,
    pub filename: String
}

impl Default for ELFPayload {
    fn default() -> Self {
        ELFPayload {
            loaded: false,
            error: String::from(""),
            filename: String::from("")
        }
    }
}

// Result<(pc, endianness), Error>
pub async fn read_elf_file(path: BasePathBuf, app_handle: AppHandle) -> Result<(Word, Endianness), std::io::Error> {
    let ram_state: RAMState = app_handle.state();

    let bin_data_result = std::fs::read(path);
    let pc: Word;
    let endianness: Endianness;

    match bin_data_result {
        Ok(bin_data) => {
            let elf_object = match elf::FileHeader32::<Endianness>::parse(&*bin_data) {
                Ok(header) => { header }
                Err(e) => { return Err(Error::new(std::io::ErrorKind::InvalidData, e.to_string())) }
            };

            endianness = elf_object.endian().unwrap();
            pc = elf_object.e_entry.get(endianness);

            trace!("read_elf_file: {}e_entry {} segments", pc, elf_object.e_phnum(endianness));

            // loop over program header segments (e_phnum)
            for segment in elf_object.program_headers(endianness, &*bin_data).unwrap() {
                let offset = segment.p_offset(endianness);
                let memsz = segment.p_memsz(endianness);
                let paddr = segment.p_paddr(endianness);
                
                // write segment data to RAM starting at paddr
                let ram_lock = &mut ram_state.lock().await;
                let segment_data = segment.data(endianness, &*bin_data).unwrap();

                trace!("read_elf_file: segment {}memsz {}offset {}paddr {}segsz", memsz, offset, paddr, segment.file_range(endianness).1);

                for i in 0..memsz {
                    // segment data read from elf library may not be the actual size indicated by p_memsz, so fill the rest with 0's if needed
                    if i as usize >= segment_data.len() {
                        ram_lock.write_byte(paddr + i, 0)
                    } else {
                        ram_lock.write_byte(paddr + i, segment_data[i as usize] as u8);
                    }
                }
            }

            Ok((pc, endianness))
        },
        Err(e) => {
            Err(e)
        }
    }
}