import { createSignal } from "solid-js"

const [filename, setFilename] = createSignal("None.")

export {filename, setFilename}