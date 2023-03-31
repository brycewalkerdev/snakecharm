<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { TextBox } from "fluent-svelte";
  import { Button } from "fluent-svelte";
  import { textColor } from "../stores";
  let addOne = "";
  let addTwo = "";
  let result = "";

  let textColorValue;

  textColor.subscribe((value) => {
    textColorValue = value;
  });

  async function add() {
    result = await invoke("py_add", {
      addOne,
      addTwo,
    });
  }
</script>

<div>
  <div class="container">
    <h2>Addition</h2>
    <div class="two-wide">
      <div class="two-wide">
        <div class="input-container">
          <TextBox
            labelText="First Number to Add"
            placeholder="First Number"
            bind:value={addOne}
          />
        </div>
        <div class="input-contianer">
          <TextBox
            labelText="Second Number to Add"
            placeholder="Second Number"
            bind:value={addTwo}
          />
        </div>
      </div>
      <Button on:click={add} variant="accent">+</Button>
      <p style="color:{textColorValue}">
        Addition Result: {result}
      </p>
    </div>
  </div>
</div>

<style>
  h2 {
    font-size: 15pt;
    color: black;
  }
  @media (prefers-color-scheme: dark) {
    h2 {
      color: white;
    }
  }
</style>
