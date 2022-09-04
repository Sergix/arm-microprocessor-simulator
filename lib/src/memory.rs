use log::trace;
use log::error;
use object::Endianness;

pub type Byte = u8;
type HalfWord = u16;
type Word = u32;
type AddressSize = u32;
type Checksum = u32;

pub const DEFAULT_MEMORY_SIZE: usize = 32768;

// payload for tauri event emitter to send to frontend
// https://tauri.app/v1/guides/features/events/#global-events-1
#[derive(Clone, serde::Serialize)]
pub struct MemoryPayload {
    pub checksum: Checksum,
    pub loaded: bool,
    pub memory_array: Vec<Byte>,
    pub error: String,
    pub filename: String
}

impl Default for MemoryPayload {
    fn default() -> Self {
        MemoryPayload {
            checksum: 0,
            loaded: false,
            memory_array: vec![0, 0],
            error: String::from(""),
            filename: String::from("")
        }
    }
}

pub struct Memory {
    pub checksum: Checksum,
    pub endianness: Endianness,
    pub loaded: bool, // this is included in the case that the frontend was loaded after the elf loader tried to emit an event
    pub memory_array: Vec<Byte>, // unsigned Byte array
    pub size: usize
}

impl Memory {    
    pub fn read_word(&self, addr: AddressSize) -> Word {
        if (addr + 3) as usize > self.size {
            panic!("Memory[read_word]: addr extends past memory size");
        }

        if addr % 4 != 0 {
            error!("Memory[read_word]: Word address not valid");
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

    
    pub fn write_word(&mut self, addr: AddressSize, value: Word) {
        if (addr + 3) as usize > self.size {
            panic!("Memory[write_word]: addr extends past memory size");
        }

        if addr % 4 != 0 {
            error!("Memory[write_word]: Word address not valid");
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

    
    pub fn read_half_word(&self, addr: AddressSize) -> HalfWord {
        if (addr + 1) as usize > self.size {
            panic!("Memory[read_half_word]: addr extends past memory size");
        }

        if addr % 2 != 0 {
            error!("Memory[write_word]: Word address not valid");
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

    
    pub fn write_half_word(&mut self, addr: AddressSize, value: HalfWord) {
        if (addr + 1) as usize > self.size {
            panic!("Memory[write_half_word]: addr extends past memory size");
        }

        if addr % 2 != 0 {
            error!("Memory[write_half_word]: Word address not valid");
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

    
    pub fn read_byte(&self, addr: AddressSize) -> Byte {
        if addr as usize > self.size {
            panic!("Memory[read_byte]: addr extends past memory size");
        }

        *self.memory_array.get(addr as usize).unwrap() as Byte
    }

    
    pub fn write_byte(&mut self, addr: AddressSize, value: Byte) {
        if addr as usize > self.size {
            error!("Memory[write_byte]: addr extends past memory size");
            return
        }

        self.memory_array[addr as usize] = value;
    }

    
    pub fn calculate_checksum(&self) -> Checksum {
        let mut checksum: u32 = 0;
    
        for address in 0..self.memory_array.len() {
            checksum += self.read_byte(address as AddressSize) as u32 ^ (address as u32);
        }
    
        return checksum;
    }

    
    pub fn test_flag(&self, addr: AddressSize, bit: u8) -> bool {
        // bit is in the range of [0..31]
        if bit > 31 {
            panic!("Memory[test_flag]: bit is out of range")
        }
        
        let w: Word = self.read_word(addr);
        trace!("{}", w);

        if (w >> bit) & 1 == 1 { true } else { false }
    }

    
    pub fn set_flag(&mut self, addr: AddressSize, bit: u8, flag: bool) {
        // bit is in the range of [0..31]
        if bit > 31 {
            panic!("Memory[set_flag]: bit is out of range")
        }

        let mut w: Word = self.read_word(addr);

        if flag {
            // set bit
            w |= 1 << bit;

        } else {
            // clear bit
            w &= !(1 << bit);
        }

        self.write_word(addr, w);
    }

    // static utility
    
    pub fn extract_bits(w: Word, start_bit: u8, end_bit: u8) -> Word {
        // bit is in the range of [0..31]
        if start_bit > 31 || end_bit > 31{
            panic!("Memory[extract_bits]: bit is out of range")
        }

        if start_bit > end_bit {
            panic!("Memory[extract_bits]: startBit must be <= endBit");
        }

        let mut mask: Word = 0;
        for i in start_bit..end_bit {
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

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_read_word() {
        let mut mem = Memory::default();

        mem.memory_array[0] = 0x05;
        mem.memory_array[1] = 0xFF;
        mem.memory_array[2] = 0x06;
        mem.memory_array[3] = 0xA0;

        let be = mem.read_word(0);
        assert_eq!(be, 0x05FF06A0);
        
        mem.endianness = Endianness::Little;
        assert_eq!(mem.read_word(0), 0xA006FF05);
        
    }

    
    #[test]
    fn test_read_word_alignment_error() {
        let mut mem = Memory::default();

        mem.memory_array[3] = 0xFF;

        assert_eq!(mem.read_word(3), 0);
    }

    #[test]
    #[should_panic]
    fn test_read_word_bounds_error() {
        let mem = Memory::default();

        mem.read_word(32768);
    }
    
    
    #[test]
    fn test_write_word() {
        let mut mem = Memory::default();

        mem.write_word(0, 0x05FF06A0);
        
        assert_eq!(mem.memory_array[0], 0x05);
        assert_eq!(mem.memory_array[1], 0xFF);
        assert_eq!(mem.memory_array[2], 0x06);
        assert_eq!(mem.memory_array[3], 0xA0);

        mem.endianness = Endianness::Little;
        mem.write_word(0, 0x05FF06A0);

        assert_eq!(mem.memory_array[0], 0xA0);
        assert_eq!(mem.memory_array[1], 0x06);
        assert_eq!(mem.memory_array[2], 0xFF);
        assert_eq!(mem.memory_array[3], 0x05);
    }

    #[test]
    fn test_write_word_alignment_error() {
        let mut mem = Memory::default();

        mem.write_word(3, 1);

        assert_ne!(6, 1);
    }

    #[test]
    #[should_panic]
    fn test_write_word_bounds_error() {
        let mut mem = Memory::default();

        mem.write_word(32768, 0);
    }
    
    
    #[test]
    fn test_read_half_word() {
        let mut mem = Memory::default();
        
        mem.memory_array[0] = 0x05;
        mem.memory_array[1] = 0xFF;
        
        assert_eq!(mem.read_half_word(0), 0x05FF);
        
        mem.endianness = Endianness::Little;
        assert_eq!(mem.read_half_word(0), 0xFF05);
    }

    
    #[test]
    fn test_read_half_word_alignment_error() {
        let mut mem = Memory::default();

        mem.memory_array[1] = 0xFF;

        assert_eq!(mem.read_half_word(1), 0);
    }

    #[test]
    #[should_panic]
    fn test_read_half_word_bounds_error() {
        let mem = Memory::default();

        mem.read_half_word(32768);
    }

    
    #[test]
    fn test_write_half_word() {
        let mut mem = Memory::default();

        mem.write_half_word(0, 0x05FF);
        
        assert_eq!(mem.memory_array[0], 0x05);
        assert_eq!(mem.memory_array[1], 0xFF);

        mem.endianness = Endianness::Little;
        mem.write_half_word(0, 0x05FF);

        assert_eq!(mem.memory_array[0], 0xFF);
        assert_eq!(mem.memory_array[1], 0x05);
    }

    #[test]
    fn test_write_half_word_alignment_error() {
        let mut mem = Memory::default();

        mem.write_half_word(3, 1);

        assert_ne!(4, 1);
    }

    #[test]
    #[should_panic]
    fn test_write_half_word_bounds_error() {
        let mut mem = Memory::default();

        mem.write_half_word(32768, 0);
    }
    
    #[test]
    fn test_read_byte() {
        let mut mem = Memory::default();
        
        mem.memory_array[0] = 0x05;
        
        assert_eq!(mem.read_byte(0), 0x05);
        
        mem.endianness = Endianness::Little;
        assert_eq!(mem.read_byte(0), 0x05);
    }

    #[test]
    #[should_panic]
    fn test_read_byte_bounds_error() {
        let mem = Memory::default();

        mem.read_byte(32768);
    }

    
    #[test]
    fn test_write_byte() {
        let mut mem = Memory::default();
        
        mem.write_byte(0, 0x05);
        
        assert_eq!(mem.memory_array[0], 0x05);

        mem.endianness = Endianness::Little;
        mem.write_byte(0, 0x05);

        assert_eq!(mem.memory_array[0], 0x05);
    }

    #[test]
    #[should_panic]
    fn test_write_byte_bounds_error() {
        let mut mem = Memory::default();

        mem.write_half_word(32768, 0);
    }
    
    #[test]
    fn test_calculate_checksum() {
        let mut mem = Memory::default();
        
        mem.memory_array[0] = 0x01;
        mem.memory_array[1] = 0x82;
        mem.memory_array[2] = 0x03;
        mem.memory_array[3] = 0x84;

        assert_eq!(mem.calculate_checksum(), 536854790);
    }

    
    #[test]
    fn test_test_flag() {
        let mut mem = Memory::default();

        mem.memory_array[0] = 0x1C;
        mem.memory_array[1] = 0xCB;
        mem.memory_array[2] = 0x1D;
        mem.memory_array[3] = 0x1A;

        assert_eq!(mem.test_flag(0, 11), true);
        assert_eq!(mem.test_flag(0, 13), false);
    }

    
    #[test]
    #[should_panic]
    fn test_test_flag_bit_range_error() {
        let mem = Memory::default();

        mem.test_flag(0, 32);
    }

    
    #[test]
    fn test_set_flag() {
        let mut mem = Memory::default();

        mem.set_flag(0, 12, true);
        assert_eq!(mem.memory_array[2], 0x10);

        mem.set_flag(0, 12, false);
        assert_eq!(mem.memory_array[2], 0x00);
    }

    
    #[test]
    #[should_panic]
    fn test_set_flag_bit_range_error() {
        let mut mem = Memory::default();

        mem.set_flag(0, 32, true);
    }

    
    #[test]
    fn test_extract_bits() {
        let w = Memory::extract_bits(0xC7A2511E, 5, 20);
        assert_eq!(w, 0x25100);
    }

    
    #[test]
    #[should_panic]
    fn test_extract_bits_invalid_start_bit() {
        Memory::extract_bits(0x0, 32, 0);
    }

    
    #[test]
    #[should_panic]
    fn test_extract_bits_invalid_end_bit() {
        Memory::extract_bits(0x0, 0, 32);
    }

    
    #[test]
    #[should_panic]
    fn test_extract_bits_bit_inequality() {
        Memory::extract_bits(0x0, 12, 10);
    }
}