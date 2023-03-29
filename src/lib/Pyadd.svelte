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
  <div class="container">
    <div class="two-wide">
    <div class="two-wide">
      <div class="input-container">
        <p>First number to add:</p>
        <input type="text" bind:value={addOne} />
      </div>
      <div class="input-contianer">
        <p>Second number to add:</p>
        <input type="text" bind:value={addTwo} />
      </div>
    </div>
    <button on:click={add}> Add </button>
    <p>Result is: {result}</p>
  </div>
</div>

<style>
  .container {
    display: grid;
    grid-template-columns: 1fr;
    justify-items: center;
    text-align: center;
  }
  .two-wide {
    grid-column: 1 / 3;
  }
  .input-container {
    display: grid;
    grid-template-columns: 1fr;
    justify-items: center;
    text-align: center;
  }
</style>
