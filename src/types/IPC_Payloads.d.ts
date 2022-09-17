// from backend memory.rs
interface IELFPayload {
	checksum: number
	loaded: boolean
	memory_array: Array<number>
	error: string
	filename: string
}

interface IRegistersPayload {
	register_array: Array<number>
}

interface IRAMPayload {
	memory_array: Array<number>
}

interface IFlagsPayload {
	n: boolean
    z: boolean
    c: boolean
    v: boolean
}

// array containing tuple, each with
// - instruction address (focuses at current PC): Word/number
// - instruction at that address: Word/number
// - disassembled representation: String/string
// program counter: Word/number
interface IDisassemblyInstruction extends Array<number | string | boolean> { 0: boolean, 1: number, 2: number, 3: string }
interface IDisassemblyPayload {
	pc: number
	instructions: Array<IDisassemblyInstruction>
	breakpoint: boolean
}