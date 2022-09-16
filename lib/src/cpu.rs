use core::time;
use std::{thread::{self}};

use log::trace;
use tauri::{AppHandle, Manager};
use tokio::sync::MutexGuard;

use crate::{memory::{Registers, RAM, Memory, Word, AddressSize}, state::{RAMState, RegistersState, CPUThreadWatcherState}};

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

pub struct CPU {
    breakpoints: Vec<AddressSize>
}

impl CPU {
    pub fn new() -> Self {
        Self {
            breakpoints: vec![0; 0]
        }
    }

    pub async fn stop(&self, app_handle: AppHandle) {
        let cpu_thread_state: CPUThreadWatcherState = app_handle.state();
        cpu_thread_state.lock().await.set_running(false);
        trace!("stop: set running flag to false")
    }

    // TODO: add tests
    pub fn fetch(&self, ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>) -> Word {
        // return read word from RAM address specified by value of PC register
        ram_lock.read_word(registers_lock.get_pc())
    }

    // TODO: add tests
    pub fn decode(&self) {
        // do nothing
    }

    // TODO: add tests
    pub fn execute(&self) {
        // pause for 1/4 sec
        thread::sleep(time::Duration::from_millis(250))
    }

    // TODO: add tests
    pub fn add_breakpoint(&mut self, address: AddressSize) {
        trace!("add_breakpoint: {}", address);
        self.breakpoints.push(address)
    }

    // TODO: add tests
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
            // stop when running flag is updated
            let cpu_thread_state: CPUThreadWatcherState = app_handle.state();
            if !cpu_thread_state.lock().await.is_running() { break }

            // stop when HLT instruction is reached
            if self.step(app_handle.clone()).await { break }
        }

        trace!("run: cpu stopped");
    }

    // returns true if HLT
    pub async fn step(&mut self, app_handle: AppHandle) -> bool {
        let ram_state: RAMState = app_handle.state();
        let registers_state: RegistersState = app_handle.state();
        let ram_lock = &mut ram_state.lock().await;
        let registers_lock = &mut registers_state.lock().await;

        // stop when pc hits breakpoint address
        if self.is_breakpoint(&registers_lock.get_pc()) { self.stop(app_handle.clone()).await; return true }
        
        let instruction = self.fetch(ram_lock, registers_lock);
        trace!("step: {}pc = {}", registers_lock.get_pc(), instruction);

        // halt when instruction is HLT
        if instruction == 0 { self.stop(app_handle.clone()).await; return true }

        self.decode();
        self.execute();

        // increment program counter
        registers_lock.inc_pc();

        return false;
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            breakpoints: vec![0; 0]
        }
    }
}