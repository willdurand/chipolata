use std::convert::TryInto;
use wasm_bindgen::prelude::*;

use crate::chip8;

#[wasm_bindgen]
pub struct JsInterpreter {
    interpreter: chip8::Interpreter,
}

#[wasm_bindgen]
impl JsInterpreter {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: Vec<u8>) -> Self {
        JsInterpreter {
            interpreter: chip8::Interpreter::new(rom),
        }
    }

    pub fn update_keypad(&mut self, keypad: Vec<u8>) {
        let keypad = keypad
            .iter()
            .map(|x| *x == 1)
            .collect::<Vec<bool>>()
            .try_into()
            .unwrap();
        self.interpreter.update_keypad(keypad);
    }

    pub fn step(&mut self) {
        self.interpreter.step();
    }

    pub fn should_redraw(&self) -> bool {
        self.interpreter.should_redraw()
    }

    pub fn should_beep(&self) -> bool {
        self.interpreter.should_beep()
    }

    pub fn update_timers(&mut self) {
        self.interpreter.update_timers();
    }

    pub fn get_vram_ptr(&mut self) -> *const u8 {
        self.interpreter.get_vram_ptr()
    }

    pub fn get_ram_ptr(&mut self) -> *const u8 {
        self.interpreter.get_ram_ptr()
    }

    pub fn get_pc(&self) -> u16 {
        self.interpreter.cpu.get_pc()
    }

    pub fn get_v_registers_ptr(&self) -> *const u8 {
        self.interpreter.get_v_registers_ptr()
    }

    pub fn reset(&mut self) {
        self.interpreter.reset();
    }
}
