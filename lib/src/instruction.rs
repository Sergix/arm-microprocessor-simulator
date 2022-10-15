use crate::memory::{Byte, AddressSize, Word, Register, HalfWord};
use crate::cpu_enum::{Condition, ShiftType, DataOpcode, LSMCode, AddressingMode, LSH, InstrType};

/*
Supertype structs
*/
pub trait TInstruction {
    fn new(_type: InstrType) -> Self;

    fn decode(&self);
    fn encode(&self);

    // fn set_execute();
    fn execute(&self);

    fn get_name(&self) -> String;
    // fn get_instr(&self) -> Word;

    fn get_type(&self) -> InstrType;
    fn set_type(&mut self, _type: InstrType);

    fn get_data_opcode(&self) -> Option<DataOpcode>;
    fn set_data_opcode(&mut self, code: Word);

    fn get_s_bit(&self) -> Option<bool>;
    fn set_s_bit(&mut self, bit: Word);

    fn get_shift_type(&self) -> Option<ShiftType>;
    fn set_shift_type(&mut self, sh: Word);

    fn get_imm_shift(&self) -> Option<Byte>;
    fn set_imm_shift(&mut self, imm: Word);

    fn get_imm(&self) -> Option<Byte>;
    fn set_imm(&mut self, rotate: Word, imm: Word);

    // TODO:
    // - get/set writeback
    // - get/set addressing mode
    // - get/set swi
    // - get/set l bit
    // - get/set LSH
    // - get/set add/sub
    // - get/set byte/word
    // - get/set ldr/str
    // - store constant

    // shift value in immediate_shift field by shift amount
    fn shift_value(&mut self, shift_amount: Byte, shift_type: ShiftType);

    // output instruction string (bytes -> assembly macros)
    fn disassemble(&self) -> String {
        // let opcode_str: Option<String>;
        // let s_bit: Option<String>;
        // let rn_str: Option<String>;
        // let rd_str: Option<String>;
        // let imm_str: Option<String>;

        // match self.get_type() {
        //     InstrType::DataImm => {
        //         match self.get_data_opcode().unwrap() {
        //             DataOpcode::MOV => opcode_str = Some(String::from("mov")),
        //         };

        //         match self.get_s_bit().unwrap() {
        //             true => s_bit = Some(String::from("S")),
        //             false => s_bit = None
        //         }

        //         rd_str = Some(self.get_rd().unwrap().to_string());
        //         // imm_str = Some(self.get_imm())
        //     },
        //     _ => {}
        // };

        // format!("{} {}, #{}", opcode_str.unwrap(), rd_str.unwrap(), imm_str.unwrap())
        String::new()
    }

    fn get_rd(&self) -> Option<Register>;
    fn set_rd(&mut self, rd: Word);
    fn get_rn(&self) -> Option<Register>;
    fn set_rn(&mut self, rn: Word);
    fn get_rm(&self) -> Option<Register>;
    fn set_rm(&mut self, rm: Word);

    // NZCV flag
    fn set_condition(&mut self, cond: Word);
    fn get_condition(&self) -> Condition;
}

#[derive(Clone, Copy)]
pub struct Instruction {
    _type: InstrType,
    // instr: Word,
    rn: Option<Register>,
    rd: Option<Register>,
    rm: Option<Register>,
    condition: Condition,
    data_opcode: Option<DataOpcode>,
    s_bit: Option<bool>,
    shift_type: Option<ShiftType>,
    imm_shift: Option<Byte>,
    imm: Option<Byte>
}

impl TInstruction for Instruction {
    fn new(_type: InstrType) -> Self {
        Instruction {
            _type: _type,
            // instr: instr,
            rn: None,
            rd: None,
            rm: None,
            condition: Condition::AL,
            data_opcode: None,
            s_bit: None,
            shift_type: None,
            imm_shift: None,
            imm: None
        }
    }

    fn get_type(&self) -> InstrType {
        self._type
    }

    fn set_type(&mut self, _type: InstrType) {
        self._type = _type;
    }

    // fn get_instr(&self) -> Word {
    //     self.instr
    // }

    fn decode(&self) {
        todo!()
    }

    fn encode(&self) {
        todo!()
    }

    fn execute(&self) {
        todo!()
    }

    fn get_name(&self) -> String {
        todo!()
    }

    fn shift_value(&mut self, shift_amount: Byte, shift_type: ShiftType) {
        todo!()
    }

    fn disassemble(&self) -> String {
        todo!()
    }

    fn get_rd(&self) -> Option<Register> {
        self.rd
    }

    fn set_rd(&mut self, rd: Word) {
        let rd: Register = num::FromPrimitive::from_u32(rd).unwrap();
        self.rd = Some(rd);
    }

    fn get_rn(&self) -> Option<Register> {
        self.rn
    }

    fn set_rn(&mut self, rn: Word) {
        let rn: Register = num::FromPrimitive::from_u32(rn).unwrap();
        self.rn = Some(rn);
    }

    fn get_rm(&self) -> Option<Register> {
        self.rm
    }

    fn set_rm(&mut self, rm: Word) {
        let rm: Register = num::FromPrimitive::from_u32(rm).unwrap();
        self.rm = Some(rm);
    }

    fn set_condition(&mut self, cond: Word) {
        let condition: Condition = num::FromPrimitive::from_u32(cond).unwrap();
        self.condition = condition;
    }

    fn get_condition(&self) -> Condition {
        self.condition
    }

    fn get_data_opcode(&self) -> Option<DataOpcode> {
        self.data_opcode
    }

    fn set_data_opcode(&mut self, code: Word) {
        let opcode: DataOpcode = num::FromPrimitive::from_u32(code).unwrap();
        self.data_opcode = Some(opcode);
    }

    fn get_s_bit(&self) -> Option<bool> {
        self.s_bit
    }

    fn set_s_bit(&mut self, bit: Word) {
        match bit & 0x1 {
            0 => self.s_bit = Some(false),
            1 => self.s_bit = Some(true),
            _ => self.s_bit = None
        }
    }

    fn get_shift_type(&self) -> Option<ShiftType> {
        self.shift_type
    }

    fn set_shift_type(&mut self, sh: Word) {
        let shift: ShiftType = num::FromPrimitive::from_u32(sh).unwrap();
        self.shift_type = Some(shift);
    }

    fn get_imm_shift(&self) -> Option<Byte> {
        self.imm_shift
    }

    fn set_imm_shift(&mut self, imm: Word) {
        self.imm_shift = Some((imm & 0xff) as Byte);
    }

    fn get_imm(&self) -> Option<Byte> {
        self.imm
    }

    fn set_imm(&mut self, rotate: Word, imm: Word) {
        let immediate_to_rotate: Byte = (imm & 0xff) as Byte;
        // TODO: rotate
        // let rotate_amount = ((rotate as Byte) * 2) as HalfWord;
        self.imm = Some(immediate_to_rotate);
    }
}

/*
Individual type constructs
*/

pub trait TTypeData: TInstruction {
    fn get_opcode(&self) -> DataOpcode;
    fn get_s_bit(&self) -> bool;
    fn get_shift_type(&self) -> ShiftType;
}

pub struct TypeData {

}

// impl TTypeData for TypeData {

// }

pub trait TypeBranch: TInstruction {
    fn get_l_bit() -> bool;
    fn get_offset() -> Word; // (3 bytes)
}

pub trait TypeLSM: TInstruction {
    fn get_lsm_code() -> LSMCode;
    fn get_writeback() -> bool;
    fn get_ldr_str() -> bool; // is load (1) or store (0)
    fn get_s_bit() -> bool;
    fn get_reg_list() -> Vec<Register>;
}
pub trait TypeLDRSTR: TInstruction {
    fn get_addressing_mode() -> AddressingMode;
    fn get_add_sub() -> bool; // add (1) or sub (0)
    fn get_byte_word() -> bool; // byte (1) or word (0)
    fn get_ldr_str() -> bool; // load (1) or store (0)
    fn get_shift_type() -> ShiftType;
    fn store_constant(&self, constant: Word) -> AddressSize; // stores constant in constant pool, returns address of stored const
}

pub trait TypeLDRHSTRH: TypeLDRSTR {
    fn get_lsh() -> LSH;
}

pub trait TypeSWI: TInstruction {
    fn get_swi() -> Word; // (3 bytes)
}

pub trait TypeMultiply: TInstruction {
    fn get_s_bit() -> bool;
}

/*
Individual instruction factories
yay java
*/

pub fn instr_data_reg_imm(condition: Word, opcode: Word, s_bit: Word, rn: Word, rd: Word, imm: Word, shift_type: Word, rm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::DataRegImm);
    instr.set_condition(condition);
    instr.set_data_opcode(opcode);
    instr.set_s_bit(s_bit);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm_shift(imm);
    instr.set_shift_type(shift_type);
    instr.set_rm(rm);

    instr
}

pub fn instr_data_imm(condition: Word, opcode: Word, s_bit: Word, rn: Word, rd: Word, rotate: Word, imm: Word) -> Instruction {
    let mut instr = Instruction::new(InstrType::DataImm);
    instr.set_condition(condition);
    instr.set_data_opcode(opcode);
    instr.set_s_bit(s_bit);
    instr.set_rn(rn);
    instr.set_rd(rd);
    instr.set_imm(rotate, imm);

    instr
}