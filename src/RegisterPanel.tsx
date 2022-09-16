import { invoke } from "@tauri-apps/api"
import { listen } from "@tauri-apps/api/event"
import { Accessor, Component, createEffect, createSignal, indexArray, onMount } from "solid-js"
import * as log from 'tauri-plugin-log-api'

import styles from './Registers.module.css'

const RegisterPanel: Component<IRegisterProp> = (prop: IRegisterProp) => {
    const [registers, setRegisters] = createSignal(Array<number>())

    createEffect(() => {
        listen('register_update', ({ payload }: { payload: Array<number> }) => {
            setRegisters(payload)
        })
    })

    onMount(async () => {
        log.trace("SolidJS[RegisterPanel.onMount]: getting register state...")

        const payload: IRegistersPayload = await invoke('cmd_get_registers')
        
        setRegisters(payload.register_array)
    })

    return (
        <section>
            <h3>Registers</h3>
            <ul class="font-mono">
                {
                    indexArray(registers, (register: Accessor<number>, i: number) => {
                        return (
                            <li>
                                {`r${i}: `} {register().toString(16).padStart(8, '0')}
                            </li>
                        )
                    })
                }
            </ul>
        </section>
    )
}

export default RegisterPanel