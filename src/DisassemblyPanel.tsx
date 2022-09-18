import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import hotkeys from "hotkeys-js";
import { Component, createSignal, onMount } from "solid-js"
import * as log from 'tauri-plugin-log-api'

const DisassemblyPanel: Component<IDisassemblyProp> = (prop: IDisassemblyProp) => {
    const [programCounter, setProgramCounter] = createSignal(0)
    const [instructions, setInstructions] = createSignal(new Array<IDisassemblyInstruction>())

    listen("disassembly_update", ({payload}: { payload: IDisassemblyPayload }) => {
        setProgramCounter(payload.pc)
        setInstructions(payload.instructions)
    });

    hotkeys('ctrl+b', (e, _) => {
		e.preventDefault()
		toggleBreakpoint(3) // 3 is middle element / program counter index
	})

    const toggleBreakpoint = (i: number) => {
        invoke('cmd_toggle_breakpoint', { address: instructions()[i][1] })
    }

    onMount(async () => {
        log.trace("SolidJS[DisassemblyPanel.onMount]: getting disassembly...")

        const payload: IDisassemblyPayload = await invoke('cmd_get_disassembly')
        setProgramCounter(payload.pc)
        setInstructions(payload.instructions)
    })

    return (
        <section>
            <h3>Disassembly</h3>
            <div class="p-2 rounded-sm bg-gray-700">
                <table onScroll={() => {}} class="font-mono w-full">
                    <thead>
                        <tr class="bg-gray-700">
                            <td class="pl-2" colspan="2">BP</td>
                            <td class="pl-2">Address</td>
                            <td class="pl-6">Instruction</td>
                            <td class="pl-6">Assembly</td>
                        </tr>
                    </thead>
                    <tbody>
                        {instructions().map((instruction: IDisassemblyInstruction, i: number) => {
                            return (
                                <tr class={instruction[1] === programCounter() ? 'bg-blue-900' : 'bg-gray-800'}>
                                    <td class="pl-2 text-red-700 cursor-pointer opacity-0 hover:opacity-50 active:opacity-100" classList={ {['opacity-100']: instruction[0]} } colspan="2" onClick={(_) => toggleBreakpoint(i)}>◉</td>
                                    <td class="pl-2">{instruction[1].toString(16).padStart(8, '0')}</td>
                                    <td class="pl-6">{instruction[2].toString(16).padStart(8, '0')}</td>
                                    <td class="pl-6">{instruction[3]}</td>
                                </tr>
                            )
                        })}
                    </tbody>
                </table>
            </div>
        </section>
    )
}

export default DisassemblyPanel