use core::time;
use std::{thread::{self}};

use log::trace;
use tauri::{async_runtime::{Mutex}, AppHandle, Manager};
use tokio::sync::MutexGuard;

use crate::{memory::{Registers, RAM, Memory, Word, AddressSize}, state::{RAMState, RegistersState}};

pub struct CPU {
    breakpoints: Vec<AddressSize>
}

impl CPU {
    pub fn new() -> Self {
        Self {
            breakpoints: vec![0; 0]
        }
    }

    pub fn fetch(&self, ram_lock: &mut MutexGuard<'_, RAM>, registers_lock: &mut MutexGuard<'_, Registers>) -> Word {
        // return read word from RAM address specified by value of PC register
        ram_lock.read_word(registers_lock.get_pc())
    }

    pub fn decode(&self) {
        // do nothing
    }

    pub fn execute(&self) {
        // pause for 1/4 sec
        thread::sleep(time::Duration::from_millis(250))
    }

    pub fn add_breakpoint(&mut self, address: AddressSize) {
        trace!("add_breakpoint");
        self.breakpoints.push(address)
    }

    pub fn is_breakpoint(&self, address: &AddressSize) -> bool {
        self.breakpoints.contains(&address)
    }
    
    pub async fn run(&self, app_handle: AppHandle) {
        // fetch-decode-execute
        let ram_state: RAMState = app_handle.state();
        let registers_state: RegistersState = app_handle.state();
        let ram_lock = &mut ram_state.lock().await;
        let registers_lock = &mut registers_state.lock().await;

        loop {
            // TODO: stop when "CPU.stopped" flag is updated externally?

            // stop when pc hits breakpoint address
            if self.is_breakpoint(&registers_lock.get_pc()) { break }

            let instruction = self.fetch(ram_lock, registers_lock);
            // stop when instruction is 0
            if instruction == 0 { break }

            self.decode();
            self.execute();
        }
    }

    pub async fn step(&self, app_handle: AppHandle) {
        let ram_state: RAMState = app_handle.state();
        let registers_state: RegistersState = app_handle.state();
        let ram_lock = &mut ram_state.lock().await;
        let registers_lock = &mut registers_state.lock().await;

        self.fetch(ram_lock, registers_lock);
        self.decode();
        self.execute();
    }
}

impl Default for CPU {
    fn default() -> Self {
        Self {
            breakpoints: vec![0; 0]
        }
    }
}