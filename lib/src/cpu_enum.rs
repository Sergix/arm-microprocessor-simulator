use std::fmt;

use num_derive::FromPrimitive;

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InstrType {
    DataRegImm,
    DataRegReg,
    DataImm,
    LDRSTRShiftRegPre,
    LDRSTRShiftRegPost,
    LDRSTRRegPre,
    LDRSTRRegPost,
    LDRSTRImmPre,
    LDRSTRImmPost,
    LDRHSTRHImmPre,
    LDRHSTRHImmPost,
    LDRHSTRHRegPre,
    LDRHSTRHRegPost,
    LDMSTM,
    B,
    SWI,
    MUL,
    NOP,
}

#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
pub enum ShiftType {
    LSL = 0,
    LSR = 1,
    ASR = 2,
    ROR = 3,
    // RRX = 3
}

impl fmt::Display for ShiftType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
pub enum LDMCode {
    DecAfter = 0,
    IncAfter = 1,
    DecBefore = 2,
    IncBefore = 3
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum AddressingMode {
    PreIndex,
    PostIndex,
    PreIndexWriteback
}

#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
pub enum DataOpcode {
    AND = 0,
    EOR = 1,
    SUB = 2,
    RSB = 3,
    ADD = 4,
    ADC = 5,
    SBC = 6,
    RSC = 7,
    TST = 8,
    TEQ = 9,
    CMP = 10,
    CMN = 11,
    ORR = 12,
    MOV = 13,
    BIC = 14,
    MVN = 15
}

impl fmt::Display for DataOpcode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
pub enum LSH {
    StrHalfWord = 1,
    LdrDoubleWord = 2,
    StrDoubleWord = 3,
    LdrUHalfWord = 5,
    LdrSByte = 6,
    LdrSHalfWord = 7
}

#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
pub enum Condition {
    EQ = 0,
    NE = 1,
    CSHS = 2,
    CCLO = 3,
    MI = 4,
    PL = 5,
    VS = 6,
    VC = 7,
    HI = 8,
    LS = 9,
    GE = 10,
    LT = 11,
    GT = 12,
    LE = 13,
    AL = 14
}

impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Copy, Clone, FromPrimitive, PartialEq, Debug)]
pub enum Mode {
    User       = 0b10000,
    FIQ        = 0b10001,
    IRQ        = 0b10010,
    Supervisor = 0b10011,
    Abort      = 0b10111,
    Undefined  = 0b11011,
    System     = 0b11111
}