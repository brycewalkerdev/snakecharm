<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Button } from "flowbite-svelte";
  import { CodeSnippet } from "carbon-components-svelte";

  let result = "";

  async function pyver() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    result = await invoke("py_ver");
  }
</script>

<div>
  <div class="container">
    <Button color="green" on:click={pyver}>Get Version</Button>
    <CodeSnippet code="Python Version: {result}" />
  </div>
</div>

<style>
</style>
