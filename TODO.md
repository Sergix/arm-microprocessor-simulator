# TODO

- configuration for disabling/enabling logging and logfile destination
- code style (https://protect.bju.edu/cps/courses/armsim-project/codestyle.html)
- refactor memory panel updating
  - remove memory chunking -- most expensive operation
  - passing memory from backend is also expensive; compress?
  - use custom protocol (websocket) instead of Tauri events