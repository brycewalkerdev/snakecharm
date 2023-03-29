<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let addOne = "";
  let addTwo = "";
  let result = "";

  async function add() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    result = await invoke("py_add", {
      addOne,
      addTwo,
    });
  }
</script>

<div>
  <div class="row">
    <p>First number to add:</p>
    <input type="text" bind:value={addOne} />
  </div>
  <div class="row">
    <p>Second number to add:</p>
    <input type="text" bind:value={addTwo} />
  </div>
  <div class="row">
    <button on:click={add}> Add </button>
  </div>
  <p>Result is: {result}</p>
</div>
