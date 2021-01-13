import * as libchipolata from "libchipolata";
import { memory } from "libchipolata/libchipolata_bg";
import { createAudio, createDisplay, makeKeypad } from "./helpers";

// TODO: retrieve these values via the interpreter instance.
const WIDTH = 64;
const HEIGHT = 32;

// TODO: add controls to mute sound
const audio = createAudio();
// TODO: add controls to change display dimensions
const display = createDisplay(
  document.getElementById("display-canvas"),
  WIDTH,
  HEIGHT,
  screen.width,
  screen.height
);

const keysPressed = {};

document.addEventListener("keydown", (event) => {
  keysPressed[event.key] = true;
});

document.addEventListener("keyup", (event) => {
  keysPressed[event.key] = false;
});

// TODO: make it configurable
const speed = 9;

fetch("./space-invaders.ch8").then(async (response) => {
  const rom = await response.arrayBuffer();
  const interpreter = new libchipolata.JsInterpreter(new Uint8Array(rom));
  const vram = new Uint8Array(
    memory.buffer,
    interpreter.get_vram_ptr(),
    WIDTH * HEIGHT
  );

  const renderLoop = () => {
    let redraw = false;
    for (let i = 0; i < speed; i++) {
      interpreter.update_keypad(makeKeypad(keysPressed));
      interpreter.step();

      if (interpreter.should_redraw()) {
        redraw = true;
      }
    }

    if (redraw) {
      for (let x = 0; x < WIDTH; x++) {
        for (let y = 0; y < HEIGHT; y++) {
          display.drawPixelAt(vram[x + y * WIDTH] == 1, x, y);
        }
      }
    }

    if (interpreter.should_beep()) {
      audio.start();
    } else {
      audio.stop();
    }

    interpreter.update_timers();

    requestAnimationFrame(renderLoop);
  };

  renderLoop();
});
