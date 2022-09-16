import { Component, createSignal } from "solid-js"
import { invoke } from "@tauri-apps/api"
import { style } from "solid-js/web/types"

import styles from './Toolbar.module.css'

const Toolbar: Component = () => {
    const [running, setRunning] = createSignal(false);

    const run = async () => {
        setRunning(true)
        await invoke('cmd_run')
        setRunning(false)
    }

    const step = async () => {
        await invoke('cmd_step')
    }

    const stop = async () => {
        await invoke('cmd_stop')
    }

    const addBreakpoint = async () => {
        let input = prompt('Enter a breakpoint address (in hex)')
        if (!input) return
        
        // enforce base16 encoding
        let address = parseInt(input, 16)
        if (isNaN(address)) alert('Breakpint address invalid.')
        
        // add to CPU breakpoint list
        await invoke('cmd_add_breakpoint', { address });
    }

    return (
        <header class="w-screen p-2 bg-gray-700 flex flex-row simulator_controls rounded-b-md">
            <button class={styles.toolbar_btn} onClick={run} disabled={running()}>Run</button>
            <button class={styles.toolbar_btn} onClick={step}>Step</button>
            <button class={styles.toolbar_btn} onClick={stop} disabled={!running()}>Stop</button>
            <button class={styles.toolbar_btn} onClick={addBreakpoint}>Add Breakpoint</button>
            <button class={styles.toolbar_btn}>Reset</button>
        </header>
    )
}

export default Toolbar