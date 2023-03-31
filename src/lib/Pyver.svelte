<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { Flyout } from "fluent-svelte";
  import { Button } from "fluent-svelte";

  let result = "";

  async function pyver() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    result = await invoke("py_ver");
  }
</script>

<div class="container">
  <Flyout>
    <Button on:click={pyver}>Get Version</Button>
    <svelte:fragment slot="flyout">Python Version: {result}</svelte:fragment>
  </Flyout>
</div>

<style>
</style>
