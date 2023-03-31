<script lang="ts">
  import PyIngest from "./lib/PyIngest.svelte";
  import PyVer from "./lib/PyVer.svelte";
  import PyAdd from "./lib/PyAdd.svelte";
  import TitleBar from "./lib/TitleBar.svelte";
  import "fluent-svelte/theme.css";
  import { textColor } from "./stores";

  let textColorValue;
  let titleBarColor;

  textColor.subscribe((value) => {
    textColorValue = value;
  });
</script>

<TitleBar />
<div class="titlebar-spacer" style="background: inherit" />
<div class="wrapper">
  <div class="left-container">
    <h1>Sidebar Text</h1>
    <p style="color: {textColorValue};">
      Lorem ipsum dolor sit amet, consectetur adipiscing elit. Morbi tristique a
      lorem vitae fringilla. Sed sit amet lorem condimentum, egestas purus in,
      sagittis dui. Nam a volutpat tellus, sit amet auctor mi. Nulla vulputate
      dui sodales lectus consectetur, in imperdiet lectus ornare. Pellentesque
      vitae viverra purus. Fusce nec feugiat ipsum. Pellentesque ipsum ante,
      porttitor in lobortis quis, porttitor vel erat. Duis consectetur a metus
      vitae fringilla. Donec elementum mauris lorem, sollicitudin scelerisque
      est bibendum a. Aliquam lacus velit, tristique sed dictum nec, laoreet
      eget massa. Phasellus mattis tellus at magna efficitur, sed malesuada odio
      auctor. Nullam lacinia libero purus, non pharetra libero vestibulum sit
      amet. Cras orci ipsum, suscipit sed molestie sit amet, fermentum vel leo.
      Vestibulum vitae malesuada purus.
    </p>
  </div>
  <div class="right-container">
    <div class="header">
      <h1>Test Suite for Python-Rust Binding</h1>
    </div>
    <div class="row">
      <PyAdd />
    </div>

    <div class="row">
      <PyVer />
    </div>

    <div class="row">
      <PyIngest />
    </div>
  </div>
</div>

<style>
  .wrapper {
    display: flex;
  }
  .left-container {
    width: 200px;
    margin-left: 10px;
    margin-right: 10px;
  }
  .right-container {
    color: white;
    grid-column: 2;
    width: 100%;
    margin-left: 10px;
    margin-right: 10px;
  }
  .titlebar-spacer {
    height: 35px;
  }

  h1 {
    color: black;
    font-size: 20px;
  }

  /* Media query for dark mode */
  @media (prefers-color-scheme: dark) {
    h1 {
      color: white;
      font-size: 20pt;
    }
  }
</style>
