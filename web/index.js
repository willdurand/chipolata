import * as libchipolata from "libchipolata";
import { memory } from "libchipolata/libchipolata_bg";
import { createAudio } from "./audio";
import { createDisplay } from "./display";
import { disassemble } from "./disassembler";
import { makeKeypad } from "./keypad";
import { hexformat } from "./utils";

// TODO: retrieve these values via the interpreter instance.
const WIDTH = 64;
const HEIGHT = 32;
// TODO: make it configurable.
const DEFAULT_SPEED = 9;

const $display = document.getElementById("display-canvas");
const $opcode = document.querySelector(".opcode .values");

// TODO: add controls to mute sound
const audio = createAudio();
// TODO: add controls to change display dimensions
const display = createDisplay(
  $display,
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

const run = (rom, speed) => {
  const interpreter = new libchipolata.JsInterpreter(rom);

  const vram = new Uint8Array(
    memory.buffer,
    interpreter.get_vram_ptr(),
    WIDTH * HEIGHT
  );

  const ram = new Uint8Array(memory.buffer, interpreter.get_ram_ptr(), 0x1000);
  $opcode.innerHTML = disassemble(ram);

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
      display.draw(vram);
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
};

fetch("./space-invaders.ch8").then(async (response) => {
  const buffer = await response.arrayBuffer();
  const rom = new Uint8Array(buffer);

  run(rom, DEFAULT_SPEED);
});
