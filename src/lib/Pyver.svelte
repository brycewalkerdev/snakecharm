<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let result = "";

  async function pyver() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    result = await invoke("py_ver");
  }
</script>

<div>
  <div class="container">
    <button on:click={pyver}> Get Version </button>
    <p>Current Python Version is: {result}</p>
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 2, 1fr;
    justify-items: center;
    text-align: center;
  }
</style>
