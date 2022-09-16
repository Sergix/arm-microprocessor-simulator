import { Component, Show } from 'solid-js';
import { createSignal, createEffect, onMount, onCleanup } from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/dialog'
import * as log from 'tauri-plugin-log-api'

import {setMemory, setChecksum} from './state'

import styles from './App.module.css';
import MemoryGrid from './MemoryGrid';
import RegisterPanel from './RegisterPanel';
import StackPanel from './StackPanel';
import TerminalPanel from './TerminalPanel';
import DisassemblyPanel from './DisassemblyPanel';
import FlagsPanel from './FlagsPanel';
import Toolbar from './Toolbar';

const App: Component = () => {
	log.attachConsole();

	const [filename, setFilename] = createSignal("")
	const [loaded, setLoaded] = createSignal(false)
	
	createEffect(() => {
		listen('elf_load', ({ payload }: { payload: IELFPayload }) => {
			log.trace("SolidJS[App]: loading ELF...")

			setLoaded(payload.loaded)
			setChecksum(payload.checksum)
			setMemory(payload.memory_array)

			log.trace("SolidJS[App]: loaded ELF")
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
			const payload: IELFPayload = await invoke('cmd_get_memory')

			setLoaded(payload.loaded)
			setChecksum(payload.checksum)
			setFilename(payload.filename)
			setMemory(payload.memory_array)

			log.trace("SolidJS[App.onMount]: loaded elf")
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
			title: "Select ELF binary",
			filters: [
				{
					extensions: ['exe'],
					name: ".exe"
				},
				{
					extensions: ['*'],
					name: "All files"
				}
			]
		})
		setFilename(() => (selected?.toString() || ""))
		
		log.trace("SolidJS[App.handleLoad]: calling elf loader");
		await invoke('cmd_load_elf', { filename: selected });
	};
	
	return (
		<div class={styles.App}>
			<header class="flex flex-row items-center px-4 py-1 bg-gray-800">
				<p class="font-bold">ARMSim</p>
				<button class={styles.file_loader_button} onClick={handleLoad}>
					Load ELF
				</button>
				<p class="font-mono text-left text-sm">{ loaded() ? filename : "None." }</p>
			</header>
			<Show when={loaded()}>
				<Toolbar/>
				<div class="flex flex-row">
					<div class="flex flex-col">
						<MemoryGrid/>
						<TerminalPanel/>
						<DisassemblyPanel/>
					</div>
					<div class="flex flex-col">
						<RegisterPanel/>
						<StackPanel/>
					</div>
					<div class="flex flex-col">
						<FlagsPanel />
					</div>
				</div>
			</Show>
		</div>
		);
	};
	
	export default App;
	