use log::trace;
use tauri::{AppHandle, Manager};
use bitmatch::bitmatch;
use tokio::sync::MutexGuard;

use crate::{memory::{Registers, RAM, Memory, Word, AddressSize, Byte}, state::{RAMState, RegistersState, CPUThreadWatcherState, TraceFileState}, instruction::*, cpu_enum::{InstrType, Mode}};

pub struct CPUThreadWatcher {
    running: bool
}

impl CPUThreadWatcher {
    pub fn set_running(&mut self, state: bool) {
        self.running = state;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }
}

impl Default for CPUThreadWatcher {
    fn default() -> Self {
        Self {
            running: false
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct CPUPayload {
    pub trace: bool
}

pub struct CPU {
    breakpoints: Vec<AddressSize>,
    trace: bool,
    trace_step: Word
}

impl CPU {
    pub fn new() -> Self {
        Self {
            breakpoints: vec![0; 0],
            trace: false,
            trace_step: 1
        }
    }

    pub fn get_trace(&self) -> bool {
        self.trace
    }

    pub fn toggle_trace(&mut self) -> bool {
        self.trace = !self.trace;

        self.trace
    }

    pub fn reset_trace_step(&mut self) {
        self.trace_step = 1
    }

    pub async fn stop(&self, app_handle: AppHandle) {
        let cpu_thread_state: CPUThreadWatcherState = app_handle.state();
        cpu_thread_state.lock().await.set_running(false);
        trace!("stop: set running flag to false")
    }

    pub fn fetch(&self, ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>) -> Word {
        // return read word from RAM address specified by value of PC register
        ram_lock.read_word(registers_lock.get_pc_current_address())
    }

    #[bitmatch]
    pub fn decode(&self, instr: Word) -> Instruction {
        // get instruction data from bits
        // the bitmatcher matches a bit pattern to a specific instruction factory
        #[bitmatch]
        match instr {
            "cccc_000_oooo_s_nnnn_dddd_iiiii_tt_0_mmmm"     => instr_data_reg_imm(c, o, s, n, d, i, t, m),
            "cccc_000_oooo_s_nnnn_dddd_ssss_0_tt_1_mmmm"    => instr_data_reg_reg(c, o, s, n, d, s, t, m),
            "cccc_001_oooo_s_nnnn_dddd_rrrr_iiiiiiii"       => instr_data_imm(c, o, s, n, d, r, i),
            "cccc_011_1_ubwl_nnnn_dddd_iiiii_tt_0_mmmm"     => instr_ldrstr_shifted_reg_pre(c, u, b, w, l, n, d, i, t, m),
            "cccc_011_0_ubwl_nnnn_dddd_iiiii_tt_0_mmmm"     => instr_ldrstr_shifted_reg_post(c, u, b, w, l, n, d, i, t, m),
            "cccc_011_1_ubwl_nnnn_dddd_00000000_mmmm"       => instr_ldrstr_reg_pre(c, u, b, w, l, n, d, m),
            "cccc_011_0_ubwl_nnnn_dddd_00000000_mmmm"       => instr_ldrstr_reg_post(c, u, b, w, l, n, d, m),
            "cccc_010_1_ubwl_nnnn_dddd_ssssssssssss"        => instr_ldrstr_imm_pre(c, u, b, w, l, n, d, s),
            "cccc_010_0_ubwl_nnnn_dddd_ssssssssssss"        => instr_ldrstr_imm_post(c, u, b, w, l, n, d, s),
            "cccc_000_1_u1wl_nnnn_dddd_hhhh_1_ss_1_iiii"    => instr_ldrhstrh_imm_pre(c, u, w, l, n, d, h, s, i),
            "cccc_000_0_u1wl_nnnn_dddd_hhhh_1_ss_1_iiii"    => instr_ldrhstrh_imm_post(c, u, w, l, n, d, h, s, i),
            "cccc_000_1_u0wl_nnnn_dddd_0000_1_ss_1_mmmm"    => instr_ldrhstrh_reg_pre(c, u, w, l, n, d, s, m),
            "cccc_000_0_u0wl_nnnn_dddd_0000_1_ss_1_mmmm"    => instr_ldrhstrh_reg_post(c, u, w, l, n, d, s, m),
            "cccc_101_l_oooooooooooooooooooooooo"           => instr_branch(c, l, o),
            "cccc_100_uuswl_nnnn_rrrrrrrrrrrrrrrr"          => instr_ldmstm(c, u, s, w, l, n, r),
            "cccc_000_0000_s_dddd_0000_ssss_1001_mmmm"      => instr_mul(c, s, d, s, m),
            "cccc_1111_ssssssssssssssssssssssss"            => instr_swi(c, s),
            "????????????????????????????????"              => Instruction::new(InstrType::NOP)
        }
    }

    pub fn execute(&self, ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: Instruction) -> Word {
        // TODO: check if condition passed and that the next instructions conditions allow if it did not pass

        // grab the execute method for the specific instruction and pass the state objects
        instr.get_execute()(ram_lock, registers_lock, instr)
    }

    // will be called by SWI instruction
    pub fn interrupt(&self) {
        todo!();
    }

    pub fn add_breakpoint(&mut self, address: AddressSize) {
        trace!("add_breakpoint: {}", address);
        self.breakpoints.push(address)
    }

    pub fn remove_breakpoint(&mut self, address: AddressSize) {
        trace!("add_breakpoint: {}", address);
        // https://stackoverflow.com/a/26243276
        let index = self.breakpoints.iter().position(|breakpoint| *breakpoint == address).unwrap();
        self.breakpoints.remove(index);
    }

    pub fn is_breakpoint(&self, address: &AddressSize) -> bool {
        self.breakpoints.contains(&address)
    }
    
    pub async fn run(&mut self, app_handle: AppHandle) {
        

        // update thread state and drop immediately
        let cpu_thread_state: CPUThreadWatcherState = app_handle.state();
        cpu_thread_state.lock().await.set_running(true);
        
        trace!("run: stepping into cycle");
        // fetch-decode-execute
        loop {
            // stop when thread flag is updated
            let cpu_thread_state: CPUThreadWatcherState = app_handle.state();
            if !cpu_thread_state.lock().await.is_running() { break }

            // stop when HLT instruction or breakpoint is reached
            if self.step(app_handle.clone()).await { break }

            // pause for 1/4 sec
            // thread::sleep(time::Duration::from_millis(250));
        }

        trace!("run: cpu stopped");
    }

    // returns true if HLT
    pub async fn step(&mut self, app_handle: AppHandle) -> bool {
        let ram_state: RAMState = app_handle.state();
        let registers_state: RegistersState = app_handle.state();
        let ram_lock = &mut ram_state.lock().await;
        let registers_lock = &mut registers_state.lock().await;
        let trace_state: TraceFileState = app_handle.state();
        let trace_lock = &mut trace_state.lock().await;

        trace!("step: trace_step: {}", self.trace_step);
        trace!("step: cpsr: {}", registers_lock.get_cpsr());

        // save PC for trace log
        let saved_pc = registers_lock.get_pc_current_address();

        // stop when pc hits breakpoint address
        if self.is_breakpoint(&registers_lock.get_pc_current_address()) { 
            trace!("step: hit breakpoint");
            self.stop(app_handle.clone()).await;
            return true
        }

        let instr_raw = self.fetch(ram_lock, registers_lock);
        trace!("step: {}pc = {:x}", registers_lock.get_pc_current_address(), instr_raw);

        // halt when instruction is HLT
        if instr_raw == 0 { self.stop(app_handle.clone()).await; return true }

        // get the instruction struct from the raw Word
        let instr: Instruction = self.decode(instr_raw);

        trace!("step: instr = {}", instr.to_string());

        // pass the necessary state objects and instruction struct
        // state guards need to be passed so that the execute method can properly access/modify
        // the application state
        self.execute(ram_lock, registers_lock, instr);

        // increment program counter
        registers_lock.inc_pc();

        // get all registers and remove r15
        let mut reg_all = registers_lock.get_all();
        reg_all.pop();

        // update trace log and step counter
        trace_lock.append_trace_file_line(
            self.trace_step,
            saved_pc,
            ram_lock.get_checksum(),
            registers_lock.get_n_flag() as Byte,
            registers_lock.get_z_flag() as Byte,
            registers_lock.get_c_flag() as Byte,
            registers_lock.get_v_flag() as Byte,
            registers_lock.get_cpsr_mode(),
            reg_all
        );
        self.trace_step += 1;

        // halt if SWI instruction was executed (check if Supervisor mode)
        // TODO: remove in Phase 4
        if registers_lock.get_cpsr_mode() == Mode::Supervisor {
            self.stop(app_handle.clone()).await;
            return true;
        }

        return false;
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            breakpoints: vec![0; 0],
            trace: false,
            trace_step: 1
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{cpu_enum::{DataOpcode, LDMCode}, memory::Register};

    use super::*;
    
    #[test]
    fn test_fetch() {
        // let mut cpu = CPU::default();
    }

    #[test]
    fn test_decode_mov() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xe3a02030);
        assert_eq!(instr.get_type(), InstrType::DataImm);
        assert_eq!(instr.get_data_opcode().unwrap(), DataOpcode::MOV);
        assert!(!instr.get_s_bit().unwrap());
        assert_eq!(instr.get_rn().unwrap(), Register::r0);
        assert_eq!(instr.get_rd().unwrap(), Register::r2);
        assert_eq!(instr.get_imm().unwrap(), 48);
    }

    #[test]
    fn test_decode_stm() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xe92d0016);
        assert_eq!(instr.get_type(), InstrType::LDMSTM);
        assert_eq!(instr.get_ldm().unwrap(), LDMCode::DecBefore);
        assert_eq!(instr.get_s_bit().unwrap(), false);
        assert_eq!(instr.get_writeback().unwrap(), true);
        assert_eq!(instr.get_ldr_str().unwrap(), false);
        assert_eq!(instr.get_rn().unwrap(), Register::r13);
        assert_eq!(instr.get_reg_list().unwrap(), 0b10110);
    }

    #[test]
    fn test_execute() {
        // let cpu = CPU::default();
    }

    #[test]
    fn test_run() {
        // let cpu = CPU::default();
    }

    #[test]
    fn test_step() {
        // let cpu = CPU::default();
    }

    #[test]
    fn test_add_breakpoint() {
        let mut cpu = CPU::default();
        cpu.add_breakpoint(1);
        assert_eq!(1, cpu.breakpoints[0])
    }

    #[test]
    fn test_is_breakpoint() {
        let mut cpu = CPU::default();
        cpu.breakpoints.push(1);
        assert_eq!(true, cpu.is_breakpoint(&1))
    }

    #[test]
    fn test_remove_breakpoint() {
        let mut cpu = CPU::default();
        cpu.breakpoints.push(1);
        cpu.breakpoints.push(2);
        cpu.breakpoints.push(3);
        cpu.remove_breakpoint(2);
        assert_eq!(false, cpu.is_breakpoint(&2))
    }
}