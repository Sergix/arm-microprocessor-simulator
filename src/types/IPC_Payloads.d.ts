// from backend

interface IELFPayload {
	loaded: boolean
	error: string
	filename: string
}

interface IRegistersPayload {
	register_array: Array<number>
}

interface IRAMPayload {
	checksum: number
	memory_array: Array<Array<number>>
}

interface IFlagsPayload {
	n: boolean
    z: boolean
    c: boolean
    v: boolean
}

interface ICPUPayload {
	trace: boolean
}

// array containing tuple, each with
// - instruction address (focuses at current PC): Word/number
// - instruction at that address: Word/number
// - disassembled representation: String/string
interface IDisassemblyInstruction extends Array<number | string | boolean> { 0: boolean, 1: number, 2: number, 3: string }
interface IDisassemblyPayload {
	pc: number
	instructions: Array<IDisassemblyInstruction>
}