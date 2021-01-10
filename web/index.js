import * as chipolata from "chipolata";
import { memory } from "chipolata/chipolata_bg";
import { createAudio, createScreen, makeKeypad } from "./helpers";

// TODO: retrieve these values via the interpreter instance.
const WIDTH = 64;
const HEIGHT = 32;

// TODO: add controls to mute sound
const audio = createAudio();
// TODO: add controls to change screen dimensions
const screen = createScreen(document.getElementById("screen"), WIDTH, HEIGHT);

const keysPressed = {};

document.addEventListener("keydown", (event) => {
  keysPressed[event.key] = true;
});

document.addEventListener("keyup", (event) => {
  keysPressed[event.key] = false;
});

// TODO: make it configurable
const speed = 8;

fetch("./space-invaders.ch8").then(async (response) => {
  const rom = await response.arrayBuffer();
  const interpreter = new chipolata.Interpreter(new Uint8Array(rom));

  const renderLoop = () => {
    const keypad = makeKeypad(keysPressed);

    interpreter.tick(new Uint8Array(keypad), speed);

    if (interpreter.should_redraw()) {
      const framebuffer = new Uint8Array(
        memory.buffer,
        interpreter.get_framebuffer_ptr(),
        WIDTH * HEIGHT
      );

      for (let x = 0; x < WIDTH; x++) {
        for (let y = 0; y < HEIGHT; y++) {
          screen.drawPixelAt(framebuffer[x + y * WIDTH] == 1, x, y);
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
