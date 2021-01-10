extern crate console_error_panic_hook;
extern crate web_sys;

use std::convert::TryInto;
use wasm_bindgen::prelude::*;

pub mod cpu;
pub mod mmu;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub struct Interpreter {
    cpu: cpu::CPU,
    redraw: bool,
    framebuffer: [u8; cpu::WIDTH * cpu::HEIGHT],
}

#[wasm_bindgen]
impl Interpreter {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: Vec<u8>) -> Self {
        console_error_panic_hook::set_once();
        log!("creating interpreter: rom size={:}", rom.len());

        let mmu = mmu::MMU::new(rom);
        let cpu = cpu::CPU::new(mmu);

        Interpreter {
            cpu,
            redraw: false,
            framebuffer: [0; cpu::WIDTH * cpu::HEIGHT],
        }
    }

    pub fn tick(&mut self, keypad: Vec<u8>, speed: u8) {
        let keypad: [bool; 16] = keypad
            .iter()
            .map(|x| *x == 1)
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap();

        self.redraw = false;

        for _ in 0..speed {
            self.cpu.step(keypad);

            if self.cpu.should_redraw() {
                self.redraw = true;
            }
        }
    }

    pub fn should_redraw(&self) -> bool {
        self.redraw
    }

    pub fn should_beep(&self) -> bool {
        self.cpu.should_beep()
    }

    pub fn update_timers(&mut self) {
        self.cpu.update_timers();
    }

    pub fn get_framebuffer_ptr(&mut self) -> *const u8 {
        for x in 0..cpu::WIDTH {
            for y in 0..cpu::HEIGHT {
                self.framebuffer[x + (y * cpu::WIDTH)] = if self.cpu.vram[x][y] { 1 } else { 0 };
            }
        }

        self.framebuffer.as_ptr()
    }
}
