import { Component, createSignal, Show } from "solid-js"
import { invoke } from "@tauri-apps/api"
import { filename } from "./state"
import hotkeys from "hotkeys-js"

import styles from './css/Toolbar.module.css'

const Toolbar: Component = () => {
    const [running, setRunning] = createSignal(false);
    const [resetting, setResetting] = createSignal(false);

    hotkeys('f5,f10,ctrl+q,ctrl+r', (e, handler) => {
		e.preventDefault();
		switch (handler.key) {
			case 'f5': run(); break;
			case 'f10': step(); break;
			case 'ctrl+q': stop(); break;
			case 'ctrl+r': reset(); break;
			default: break;
		}

        // TODO: have small box in toolbar to represent when a key action is performed
	})

    const run = async () => {
        setRunning(true)
        await invoke('cmd_run')
        setRunning(false)
    }

    const step = () => {
        invoke('cmd_step')
    }

    const stop = () => {
        invoke('cmd_stop')
    }

    const addBreakpoint = () => {
        let input = prompt('Enter a breakpoint address (in hex)')
        if (!input) return
        
        // enforce base16 encoding
        let address = parseInt(input, 16)
        if (isNaN(address)) alert('Breakpint address invalid.')
        
        // add to CPU breakpoint list
        invoke('cmd_add_breakpoint', { address })
    }

    const reset = async () => {
        setResetting(true)
        await invoke('cmd_reset', { filename: filename() })
        setResetting(false)
    }

    return (
        <header class={styles.toolbar}>
            <button onClick={run} disabled={running() || resetting()}>Run</button>
            <button onClick={step} disabled={resetting()}>Step</button>
            <button onClick={stop} disabled={!running() || resetting()}>Stop</button>
            <button onClick={addBreakpoint}>Add Breakpoint</button>
            <button onClick={reset} disabled={resetting()}>Reset</button>
            <Show when={resetting()}>
                <p class="ml-4 font-sans text-white italic text-md my-auto">Resetting...</p>
            </Show>
        </header>
    )
}

export default Toolbar