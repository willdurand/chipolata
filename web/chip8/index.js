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
  muted: false,
  keysPressed: {},

  display: null,
  audio: null,

  $pauseBtn: null,
  $muteBtn: null,
  $resetBtn: null,
  $registers1: null,
  $registers2: null,

  interpreter: null,
  v_registers: null,

  init(_document, _screen) {
    this.display = createDisplay(
      _document.getElementById("display-canvas"),
      this.WIDTH,
      this.HEIGHT,
      _screen.width,
      _screen.height
    );

    this.audio = createAudio();

    this.$pauseBtn = _document.querySelector("#btn-pause");
    this.$muteBtn = _document.querySelector("#btn-mute");
    this.$resetBtn = _document.querySelector("#btn-reset");
    this.$registers1 = _document.querySelector(".registers .values-1");
    this.$registers2 = _document.querySelector(".registers .values-2");

    this.onKeyDown = this.onKeyDown.bind(this);
    this.onKeyUp = this.onKeyUp.bind(this);
    this.onPauseClick = this.onPauseClick.bind(this);
    this.onMuteClick = this.onMuteClick.bind(this);
    this.onResetClick = this.onResetClick.bind(this);

    _document.addEventListener("keydown", this.onKeyDown);
    _document.addEventListener("keyup", this.onKeyUp);
    this.$pauseBtn.addEventListener("click", this.onPauseClick);
    this.$muteBtn.addEventListener("click", this.onMuteClick);
    this.$resetBtn.addEventListener("click", this.onResetClick);
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

  onMuteClick() {
    this.$muteBtn.classList.toggle("btn-ghost");
    this.$muteBtn.textContent = this.muted ? "mute" : "unmute";
    this.muted = !this.muted;
  },

  onResetClick() {
    this.interpreter.reset();
  },

  updateInfo() {
    const pc = this.interpreter.get_pc();

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
        `v${String(i).padStart(2, "0")}=${hexformat(this.v_registers[i], 2)}`
      );
      values2.push(
        `v${String(i + 8).padStart(2, "0")}=${hexformat(
          this.v_registers[i + 8],
          2
        )}`
      );
    }

    this.$registers1.innerHTML = values1.join("<br>");
    this.$registers2.innerHTML = values2.join("<br>");
  },

  run(rom) {
    this.interpreter = new libchipolata.JsInterpreter(rom);
    this.v_registers = new Uint8Array(
      memory.buffer,
      this.interpreter.get_v_registers_ptr(),
      16
    );

    const vram = new Uint8Array(
      memory.buffer,
      this.interpreter.get_vram_ptr(),
      this.WIDTH * this.HEIGHT
    );

    const ram = new Uint8Array(
      memory.buffer,
      this.interpreter.get_ram_ptr(),
      0x1000
    );

    const $opcode = document.querySelector(".opcode .values");
    $opcode.innerHTML = disassemble(ram);

    const renderLoop = () => {
      if (!this.paused) {
        let redraw = false;
        for (let i = 0; i < this.speed; i++) {
          this.interpreter.update_keypad(makeKeypad(this.keysPressed));
          this.interpreter.step();

          if (this.interpreter.should_redraw()) {
            redraw = true;
          }
        }

        if (redraw) {
          this.display.draw(vram);
        }

        if (!this.muted) {
          if (this.interpreter.should_beep()) {
            this.audio.start();
          } else {
            this.audio.stop();
          }
        }

        this.interpreter.update_timers();

        this.updateInfo();
      }

      requestAnimationFrame(renderLoop);
    };

    renderLoop();
  },
};

export default Chip8;
