<script lang="ts">
  import PyIngest from "./lib/PyIngest.svelte";
  import PyVer from "./lib/PyVer.svelte";
  import PyAdd from "./lib/PyAdd.svelte";
</script>

<main class="container">
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
</main>

<style>
  .container {
    display: grid;
    grid-template-columns: 1fr;
  }
  .header {
    text-align: center;
    font-family: "JetBrains Mono", monospace;
    font-weight: 900;
    color: whitesmoke;
  }
</style>
