use std::{fs::{ File, self }, io::Write};
use log::trace;

use crate::{memory::{Checksum, Word, Byte}, cpu_enum::Mode};

const TRACE_LOG_FILENAME: &str = "trace.log";

pub struct TraceFile {
    f: Option<File>,
    traceall: bool
}

impl TraceFile {
    pub fn set_traceall(&mut self) {
        self.traceall = true;
    }

    pub fn clear_trace_file(&self) -> Result<(), std::io::Error> {
        fs::write(TRACE_LOG_FILENAME, "")
    }

    pub fn open_trace_file(&mut self) -> Option<std::io::Error> {
        trace!("open_trace_file: opening trace file");
        
        match File::options().append(true).open(TRACE_LOG_FILENAME) {
            Ok(f) => { self.f = Some(f); None },
            Err(e) => Some(e)
        }
    }
    
    pub fn close_trace_file(&mut self) {
        trace!("close_trace_file: closing trace file");
        drop(self.f.as_ref().unwrap());
        self.f = None;
    }
    
    pub fn append_trace_file_line(&mut self, trace_step: Word, pc: Word, checksum: Checksum, n: Byte, c: Byte, z: Byte, v: Byte, mode: Mode, regs: Vec<Word>) { 
        if self.f.is_none() {
            return
        }

        // if --traceall is disabled, only log SYS instructions
        if !self.traceall && mode != Mode::SYS {
            return
        }
        
        trace!("append_trace_file: adding trace file entry");

        // step_number program_counter checksum nzcv mode r0 r1 r2 r3 r4 r5 r6 r7 r8 r9 r10 r11 r12 r13 r14
        let mut i = 0;

        let regs_string = regs.iter()
            .map(|val| (format!("{}={:08X}", i, val), i += 1))
            .map(|tuple| tuple.0)
            .collect::<Vec<String>>()
            .join(" ");

        writeln!(self.f.as_ref().unwrap(), "{:06} {:08X} {:08X} {}{}{}{} {} {} ", trace_step, pc, checksum, n, c, z, v, mode.to_string(), regs_string).unwrap();

        self.f.as_ref().unwrap().flush().unwrap();
    }
}

impl Default for TraceFile {
    fn default() -> Self {
        Self { f: None, traceall: false }
    }
}