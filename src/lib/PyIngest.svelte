<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button } from "carbon-components-svelte";

  let PyResult: string = "";

  async function ingest() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    PyResult = await invoke("py_ingest");
  }
</script>

<div>
  <div class="row">
    <Button on:click={ingest}>Get Text</Button>
  </div>
  <p>
    {PyResult}
  </p>
</div>
