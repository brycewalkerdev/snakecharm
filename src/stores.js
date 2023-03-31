import { writable } from 'svelte/store';

  let bootColor = "";
  if (
    window.matchMedia &&
    window.matchMedia("(prefers-color-scheme: dark)").matches
  ) {
    bootColor = "white";
  } else {
    bootColor = "black";
  }


export let textColor = writable(bootColor);

window
    .matchMedia("(prefers-color-scheme: dark)")
    .addEventListener("change", (event) => {
      event.matches ? textColor.set("white") : textColor.set("black");
    });