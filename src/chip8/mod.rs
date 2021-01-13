mod cpu;
mod mmu;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

pub struct Interpreter {
    // This has to be open for the debugger until I learn about a better way to do it.
    pub cpu: cpu::CPU,
    keypad: [bool; 16],
}

impl Interpreter {
    pub fn new(rom: Vec<u8>) -> Self {
        let mmu = mmu::MMU::new(rom);
        let cpu = cpu::CPU::new(mmu);

        Interpreter {
            cpu,
            keypad: [false; 16],
        }
    }

    pub fn update_keypad(&mut self, keypad: [bool; 16]) {
        self.keypad = keypad;
    }

    pub fn step(&mut self) {
        self.cpu.step(self.keypad);
    }

    pub fn should_redraw(&self) -> bool {
        self.cpu.should_redraw()
    }

    pub fn should_beep(&self) -> bool {
        self.cpu.should_beep()
    }

    pub fn update_timers(&mut self) {
        self.cpu.update_timers();
    }

    pub fn get_vram(&self) -> [u8; HEIGHT * WIDTH] {
        self.cpu.vram
    }

    pub fn get_vram_ptr(&self) -> *const u8 {
        self.cpu.vram.as_ptr()
    }

    pub fn reset(&mut self) {
        self.cpu.reset()
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.cpu.load_rom(rom);
    }
}
