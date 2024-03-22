import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { Component, createEffect, createSignal, onMount } from 'solid-js'
import * as log from 'tauri-plugin-log-api'
import { filename } from './state'

const FlagsPanel: Component<IFlagsProp> = (prop: IFlagsProp) => {
    const [nFlag, setNFlag] = createSignal(false)
    const [zFlag, setZFlag] = createSignal(false)
    const [cFlag, setCFlag] = createSignal(false)
    const [vFlag, setVFlag] = createSignal(false)
    const [iFlag, setIFlag] = createSignal(false)

    const setFlags = (payload: IFlagsPayload) => {
        setNFlag(payload.n)
        setZFlag(payload.z)
        setCFlag(payload.c)
        setVFlag(payload.v)
        setIFlag(payload.i)
    }

    listen('flags_update', ({ payload }: { payload: IFlagsPayload }) => {
        log.trace("SolidJS[FlagsPanel.listen]: updating flags...")
        setFlags(payload)
    })

    // clear the output on filename change
    createEffect(() => {
        if (filename()) {
            setNFlag(false)
            setZFlag(false)
            setCFlag(false)
            setVFlag(false)
            setIFlag(false)
        }
    })

    return (
        <section>
            <h3>Flags</h3>
            <ul class="flex flex-row font-mono">
                <li class={nFlag() ? 'text-green-600' : 'text-gray-700'}>N</li>
                <li class={zFlag() ? 'text-green-600' : 'text-gray-700'}>Z</li>
                <li class={cFlag() ? 'text-green-600' : 'text-gray-700'}>C</li>
                <li class={vFlag() ? 'text-green-600' : 'text-gray-700'}>V</li>
                <li class={iFlag() ? 'text-green-600' : 'text-gray-700'}>I</li>
            </ul>
        </section>
    )
}

export default FlagsPanel