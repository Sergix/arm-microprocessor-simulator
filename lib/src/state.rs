use crate::memory;
use crate::options;
use crate::cpu;
use tauri::{ State, async_runtime::Mutex };

pub type CPUState<'a> = State<'a, Mutex<cpu::CPU>>;
pub type OptionsState<'a> = State<'a, Mutex<options::Options>>;
pub type RAMState<'a> = State<'a, Mutex<memory::RAM>>;
pub type RegistersState<'a> = State<'a, Mutex<memory::Registers>>;
pub type CPUThreadWatcherState<'a> = State<'a, Mutex<cpu::CPUThreadWatcher>>;