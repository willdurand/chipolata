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
const $registers1 = document.querySelector(".registers .values-1");
const $registers2 = document.querySelector(".registers .values-2");

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

  const v_registers = new Uint8Array(
    memory.buffer,
    interpreter.get_v_registers_ptr(),
    16
  );

  const updateInfo = () => {
    const pc = interpreter.get_pc();

    const className = "current-addr";
    const oldAddr = document.querySelector(`.${className}`);
    oldAddr && oldAddr.classList.toggle(className);

    const newAddr = document.querySelector(`.addr-${pc}`);
    if (newAddr) {
      newAddr.classList.toggle(className);
      newAddr.parentElement.scrollTo(
        0,
        newAddr.offsetTop - newAddr.parentElement.offsetTop - 70
      );
    }

    const values1 = [];
    const values2 = [];
    for (let i = 0; i < 8; i++) {
      values1.push(
        `v${String(i).padStart(2, "0")}=${hexformat(v_registers[i], 2)}`
      );
      values2.push(
        `v${String(i + 8).padStart(2, "0")}=${hexformat(v_registers[i + 8], 2)}`
      );
    }

    $registers1.innerHTML = values1.join("<br>");
    $registers2.innerHTML = values2.join("<br>");
  };

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

    updateInfo();

    requestAnimationFrame(renderLoop);
  };

  renderLoop();
};

fetch("./space-invaders.ch8").then(async (response) => {
  const buffer = await response.arrayBuffer();
  const rom = new Uint8Array(buffer);

  run(rom, DEFAULT_SPEED);
});
