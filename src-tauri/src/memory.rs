use log::trace;
use log::error;
use object::Endian;
use object::Endianness;
use tauri::{ State, async_runtime::Mutex };
use ts_rs::TS;

pub(crate) type Byte = u8;
type HalfWord = u16;
type Word = u32;
type AddressSize = u32;
type Checksum = u32;
pub(crate) type MemoryState<'a> = State<'a, Mutex<Memory>>;

pub(crate) const DEFAULT_MEMORY_SIZE: usize = 32768;

// payload for tauri event emitter to send to frontend
// https://tauri.app/v1/guides/features/events/#global-events-1
#[derive(Clone, serde::Serialize, TS)]
#[ts(export_to = "../src/types/MemoryPayload.ts")]
pub struct MemoryPayload {
    pub(crate) checksum: Checksum,
    pub(crate) loaded: bool,
    pub(crate) memory_array: Vec<Byte>,
    pub(crate) error: String
}

impl Default for MemoryPayload {
    fn default() -> Self {
        MemoryPayload {
            checksum: 0,
            loaded: false,
            memory_array: vec![0, 0],
            error: String::from("")
        }
    }
}

pub struct Memory {
    pub(crate) checksum: Checksum,
    pub(crate) endianness: Endianness,
    pub(crate) loaded: bool, // this is included in the case that the frontend was loaded after the elf loader tried to emit an event
    pub(crate) memory_array: Vec<Byte>, // unsigned Byte array
    pub(crate) size: usize
}

impl Memory {
    // TODO: big- and little- endian
    // have flag in each method that is passed from global tauri state

    #[allow(non_snake_case)]
    pub fn ReadWord(&self, addr: AddressSize) -> Word {
        // TODO: make sure doesnt read past end

        if addr % 4 != 0 {
            error!("Memory[ReadWord]: Word address not valid");
            return 0
        }

        let w0: Word = *self.memory_array.get(addr as usize).unwrap() as Word;
        let w1: Word = *self.memory_array.get((addr + 1) as usize).unwrap() as Word;
        let w2: Word = *self.memory_array.get((addr + 2) as usize).unwrap() as Word;
        let w3: Word = *self.memory_array.get((addr + 3) as usize).unwrap() as Word;

        if self.endianness == Endianness::Little {
            (w3 << 24) | (w2 << 16) | (w1 << 8) | w0
        } else {
            (w0 << 24) | (w1 << 16) | (w2 << 8) | w3
        }
    }

    #[allow(non_snake_case)]
    pub fn WriteWord(&mut self, addr: AddressSize, value: Word) {
        // TODO: make sure doesnt write past end

        if addr % 4 != 0 {
            error!("Memory[WriteWord]: Word address not valid");
            return
        }

        let b0: Byte = ((value >> 24) & 0xff) as Byte;
        let b1: Byte = ((value >> 16) & 0xff) as Byte;
        let b2: Byte = ((value >> 8) & 0xff) as Byte;
        let b3: Byte = (value & 0xff) as Byte;

        if self.endianness == Endianness::Little {
            self.memory_array[addr as usize] = b3;
            self.memory_array[(addr + 1) as usize] = b2;
            self.memory_array[(addr + 2) as usize] = b1;
            self.memory_array[(addr + 3) as usize] = b0;
        } else {
            self.memory_array[addr as usize] = b0;
            self.memory_array[(addr + 1) as usize] = b1;
            self.memory_array[(addr + 2) as usize] = b2;
            self.memory_array[(addr + 3) as usize] = b3;
        }
    }

    #[allow(non_snake_case)]
    pub fn ReadHalfWord(&self, addr: AddressSize) -> HalfWord {
        // TODO: make sure doesnt read past end

        if addr % 2 != 0 {
            error!("Memory[WriteWord]: Word address not valid");
            return 0
        }

        let hw0: HalfWord = *self.memory_array.get(addr as usize).unwrap() as HalfWord;
        let hw1: HalfWord = *self.memory_array.get((addr + 1) as usize).unwrap() as HalfWord;

        if self.endianness == Endianness::Little {
            (hw1 << 8) | hw0
        } else {
            (hw0 << 8) | hw1
        }
    }

    #[allow(non_snake_case)]
    pub fn WriteHalfWord(&mut self, addr: AddressSize, value: HalfWord) {
        // TODO: make sure doesnt write past end

        if addr % 2 != 0 {
            error!("Memory[WriteWord]: Word address not valid");
            return
        }

        // example:
        //  0x74 EC
        //    b0 b1
        let b0: Byte = ((value >> 8) & 0xff) as Byte;
        let b1: Byte = (value & 0xff) as Byte;
        
        // big endian: b0 b1
        // little endian: b1 b0
        if self.endianness == Endianness::Little {
            self.memory_array[addr as usize] = b1;
            self.memory_array[(addr + 1) as usize] = b0;
        } else {
            self.memory_array[addr as usize] = b0;
            self.memory_array[(addr + 1) as usize] = b1;
        }
    }

    #[allow(non_snake_case)]
    pub fn ReadByte(&self, addr: AddressSize) -> Byte {
        *self.memory_array.get(addr as usize).unwrap() as Byte
    }

    #[allow(non_snake_case)]
    pub fn WriteByte(&mut self, addr: AddressSize, value: Byte) {
        self.memory_array[addr as usize] = value;
    }

    #[allow(non_snake_case)]
    pub fn CalculateChecksum(&self) -> Checksum {
        let mut checksum: u32 = 0;
    
        for address in 0..self.memory_array.len() {
            checksum += self.ReadByte(address as AddressSize) as u32 ^ (address as u32);
        }
    
        return checksum;
    }

    #[allow(non_snake_case)]
    pub fn TestFlag(&self, addr: AddressSize, bit: u8) -> bool {
        // bit is in the range of [0..31]
        if bit > 31 {
            panic!("Memory[TestFlag]: bit is out of range")
        }
        
        let w: Word = self.ReadWord(addr);
        trace!("{}", w);

        if (w >> bit) & 1 == 1 { true } else { false }
    }

    #[allow(non_snake_case)]
    pub fn SetFlag(&mut self, addr: AddressSize, bit: u8, flag: bool) {
        // bit is in the range of [0..31]
        if bit > 31 {
            panic!("Memory[SetFlag]: bit is out of range")
        }

        let mut w: Word = self.ReadWord(addr);

        if flag {
            // set bit
            w |= 1 << bit;

        } else {
            // clear bit
            w &= !(1 << bit);
        }

        self.WriteWord(addr, w);
    }

    // static utility
    #[allow(non_snake_case)]
    pub fn ExtractBits(w: Word, startBit: u8, endBit: u8) -> Word {
        // bit is in the range of [0..31]
        if startBit > 31 || endBit > 31{
            panic!("Memory[ExtractBits]: bit is out of range")
        }

        if startBit > endBit {
            panic!("Memory[ExtractBits]: startBit must be <= endBit");
        }

        let mut mask: Word = 0;
        for i in startBit..endBit {
            let bit: Word = 1 << i;

            mask |= bit;
        }
        mask & w
    }
}

impl Default for Memory {
    fn default() -> Self {
        Memory {
            checksum: 0,
            endianness: Endianness::Big,
            loaded: false,
            memory_array: vec![0; DEFAULT_MEMORY_SIZE],
            size: DEFAULT_MEMORY_SIZE
        }
    }
}