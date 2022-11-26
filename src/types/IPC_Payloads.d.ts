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
    i: boolean
}

interface ICPUPayload {
	trace: boolean
	mode: string
}

interface ITerminalPutcharPayload {
	char: string
}

interface ITerminalReadlinePayload {
	max_bytes: number
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

// array containing tuple, each with
// - address (focuses at current SP): Word/number
// - value at that address: Word/number
interface IStackAddress extends Array<number> { 0: number, 1: number }
interface IStackPayload {
	sp: number
	addresses: Array<IStackAddress>
}