use std::convert::TryInto;
use wasm_bindgen::prelude::*;

use crate::chip8;

#[wasm_bindgen]
pub struct JsInterpreter {
    interpreter: chip8::Interpreter,
    // We need a framebuffer in the JsInterpreter in order to read it in the WASM memory.
    framebuffer: [u8; chip8::WIDTH * chip8::HEIGHT],
}

#[wasm_bindgen]
impl JsInterpreter {
    #[wasm_bindgen(constructor)]
    pub fn new(rom: Vec<u8>) -> Self {
        JsInterpreter {
            interpreter: chip8::Interpreter::new(rom),
            framebuffer: [0; chip8::WIDTH * chip8::HEIGHT],
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

    pub fn get_framebuffer_ptr(&mut self) -> *const u8 {
        for x in 0..chip8::WIDTH {
            for y in 0..chip8::HEIGHT {
                self.framebuffer[x + (y * chip8::WIDTH)] = if self.interpreter.get_vram()[x][y] {
                    1
                } else {
                    0
                };
            }
        }

        self.framebuffer.as_ptr()
    }
}
