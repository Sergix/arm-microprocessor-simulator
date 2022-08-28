import type { Component } from 'solid-js';
import { createSignal } from 'solid-js';
import { invoke } from '@tauri-apps/api/tauri'
import * as log from 'tauri-plugin-log-api'

import logo from './logo.svg';
import styles from './App.module.css';

// TODO: listen for events from backend in case cmd args were passed
// https://tauri.app/v1/api/js/modules/event#listen

const App: Component = () => {
  log.attachConsole();

  const handleLoad = async () => {
    const res: string = await invoke('load_elf', { filename: 'test' });
    log.info("Called loader");
  };

  return (
    <div class={styles.App}>
      <header class={styles.header}>
        <img src={logo} class={styles.logo} alt="logo" />
        <p>
          Edit <code>src/App.tsx</code> and save to reload.
        </p>
        <button style="color:black;" onClick={handleLoad}>Load ELF</button>
      </header>
    </div>
  );
};

export default App;
