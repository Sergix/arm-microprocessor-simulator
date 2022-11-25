import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { Component, createSignal, Show } from 'solid-js'
import { trace } from 'tauri-plugin-log-api'

const TerminalPanel: Component<ITerminalProp> = (prop: ITerminalProp) => {
    const [output, setOutput] = createSignal("")
    const [prompt, setPrompt] = createSignal(false)
    const [promptInput, setPromptInput] = createSignal("")
    const [promptLen, setPromptLen] = createSignal(0)

    const emitInputInterrupt = (e: InputEvent) => {
        trace(`SolidJS[TerminalPanel.emitInputInterrupt] sending char ${e.data} to IRQ interrupt line`)

        let last_char: number = 0

        if (e.inputType === "insertText") {
            last_char = e.data?.charCodeAt(0) || 0
        } else {
            last_char = 8 // backspace
        }

        invoke('cmd_terminal_input_interrupt', { last_char })
    }

    const emitPromptInput = (e: SubmitEvent) => {
        trace(`SolidJS[TerminalPanel.emitPromptInput] sending prompt input "${promptInput()}" to CPU`)

        invoke('cmd_terminal_prompt_input', { prompt_input: promptInput() })
    }

    listen('cmd_terminal_prompt', ({ payload }: { payload: ITerminalReadlinePayload }) => {
        trace('SolidJS[TerminalPanel.listen] prompting user for input')
        setPrompt(true)
        setPromptLen(payload.max_bytes)
        setPrompt(true)
    })

    listen('cmd_terminal_append', ({ payload }: { payload: ITerminalPutcharPayload }) => {
        trace(`SolidJS[TerminalPanel.listen] appending char ${payload.char} to terminal`)
        setOutput(output() + payload.char)
    })

    return (
        <section class="w-full">
            <h3>Terminal</h3>
            <textarea class="w-full h-52 resize-none rounded-sm bg-slate-800 p-2 focus:shadow-lg transition-all font-mono" onInput={emitInputInterrupt}>
                {output()}
            </textarea>
            <Show when={prompt()}>
                <form onSubmit={emitPromptInput}>
                    <input type="text" placeholder='>' maxlength={promptLen()} onInput={(e) => setPromptInput((e.target as HTMLInputElement)?.value || "")}/>
                    <button type="submit">ENTER</button>
                </form>
            </Show>
        </section>
    )
}

export default TerminalPanel