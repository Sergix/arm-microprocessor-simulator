import { Component, createSignal, Show } from 'solid-js';
import { onMount, onCleanup } from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri'
import { listen } from '@tauri-apps/api/event'
import { open } from '@tauri-apps/api/dialog'
import * as log from 'tauri-plugin-log-api'
import hotkeys from 'hotkeys-js'

import { setFilename, filename } from './state'

import styles from './css/App.module.css';
import MemoryPanel from './MemoryPanel';
import RegisterPanel from './RegisterPanel';
import StackPanel from './StackPanel';
import TerminalPanel from './TerminalPanel';
import DisassemblyPanel from './DisassemblyPanel';
import FlagsPanel from './FlagsPanel';
import Toolbar from './Toolbar';

const App: Component = () => {
	const [loaded, setLoaded] = createSignal(false)
	log.attachConsole();

	// attach keybind event listeners
	hotkeys('ctrl+o', (e, _) => {
		e.preventDefault()
		handleLoad()
	})
	
	listen('elf_load', ({ payload }: { payload: IELFPayload }) => {
		log.trace("SolidJS[App]: loading ELF...")

		setLoaded(payload.loaded)
		setFilename(payload.filename)
		
		log.trace("SolidJS[App]: loaded ELF")
	});
		
	listen('invalid_elf', (payload) => {
		log.trace("SolidJS[App]: invalid ELF, clearing UI")
		alert("Invalid ELF file.")
		setLoaded(false)
		setFilename("")
	})
	
	onMount(async () => {
		// check if a binary has been loaded by command-line args
		try {
			const payload: IELFPayload = await invoke('cmd_get_elf')

			setLoaded(payload.loaded)
			setFilename(payload.filename)

			log.trace("SolidJS[App.onMount]: loaded elf")
		} catch {
			log.trace(`SolidJS[App.onMount]: no elf loaded`)
		}
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
				<p class="font-mono text-left text-sm">{ loaded() ? filename() : "None." }</p>
			</header>
			{/* <Show when={loaded()}> */}
				<Toolbar/>
				<div class="flex flex-row pl-1 pr-2 h-5/6">
					<div class="flex flex-col p-1 flex-1 overflow-x-hidden">
						<MemoryPanel/>
						<DisassemblyPanel/>
					</div>
					<div class="flex flex-col p-1 flex-0 overflow-x-hidden">
						<RegisterPanel/>
						<StackPanel/>
					</div>
					<div class="flex flex-col p-1 flex-1 overflow-x-hidden">
						<FlagsPanel />
						<TerminalPanel/>
					</div>
				</div>
			{/* </Show> */}
		</div>
		);
	};
	
	export default App;
	