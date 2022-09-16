import { invoke } from '@tauri-apps/api'
import { Component, createSignal, onMount } from 'solid-js'
import * as log from 'tauri-plugin-log-api'

const FlagsPanel: Component<IFlagsProp> = (prop: IFlagsProp) => {
    const [nFlag, setNFlag] = createSignal(false)
    const [zFlag, setZFlag] = createSignal(false)
    const [cFlag, setCFlag] = createSignal(false)
    const [vFlag, setVFlag] = createSignal(false)

    onMount(async () => {
        log.trace("SolidJS[FlagsPanel.onMount]: getting CPSR flags state...")

        const payload: IFlagsPayload = await invoke('cmd_get_flags')
        
        setNFlag(payload.n)
        setZFlag(payload.z)
        setCFlag(payload.c)
        setVFlag(payload.v)
    })

    return (
        <section>
            <h3>Flags</h3>
            <ul class="flex flex-row font-mono">
                <li class={nFlag() ? 'text-green-600' : 'text-gray-700'}>N</li>
                <li class={zFlag() ? 'text-green-600' : 'text-gray-700'}>Z</li>
                <li class={cFlag() ? 'text-green-600' : 'text-gray-700'}>C</li>
                <li class={vFlag() ? 'text-green-600' : 'text-gray-700'}>V</li>
            </ul>
        </section>
    )
}

export default FlagsPanel