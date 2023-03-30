<script lang="ts">
  import PyIngest from "./lib/PyIngest.svelte";
  import PyVer from "./lib/PyVer.svelte";
  import PyAdd from "./lib/PyAdd.svelte";
  import TitleBar from "./lib/TitleBar.svelte";
  import "fluent-svelte/theme.css";
</script>

<TitleBar />
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
    color: white;
  }
</style>
