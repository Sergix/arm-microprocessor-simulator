# Application Architecture

## Design Principles

- Do as much logic and data processing as possible in Rust/Tauri
- Each UI panel should be responsible as a component for getting all the data it needs from the backend, not waiting for prop-passing from a parent component
- Mutable global state is OK in the frontend (but should be mitigated), but should be carefully managed in the backend
- State should be managed in as tight of a scope as possible (see *Thread Locking*)

## Directories

- `/lib`: logic that doesn't interact with the interface
    This logic may interact with Tauri's application state, but does not interact with any UI logic
- `/src-tauri`: Tauri commands, state, events, and other interface logic that sets up the frontend and responds to UI events
- `/src`: frontend and UI

## "Core" Logic

Due to the way the application state is managed through mutexes, not all core logic is completely separable from interfering with Tauri's application state. The alternative would be to pass around massive state payloads back and forth from the core logic. Because of this, the core logic is not completely testable (yet). Therefore, to differentiate "core" (`/lib`) logic from regular Tauri logic, "core" logic is that which does not interact with the frontend *at all* but can mutate Tauri's state.

This results in complexity when calling core logic from Tauri logic: when should state locks need to be freed before calling core logic and when can they be kept? See *Thread Locking* for an explanation.

## Thread Locking

Thread locks should be dropped as quickly as possible within its context, especially before calling core logic. Explicitly scoped blocks always guarantee a lock will be dropped as soon as the block's scope ends.

## Instruction Decoding

Instruction decoding uses one primary `Instruction` class since structs cannot be inherited.

## Banked Registers

Since this program only supports 3 modes (SYS, SVC, IRQ) with 6 extra banked registers (SVC r13/r14, IRQ r13/r14, SPSR_svc, SPSR_irq), banked registers are simply an extension of the register memory array. Constant values and methods are provided in `lib::memory` and `lib::memory::Registers` to calculate the appropriate indices for those registers when needed.

## Terminal I/O


## Frontend UI Component Updates

Because the backend cannot guaruantee that once it attempts to send the original state update to the frontend that the frontend components have been mounted, every component individually requests a state update once it mounts.