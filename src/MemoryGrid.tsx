import { Component } from "solid-js"

import styles from './MemoryGrid.module.css'

interface IMemoryGridProps {
    memory: Array<Array<number>>
    checksum: number
}

const MemoryGrid: Component<IMemoryGridProps> = (props: IMemoryGridProps) => {
    return (
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
    )
}

export default MemoryGrid
