<script>
  import { appWindow } from "@tauri-apps/api/window";
  import Icon from "@iconify/svelte";
  import { textColor } from "../stores";

  let textColorValue;

  textColor.subscribe((value) => {
    textColorValue = value;
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
  <div class="titlebar-text" style="color: {textColorValue};">SnakeCharm</div>
  <div class="Button Container">
    <div
      class="titlebar-button"
      id="titlebar-minimize"
      on:click={appWindow.minimize}
      on:keydown={appWindow.minimize}
    >
      <Icon icon="fluent:subtract-20-regular" color={textColorValue} />
    </div>
    <div
      class="titlebar-button"
      id="titlebar-maximize"
      on:click={maximize}
      on:keydown={maximize}
    >
      <Icon icon="fluent:maximize-20-regular" color={textColorValue} />
    </div>
    <div
      class="titlebar-button-close"
      id="titlebar-close"
      on:click={appWindow.close}
      on:keydown={appWindow.close}
    >
      <Icon icon="fluent:dismiss-20-regular" color={textColorValue} />
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
    background: #c42b1c;
  }
  .titlebar-button:hover {
    background: #e9e9e9;
  }
  .titlebar-text {
    text-align: center;
    align-items: center;
    margin-top: 5px;
    margin-left: 12px;
  }

  @media (prefers-color-scheme: dark) {
    .titlebar-button:hover {
      background: #2d2d2d;
    }
  }
</style>
