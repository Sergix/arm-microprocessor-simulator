import type { Component } from 'solid-js';
import { createSignal } from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri'
import * as log from 'tauri-plugin-log-api'

import logo from './logo.svg';
import styles from './App.module.css';
import MemoryGrid from './MemoryGrid';

const App: Component = () => {
  log.attachConsole();

  const handleLoad = async () => {
    const res: string = await invoke('cmd_load_elf', { filename: 'test.bin' });
    log.info("Called loader");
  };

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        {/* <img src={logo} class={styles.logo} alt="logo" /> */}
        
      </header>
      <form onSubmit={e => { e.preventDefault(); e.stopPropagation(); }}>
        <input type="text" name="elf_filename" />
        <button style="color:black;" onClick={handleLoad}>Load ELF</button>
      </form>
      <MemoryGrid />
    </div>
  );
};

export default App;
