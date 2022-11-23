import { Component, createSignal, onMount } from "solid-js"
import * as log from 'tauri-plugin-log-api'

import styles from './css/MemoryPanel.module.css'
import { listen } from "@tauri-apps/api/event"
import { invoke } from "@tauri-apps/api"

const MEMORY_ROW_SIZE = 16;

const MemoryGrid: Component<IMemoryProp> = (memory_prop: IMemoryProp) => {
    const [checksum, setChecksum] = createSignal(0)
    const [chunking, setChunking] = createSignal(false)
    const [chunkedMemory, setChunkedMemory] = createSignal(new Array<Array<number>>())

    // have them separate so that the visible state isn't updated as the user types
    const [offset, setOffset] = createSignal(0)
    const [inputOffset, setInputOffset] = createSignal(0)

    listen('ram_chunking_signal', () => {
        log.trace('SolidJS[MemoryPanel.listen]: updating memory chunking signal...')

        setChunking(true)
    })

    // updates from backend
    onMount(async () => {
        log.trace('SolidJS[MemoryPanel.onMount]: updating global memory state...')
        const payload: IRAMPayload = await invoke('cmd_get_ram')

        setChecksum(payload.checksum)
        setChunkedMemory(payload.memory_array)

        setChunking(false)
    })

    listen('ram_update', ({ payload }: { payload: IRAMPayload }) => {
        log.trace('SolidJS[MemoryPanel.listen]: updating global memory state...')

        setChecksum(payload.checksum)
        setChunkedMemory(payload.memory_array)

        setChunking(false)
    })

    const validateAndSetInputOffset = (value: string) => {
        let n = parseInt(value, 16)
        if (n === NaN)
            alert("Invalid starting address; must be base 16 value")
        else {
            setInputOffset(n)
        }
    }

    const getChunkedMemory = async () => {
        log.trace(`SolidJS[MemoryPanel.getChunkedMemory]: rechunking of memory table with offset ${offset()}...`)

        setChunking(true)
        
        const payload: IRAMPayload = await invoke('cmd_set_offset', { offset: offset() });
        
        setChunking(false)
        
        setChecksum(payload.checksum || checksum())
        setChunkedMemory(payload.memory_array)
        
    }
    
    const scrollToStartingAddress = () => {
        const offsetRow = Math.ceil(offset() / MEMORY_ROW_SIZE)
        document.getElementById(`${offsetRow}-0`)?.scrollIntoView()
    }

    return (
        <section class="h-96 relative">
            <h3>Memory</h3>
            <div class="flex flex-row align-middle my-2 flex-wrap">
                <p class="font-mono text-sm my-auto">Checksum: {checksum()}</p>
                <form class="flex align-middle justify-start ml-4" onSubmit={(e) => e.preventDefault()}>
                    <input onInput={(e) => validateAndSetInputOffset(e.currentTarget.value)} type="text" id="starting_address" name="starting_address" placeholder="Address 0x..."/>
                    <button class="text-sm ml-2" onClick={(_) => {
                        setOffset(inputOffset())
                        getChunkedMemory()
                        scrollToStartingAddress()
                    }}>GO</button>
                </form>
            </div>
            <aside class="overlay" classList={{ ['invisible']: !chunking() }}>
                Chunking...
            </aside>
            <table class={styles.MemoryGrid} onScroll={() => {}}>
                {chunkedMemory().map((row, index) => {
                    // if first row and chunked at an offset
                    if (index === 0 && row.length < MEMORY_ROW_SIZE) {
                        return (
                            <tr>
                                <td>{(0).toString(16).padStart(8, '0')}</td>
                                {/* pad the cells if needed for the initial offset; the "XX" is just a flag that the cell should be invisible */}
                                {([...Array(MEMORY_ROW_SIZE - row.length).fill("XX"), ...row]).map((item, i) => {
                                    return (
                                        <td class={item === "XX" ? "invisible" : "visible"} id={`${index}-${i}`}>{item.toString(16).padStart(2, '0')}</td>
                                    )
                                })}
                            </tr>
                        )
                    } else {
                        return (
                            <tr>
                                {/* if there is an offset, calculate the row address relative to the starting address. otherwise, calculate it as normal. */}
                                <td>{(chunkedMemory()[0].length < MEMORY_ROW_SIZE ? (index * 16) + (offset() % 16) - 16 : (index * 16)).toString(16).padStart(8, '0')}</td>
                                {row.map((item, i) => {
                                    return (
                                        <td id={`${index}-${i}`}>{item.toString(16).padStart(2, '0')}</td>
                                    )
                                })}
                            </tr>
                        )
                    }
                })}
            </table>
        </section>
    )
}

export default MemoryGrid
