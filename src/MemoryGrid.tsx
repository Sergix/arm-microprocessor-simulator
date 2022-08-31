import { Component, onMount } from "solid-js"
import { listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api/tauri"

// listen for elf_load event from backend
// https://tauri.app/v1/api/js/modules/event#listen

// TODO: define memory::MemoryPayload type struct

const elf_load_event = await listen('elf_load', ({ payload }) => {
    console.log(payload);
})

const MemoryGrid: Component = () => {
    onMount(() => {
        invoke('cmd_get_memory').then((message) => {
            console.log(message)
        })
    });

    return (
        <div>
            Memory Grid
        </div>
    );
}

export default MemoryGrid;