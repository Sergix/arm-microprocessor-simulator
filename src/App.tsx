import { Component, Show } from 'solid-js';
import { createSignal, createEffect, onMount, onCleanup } from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/dialog'
import * as log from 'tauri-plugin-log-api'

import styles from './App.module.css';
import MemoryGrid from './MemoryGrid';

const MEMORY_ROW_SIZE = 16;

// from backend memory.rs
interface IMemoryPayload {
	checksum: number
	loaded: boolean
	memory_array: Array<number>
	error: string
	filename: string
}

const App: Component = () => {
	log.attachConsole();
	
	const [memory, setMemory] = createSignal(new Array<Array<number>>())
	const [checksum, setChecksum] = createSignal(0)
	const [filename, setFilename] = createSignal("")
	const [loaded, setLoaded] = createSignal(false)
	
	// split into row chunks
    const chunk = (payload_memory_array: Array<number>) => {
        let memory_array = new Array<Array<number>>();
        while (payload_memory_array.length > 0)
            memory_array.push(payload_memory_array.splice(0, MEMORY_ROW_SIZE))

        return memory_array
    }

	const setMemoryFromPayload = (payload: IMemoryPayload) => setMemory(chunk(payload.memory_array))
	
	createEffect(() => {
		listen('elf_load', ({ payload }: { payload: IMemoryPayload }) => {
			log.trace("SolidJS[App]: loading ELF...")

			setLoaded(payload.loaded)
			setChecksum(payload.checksum)
			setMemoryFromPayload(payload)
		});
		
		// TODO: get unlistener for unmount (if needed)
		// https://github.com/FabianLars/mw-toolbox/blob/main/gui/src/pages/Upload/Upload.tsx#L70
	})

	createEffect(() => {
		listen('invalid_elf', (payload) => {
			log.trace("SolidJS[App]: invalid ELF, clearing UI")
			alert("Invalid ELF file.")
			setLoaded(false)
		})
	})
	
	// check if a binary has been loaded by command-line args
	onMount(async () => {
		try {
			const payload: IMemoryPayload = await invoke('cmd_get_memory')

			log.trace("SolidJS[App.onMount]: loaded elf")
			
			setLoaded(payload.loaded)
			setChecksum(payload.checksum)
			setFilename(payload.filename)
			setMemoryFromPayload(payload)
		} catch {
			log.trace(`SolidJS[App.onMount]: no elf loaded`)
		}
	})
	
	onCleanup(() => {
		// TODO: call unlistener for elf_load (see above)
	})
	
	const handleLoad = async () => {
		setLoaded(false);

		const selected = await open({
			title: "Select ELF binary"
		})
		setFilename(() => (selected?.toString() || ""))
		
		await invoke('cmd_load_elf', { filename: selected });
		log.trace("SolidJS[App.handleLoad]: Called loader");
	};
	
	return (
		<div class={styles.App}>
			<header class={styles.header}>
				<h1 class="logo">ARMSim</h1>
			</header>
			<p class={styles.filename}>{ loaded() ? filename : "None." }</p>
			<button class={styles.file_loader_button} onClick={handleLoad}>
				Load ELF
			</button>
			<Show when={loaded()}>
				<MemoryGrid checksum={checksum()} memory={memory()}/>
			</Show>
		</div>
		);
	};
	
	export default App;
	