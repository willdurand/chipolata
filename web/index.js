import * as libchipolata from "libchipolata";
import { memory } from "libchipolata/libchipolata_bg";
import { createAudio } from "./audio";
import { createDisplay } from "./display";
import { disassemble } from "./disassembler";
import { makeKeypad } from "./keypad";
import { hexformat } from "./utils";

const Chip8 = {
  // TODO: retrieve these values via the interpreter instance.
  WIDTH: 64,
  HEIGHT: 32,
  // TODO: make it configurable.
  speed: 9,
  paused: false,

  display: null,
  audio: null,

  keysPressed: {},

  $pauseBtn: null,

  init(_document, _screen) {
    // TODO: add controls to change display dimensions
    this.display = createDisplay(
      _document.getElementById("display-canvas"),
      this.WIDTH,
      this.HEIGHT,
      _screen.width,
      _screen.height
    );

    // TODO: add controls to mute sound
    this.audio = createAudio();

    this.$pauseBtn = _document.querySelector("#btn-pause");

    this.onKeyDown = this.onKeyDown.bind(this);
    this.onKeyUp = this.onKeyUp.bind(this);
    this.onPauseClick = this.onPauseClick.bind(this);

    _document.addEventListener("keydown", this.onKeyDown);
    _document.addEventListener("keyup", this.onKeyUp);
    this.$pauseBtn.addEventListener("click", this.onPauseClick);
  },

  onKeyDown(event) {
    this.keysPressed[event.key] = true;
  },

  onKeyUp(event) {
    this.keysPressed[event.key] = false;
  },

  onPauseClick() {
    this.$pauseBtn.classList.toggle("btn-ghost");
    this.$pauseBtn.textContent = this.paused ? "stop" : "start";
    this.paused = !this.paused;
  },

  run(rom) {
    const interpreter = new libchipolata.JsInterpreter(rom);

    const vram = new Uint8Array(
      memory.buffer,
      interpreter.get_vram_ptr(),
      this.WIDTH * this.HEIGHT
    );

    const ram = new Uint8Array(
      memory.buffer,
      interpreter.get_ram_ptr(),
      0x1000
    );

    const $opcode = document.querySelector(".opcode .values");
    $opcode.innerHTML = disassemble(ram);

    const v_registers = new Uint8Array(
      memory.buffer,
      interpreter.get_v_registers_ptr(),
      16
    );

    const $registers1 = document.querySelector(".registers .values-1");
    const $registers2 = document.querySelector(".registers .values-2");

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
          `v${String(i + 8).padStart(2, "0")}=${hexformat(
            v_registers[i + 8],
            2
          )}`
        );
      }

      $registers1.innerHTML = values1.join("<br>");
      $registers2.innerHTML = values2.join("<br>");
    };

    const renderLoop = () => {
      if (!this.paused) {
        let redraw = false;
        for (let i = 0; i < this.speed; i++) {
          interpreter.update_keypad(makeKeypad(this.keysPressed));
          interpreter.step();

          if (interpreter.should_redraw()) {
            redraw = true;
          }
        }

        if (redraw) {
          this.display.draw(vram);
        }

        if (interpreter.should_beep()) {
          this.audio.start();
        } else {
          this.audio.stop();
        }

        interpreter.update_timers();

        updateInfo();
      }

      requestAnimationFrame(renderLoop);
    };

    renderLoop();
  },
};

Chip8.init(document, screen);

fetch("./space-invaders.ch8").then(async (response) => {
  const buffer = await response.arrayBuffer();
  const rom = new Uint8Array(buffer);

  Chip8.run(rom);
});
