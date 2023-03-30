<script>
  import { appWindow } from "@tauri-apps/api/window";
  import Icon from "@iconify/svelte";
</script>

<div data-tauri-drag-region class="titlebar">
  <div
    class="titlebar-button"
    id="titlebar-minimize"
    on:click={appWindow.minimize}
    on:keydown={appWindow.minimize}
  >
    <Icon icon="fluent:subtract-20-regular" color="white" />
  </div>
  <div
    class="titlebar-button"
    id="titlebar-maximize"
    on:click={appWindow.maximize}
    on:keydown={appWindow.maximize}
  >
    <Icon icon="fluent:maximize-20-regular" color="white" />
  </div>
  <div
    class="titlebar-button-close"
    id="titlebar-close"
    on:click={appWindow.close}
    on:keydown={appWindow.close}
  >
    <Icon icon="fluent:dismiss-20-regular" color="white" />
  </div>
</div>

<style>
  .titlebar {
    height: 35px;
    background: #000e11b6;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 35px;
    height: 35px;
    color: white;
  }
  .titlebar-button-close {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 35px;
    height: 35px;
    color: white;
  }
  .titlebar-button-close:hover {
    background: #c62e2f;
  }
  .titlebar-button:hover {
    background: #424247;
  }
</style>
