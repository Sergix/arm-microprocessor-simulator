// use tauri::async_runtime::RwLock;
use tauri::{ State, async_runtime::Mutex };
use ts_rs::TS;

pub(crate) type Byte = u8;
type HalfWord = u16;
type Word = u32;
type AddressSize = u32;
pub(crate) type MemoryState<'a> = State<'a, Mutex<Memory>>;

pub(crate) const DEFAULT_MEMORY_SIZE: usize = 32768;

// payload for tauri event emitter to send to frontend
// https://tauri.app/v1/guides/features/events/#global-events-1
#[derive(Clone, serde::Serialize, TS)]
#[ts(export_to = "../src/types/MemoryPayload.ts")]
pub struct MemoryPayload {
    pub(crate) loaded: bool,
    pub(crate) memory_array: Vec<[Byte; 16]>
}

pub struct Memory {
    pub(crate) size: usize,
    pub(crate) loaded: bool, // this is included in the case that the frontend was loaded after the elf loader tried to emit an event
    pub(crate) memory_array: Vec<[Byte; 16]> // unsigned Byte array
}

impl Memory {
    pub fn ReadWord(&self, address: AddressSize) -> Word {
        0
    }

    pub fn WriteWord(&self, address: AddressSize, value: Word) {

    }

    pub fn ReadHalfWord(&self, address: AddressSize) -> HalfWord {
        0
    }

    pub fn WriteHalfWord(&self, address: AddressSize, value: HalfWord) {

    }

    pub fn ReadByte(&self, address: AddressSize) -> Byte {
        0
    }

    pub fn WriteByte(&self, address: AddressSize, value: Byte) {

    }

    pub fn CalculateChecksum(&self) {
        
    }

    pub fn TestFlag(&self, address: AddressSize, bit: u8) {
        // bit is in the range of [0..31]
        // ReadWord(address)
    }

    pub fn SetFlag(&self, address: AddressSize, bit: u8, flag: bool) {
        // bit is in the range of [0..31]
        // ReadWord(address)
    }

    pub fn ExtractBits() -> Word {
        // static utility
        0
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            size: 0,
            loaded: false,
            memory_array: vec![[0; 16]]
        }
    }
}