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

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(non_snake_case)]
    #[test]
    fn test_ReadWord() {
        let mut mem = Memory::default();

        mem.memory_array[0] = 0x05;
        mem.memory_array[1] = 0xFF;
        mem.memory_array[2] = 0x06;
        mem.memory_array[3] = 0xA0;

        let be = mem.ReadWord(0);
        assert_eq!(be, 0x05FF06A0);
        
        mem.endianness = Endianness::Little;
        assert_eq!(mem.ReadWord(0), 0xA006FF05);
        
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_ReadWord_AlignmentError() {
        let mut mem = Memory::default();

        mem.memory_array[3] = 0xFF;

        assert_eq!(mem.ReadWord(3), 0);
    }
    
    #[allow(non_snake_case)]
    #[test]
    fn test_WriteWord() {
        let mut mem = Memory::default();

        mem.WriteWord(0, 0x05FF06A0);
        
        assert_eq!(mem.memory_array[0], 0x05);
        assert_eq!(mem.memory_array[1], 0xFF);
        assert_eq!(mem.memory_array[2], 0x06);
        assert_eq!(mem.memory_array[3], 0xA0);

        mem.endianness = Endianness::Little;
        mem.WriteWord(0, 0x05FF06A0);

        assert_eq!(mem.memory_array[0], 0xA0);
        assert_eq!(mem.memory_array[1], 0x06);
        assert_eq!(mem.memory_array[2], 0xFF);
        assert_eq!(mem.memory_array[3], 0x05);
    }
    
    #[allow(non_snake_case)]
    #[test]
    fn test_ReadHalfWord() {
        let mut mem = Memory::default();
        
        mem.memory_array[0] = 0x05;
        mem.memory_array[1] = 0xFF;
        
        assert_eq!(mem.ReadHalfWord(0), 0x05FF);
        
        mem.endianness = Endianness::Little;
        assert_eq!(mem.ReadHalfWord(0), 0xFF05);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_ReadHalfWord_AlignmentError() {
        let mut mem = Memory::default();

        mem.memory_array[1] = 0xFF;

        assert_eq!(mem.ReadHalfWord(1), 0);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_WriteHalfWord() {
        let mut mem = Memory::default();

        mem.WriteHalfWord(0, 0x05FF);
        
        assert_eq!(mem.memory_array[0], 0x05);
        assert_eq!(mem.memory_array[1], 0xFF);

        mem.endianness = Endianness::Little;
        mem.WriteHalfWord(0, 0x05FF);

        assert_eq!(mem.memory_array[0], 0xFF);
        assert_eq!(mem.memory_array[1], 0x05);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_ReadByte() {
        let mut mem = Memory::default();
        
        mem.memory_array[0] = 0x05;
        
        assert_eq!(mem.ReadByte(0), 0x05);
        
        mem.endianness = Endianness::Little;
        assert_eq!(mem.ReadByte(0), 0x05);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_WriteByte() {
        let mut mem = Memory::default();
        
        mem.WriteByte(0, 0x05);
        
        assert_eq!(mem.memory_array[0], 0x05);

        mem.endianness = Endianness::Little;
        mem.WriteByte(0, 0x05);

        assert_eq!(mem.memory_array[0], 0x05);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_CalculateChecksum() {
        let mut mem = Memory::default();
        
        mem.memory_array[0] = 0x01;
        mem.memory_array[1] = 0x82;
        mem.memory_array[2] = 0x03;
        mem.memory_array[3] = 0x84;

        assert_eq!(mem.CalculateChecksum(), 536854790);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_TestFlag() {
        let mut mem = Memory::default();

        mem.memory_array[0] = 0x1C;
        mem.memory_array[1] = 0xCB;
        mem.memory_array[2] = 0x1D;
        mem.memory_array[3] = 0x1A;

        assert_eq!(mem.TestFlag(0, 11), true);
        assert_eq!(mem.TestFlag(0, 13), false);
    }

    #[allow(non_snake_case)]
    #[test]
    #[should_panic]
    fn test_TestFlag_BitRangeError() {
        let mem = Memory::default();

        mem.TestFlag(0, 32);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_SetFlag() {
        let mut mem = Memory::default();

        mem.SetFlag(0, 12, true);
        assert_eq!(mem.memory_array[2], 0x10);

        mem.SetFlag(0, 12, false);
        assert_eq!(mem.memory_array[2], 0x00);
    }

    #[allow(non_snake_case)]
    #[test]
    #[should_panic]
    fn test_SetFlag_BitRangeError() {
        let mut mem = Memory::default();

        mem.SetFlag(0, 32, true);
    }

    #[allow(non_snake_case)]
    #[test]
    fn test_ExtractBits() {
        let w = Memory::ExtractBits(0xC7A2511E, 5, 20);
        assert_eq!(w, 0x25100);
    }

    #[allow(non_snake_case)]
    #[test]
    #[should_panic]
    fn test_ExtractBits_InvalidStartBit() {
        Memory::ExtractBits(0x0, 32, 0);
    }

    #[allow(non_snake_case)]
    #[test]
    #[should_panic]
    fn test_ExtractBits_InvalidEndBit() {
        Memory::ExtractBits(0x0, 0, 32);
    }

    #[allow(non_snake_case)]
    #[test]
    #[should_panic]
    fn test_ExtractBits_BitInequality() {
        Memory::ExtractBits(0x0, 12, 10);
    }
}