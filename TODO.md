# TODO

- code style (https://protect.bju.edu/cps/courses/armsim-project/codestyle.html)
- refactor memory panel updating (may not be necessary -- moving logic to Rust stopped freezing UI, but still takes time to rechunk and send over the IPC)
  - remove memory chunking -- most expensive operation
  - passing memory from backend is also expensive; compress?
  - use custom protocol instead of Tauri events
- vertical resizing of the window affects panels
- visual indicators for when the the cpu is running or keybinds have been pressed

## Stage 3

- remake diagram
- all data opcodes
  - map instructions in ARM manual
- ldr/str
- swi should halt
- trace methods
- make sure memory panel updates when ldr/str are called