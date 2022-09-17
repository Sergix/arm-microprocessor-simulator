import { createSignal, createRoot } from "solid-js"

const [memory, setMemory] = createSignal(new Array<number>())
const [checksum, setChecksum] = createSignal(0)
const [filename, setFilename] = createSignal("None.")
const [loaded, setLoaded] = createSignal(false)

export {memory, setMemory, checksum, setChecksum, filename, setFilename, loaded, setLoaded}