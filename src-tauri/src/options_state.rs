use lib::options;
use tauri::{ State, async_runtime::Mutex };

pub(crate) type OptionsState<'a> = State<'a, Mutex<options::Options>>;