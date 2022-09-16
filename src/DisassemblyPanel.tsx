import { Component } from "solid-js"

const DisassemblyPanel: Component<IDisassemblyProp> = (prop: IDisassemblyProp) => {
    // TODO:
    // grabs instruction from memory store from prop.instruction_address
    // 

    return (
        <section>
            <h3>Disassembly</h3>
            <table onScroll={() => {}}>
                {/* <caption>Checksum: <span style="font-family: monospace;">{memory_prop.checksum}</span></caption>
                
                {memory_prop.memory.map((row, index) => {
                    return (
                        <tr>
                            <td>{(index * 16).toString(16).padStart(8, '0')}</td>
                            {row.map((item, i) => {
                                return (
                                    <td>{item.toString(16).padStart(2, '0')}</td>
                                )
                            })}
                        </tr>
                    )
                })} */}
            </table>
        </section>
    )
}

export default DisassemblyPanel