<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let result = "";

  async function pyver() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    result = await invoke("py_ver");
  }
</script>

<div>
  <div class="row">
    <button on:click={pyver}> Get Version </button>
  </div>
  <p>Current Python Version is: {result}</p>
</div>
