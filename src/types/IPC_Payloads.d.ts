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