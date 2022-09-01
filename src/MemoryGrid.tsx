import { Index, Component, onMount, createSignal, onCleanup, createEffect, indexArray } from "solid-js"
import { listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api/tauri"
import * as log from 'tauri-plugin-log-api'

import styles from './MemoryGrid.module.css'

// from backend memory.rs
interface IMemoryPayload {
    loaded: boolean
    memory_array: Array<Array<number>>
}

const MemoryGrid: Component = () => {
    const [filename, setFilename] = createSignal("")
    const [memory, setMemory] = createSignal(new Array<Array<number>>())

    createEffect(() => {
        listen('elf_load', ({ payload }: { payload: IMemoryPayload }) => {
            setMemory(payload.memory_array)
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
        <div>
            <h1>Memory Grid</h1>
            <table class={styles.MemoryGrid} onScroll={() => {}}>
                
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
        </div>
    )
}

export default MemoryGrid
