use std::fmt;

use log::trace;

use crate::{cpu_enum::{Condition, ShiftType, LDMCode, InstrType, DataOpcode}, memory::{Word, Register}, instruction::{Instruction, TInstruction}, util};

fn get_s_bit_str(s_bit: bool) -> String {
    match s_bit {
        true => "s".to_string(),
        false => "".to_string()
    }
}

fn get_l_bit_str(l_bit: bool) -> String {
    match l_bit {
        true => "l".to_string(),
        false => "".to_string()
    }
}

fn get_condition_str(condition: Condition) -> String {
    match condition {
        Condition::AL => "".to_string(),
        _ => condition.to_string().to_lowercase()
    }
}

fn get_byte_word_str(op: bool) -> String {
    match op {
        true => "b".to_string(),
        false => "".to_string()
    }
}

fn get_ldr_str_str(op: bool) -> String {
    match op {
        true => "ldr".to_string(),
        false => "str".to_string()
    }
}

fn get_ldm_stm_str(op: bool) -> String {
    match op {
        true => "ldm".to_string(),
        false => "stm".to_string()
    }
}

fn get_writeback_str(op: bool) -> String {
    match op {
        true => "!".to_string(),
        false => "".to_string()
    }
}

// add is true and sub is false
fn get_imm_sign_str(imm: Word, add: bool) -> String {
    match add {
        true => format!("#{}", imm),
        false => format!("#-{}", imm)
    }
}

fn get_rm_sign_str(rm: Register, add: bool) -> String {
    match add {
        true => format!("{}", rm.to_string()),
        false => format!("-{}", rm.to_string())
    }
}

fn get_shift_str(shift_type: ShiftType, imm: Word) -> String {
    // TODO: RRX shift? p.462
    format!("{} #{}",
        shift_type.to_string().to_lowercase(),
        imm
    )
}

fn get_reg_list_str(reg_list: Word) -> String {
    let mut regs: Vec<String> = Vec::new();

    for ri in 0..=15 {
        if (reg_list >> ri) & 0b1 == 1 {
            let r: Register = num::FromPrimitive::from_u32(ri).unwrap();
            regs.push(r.to_string());
        }
    }

    regs.join(", ")
}

fn get_ldm_code_str(ldm: LDMCode) -> String {
    match ldm {
        LDMCode::DecAfter => "da".to_string(),
        LDMCode::IncAfter => "ia".to_string(),
        LDMCode::DecBefore => "db".to_string(),
        LDMCode::IncBefore => "ib".to_string(),
    }
}

fn get_cpsr_spsr_str(gpregister: bool) -> String {
    match gpregister {
        true => "SPSR".to_string(),
        false => "CPSR".to_string()
    }
}

fn get_fields_str(field_mask: Word) -> String {
    let mut str = "".to_string();
    if util::test_bit(field_mask, 3) {
        str.push('f');
    }
    if util::test_bit(field_mask, 2) {
        str.push('s');
    }
    if util::test_bit(field_mask, 1) {
        str.push('x');
    }
    if util::test_bit(field_mask, 0) {
        str.push('c');
    }
    str
}

// formatted output for the instructions
// used for disassembly
impl fmt::Display for Instruction {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self.get_type() {
            InstrType::NOP => {
                fmt.write_str("nop")?;
            },
            InstrType::DataImm => {
                // mov rd, #imm8
                // optional operand2
                let rn = match self.get_data_opcode().unwrap() {
                    DataOpcode::MOV | DataOpcode::MVN => "".to_string(), 
                    _ => match self.get_rn() {
                        Some(r) => format!("{},", r.to_string()),
                        None => "".to_string(),
                    },
                };

                let (shifter_operand, _shifter_carry_out) = Instruction::rotate_value(
                    self.get_rotate().unwrap(),
                    self.get_imm().unwrap_or(0),
                    0);

                // for each instruction, produce the opcode
                // and all information associated with it
                fmt.write_str(
                    format!(
                        "{}{}{} {}, {} #{}",
                        self.get_data_opcode().unwrap().to_string().to_lowercase(),
                        get_condition_str(self.get_condition()),
                        get_s_bit_str(self.get_s_bit().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        rn,
                        shifter_operand
                    ).as_str()
                )?;
            },
            InstrType::DataRegImm => {
                // TODO: ignore rn if mov or mvn
                // TODO: refactor the data disassemblies

                // optional rn
                let rn = match self.get_data_opcode().unwrap() {
                    DataOpcode::MOV | DataOpcode::MVN => "".to_string(), 
                    _ => match self.get_rn() {
                        Some(r) => format!("{},", r.to_string()),
                        None => "".to_string(),
                    },
                };
                
                // optional operand2
                // TODO: ignore if zero shift
                let operand2 = match self.get_imm_shift() {
                    Some(_) => format!("{} #{}", self.get_shift_type().unwrap().to_string().to_lowercase(), self.get_imm_shift().unwrap().to_string()),
                    None => "".to_string(),
                };
                
                fmt.write_str(
                    format!(
                        "{}{}{} {}, {} {} {}",
                        self.get_data_opcode().unwrap().to_string().to_lowercase(),
                        get_condition_str(self.get_condition()),
                        get_s_bit_str(self.get_s_bit().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        rn,
                        self.get_rm().unwrap().to_string(),
                        operand2
                    ).as_str()
                )?;
            },
            InstrType::DataRegReg => {
                // add rd, rm, sh rs
                // optional rn
                let rn = match self.get_data_opcode().unwrap() {
                    DataOpcode::MOV | DataOpcode::MVN => "".to_string(), 
                    _ => match self.get_rn() {
                        Some(r) => format!("{},", r.to_string()),
                        None => "".to_string(),
                    }
                };
                
                // optional operand2
                let operand2 = match self.get_imm_shift() {
                    Some(_) => format!("{} {}", self.get_shift_type().unwrap().to_string().to_lowercase(), self.get_rs().unwrap().to_string()),
                    None => "".to_string(),
                };
                

                fmt.write_str(
                    format!(
                        "{}{}{} {}, {} {}, {}",
                        self.get_data_opcode().unwrap().to_string().to_lowercase(),
                        get_condition_str(self.get_condition()),
                        get_s_bit_str(self.get_s_bit().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        rn,
                        self.get_rm().unwrap().to_string(),
                        operand2
                    ).as_str()
                )?;
            },
            InstrType::LDRSTRShiftRegPre => {
                // ex: ldralb  rd, [rn, rm, lsl -#8]!
                //     {}{}{} {}, [{}, {}, {}]{}

                fmt.write_str(
                    format!(
                        "{}{}{} {}, [{}, {}, {}]{}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_byte_word_str(self.get_byte_word().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_rm_sign_str(self.get_rm().unwrap(), self.get_add_sub().unwrap()),
                        get_shift_str(self.get_shift_type().unwrap(), self.get_imm_shift().unwrap()),
                        get_writeback_str(self.get_writeback().unwrap())
                    ).as_str()
                )?;
            },
            InstrType::LDRSTRImmPre => {
                // ex: strb rd,  [rn, #8]!
                //     {}{} {},  {{}, {}}{}

                fmt.write_str(
                    format!(
                        "{}{}{} {}, [{}, {}]{}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_byte_word_str(self.get_byte_word().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_imm_sign_str(self.get_imm_shift().unwrap(), self.get_add_sub().unwrap()),
                        get_writeback_str(self.get_writeback().unwrap())
                    ).as_str()
                )?;
            },
            InstrType::LDRSTRImmPost => {
                // ex: stralb rd,  [rn], #-8
                //     {}{}{} {},   {},  {}

                fmt.write_str(
                    format!(
                        "{}{}{} {}, [{}], {}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_byte_word_str(self.get_byte_word().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_imm_sign_str(self.get_imm_shift().unwrap(), self.get_add_sub().unwrap())
                    ).as_str()
                )?;
            },
            InstrType::B => {
                // ex: ball #20
                //     {}{} {} 

                let target_address = self.get_pc_address() as i32 + self.get_offset().unwrap();

                fmt.write_str(
                    format!(
                        "b{}{} {:X}",
                        get_condition_str(self.get_condition()),
                        get_l_bit_str(self.get_l_bit().unwrap()),
                        target_address
                    ).as_str()
                )?;
            },
            InstrType::BX => {
                // ex: bxal r4
                //       {} {} 

                fmt.write_str(
                    format!(
                        "bx{} {}",
                        get_condition_str(self.get_condition()),
                        self.get_rm().unwrap().to_string()
                    ).as_str()
                )?;
            },
            InstrType::LDRSTRShiftRegPost => {
                // ex: ldralb  rd, [rn], rm, lsl -#8
                //     {}{}{} {}, [{}], {},  {}

                fmt.write_str(
                    format!(
                        "{}{}{} {}, [{}], {}, {}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_byte_word_str(self.get_byte_word().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_rm_sign_str(self.get_rm().unwrap(), self.get_add_sub().unwrap()),
                        get_shift_str(self.get_shift_type().unwrap(), self.get_imm_shift().unwrap()),
                    ).as_str()
                )?;
            },
            InstrType::LDRSTRRegPost => {
                // ex: ldralb  rd, [rn], rm
                //     {}{}{} {}, [{}], {}

                fmt.write_str(
                    format!(
                        "{}{}{} {}, [{}], {}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_byte_word_str(self.get_byte_word().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_rm_sign_str(self.get_rm().unwrap(), self.get_add_sub().unwrap()),
                    ).as_str()
                )?;
            },
            InstrType::LDRSTRRegPre => {
                // ex: ldralb  rd, [rn, rm]!
                //     {}{}{} {}, [{}, {}]{}

                fmt.write_str(
                    format!(
                        "{}{}{} {}, [{}, {}]{}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        get_byte_word_str(self.get_byte_word().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_rm_sign_str(self.get_rm().unwrap(), self.get_add_sub().unwrap()),
                        get_writeback_str(self.get_writeback().unwrap())
                    ).as_str()
                )?;
            },
            InstrType::LDRHSTRHImmPre => {
                // ex: ldralh rd, [rn, #n]!
                //     {}{}   {}, [{}, {}]{}

                fmt.write_str(
                    format!(
                        "{}{}h {}, [{}, {}]{}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_imm_sign_str(self.get_imm().unwrap() as Word, self.get_add_sub().unwrap()),
                        get_writeback_str(self.get_writeback().unwrap())
                    ).as_str()
                )?;
            },
            InstrType::LDRHSTRHImmPost => {
                // ex: ldralh rd, [rn], #n
                //     {}{}   {}, [{}], {}

                fmt.write_str(
                    format!(
                        "{}{}h {}, [{}], {}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_imm_sign_str(self.get_imm().unwrap() as Word, self.get_add_sub().unwrap()),
                    ).as_str()
                )?;
            },
            InstrType::LDRHSTRHRegPost => {
                // ex: ldralh rd, [rn], rm
                //     {}{}   {}, [{}], {}

                fmt.write_str(
                    format!(
                        "{}{}h {}, [{}], {}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_rm_sign_str(self.get_rm().unwrap(), self.get_add_sub().unwrap()),
                    ).as_str()
                )?;
            },
            InstrType::LDRHSTRHRegPre => {
                // ex: ldralh rd, [rn, rm]!
                //     {}{}   {}, [{}, {}]{}

                fmt.write_str(
                    format!(
                        "{}{}h {}, [{}, {}]{}",
                        get_ldr_str_str(self.get_ldr_str().unwrap()),
                        get_condition_str(self.get_condition()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rn().unwrap().to_string(),
                        get_rm_sign_str(self.get_rm().unwrap(), self.get_add_sub().unwrap()),
                        get_writeback_str(self.get_writeback().unwrap())
                    ).as_str()
                )?;
            },
            InstrType::LDMSTM => {
                // ex: ldmalda rn! , {r1, r2, r5}
                //     {} {}{} {}{},      {}

                if self.get_ldr_str().unwrap()
                    && self.get_ldm().unwrap() == LDMCode::IncAfter
                    && self.get_rn().unwrap() == Register::r13
                    && self.get_writeback().unwrap() { // pop
                    fmt.write_str(
                        format!(
                            "pop{} {{{}}}",
                            get_condition_str(self.get_condition()),
                            get_reg_list_str(self.get_reg_list().unwrap()),
                        ).as_str()
                    )?;
                } else if !self.get_ldr_str().unwrap()
                       && self.get_ldm().unwrap() == LDMCode::DecBefore
                       && self.get_rn().unwrap() == Register::r13
                       && self.get_writeback().unwrap() { // push
                    fmt.write_str(
                        format!(
                            "push{} {{{}}}",
                            get_condition_str(self.get_condition()),
                            get_reg_list_str(self.get_reg_list().unwrap()),
                        ).as_str()
                    )?;
                } else {
                    fmt.write_str(
                        format!(
                            "{}{}{} {}{}, {{{}}}",
                            get_ldm_stm_str(self.get_ldr_str().unwrap()),
                            get_condition_str(self.get_condition()),
                            get_ldm_code_str(self.get_ldm().unwrap()),
                            self.get_rn().unwrap().to_string(),
                            get_writeback_str(self.get_writeback().unwrap()),
                            get_reg_list_str(self.get_reg_list().unwrap()),
                        ).as_str()
                    )?;
                }
            },
            InstrType::MUL => {
                // ex: mulals  rd, rm, rs
                //        {}{} {}, {}, {}

                fmt.write_str(
                    format!(
                        "mul{}{} {}, {}, {}",
                        get_condition_str(self.get_condition()),
                        get_s_bit_str(self.get_s_bit().unwrap()),
                        self.get_rd().unwrap().to_string(),
                        self.get_rm().unwrap().to_string(),
                        self.get_rs().unwrap().to_string(),
                    ).as_str()
                )?;
            },
            InstrType::SWI => {
                // ex: swial imm
                //        {} {}

                fmt.write_str(
                    format!(
                        "swi{} {}",
                        get_condition_str(self.get_condition()),
                        self.get_swi().unwrap().to_string(),
                    ).as_str()
                )?;
            },
            InstrType::MSRImm => {
                // ex: msral CPSR_fields #imm
                //        {} {}     {}    {}

                fmt.write_str(
                    format!(
                        "msr{} {}_{} {}",
                        get_condition_str(self.get_condition()),
                        get_cpsr_spsr_str(self.get_gpregister().unwrap()),
                        get_fields_str(self.get_field_mask().unwrap() as Word),
                        get_imm_sign_str(self.get_imm().unwrap() as Word, true)
                    ).as_str()
                )?;
            },
            InstrType::MSRReg => {
                // ex: msral CPSR_fc, rm
                //        {} {}   {}  {}

                fmt.write_str(
                    format!(
                        "msr{} {}_{}, {}",
                        get_condition_str(self.get_condition()),
                        get_cpsr_spsr_str(self.get_gpregister().unwrap()),
                        get_fields_str(self.get_field_mask().unwrap() as Word),
                        self.get_rm().unwrap().to_string()
                    ).as_str()
                )?;
            },
            InstrType::MRS => {
                // ex: mrsal rd, CPSR
                //        {} {}   {}

                fmt.write_str(
                    format!(
                        "mrs{} {}, {}",
                        get_condition_str(self.get_condition()),
                        self.get_rd().unwrap().to_string(),
                        get_cpsr_spsr_str(self.get_gpregister().unwrap())
                    ).as_str()
                )?;
            },
        };
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_reg_list_str() {
        assert_eq!(get_reg_list_str(0b10110), "r1, r2, r4");
    }
}