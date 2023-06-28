<script lang="ts">

import { invoke } from "@tauri-apps/api/tauri"

async function callExport() {
  await invoke('runexport');
}
</script>


<main class="container">
    <button class="dropdown-button" on:click={callExport}>Экспорт</button>
    
    
</main>
    