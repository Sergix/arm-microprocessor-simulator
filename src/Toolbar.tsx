import { Component, createSignal, onMount, Show } from "solid-js"
import { invoke } from "@tauri-apps/api"
import { listen } from '@tauri-apps/api/event'
import { filename } from "./state"
import hotkeys from "hotkeys-js"
import * as log from 'tauri-plugin-log-api'

import styles from './css/Toolbar.module.css'

const Toolbar: Component = () => {
    const [running, setRunning] = createSignal(false);
    const [resetting, setResetting] = createSignal(false);
    const [trace, setTrace] = createSignal(false);
    const [hotkey, setHotkey] = createSignal("");
    const [mode, setMode] = createSignal("");

    hotkeys('f5,f10,ctrl+q,ctrl+r', (e, handler) => {
		e.preventDefault();
		switch (handler.key) {
			case 'f5': run(); break;
			case 'f10': step(); break;
			case 'ctrl+q': stop(); break;
			case 'ctrl+r': reset(); break;
            case 'ctrl+t': toggle_trace(); break;
			default: break;
		}

        // have small box in toolbar to represent when a key action is performed
        setHotkey(handler.key)
        setTimeout(() => {
            setHotkey("")
        }, 1000)
	})

    const run = async () => {
        setRunning(true)
        await invoke('cmd_run')
        setRunning(false)
    }

    const step = async () => {
        setRunning(true)
        await invoke('cmd_step')
        setRunning(false)
    }

    const stop = () => {
        invoke('cmd_stop')
        setRunning(false)
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
        setRunning(false)
        setResetting(false)
    }

    const toggle_trace = () => {
        setTrace(!trace())
        invoke('cmd_toggle_trace')
    }

    listen('cpu_update', ({ payload }: { payload: ICPUPayload }) => {
        log.trace("SolidJS[FlagsPanel.listen]: updating flags...")
        setTrace(payload.trace)
        setMode(payload.mode)
    })

    return (
        <header class={styles.toolbar}>
            <button onClick={run} disabled={running() || resetting() || filename() === ""}>Run</button>
            <button onClick={step} disabled={running() || resetting() || filename() === ""}>Step</button>
            <button onClick={stop} disabled={!running() || resetting() || filename() === ""}>Stop</button>
            <button onClick={addBreakpoint}>Add Breakpoint</button>
            <button onClick={reset} disabled={resetting()}>Reset</button>
            <button onClick={toggle_trace} classList={ {['!bg-green-700']: trace() } }>Trace</button>
            <Show when={resetting()}>
                <p class="ml-4 font-sans text-white italic text-md my-auto">Resetting...</p>
            </Show>
            <p class="ml-auto mr-4 bg-slate-800 px-4 py-2 rounded-sm font-mono italic shadow-md text-violet-400 border-slate-600 border">{ mode() }</p>
            <aside class="absolute top-0 right-0 m-6 px-6 py-3 bg-slate-900 text-white border-2 border-double border-b-4 border-spacing-2 border-violet-900 font-mono italic rounded-lg shadow-md transition-opacity ease-in duration-150" classList={ {['opacity-90']: hotkey() !== "", ['opacity-0']: hotkey() === ""} }>{hotkey()}</aside>
        </header>
    )
}

export default Toolbar