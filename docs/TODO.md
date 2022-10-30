# TODO

- code style (https://protect.bju.edu/cps/courses/armsim-project/codestyle.html)
- refactor memory panel updating (may not be necessary -- moving logic to Rust stopped freezing UI, but still takes time to rechunk and send over the IPC)
  - remove memory chunking -- most expensive operation
  - passing memory from backend is also expensive; compress?
  - use custom protocol instead of Tauri events
- vertical resizing of the window affects panels
- visual indicators for when the the cpu is running or keybinds have been pressed

## Phase 3

- unit tests?
- file comments
- function comments
- struct comments
- struct field comments
- move payloads to one file

## Phase 4
- all data opcode s-bit updates
- CPSR conditions in CPU::execute
- LDRH/STRH LSH codes (-> LDRD, LDREX, etc.)