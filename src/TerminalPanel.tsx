import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { Component, createSignal, Show } from 'solid-js'
import { trace } from 'tauri-plugin-log-api'

const TerminalPanel: Component<ITerminalProp> = (prop: ITerminalProp) => {
    const [output, setOutput] = createSignal("")
    const [prompt, setPrompt] = createSignal(false)
    const [promptInput, setPromptInput] = createSignal("")
    const [promptLen, setPromptLen] = createSignal(0)

    const emitInputInterrupt = (e: KeyboardEvent) => {
        e.preventDefault()

        // ignore modifiers
        if (e.shiftKey || e.metaKey || e.ctrlKey || e.altKey) {
            return
        }

        trace(`SolidJS[TerminalPanel.emitInputInterrupt] sending char ${e.key}=${e.keyCode} to IRQ interrupt line`)
        
        let lastChar = e.key.charCodeAt(0);

        // e.keyCode does not map to uppercase/lowercase automatically, so use charCode,
        // but if it's not a letter key, map to keyCode
        if (!RegExp(/^[a-z0-9]+$/i).test(String.fromCharCode(e.keyCode))) {
            lastChar = e.keyCode
        }
        invoke('cmd_terminal_input_interrupt', { lastChar })
    }

    const emitPromptInput = (e: SubmitEvent) => {
        trace(`SolidJS[TerminalPanel.emitPromptInput] sending prompt input "${promptInput()}" to CPU`)
        setPrompt(false)
        
        invoke('cmd_terminal_prompt_input', { promptInput: promptInput() })
    }

    listen('terminal_prompt', ({ payload }: { payload: ITerminalReadlinePayload }) => {
        trace('SolidJS[TerminalPanel.listen] prompting user for input')
        setPrompt(true)
        setPromptLen(payload.max_bytes)
        setPrompt(true)
    })

    listen('terminal_append', ({ payload }: { payload: ITerminalPutcharPayload }) => {
        trace(`SolidJS[TerminalPanel.listen] appending char ${payload.char} to terminal`)
        setOutput(output() + payload.char)
    })

    listen('terminal_clear', ({ payload }: { payload: ITerminalPutcharPayload }) => {
        trace(`SolidJS[TerminalPanel.listen] clearing terminal`)
        setOutput("")
    })

    return (
        <section class="w-full">
            <h3>Terminal</h3>
            <textarea readonly class="w-full h-52 resize-none rounded-sm bg-slate-800 p-2 focus:shadow-lg transition-all font-mono" onKeyUp={emitInputInterrupt}>
                {output()}
            </textarea>
            <Show when={prompt()}>
                <form onSubmit={emitPromptInput}>
                    <input type="text" class="bg-slate-800 p-2 rounded-sm focus:shadow-lg" placeholder={"Enter up to " + promptLen() + " characters..."} maxlength={promptLen()} onInput={(e) => setPromptInput((e.target as HTMLInputElement)?.value || "")}/>
                    <button type="submit" class="ml-4 bg-violet-900">ENTER</button>
                </form>
            </Show>
        </section>
    )
}

export default TerminalPanel