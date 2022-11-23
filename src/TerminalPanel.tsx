import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { Component, createSignal, Show } from 'solid-js'
import { trace } from 'tauri-plugin-log-api'

const TerminalPanel: Component<ITerminalProp> = (prop: ITerminalProp) => {
    const [output, setOutput] = createSignal("")
    const [prompt, setPrompt] = createSignal(false)
    const [promptLen, setPromptLen] = createSignal(0)
    const [lastChar, setLastChar] = createSignal("")

    const emitInput = (e: InputEvent) => {
        setLastChar(e.data || "")
        trace(lastChar().charCodeAt(0).toString())

        let last_char: number = 0

        if (e.inputType === "insertText") {
            last_char = e.data?.charCodeAt(0) || 0
        } else {
            last_char = 8 // backspace
        }

        invoke('cmd_terminal_input', { last_char })
    }

    listen('cmd_terminal_prompt', ({ payload }: { payload: ITerminalPayload }) => {
        setPrompt(true)
        setPromptLen(payload.prompt_bytes)
        setPrompt(true)
    })

    listen('cmd_terminal_append', ({ payload }: { payload: ITerminalPayload }) => {
        setOutput(output() + payload.char)
    })

    return (
        <section class="w-full">
            <h3>Terminal</h3>
            <textarea class="w-full h-52 resize-none rounded-sm bg-slate-800 p-2 focus:shadow-lg transition-all font-mono" onInput={emitInput}>
                {output()}
            </textarea>
            <Show when={prompt()}>
                <input type="text" placeholder='>' maxlength={promptLen()}/>
            </Show>
        </section>
    )
}

export default TerminalPanel