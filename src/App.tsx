import { Component, Show } from 'solid-js';
import { createSignal, createEffect, onMount, onCleanup } from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/dialog'
import * as log from 'tauri-plugin-log-api'

import styles from './App.module.css';
import MemoryGrid from './MemoryGrid';
import { style } from 'solid-js/web/types';

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
			setChecksum(payload.checksum)
			setMemoryFromPayload(payload)
		});
		
		// TODO: get unlistener for unmount (if needed)
		// https://github.com/FabianLars/mw-toolbox/blob/main/gui/src/pages/Upload/Upload.tsx#L70
	})
	
	// check if a binary has been loaded by command-line args
	onMount(() => {
		invoke('cmd_get_memory')
		.then((payload: any) => {
			log.trace("SolidJS[App.onMount]: loaded elf")
			setMemoryFromPayload(payload)
		})
		.catch((message: any) => {
			log.trace(`SolidJS[App.onMount]: no elf loaded`)
		})
	})
	
	onCleanup(() => {
		// TODO: call unlistener for elf_load (see above)
	})
	
	const handleLoad = async () => {
		const selected = await open({
			title: "Select ELF binary"
		})
		setFilename(() => (selected?.toString() || ""))
		
		const res: string = await invoke('cmd_load_elf', { filename: selected });
		log.trace("SolidJS[App.handleLoad]: Called loader");
	};
	
	return (
		<div class={styles.App}>
			<header class={styles.header}>
				<h1 class="logo">ARMSim</h1>
			</header>
			<p class={styles.filename}>{ filename() === "" ? "None." : filename }</p>
			<button class={styles.file_loader_button} onClick={handleLoad}>
				Load ELF
			</button>
			<Show when={checksum() != 0}>
				<MemoryGrid checksum={checksum()} memory={memory()}/>
			</Show>
		</div>
		);
	};
	
	export default App;
	