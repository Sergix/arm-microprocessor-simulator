import { invoke } from "@tauri-apps/api";
import { listen } from "@tauri-apps/api/event";
import hotkeys from "hotkeys-js";
import { Accessor, Component, createSignal, indexArray } from "solid-js"

const DisassemblyPanel: Component<IDisassemblyProp> = (prop: IDisassemblyProp) => {
    const [programCounter, setProgramCounter] = createSignal(0)
    const [instructions, setInstructions] = createSignal(new Array<IDisassemblyInstruction>())

    listen("disassembly_update", ({payload}: { payload: IDisassemblyPayload }) => {
        setProgramCounter(payload.pc)
        setInstructions(payload.instructions)
    });

    hotkeys('ctrl+b', (e, _) => {
		e.preventDefault()
		toggleBreakpoint()
	})

    const toggleBreakpoint = () => {
        // middle element is current instruction (TODO: refactor)
        invoke('cmd_toggle_breakpoint', { address: instructions()[3][1] })
    }

    return (
        <section>
            <h3>Disassembly</h3>
            <div class="p-2 rounded-sm bg-gray-700">
                <table onScroll={() => {}} class="font-mono w-full">
                    <thead>
                        <tr class="bg-gray-700">
                            <td class="pl-2"></td>
                            <td class="pl-2">Address</td>
                            <td class="pl-6">Instruction</td>
                            <td class="pl-6">Assembly</td>
                        </tr>
                    </thead>
                    <tbody>
                        {instructions().map((instruction: IDisassemblyInstruction, _) => {
                            return (
                                <tr class={instruction[1] === programCounter() ? 'bg-blue-900' : 'bg-gray-800'}>
                                    <td class="pl-2 text-red-700">{ instruction[0] ? '◉' : ' ' }</td>
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