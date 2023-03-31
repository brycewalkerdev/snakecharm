<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { TextBox } from "fluent-svelte";
  import { Button } from "fluent-svelte";
  import { textColor } from "../stores";
  import { Flyout } from "fluent-svelte";
  import Icon from "@iconify/svelte";
  let addOne = "";
  let addTwo = "";
  let result = "";

  let textColorValue;

  textColor.subscribe((value) => {
    textColorValue = value;
  });

  function containsOnlyNumbers(str) {
    return /^\d+$/.test(str);
  }

  async function add() {
    if (!containsOnlyNumbers(addOne) || !containsOnlyNumbers(addTwo)) {
      result = "Please enter only numbers";
      return;
    }
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
      <Flyout>
        <Button on:click={add} variant="accent"
          ><Icon icon="fluent:add-20-regular" /></Button
        >
        <svelte:fragment slot="flyout"
          >{addOne} + {addTwo} = {result}</svelte:fragment
        >
      </Flyout>
      <p style="color:{textColorValue}">
        Python: {result}
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
