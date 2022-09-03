import { Component, createSignal, onMount, onCleanup, createEffect, indexArray, Show, mapArray } from "solid-js"
import { listen } from '@tauri-apps/api/event'
import { invoke } from '@tauri-apps/api/tauri'
import * as log from 'tauri-plugin-log-api'

import styles from './MemoryGrid.module.css'

interface IMemoryGridProps {
    memory: Array<Array<number>>
    checksum: number
}

const MemoryGrid: Component<IMemoryGridProps> = (props: IMemoryGridProps) => {
    return (
        <Show when={props.memory.length > 0}>
            <table class={styles.MemoryGrid} onScroll={() => {}}>
                <caption>Checksum: <span style="font-family: monospace;">{props.checksum}</span></caption>
                
                {/* https://stackoverflow.com/questions/70819075/solidjs-for-vs-index */}
                {props.memory.map((row, index) => {
                    return (
                        <tr>
                            <td>{(index * 16).toString(16).padStart(8, '0')}</td>
                            {row.map((item, i) => {
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
