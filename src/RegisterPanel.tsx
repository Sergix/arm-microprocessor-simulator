import { invoke } from "@tauri-apps/api"
import { listen } from "@tauri-apps/api/event"
import { Accessor, Component, createEffect, createSignal, indexArray, onMount } from "solid-js"
import * as log from 'tauri-plugin-log-api'
import { filename } from "./state"

const RegisterPanel: Component<IRegisterProp> = (prop: IRegisterProp) => {
    const [registers, setRegisters] = createSignal(Array<number>())

    listen('registers_update', ({ payload }: { payload: IRegistersPayload }) => {
        log.trace("SolidJS[RegisterPanel.listen]: updating registers...")
        setRegisters(payload.register_array)
    })

    // clear the output on filename change
    createEffect(() => { filename() ? setRegisters([]) : "" })

    return (
        <section>
            <h3>Registers</h3>
            <table class="font-mono">
                <thead>
                    <tr>
                        <td>r#</td>
                        <td class="pl-4">Value (0x)</td>
                    </tr>
                </thead>
                <tbody>
                {
                    indexArray(registers, (register: Accessor<number>, i: number) => {
                        return (
                            <tr>
                                <td class="text-right">{`r${i}`}</td>
                                <td class="pl-4 text-right">{register().toString(16).padStart(8, '0')}</td>
                            </tr>
                        )
                    })
                }
                </tbody>
            </table>
        </section>
    )
}

export default RegisterPanel