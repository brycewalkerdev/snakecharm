<script lang="ts">
  import Pyadd from "./lib/Pyadd.svelte";
  import PyIngest from "./lib/PyIngest.svelte";
  import Pyver from "./lib/Pyver.svelte";
</script>

<main class="container">
  <h1>Press the buttons</h1>
  <div class="row">
    <Pyadd />
  </div>

  <div class="row">
    <Pyver />
  </div>

  <div class="row">
    <PyIngest />
  </div>
</main>

<style>
</style>
