import { createSignal, createRoot } from "solid-js"

const [memory, setMemory] = createSignal(new Array<number>())
const [checksum, setChecksum] = createSignal(0)

export {memory, setMemory, checksum, setChecksum}