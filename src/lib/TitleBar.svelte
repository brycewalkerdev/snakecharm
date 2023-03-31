<script>
  import { appWindow } from "@tauri-apps/api/window";
  import Icon from "@iconify/svelte";
  let colorMode = "";
  if (
    window.matchMedia &&
    window.matchMedia("(prefers-color-scheme: dark)").matches
  ) {
    colorMode = "white";
  } else {
    colorMode = "black";
  }

  window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", (event) => {
      colorMode = event.matches ? "white" : "black";
    });

  async function maximize() {
    await appWindow.isMaximized().then((value) => {
      if (value) {
        appWindow.unmaximize();
      } else {
        appWindow.maximize();
      }
    });
  }
</script>

<div data-tauri-drag-region class="titlebar">
  <div class="titlebar-text" style="color: {colorMode};">SnakeCharm</div>
  <div class="Button Container">
    <div
      class="titlebar-button"
      id="titlebar-minimize"
      on:click={appWindow.minimize}
      on:keydown={appWindow.minimize}
    >
      <Icon icon="fluent:subtract-20-regular" color={colorMode} />
    </div>
    <div
      class="titlebar-button"
      id="titlebar-maximize"
      on:click={maximize}
      on:keydown={maximize}
    >
      <Icon icon="fluent:maximize-20-regular" color={colorMode} />
    </div>
    <div
      class="titlebar-button-close"
      id="titlebar-close"
      on:click={appWindow.close}
      on:keydown={appWindow.close}
    >
      <Icon icon="fluent:dismiss-20-regular" color={colorMode} />
    </div>
  </div>
</div>

<style>
  .titlebar {
    height: 35px;
    background: inherit;
    user-select: none;
    display: flex;
    justify-content: space-between;
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
  .titlebar-text {
    text-align: center;
    align-items: center;
    padding-left: 7px;
  }
</style>
