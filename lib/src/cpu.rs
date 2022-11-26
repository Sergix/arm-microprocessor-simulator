use log::trace;
use tauri::{AppHandle, Manager};
use bitmatch::bitmatch;
use tokio::sync::MutexGuard;

use crate::{memory::{Registers, RAM, Memory, Word, AddressSize, Byte, DISPLAY_ADDR, Register}, state::{RAMState, RegistersState, CPUThreadWatcherState, TraceFileState}, instruction::*, cpu_enum::{Mode, Condition, InstrExecuteCondition}, util};

pub struct CPUThreadWatcher {
    running: bool,
    prompt_flag: bool,
    prompt_input: String,
    irq_flag: bool,
    irq_last_char: char
}

impl CPUThreadWatcher {
    pub fn set_running(&mut self, state: bool) {
        self.running = state;
    }

    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn set_prompt_input(&mut self, value: String) {
        self.prompt_flag = true;
        self.prompt_input = value;
    }

    pub fn get_prompt_input(&self) -> String {
        self.prompt_input.clone()
    }

    pub fn get_prompt_flag(&self) -> bool {
        self.prompt_flag
    }

    pub fn clear_prompt_flag(&mut self) {
        self.prompt_flag = false;
    }

    pub fn set_irq_flag(&mut self) {
        self.irq_flag = true;
    }

    pub fn clear_irq_flag(&mut self) {
        self.irq_flag = false;
    }

    pub fn get_irq_flag(&self) -> bool {
        self.irq_flag
    }

    pub fn set_irq_last_char(&mut self, c: char) {
        self.irq_last_char = c;
    }

    pub fn get_irq_last_char(&self) -> char {
        self.irq_last_char
    }
}

impl Default for CPUThreadWatcher {
    fn default() -> Self {
        Self {
            running: false,
            prompt_flag: false,
            prompt_input: String::new(),
            irq_flag: false,
            irq_last_char: '\0'
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct CPUPayload {
    pub trace: bool,
    pub mode: Mode,
}

#[derive(Clone, serde::Serialize)]
pub struct TerminalPutcharPayload {
    pub char: char
}

#[derive(Clone, serde::Serialize)]
pub struct TerminalReadlinePayload {
    pub max_bytes: Word
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
            "11100001101000000000000000000000"              => instr_nop(),
            "cccc_00010_r_00_1111_dddd_000000000000"        => instr_mrs(c, r, d),
            "cccc_00110_r_10_ffff_1111_rrrr_iiiiiiii"       => instr_msr_imm(c, r, f, r, i),
            "cccc_00010_r_10_ffff_1111_00000000_mmmm"       => instr_msr_reg(c, r, f, m),
            "cccc_00010010_111111111111_0001_mmmm"          => instr_bx(c, m),
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
            "cccc_101_l_oooooooooooooooooooooooo"           => instr_b(c, l, o),
            "cccc_100_uuswl_nnnn_rrrrrrrrrrrrrrrr"          => instr_ldmstm(c, u, s, w, l, n, r),
            "cccc_000_0000_s_dddd_0000_ssss_1001_mmmm"      => instr_mul(c, s, d, s, m),
            "cccc_1111_ssssssssssssssssssssssss"            => instr_swi(c, s),
            "????????????????????????????????"              => instr_nop(),
        }
    }

    pub fn execute(&self, ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, instr: &mut Instruction) -> InstrExecuteCondition {
        let (n, z, c, v) = registers_lock.get_nzcv_tuple();

        // check if condition passed and that the next instructions conditions allow if it did not pass
        let exec = match instr.get_condition() {
            Condition::EQ =>     z,
            Condition::NE =>    !z,
            Condition::CSHS =>   c,
            Condition::CCLO =>  !c,
            Condition::MI =>     n,
            Condition::PL =>    !n,
            Condition::VS =>     v,
            Condition::VC =>    !v,
            Condition::HI =>     c && !z,
            Condition::LS =>    !c || z,
            Condition::GE =>    (n && v) || (!n && !v),
            Condition::LT =>    (n && !v) || (!n && v),
            Condition::GT =>    !z && n == v,
            Condition::LE =>     z || (n && !v) || (!n && v),
            Condition::AL =>    true,
        };

        if exec {
            // grab the execute method for the specific instruction and pass the state objects
            instr.get_execute()(ram_lock, registers_lock, *instr)
        } else {
            InstrExecuteCondition::NOP
        }
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

    fn putchar(&self, arg_char: Word, app_handle: AppHandle) {
        // originally, this was put here because I thought that the Rust backend
        //  was sending events too quickly to the frontend and causing the main thread
        //  to stall. I probably spent around 4 1/2 hours debugging why all the threads 
        //  were locking up, because I was sure that it wasn't my fault and that I had carefully
        //  managed all the thread locks. after doing some intensive investigation, I settled on this
        //  since it seemed to work, although definitely not ideal. Eventually, though,
        //  I discovered that the root cause of the issue was that all the logs being sent
        //  through the IPC mechanism to Edge's frontend was beginning to clog the entire thread.
        //  the bug never appeared in debug mode since the output to the stdout console delayed
        //  the thread enough so that only a few logs would be sent to Edge. surpisingly,
        //  once i disabled the log streaming functionality for release mode, everything suddenly *worked*.
        //
        //  so this is left here, as a memorial.
        //
        // thread::sleep(time::Duration::from_millis(5));

        app_handle.emit_all("terminal_append", TerminalPutcharPayload {
            char: char::from_u32(arg_char).unwrap_or('\0')
        }).unwrap();
    }

    async fn readline(&self, ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>, app_handle: AppHandle) {
        let arg_dest_addr = registers_lock.get_reg_register(Register::r1);
        let arg_max_bytes = registers_lock.get_reg_register(Register::r2);

        app_handle.emit_all("terminal_prompt", TerminalReadlinePayload {
            max_bytes: arg_max_bytes - 1 // fit null terminator
        }).unwrap();

        // wait for frontend to return
        // frontend will update CPUThreadWatcher state
        loop {
            // check thread state and drop immediately
            {
                let cpu_thread_watcher_state: CPUThreadWatcherState = app_handle.state();
                if cpu_thread_watcher_state.lock().await.get_prompt_flag() {
                    break;
                }
            }
        }

        // get thread state and drop immediately
        let mut input;
        {
            let cpu_thread_watcher_state: CPUThreadWatcherState = app_handle.state();
            let cpu_thread_watcher_lock = &mut cpu_thread_watcher_state.lock().await;
            input = cpu_thread_watcher_lock.get_prompt_input();
            cpu_thread_watcher_lock.clear_prompt_flag();
        }

        trace!("step: input received: {} {}bytes {}dest", input, arg_max_bytes, arg_dest_addr);

        // append CR if received bytes < (arg_max_bytes + \0)
        if input.len() < (arg_max_bytes - 1) as usize {
            input.push(0xd as char);
        }
        input.push('\0');

        let mut i = 0;
        for c in input.chars() {
            ram_lock.write_byte(arg_dest_addr + i, c as Byte);
            i += 1;
        }
    }
    
    pub async fn run(&mut self, app_handle: AppHandle) {

        // update thread state and drop immediately
        {
            let cpu_thread_state: CPUThreadWatcherState = app_handle.state();
            cpu_thread_state.lock().await.set_running(true);
        }
        
        // fetch-decode-execute
        loop {
            // stop when thread flag is updated
            {
                let cpu_thread_state: CPUThreadWatcherState = app_handle.state();
                if !cpu_thread_state.lock().await.is_running() { break }
            }

            trace!("run: stepping...");
            if self.step(app_handle.clone()).await == InstrExecuteCondition::HLT {
                // stop when HLT instruction or other exception
                trace!("run: hit HLT or exception");
                break
            }

            // stop when pc hits breakpoint address
            {
                let registers_state: RegistersState = app_handle.state();
                if self.is_breakpoint(&(registers_state.lock().await).get_pc_current_address()) { 
                    trace!("run: hit breakpoint");
                    break
                }
            }
        }

        self.stop(app_handle.clone()).await;

        trace!("run: cpu stopped");
    }

    // returns true if HLT
    pub async fn step(&mut self, app_handle: AppHandle) -> InstrExecuteCondition {
        let ram_state: RAMState = app_handle.state();
        let registers_state: RegistersState = app_handle.state();
        let ram_lock = &mut ram_state.lock().await;
        let registers_lock = &mut registers_state.lock().await;
        let trace_state: TraceFileState = app_handle.state();
        let trace_lock = &mut trace_state.lock().await;

        trace!("step: trace_step: {}", self.trace_step);
        trace!("step: cpsr: {}", registers_lock.get_cpsr());

        // save PC before fetch begins for logging after execute
        let saved_pc = registers_lock.get_pc_current_address();

        let instr_raw = self.fetch(ram_lock, registers_lock);
        trace!("step: {}pc = {:x}", registers_lock.get_pc_current_address(), instr_raw);

        // halt when instruction is HLT (0)
        if instr_raw == 0 {
            registers_lock.inc_pc();
            return InstrExecuteCondition::HLT;
        }

        // check irq and thread state here since the state is used in the decode and later steps, then drop lock immediately
        // CPUThreadWatcherState cannot be shared across entire function since interrupt may occur mid-step
        //  or while step is waiting for prompt event from frontend
        let irq;
        let last_char: char;
        {
            let cpu_thread_watcher_state: CPUThreadWatcherState = app_handle.state();
            irq = cpu_thread_watcher_state.lock().await.get_irq_flag();
            last_char = cpu_thread_watcher_state.lock().await.get_irq_last_char();
        }

        // get the instruction struct from the raw Word
        let mut instr: Instruction = self.decode(instr_raw);

        // inject possibly needed information into instruction before executing

        // pc for branch instructions
        instr.set_pc_address(registers_lock.get_pc());
        // manually map any display hardware events before executing instruction
        // rn = DISPLAY_ADDR
        // rd = value to write
        if util::is_write_instr(instr) && instr.get_rn().is_some() && registers_lock.get_reg_register(instr.get_rn().unwrap()) == DISPLAY_ADDR {
            let arg_char = registers_lock.get_reg_register(instr.get_rd().unwrap());
            trace!("step: mapping hardware display event, char = 0x{:x}", arg_char as Byte);

            self.putchar(arg_char, app_handle.clone());
        }
        // inject last character from keyboard event if loading from keyboard hardware address
        instr.set_last_char(last_char);

        // pass the necessary state objects and instruction struct;
        // state guards need to be passed so that the execute method can properly access/modify
        // the application state
        // exit if HLT
        trace!("step: instr = {}", instr.to_string());
        let exec_result: InstrExecuteCondition = self.execute(ram_lock, registers_lock, &mut instr);

        // increment program counter
        registers_lock.inc_pc();

        // logging: get all registers and remove r15
        let mut reg_all = registers_lock.get_all();
        reg_all.pop();
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

        // handle instruction SWI exceptions
        match exec_result {
            InstrExecuteCondition::HLT => {
                trace!("step: hit SWI HLT");
                // decrement PC to stop at HLT instruction in disassembly
                registers_lock.dec_pc();
                return InstrExecuteCondition::HLT
            },
            InstrExecuteCondition::SWI => {
                // processed here so that we can properly access the app thread
                trace!("step: processing SWI event 0x{:x}swi", instr.get_swi().unwrap());

                let cpsr = registers_lock.get_cpsr();
                let pc = registers_lock.get_pc();

                registers_lock.set_reg_register(Register::r14_svc, pc - 4);
                registers_lock.set_spsr_svc(cpsr);
                registers_lock.set_cpsr_mode(Mode::SVC);
                registers_lock.set_cpsr_flag(5, false); // ARM state
                registers_lock.set_i_flag(true);  // disable interrupts
                registers_lock.set_pc(0x08+8); // add 8 bytes for PC

                match instr.get_swi().unwrap() {
                    0x0 => {
                        let arg_char = registers_lock.get_reg_register(Register::r0);
                        self.putchar(arg_char, app_handle.clone());
                    },
                    0x6a => self.readline(ram_lock, registers_lock, app_handle.clone()).await,
                    _ => ()
                }
            },
            InstrExecuteCondition::NOP => (),
        }

        // proccess IRQ interrupt from IRQ input line
        // only when IRQ interrupts are not disabled
        if irq && !registers_lock.get_i_flag() {
            // A2.6.8
            trace!("step: received IRQ event for char {}, handling...", last_char);

            // clear the IRQ flag
            {
                let cpu_thread_watcher_state: CPUThreadWatcherState = app_handle.state();
                cpu_thread_watcher_state.lock().await.clear_irq_flag();
            }

            let pc = registers_lock.get_pc();
            let cpsr = registers_lock.get_cpsr();
            registers_lock.set_reg_register(Register::r14_irq, pc - 4);
            registers_lock.set_spsr_irq(cpsr);
            registers_lock.set_cpsr_mode(Mode::IRQ);
            registers_lock.set_i_flag(true);  // disable interrupts
            registers_lock.set_cpsr_flag(8, true);  // disable imprecise data aborts
            registers_lock.set_pc(0x18+8); // add 8 bytes for PC
        }

        return exec_result
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
    use crate::{cpu_enum::{DataOpcode, LDMCode, ShiftType, InstrType}, memory::Register};

    use super::*;

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
    fn test_decode_ldr_imm_pre() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xe5925000);
        assert_eq!(instr.get_type(), InstrType::LDRSTRImmPre);
        assert_eq!(instr.get_byte_word().unwrap(), false);
        assert_eq!(instr.get_add_sub().unwrap(), true);
        assert_eq!(instr.get_writeback().unwrap(), false);
        assert_eq!(instr.get_ldr_str().unwrap(), true);
        assert_eq!(instr.get_rd().unwrap(), Register::r5);
        assert_eq!(instr.get_rn().unwrap(), Register::r2);
        assert_eq!(instr.get_imm_shift().unwrap(), 0);
    }

    #[test]
    fn test_decode_str_imm_pre() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xe5021004);
        assert_eq!(instr.get_type(), InstrType::LDRSTRImmPre);
        assert_eq!(instr.get_byte_word().unwrap(), false);
        assert_eq!(instr.get_add_sub().unwrap(), false);
        assert_eq!(instr.get_writeback().unwrap(), false);
        assert_eq!(instr.get_ldr_str().unwrap(), false);
        assert_eq!(instr.get_rd().unwrap(), Register::r1);
        assert_eq!(instr.get_rn().unwrap(), Register::r2);
        assert_eq!(instr.get_imm_shift().unwrap(), 4);
    }

    #[test]
    fn test_decode_ldrb_imm_pre() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xe5d2a00a);
        assert_eq!(instr.get_type(), InstrType::LDRSTRImmPre);
        assert_eq!(instr.get_byte_word().unwrap(), true);
        assert_eq!(instr.get_add_sub().unwrap(), true);
        assert_eq!(instr.get_writeback().unwrap(), false);
        assert_eq!(instr.get_ldr_str().unwrap(), true);
        assert_eq!(instr.get_rd().unwrap(), Register::r10);
        assert_eq!(instr.get_rn().unwrap(), Register::r2);
        assert_eq!(instr.get_imm_shift().unwrap(), 10);
    }

    #[test]
    fn test_decode_strb_imm_pre() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xe5c2100a);
        assert_eq!(instr.get_type(), InstrType::LDRSTRImmPre);
        assert_eq!(instr.get_byte_word().unwrap(), true);
        assert_eq!(instr.get_add_sub().unwrap(), true);
        assert_eq!(instr.get_writeback().unwrap(), false);
        assert_eq!(instr.get_ldr_str().unwrap(), false);
        assert_eq!(instr.get_rd().unwrap(), Register::r1);
        assert_eq!(instr.get_rn().unwrap(), Register::r2);
        assert_eq!(instr.get_imm_shift().unwrap(), 10);
    }

    #[test]
    fn test_decode_ldr_shift_reg_pre() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xe79290c4);
        assert_eq!(instr.get_type(), InstrType::LDRSTRShiftRegPre);
        assert_eq!(instr.get_byte_word().unwrap(), false);
        assert_eq!(instr.get_add_sub().unwrap(), true);
        assert_eq!(instr.get_writeback().unwrap(), false);
        assert_eq!(instr.get_ldr_str().unwrap(), true);
        assert_eq!(instr.get_rd().unwrap(), Register::r9);
        assert_eq!(instr.get_rn().unwrap(), Register::r2);
        assert_eq!(instr.get_rm().unwrap(), Register::r4);
        assert_eq!(instr.get_shift_type().unwrap(), ShiftType::ASR);
        assert_eq!(instr.get_imm_shift().unwrap(), 1);
    }

    #[test]
    fn test_decode_str_shift_reg_pre() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xe78210c4);
        assert_eq!(instr.get_type(), InstrType::LDRSTRShiftRegPre);
        assert_eq!(instr.get_byte_word().unwrap(), false);
        assert_eq!(instr.get_add_sub().unwrap(), true);
        assert_eq!(instr.get_writeback().unwrap(), false);
        assert_eq!(instr.get_ldr_str().unwrap(), false);
        assert_eq!(instr.get_rd().unwrap(), Register::r1);
        assert_eq!(instr.get_rn().unwrap(), Register::r2);
        assert_eq!(instr.get_rm().unwrap(), Register::r4);
        assert_eq!(instr.get_shift_type().unwrap(), ShiftType::ASR);
        assert_eq!(instr.get_imm_shift().unwrap(), 1);
    }

    #[test]
    fn test_decode_ldm() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xe8bd000a);
        assert_eq!(instr.get_type(), InstrType::LDMSTM);
        assert_eq!(instr.get_ldm().unwrap(), LDMCode::IncAfter);
        assert_eq!(instr.get_s_bit().unwrap(), false);
        assert_eq!(instr.get_writeback().unwrap(), true);
        assert_eq!(instr.get_ldr_str().unwrap(), true);
        assert_eq!(instr.get_rn().unwrap(), Register::r13);
        assert_eq!(instr.get_reg_list().unwrap(), 0b1010);
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
    fn test_decode_mul() {
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
    fn test_decode_branch() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xeb000006);
        assert_eq!(instr.get_type(), InstrType::B);
        assert_eq!(instr.get_l_bit().unwrap(), true);
        assert_eq!(instr.get_offset().unwrap(), 32);
    }

    #[test]
    fn test_decode_swi() {
        let cpu = CPU::default();
        let instr = cpu.decode(0xef000011);
        assert_eq!(instr.get_type(), InstrType::SWI);
        assert_eq!(instr.get_swi().unwrap(), 17);
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