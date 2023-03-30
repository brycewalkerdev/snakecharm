<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { TextBox } from "fluent-svelte";
  import { Button } from "fluent-svelte";
  let addOne = "";
  let addTwo = "";
  let result = "";

  async function add() {
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
          <TextBox
            labelText="First Number to Add"
            placeholder="23"
            bind:value={addOne}
          />
        </div>
        <div class="input-contianer">
          <TextBox
            labelText="Second Number to Add"
            placeholder="56"
            bind:value={addTwo}
          />
        </div>
      </div>
      <Button on:click={add} variant="accent">Button</Button>
      <p>
        {result}
      </p>
    </div>
  </div>
</div>
