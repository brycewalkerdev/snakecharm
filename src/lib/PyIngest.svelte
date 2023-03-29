<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let result = "";

  async function ingest() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    result = await invoke("py_ingest");
  }
</script>

<div>
  <div class="row">
    <button on:click={ingest}> Get Text </button>
  </div>
  <p>Ingested and modified text is: {result}</p>
</div>
