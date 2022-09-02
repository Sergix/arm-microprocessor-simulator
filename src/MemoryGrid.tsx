import { Component, onMount, createSignal, onCleanup, createEffect, indexArray, Show } from "solid-js"
import { listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api/tauri"
import * as log from 'tauri-plugin-log-api'

import styles from './MemoryGrid.module.css'


const MEMORY_ROW_SIZE = 16;

// from backend memory.rs
interface IMemoryPayload {
    checksum: number
    loaded: boolean
    memory_array: Array<number>
    error: string
}

const MemoryGrid: Component = () => {
    const [filename, setFilename] = createSignal("")
    const [memory, setMemory] = createSignal(new Array<Array<number>>())
    const [checksum, setChecksum] = createSignal(0)

    createEffect(() => {
        listen('elf_load', ({ payload }: { payload: IMemoryPayload }) => {
            // split into row chunks
            let memory_array = new Array<Array<number>>();
            while (payload.memory_array.length > 0)
                memory_array.push(payload.memory_array.splice(0, MEMORY_ROW_SIZE))

            setChecksum(payload.checksum)
            setMemory(memory_array)
        });

        // TODO: get unlistener for unmount (if needed)
        // https://github.com/FabianLars/mw-toolbox/blob/main/gui/src/pages/Upload/Upload.tsx#L70
    });
    
    onMount(() => {
        invoke('cmd_get_memory').then((message) => {
            console.log(message)
        })
        .catch((message) => {
            console.log("ERROR: not yet loaded")
        })
    })

    onCleanup(() => {
        // TODO: call unlistener for elf_load (see above)
    })

    return (
        <Show when={memory().length > 0}>
            {/* <span>Checksum: {checksum}</span> */}
            <table class={styles.MemoryGrid} onScroll={() => {}}>
                <caption>Checksum: <span style="font-family: monospace;">{checksum}</span></caption>
                
                {/* https://stackoverflow.com/questions/70819075/solidjs-for-vs-index */}
                {indexArray(memory, (row, index) => {
                    return (
                        <tr>
                            <td>{(index * 16).toString(16).padStart(8, '0')}</td>
                            {row().map((item, i) => {
                                return (
                                    <td>{item.toString(16).padStart(2, '0')}</td>
                                )
                            })}
                        </tr>
                    )
                })}
            </table>
        </Show>
    )
}

export default MemoryGrid
