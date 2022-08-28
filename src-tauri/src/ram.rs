type Byte = u8;
type HalfWord = u16;
type Word = u32;
type AddressSize = u32;

pub struct RAM {
    pub(crate) size: usize,
    pub(crate) memory_array: Vec<Byte> // unsigned Byte array
}

impl RAM {
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