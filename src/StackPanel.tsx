import { invoke } from '@tauri-apps/api';
import { listen } from '@tauri-apps/api/event';
import { Component, createEffect, createSignal, onMount } from 'solid-js'
import { trace } from 'tauri-plugin-log-api';
import { filename } from './state';

const StackPanel: Component<IStackProp> = (prop: IStackProp) => {
    const [sp, setSp] = createSignal(0)
    const [stack, setStack] = createSignal(new Array<IStackAddress>())

    listen("stack_update", ({payload}: { payload: IStackPayload }) => {
        trace("SolidJS[StackPanel.listen]: updating stack...")
        setSp(payload.sp)
        setStack(payload.addresses)
    })

    // clear the output on filename change
    createEffect(() => { filename() ? setStack([]) : "" })

    return (
        <section>
            <h3>Stack</h3>
            <div class="p-2 rounded-sm bg-gray-700">
                <table onScroll={() => {}} class="font-mono w-full">
                    <thead>
                        <tr class="bg-gray-700">
                            <td class="pl-6">Address</td>
                            <td class="pl-6">Value</td>
                        </tr>
                    </thead>
                    <tbody>
                        {stack().map((address: IStackAddress) => {
                            return (
                                <tr class={address[0] === sp() ? 'bg-blue-900' : 'bg-gray-800'}>
                                    <td class="pl-6">{address[0].toString(16).padStart(8, '0')}</td>
                                    <td class="pl-6">{address[1].toString(16).padStart(8, '0')}</td>
                                </tr>
                            )
                        })}
                    </tbody>
                </table>
            </div>
        </section>
    )
}

export default StackPanel