import { Component, Show } from 'solid-js';
import { createSignal } from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri'
import { open } from '@tauri-apps/api/dialog'
import * as log from 'tauri-plugin-log-api'

import logo from './logo.svg';
import styles from './App.module.css';
import MemoryGrid from './MemoryGrid';
import { style } from 'solid-js/web/types';

const App: Component = () => {
  log.attachConsole();

  const [filename, setFilename] = createSignal("")

  const handleLoad = async () => {
    const selected = await open({
      title: "Select ELF binary"
    })
    setFilename(() => (selected?.toString() || ""))
    
    const res: string = await invoke('cmd_load_elf', { filename: selected });
    log.trace("SolidJS[App]: Called loader");
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
      <Show when={filename() !== ""}>
        <MemoryGrid />
      </Show>
    </div>
  );
};

export default App;
