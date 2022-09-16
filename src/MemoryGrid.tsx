import { Component, createEffect, createMemo, createSignal, mergeProps } from "solid-js"
import * as log from 'tauri-plugin-log-api'

import { memory, checksum } from './state'
import styles from './MemoryGrid.module.css'

const MemoryGrid: Component<IMemoryProp> = (memory_prop: IMemoryProp) => {

    const MEMORY_ROW_SIZE = 16;

    // split into individual rows based on offset
    const chunk = (payload_memory_array: Array<number>, offset: number) => {

        // shallow copy since arrays are pass-by-ref
        const payload_memory_array_copy = [...payload_memory_array]

        if (offset === NaN) {
            log.error("SolidJS[MemoryGrid.chunk]: offset must be a number")
            offset = 0
        }

        if (offset < 0) {
            log.error("SolidJS[MemoryGrid.chunk]: offset must be a positive number")
            offset = 0
        }
        
        log.trace("SolidJS[MemoryGrid.chunk]: chunking memory table...")

        let memory_array = new Array<Array<number>>()
        
        const payload_memory_array_size = payload_memory_array_copy.length
        const first_row_size = offset % MEMORY_ROW_SIZE
        const full_row_count = Math.floor((payload_memory_array_size - first_row_size) / MEMORY_ROW_SIZE)
        const last_row_size = payload_memory_array_size - ((MEMORY_ROW_SIZE * full_row_count) + first_row_size)
        
        if (first_row_size > 0)
            memory_array.push(payload_memory_array_copy.splice(0, first_row_size))

        if (payload_memory_array_size < MEMORY_ROW_SIZE) {
            log.trace("SolidJS[MemoryGrid.chunk]: nothing to chunk, exiting early (memory may still be loading)")
            return memory_array
        }

        while (payload_memory_array_copy.length > last_row_size)
            memory_array.push(payload_memory_array_copy.splice(0, MEMORY_ROW_SIZE))

        if (last_row_size > 0)
            memory_array.push(payload_memory_array_copy.splice(0, last_row_size))

        log.trace("SolidJS[MemoryGrid.chunk]: finished chunking")
        return memory_array
    }
    
    // rechunk memory when state updates
    const [chunkedMemory, setChunkedMemory] = createSignal(chunk(memory(), 0))
    createEffect(() => setChunkedMemory(chunk(memory(), 0)))

    // have them separate so that the visible state isn't updated as the user types
    const [startingAddress, setStartingAddress] = createSignal(0)
    const [inputStartingAddress, setInputStartingAddress] = createSignal(0)

    
    const scrollToStartingAddress = () => {
        const startingAddressRow = Math.ceil(startingAddress() / MEMORY_ROW_SIZE)
        document.getElementById(`${startingAddressRow}-0`)?.scrollIntoView()
    }

    return (
        <section class="h-96 overflow-hidden">
            <h3>Memory</h3>
            <p class="font-mono text-sm">Checksum: {checksum()}</p>
            <div class="flex align-middle justify-start my-2">
                <input onInput={(e) => setInputStartingAddress(parseInt(e.currentTarget.value))} type="text" id="starting_address" name="starting_address" placeholder="Address 0x..."/>
                <button class="text-sm ml-2" onClick={(_) => {
                    setStartingAddress(inputStartingAddress())
                    setChunkedMemory(chunk(memory(), startingAddress()))
                    scrollToStartingAddress()
                }}>GO</button>
            </div>
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
                                <td>{(chunkedMemory()[0].length < MEMORY_ROW_SIZE ? (index * 16) + (startingAddress() % 16) - 16 : (index * 16)).toString(16).padStart(8, '0')}</td>
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
