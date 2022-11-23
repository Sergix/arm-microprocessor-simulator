use core::fmt;

use log::error;
use num_derive::FromPrimitive;
use object::Endianness;

use crate::cpu_enum::Mode;

pub type Byte = u8;
pub type HalfWord = u16;
pub type Word = u32;
pub type AddressSize = u32;
pub type Checksum = u32;

pub const DEFAULT_MEMORY_SIZE: usize = 32768;

pub const NUM_REGISTERS: usize = 23; // r0...r15, CPSR, SP_svc, LR_svc, SPSR_svc, SP_irq, LR_irq, SPSR_irq
pub const REGISTER_BYTES: usize = 4; // 4byte = 32bit

pub const CPSR_ADDR: AddressSize = (16 * REGISTER_BYTES) as AddressSize;
pub const SPSR_SVC_ADDR: AddressSize = (19 * REGISTER_BYTES) as AddressSize;
pub const SPSR_IRQ_ADDR: AddressSize = (22 * REGISTER_BYTES) as AddressSize;

// used when calculating r13 and r14 in non-system modes to properly index the register array
pub const MODE_OFFSET_SVC: usize = 4;
pub const MODE_OFFSET_IRQ: usize = 7;

pub const DISPLAY_ADDR: AddressSize  = 0x100000;
pub const KEYBOARD_ADDR: AddressSize = 0x100001;


#[allow(non_camel_case_types)]
#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
pub enum Register {
    r0 = 0,
    r1 = 1,
    r2 = 2,
    r3 = 3,
    r4 = 4,
    r5 = 5,
    r6 = 6,
    r7 = 7,
    r8 = 8,
    r9 = 9,
    r10 = 10,
    r11 = 11,
    r12 = 12,
    r13 = 13,
    r14 = 14,
    r15 = 15
}

impl fmt::Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Clone, serde::Serialize)]
pub struct RegistersPayload {
    pub register_array: Vec<Word>
}

impl Default for RegistersPayload {
    fn default() -> Self {
        RegistersPayload {
            register_array: vec![0, 0]
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct FlagsPayload {
    pub n: bool,
    pub c: bool,
    pub z: bool,
    pub v: bool,
    pub i: bool
}

impl Default for FlagsPayload {
    fn default() -> Self {
        FlagsPayload {
            n: false,
            c: false,
            z: false,
            v: false,
            i: false,
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct RAMPayload {
    pub checksum: Checksum,
    pub memory_array: Vec<Vec<Byte>>
}

impl Default for RAMPayload {
    fn default() -> Self {
        RAMPayload {
            checksum: 0,
            memory_array: vec![vec![0; 0]; 0],
        }
    }
}

pub trait Memory {
    fn new(size: usize, endianness: Endianness) -> Self;
    fn get_size(&self) -> usize;
    fn get_memory_array(&mut self) -> &mut Vec<Byte>;
    fn get_endianness(&self) -> Endianness;
    fn get_checksum(&self) -> Checksum;
    fn set_checksum(&mut self, checksum: Checksum);

    fn clear(&mut self) {
        let sz = self.get_size();
        self.get_memory_array().clear();
        self.get_memory_array().resize(sz, 0);
    }

    fn read_word(&mut self, addr: AddressSize) -> Word {
        if (addr + 3) as usize > self.get_size() {
            panic!("Memory[read_word]: addr extends past memory size");
        }

        if addr % 4 != 0 {
            error!("Memory[read_word]: Word address not valid");
            return 0
        }

        let w0: Word = *self.get_memory_array().get(addr as usize).unwrap() as Word;
        let w1: Word = *self.get_memory_array().get((addr + 1) as usize).unwrap() as Word;
        let w2: Word = *self.get_memory_array().get((addr + 2) as usize).unwrap() as Word;
        let w3: Word = *self.get_memory_array().get((addr + 3) as usize).unwrap() as Word;

        if self.get_endianness() == Endianness::Little {
            (w3 << 24) | (w2 << 16) | (w1 << 8) | w0
        } else {
            (w0 << 24) | (w1 << 16) | (w2 << 8) | w3
        }
    }

    
    fn write_word(&mut self, addr: AddressSize, value: Word) {
        if (addr + 3) as usize > self.get_size() {
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

        if self.get_endianness() == Endianness::Little {
            self.get_memory_array()[addr as usize] = b3;
            self.get_memory_array()[(addr + 1) as usize] = b2;
            self.get_memory_array()[(addr + 2) as usize] = b1;
            self.get_memory_array()[(addr + 3) as usize] = b0;
        } else {
            self.get_memory_array()[addr as usize] = b0;
            self.get_memory_array()[(addr + 1) as usize] = b1;
            self.get_memory_array()[(addr + 2) as usize] = b2;
            self.get_memory_array()[(addr + 3) as usize] = b3;
        }

        let checksum = self.calculate_checksum();
        self.set_checksum(checksum);
    }

    
    fn read_half_word(&mut self, addr: AddressSize) -> HalfWord {
        if (addr + 1) as usize > self.get_size() {
            panic!("Memory[read_half_word]: addr extends past memory size");
        }

        if addr % 2 != 0 {
            error!("Memory[write_word]: Word address not valid");
            return 0
        }

        let hw0: HalfWord = *self.get_memory_array().get(addr as usize).unwrap() as HalfWord;
        let hw1: HalfWord = *self.get_memory_array().get((addr + 1) as usize).unwrap() as HalfWord;

        if self.get_endianness() == Endianness::Little {
            (hw1 << 8) | hw0
        } else {
            (hw0 << 8) | hw1
        }
    }

    
    fn write_half_word(&mut self, addr: AddressSize, value: HalfWord) {
        if (addr + 1) as usize > self.get_size() {
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
        if self.get_endianness() == Endianness::Little {
            self.get_memory_array()[addr as usize] = b1;
            self.get_memory_array()[(addr + 1) as usize] = b0;
        } else {
            self.get_memory_array()[addr as usize] = b0;
            self.get_memory_array()[(addr + 1) as usize] = b1;
        }

        let checksum = self.calculate_checksum();
        self.set_checksum(checksum);
    }

    
    fn read_byte(&mut self, addr: AddressSize) -> Byte {
        if addr as usize > self.get_size() {
            panic!("Memory[read_byte]: addr extends past memory size");
        }

        *self.get_memory_array().get(addr as usize).unwrap() as Byte
    }

    
    fn write_byte(&mut self, addr: AddressSize, value: Byte) {
        if addr as usize > self.get_size() {
            error!("Memory[write_byte]: addr extends past memory size");
            return
        }

        self.get_memory_array()[addr as usize] = value;

        let checksum = self.calculate_checksum();
        self.set_checksum(checksum);
    }

    
    fn calculate_checksum(&mut self) -> Checksum {
        let mut checksum: u32 = 0;
    
        for address in 0..self.get_memory_array().len() {
            checksum += self.read_byte(address as AddressSize) as u32 ^ (address as u32);
        }
    
        return checksum;
    }

    
    fn test_flag(&mut self, addr: AddressSize, bit: u8) -> bool {
        // bit is in the range of [0..31]
        if bit > 31 {
            panic!("Memory[test_flag]: bit is out of range")
        }
        
        let w: Word = self.read_word(addr);
        if (w >> bit) & 1 == 1 { true } else { false }
    }

    
    fn set_flag(&mut self, addr: AddressSize, bit: u8, flag: bool) {
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
    fn extract_bits(w: Word, start_bit: u8, end_bit: u8) -> Word {
        // bit is in the range of [0..31]
        if start_bit > 31 || end_bit > 31{
            panic!("Memory[extract_bits]: bit is out of range")
        }

        if start_bit > end_bit {
            panic!("Memory[extract_bits]: startBit must be <= endBit");
        }

        let mut mask: Word = 0;
        for i in start_bit..=end_bit {
            let bit: Word = 1 << i;
            mask |= bit;
        }
        mask & w
    }
}

pub struct Registers {
    pub endianness: Endianness,
    pub memory_array: Vec<Byte>, // unsigned Byte array
    pub size: usize
}

impl Registers {
    // fix indices when attempting to access banked registers on non-system modes
    pub fn get_register_mode_index(&mut self, index: usize) -> usize {
        if index > 15 {
            panic!("Registers[get_register_mode_index]: register index out of range");
        }

        match self.get_cpsr_mode() {
            Mode::SVC => {
                if index == 13 || index == 14 {
                    index + MODE_OFFSET_SVC
                } else {
                    index
                }
            },
            Mode::IRQ => {
                if index == 13 || index == 14 {
                    index + MODE_OFFSET_IRQ
                } else {
                    index
                }
            },
            _ => index
        }
    }

    pub fn set_register(&mut self, index: usize, value: Word) {
        if index > 15 {
            panic!("Registers[set_register]: register index out of range");
        }

        let address = (self.get_register_mode_index(index) * 4) as AddressSize;
        self.write_word(address, value)
    }

    pub fn set_reg_register(&mut self, reg: Register, value: Word) {
        self.set_register(reg as usize, value)
    }

    pub fn get_register(&mut self, index: usize) -> Word {
        if index > 15 {
            panic!("Registers[get_register]: register index out of range");
        }

        let address = (self.get_register_mode_index(index) * 4) as AddressSize;
        self.read_word(address)
    }

    pub fn get_reg_register(&mut self, reg: Register) -> Word {
        self.get_register(reg as usize)
    }

    pub fn get_all(&mut self) -> Vec<Word> {
        let mut regs: Vec<Word> = vec![0; 0];
        
        // r0..r15
        for i in 0..=15 {
            regs.push(self.get_register(i));
        }
        regs
    }

    pub fn set_pc(&mut self, value: Word) {
        self.set_register(15, value)
    }

    pub fn get_pc(&mut self) -> Word {
        self.get_register(15)
    }

    pub fn get_pc_current_address(&mut self) -> Word {
        self.get_pc() - 8
    }

    pub fn inc_pc(&mut self) {
        let next_addr = self.get_pc() + 4;
        self.set_register(15, next_addr)
    }

    // CPSR register is last register
    pub fn get_cpsr(&mut self) -> Word {
        // have to manually read location since CPSR is not little- or big-endian
        self.read_word(CPSR_ADDR)
    }

    pub fn set_cpsr(&mut self, value: Word) {
        self.write_word(CPSR_ADDR, value);
    }

    pub fn set_cpsr_flag(&mut self, bit: u8, flag: bool) {
        self.set_flag(CPSR_ADDR, bit, flag)
    }

    pub fn get_cpsr_flag(&mut self, bit: u8) -> bool {
        self.test_flag(CPSR_ADDR, bit)
    }

    pub fn get_cpsr_mode(&mut self) -> Mode {
        let mode_bits = Registers::extract_bits(self.get_cpsr(), 0, 4);
        let mode: Mode = num::FromPrimitive::from_u32(mode_bits).unwrap();
        mode
    }

    pub fn set_cpsr_mode(&mut self, mode: Mode) {
        // TODO: swap banked registers
        log::trace!("set_cpsr_mode: swapping banked registers {}mode", self.get_cpsr());

        let mode_bits = mode as Byte;
        let cleared_mode_byte = self.read_byte(CPSR_ADDR + 3) & 0b11100000;
        self.write_byte(CPSR_ADDR + 3, cleared_mode_byte | mode_bits);
    }

    pub fn clear_nzcv(&mut self) {
        self.set_flag(CPSR_ADDR, 31, false);
        self.set_flag(CPSR_ADDR, 30, false);
        self.set_flag(CPSR_ADDR, 29, false);
        self.set_flag(CPSR_ADDR, 28, false);
    }

    pub fn get_n_flag(&mut self) -> bool {
        self.test_flag(CPSR_ADDR, 31)
    }

    pub fn set_n_flag(&mut self, flag: bool) {
        self.set_flag(CPSR_ADDR, 31, flag);
    }

    pub fn get_z_flag(&mut self) -> bool {
        self.test_flag(CPSR_ADDR, 30)
    }

    pub fn set_z_flag(&mut self, flag: bool) {
        self.set_flag(CPSR_ADDR, 30, flag);
    }

    pub fn get_c_flag(&mut self) -> bool {
        self.test_flag(CPSR_ADDR, 29)
    }

    pub fn set_c_flag(&mut self, flag: bool) {
        self.set_flag(CPSR_ADDR, 29, flag);
    }

    pub fn get_v_flag(&mut self) -> bool {
        self.test_flag(CPSR_ADDR, 28)
    }

    pub fn set_v_flag(&mut self, flag: bool) {
        self.set_flag(CPSR_ADDR, 28, flag);
    }

    pub fn get_i_flag(&mut self) -> bool {
        self.test_flag(CPSR_ADDR, 7)
    }

    pub fn set_i_flag(&mut self, flag: bool) {
        self.set_flag(CPSR_ADDR, 7, flag);
    }

    pub fn get_t_flag(&mut self) -> bool {
        self.test_flag(CPSR_ADDR, 5)
    }

    pub fn set_t_flag(&mut self, flag: bool) {
        self.set_flag(CPSR_ADDR, 5, flag);
    }

    pub fn get_cpsr_control_byte(&mut self) -> Byte {
        self.read_byte(CPSR_ADDR + 3)
    }

    pub fn get_spsr(&mut self) -> Word {
        match self.get_cpsr_mode() {
            Mode::SVC => self.read_word(SPSR_SVC_ADDR),
            Mode::IRQ => self.read_word(SPSR_IRQ_ADDR),
            _ => {
                // TODO
                todo!("throw? or return CPSR?");
            }
        }
    }

    pub fn set_spsr(&mut self, value: Word) {
        match self.get_cpsr_mode() {
            Mode::SVC => self.write_word(SPSR_SVC_ADDR, value),
            Mode::IRQ => self.write_word(SPSR_IRQ_ADDR, value),
            _ => {
                // TODO
                todo!("throw? or store in CPSR?");
            }
        }
    }

    pub fn current_mode_has_spsr(&mut self) -> bool {
        match self.get_cpsr_mode() {
            Mode::SVC => true,
            Mode::IRQ => true,
            _ => false
        }
    }

    pub fn get_nzcv(&mut self) -> Byte {
        let n = if self.get_n_flag() { 1 } else { 0 };
        let z = if self.get_z_flag() { 1 } else { 0 };
        let c = if self.get_c_flag() { 1 } else { 0 };
        let v = if self.get_v_flag() { 1 } else { 0 };

        n << 3 | z << 2 | c << 1 | v
    }

    pub fn get_nzcv_tuple(&mut self) -> (bool, bool, bool, bool) {
        (self.get_n_flag(), self.get_z_flag(), self.get_c_flag(), self.get_v_flag())
    }
}

impl Memory for Registers {
    fn new(size: usize, endianness: Endianness) -> Self {
        Self {
            endianness,
            memory_array: vec![0; size],
            size: size
        }
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_memory_array(&mut self) -> &mut Vec<Byte> {
        &mut self.memory_array
    }

    fn get_endianness(&self) -> Endianness {
        self.endianness
    }

    // only to fulfill method stubs
    fn get_checksum(&self) -> Checksum { 0 }
    fn set_checksum(&mut self, _checksum: Checksum) { }
}

impl Default for Registers {
    fn default() -> Self {
        Registers {
            endianness: Endianness::Big,
            memory_array: vec![0; NUM_REGISTERS * REGISTER_BYTES],
            size: NUM_REGISTERS * REGISTER_BYTES
        }
    }
}

pub struct RAM {
    pub checksum: Checksum,
    pub endianness: Endianness,
    pub loaded: bool, // this is included in the case that the frontend was loaded after the elf loader tried to emit an event
    pub memory_array: Vec<Byte>, // unsigned Byte array
    pub size: usize,
    pub display_offset: AddressSize // offset used when computing chunks for the frontend
}

impl Memory for RAM {
    fn new(size: usize, endianness: Endianness) -> Self {
        Self {
            checksum: 0,
            endianness,
            loaded: false,
            memory_array: vec![0; size],
            size,
            display_offset: 0
        }
    }

    fn get_size(&self) -> usize {
        self.size
    }

    fn get_memory_array(&mut self) -> &mut Vec<Byte> {
        &mut self.memory_array
    }

    fn get_endianness(&self) -> Endianness {
        self.endianness
    }

    fn get_checksum(&self) -> Checksum {
        self.checksum
    }

    fn set_checksum(&mut self, checksum: Checksum) {
        self.checksum = checksum
    }
}

impl Default for RAM {
    fn default() -> Self {
        RAM {
            checksum: 0,
            endianness: Endianness::Big,
            loaded: false,
            memory_array: vec![0; DEFAULT_MEMORY_SIZE],
            size: DEFAULT_MEMORY_SIZE,
            display_offset: 0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    
    #[test]
    fn test_read_word() {
        let mut mem = RAM::default();

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
        let mut mem = RAM::default();

        mem.memory_array[3] = 0xFF;

        assert_eq!(mem.read_word(3), 0);
    }

    #[test]
    #[should_panic]
    fn test_read_word_bounds_error() {
        let mut mem = RAM::default();

        mem.read_word(32768);
    }
    
    
    #[test]
    fn test_write_word() {
        let mut mem = RAM::default();

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
        let mut mem = RAM::default();

        mem.write_word(3, 1);

        assert_ne!(6, 1);
    }

    #[test]
    #[should_panic]
    fn test_write_word_bounds_error() {
        let mut mem = RAM::default();

        mem.write_word(32768, 0);
    }
    
    
    #[test]
    fn test_read_half_word() {
        let mut mem = RAM::default();
        
        mem.memory_array[0] = 0x05;
        mem.memory_array[1] = 0xFF;
        
        assert_eq!(mem.read_half_word(0), 0x05FF);
        
        mem.endianness = Endianness::Little;
        assert_eq!(mem.read_half_word(0), 0xFF05);
    }

    
    #[test]
    fn test_read_half_word_alignment_error() {
        let mut mem = RAM::default();

        mem.memory_array[1] = 0xFF;

        assert_eq!(mem.read_half_word(1), 0);
    }

    #[test]
    #[should_panic]
    fn test_read_half_word_bounds_error() {
        let mut mem = RAM::default();

        mem.read_half_word(32768);
    }

    
    #[test]
    fn test_write_half_word() {
        let mut mem = RAM::default();

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
        let mut mem = RAM::default();

        mem.write_half_word(3, 1);

        assert_ne!(4, 1);
    }

    #[test]
    #[should_panic]
    fn test_write_half_word_bounds_error() {
        let mut mem = RAM::default();

        mem.write_half_word(32768, 0);
    }
    
    #[test]
    fn test_read_byte() {
        let mut mem = RAM::default();
        
        mem.memory_array[0] = 0x05;
        
        assert_eq!(mem.read_byte(0), 0x05);
        
        mem.endianness = Endianness::Little;
        assert_eq!(mem.read_byte(0), 0x05);
    }

    #[test]
    #[should_panic]
    fn test_read_byte_bounds_error() {
        let mut mem = RAM::default();

        mem.read_byte(32768);
    }

    
    #[test]
    fn test_write_byte() {
        let mut mem = RAM::default();
        
        mem.write_byte(0, 0x05);
        
        assert_eq!(mem.memory_array[0], 0x05);

        mem.endianness = Endianness::Little;
        mem.write_byte(0, 0x05);

        assert_eq!(mem.memory_array[0], 0x05);
    }

    #[test]
    #[should_panic]
    fn test_write_byte_bounds_error() {
        let mut mem = RAM::default();

        mem.write_half_word(32768, 0);
    }
    
    #[test]
    fn test_calculate_checksum() {
        let mut mem = RAM::default();
        
        mem.memory_array[0] = 0x01;
        mem.memory_array[1] = 0x82;
        mem.memory_array[2] = 0x03;
        mem.memory_array[3] = 0x84;

        assert_eq!(mem.calculate_checksum(), 536854790);
    }

    
    #[test]
    fn test_test_flag() {
        let mut mem = RAM::default();

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
        let mut mem = RAM::default();

        mem.test_flag(0, 32);
    }

    
    #[test]
    fn test_set_flag() {
        let mut mem = RAM::default();

        mem.set_flag(0, 12, true);
        assert_eq!(mem.memory_array[2], 0x10);

        mem.set_flag(0, 12, false);
        assert_eq!(mem.memory_array[2], 0x00);
    }

    
    #[test]
    #[should_panic]
    fn test_set_flag_bit_range_error() {
        let mut mem = RAM::default();

        mem.set_flag(0, 32, true);
    }

    
    #[test]
    fn test_extract_bits() {
        let w = RAM::extract_bits(0xC7A2511E, 5, 20);
        assert_eq!(w, 0x25100);
    }

    
    #[test]
    #[should_panic]
    fn test_extract_bits_invalid_start_bit() {
        RAM::extract_bits(0x0, 32, 0);
    }

    
    #[test]
    #[should_panic]
    fn test_extract_bits_invalid_end_bit() {
        RAM::extract_bits(0x0, 0, 32);
    }

    
    #[test]
    #[should_panic]
    fn test_extract_bits_bit_inequality() {
        RAM::extract_bits(0x0, 12, 10);
    }

    #[test]
    fn test_get_as_word() {
        let mut regs = Registers::default();

        // little-endian
        regs.memory_array[4] = 0x45;
        regs.memory_array[5] = 0x55;
        regs.memory_array[6] = 0xAE;
        regs.memory_array[7] = 0xFF;

        assert_eq!(0xFFAE5545, regs.get_register(1));
    }

    #[test]
    fn test_get_register() {
        let mut regs = Registers::default();

        regs.write_word(4, 0xAEBD056D);
        assert_eq!(0xAEBD056D, regs.get_register(1));
    }

    #[test]
    #[should_panic]
    fn test_get_register_range_error() {
        let mut regs = Registers::default();

        regs.get_register(16);
    }

    #[test]
    fn test_set_register() {
        let mut regs = Registers::default();
        regs.set_register(1, 5);

        assert_eq!(5, regs.read_word(4));
    }

    #[test]
    #[should_panic]
    fn test_set_register_range_error() {
        let mut regs = Registers::default();
        regs.set_register(16, 0);
    }

    #[test]
    fn test_get_all() {
        let mut regs = Registers::default();

        regs.set_register(0, 0xAEB);
        regs.set_register(1, 5);
        regs.set_register(13, 0x11FF11FF);

        let rs = regs.get_all();
        assert_eq!(0xAEB, rs[0]);
        assert_eq!(5, rs[1]);
        assert_eq!(0x11FF11FF, rs[13]);
    }

    #[test]
    fn test_set_pc() {
        let mut regs = Registers::default();

        regs.set_pc(0x106);
        assert_eq!(0x106, regs.get_register(15));
    }

    #[test]
    fn test_get_pc() {
        let mut regs = Registers::default();

        regs.set_register(15, 0x106);
        assert_eq!(0x106, regs.get_pc());
    }

    #[test]
    fn test_inc_pc() {
        let mut regs = Registers::default();

        regs.set_pc(0x106);
        regs.inc_pc();
        assert_eq!(0x10a, regs.get_pc());
    }

    #[test]
    fn test_get_cpsr() {
        let mut regs = Registers::default();

        regs.write_word(64, 0xFF11FF11);
        assert_eq!(0xFF11FF11, regs.get_cpsr());
    }

    #[test]
    fn test_get_cpsr_control_byte() {
        let mut regs = Registers::default();

        regs.write_word(64, 0xAA000000);
        assert_eq!(0xAA, regs.get_cpsr_control_byte());
    }
}