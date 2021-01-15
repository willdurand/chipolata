use rand::Rng;
use std::fmt;

use super::mmu;

pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 32;

#[derive(Default)]
pub struct Registers {
    // Note: the VF register doubles as a flag for some instructions; thus, it should be avoided.
    // In an addition operation, VF is the carry flag, while in subtraction, it is the "no borrow"
    // flag. In the draw instruction VF is set upon pixel collision.
    pub v: [u8; 16],
    // Index register
    pub i: usize,
    // Program counter
    pub pc: usize,
    // Stack pointer
    pub sp: usize,
    // This timer is intended to be used for timing the events of games. Its value can be set and
    // read.
    pub delay: u8,
    // This timer is used for sound effects. When its value is nonzero, a beeping sound is made.
    pub sound: u8,
}

impl fmt::Debug for Registers {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  v0={:02X} v1={:02X} v2={:02X} v3={:02X} v4={:02X} v5={:02X} v6={:02X} v7={:02X}\
            \n  v8={:02X} v9={:02X} va={:02X} vb={:02X} vc={:02X} vd={:02X} ve={:02X} vf={:02X}\
            \n  i={:04X} pc={:04X} sp={:04X}\
            \n  delay={:02X} sound={:02X}\
            \n",
            self.v[0],
            self.v[1],
            self.v[2],
            self.v[3],
            self.v[4],
            self.v[5],
            self.v[6],
            self.v[7],
            self.v[8],
            self.v[9],
            self.v[10],
            self.v[11],
            self.v[12],
            self.v[13],
            self.v[14],
            self.v[15],
            self.i,
            self.pc,
            self.sp,
            self.delay,
            self.sound
        )
    }
}

#[derive(Default)]
struct Keypad {
    pub state: [bool; 16],
    pub waiting: bool,
    pub register: usize,
}

impl fmt::Debug for Keypad {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            // 1 2 3 C
            // 4 5 6 D
            // 7 8 9 E
            // A 0 B F
            "  {} {} {} {}\
            \n  {} {} {} {}\
            \n  {} {} {} {}\
            \n  {} {} {} {}\
            \n  waiting={:?} register={:02X}",
            self.state[1] as i32,
            self.state[2] as i32,
            self.state[3] as i32,
            self.state[0xC] as i32,
            self.state[4] as i32,
            self.state[5] as i32,
            self.state[6] as i32,
            self.state[0xD] as i32,
            self.state[7] as i32,
            self.state[8] as i32,
            self.state[9] as i32,
            self.state[0xE] as i32,
            self.state[0xA] as i32,
            self.state[0] as i32,
            self.state[0xB] as i32,
            self.state[0xF] as i32,
            self.waiting,
            self.register
        )
    }
}

pub struct CPU {
    pub mmu: mmu::MMU,

    pub vram: [u8; HEIGHT * WIDTH],
    vram_changed: bool,

    pub registers: Registers,
    // The stack is only used to store return addresses when subroutines are called.
    stack: [u16; 16],
    keypad: Keypad,

    rng: rand::rngs::ThreadRng,
    debug: bool,
}

impl CPU {
    pub fn new(mmu: mmu::MMU) -> Self {
        let mut cpu = CPU {
            mmu,
            vram: [0; HEIGHT * WIDTH],
            vram_changed: false,
            registers: Registers::default(),
            stack: [0; 16],
            keypad: Keypad::default(),
            rng: rand::thread_rng(),
            debug: false,
        };
        cpu.reset();
        cpu
    }

    pub fn enable_debug(&mut self) {
        self.debug = true;
    }

    pub fn disable_debug(&mut self) {
        self.debug = false;
    }

    pub fn fetch_instruction(&mut self) -> u16 {
        return self.mmu.read_word(self.registers.pc);
    }

    pub fn step(&mut self, keypad: [bool; 16]) {
        self.vram_changed = false;
        self.keypad.state = keypad;

        if self.keypad.waiting {
            for i in 0..=15 {
                if self.keypad.state[i] {
                    self.keypad.waiting = false;
                    self.registers.v[self.keypad.register] = i as u8;
                    self.registers.pc += 2;
                    break;
                }
            }
        } else {
            let opcode = self.fetch_instruction();

            if self.debug {
                println!(
                    "Executing opcode=0x{:04X} (pc=0x{:04X})",
                    opcode, self.registers.pc
                );
            }

            self.registers.pc += 2;

            self.execute(opcode);
        }
    }

    pub fn update_timers(&mut self) {
        if self.registers.delay > 0 {
            self.registers.delay -= 1;
        }

        if self.registers.sound > 0 {
            self.registers.sound -= 1;
        }
    }

    pub fn should_redraw(&self) -> bool {
        self.vram_changed
    }

    pub fn should_beep(&self) -> bool {
        self.registers.sound > 0
    }

    pub fn reset(&mut self) {
        self.mmu.reset();
        self.vram = [0; HEIGHT * WIDTH];
        self.vram_changed = false;
        self.registers = Registers {
            pc: mmu::ROM_BASE_ADDR,
            ..Registers::default()
        };
        self.stack = [0; 16];
        self.keypad = Keypad::default();
    }

    pub fn read_byte(&mut self, addr: u16) -> u8 {
        self.mmu.read_byte(addr as usize)
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.reset();
        self.mmu.load_rom(rom);
    }

    fn execute(&mut self, opcode: u16) {
        let x = ((opcode & 0x0F00) >> 8) as usize;
        let y = ((opcode & 0x00F0) >> 4) as usize;

        return match opcode & 0xF000 {
            0x0000 => match opcode & 0x000F {
                // disp_clear() (0x00E0)
                0x0000 => {
                    for x in 0..WIDTH {
                        for y in 0..HEIGHT {
                            self.vram[x + y * WIDTH] = 0;
                        }
                    }
                    self.vram_changed = true;
                }
                // return (0x00EE)
                0x000E => {
                    self.registers.sp -= 1;
                    self.registers.pc = self.stack[self.registers.sp] as usize;
                }
                _ => self.unsupported_opcode(opcode),
            },
            // goto NNN;
            0x1000 => {
                self.registers.pc = (opcode & 0x0FFF) as usize;
            }
            // *(0xNNN)()
            0x2000 => {
                self.stack[self.registers.sp] = self.registers.pc as u16;
                self.registers.sp += 1;
                self.registers.pc = (opcode & 0x0FFF) as usize;
            }
            // if (Vx == NN)
            0x3000 => {
                let val = (opcode & 0x00FF) as u8;

                if self.registers.v[x] == val {
                    self.registers.pc += 2;
                }
            }
            // if (Vx != NN)
            0x4000 => {
                let val = (opcode & 0x00FF) as u8;

                if self.registers.v[x] != val {
                    self.registers.pc += 2;
                }
            }
            // if (Vx == Vy)
            0x5000 => {
                if self.registers.v[x] == self.registers.v[y] {
                    self.registers.pc += 2;
                }
            }
            // Vx = NN
            0x6000 => {
                self.registers.v[x] = (opcode & 0x00FF) as u8;
            }
            // Vx += NN
            0x7000 => {
                let val = self.registers.v[x] as u16 + (opcode & 0x00FF) as u16;
                self.registers.v[x] = val as u8;
            }
            0x8000 => self.op_8xy(opcode, x, y),

            // if (Vx != Vy)
            0x9000 => {
                if self.registers.v[x] != self.registers.v[y] {
                    self.registers.pc += 2;
                }
            }
            // I = NNN
            0xA000 => {
                self.registers.i = (opcode & 0x0FFF) as usize;
            }
            // PC = V0 + NNN
            0xB000 => {
                self.registers.pc = (self.registers.v[0] as u16 + (opcode & 0x0FFF)) as usize;
            }
            // Vx = rand() & NN
            0xC000 => {
                self.registers.v[x] = self.rng.gen::<u8>() & (opcode & 0x00FF) as u8;
            }
            // draw(Vx, Vy, N)
            0xD000 => {
                let height = (opcode & 0x000F) as usize;
                let vx = self.registers.v[x] as usize;
                let vy = self.registers.v[y] as usize;

                self.registers.v[0xF] = 0;

                for yline in 0..height {
                    let pixel = self.mmu.read_byte(self.registers.i + yline);

                    for xline in 0..8 {
                        let vram_x = (vx + xline) % WIDTH;
                        let vram_y = (vy + yline) % HEIGHT;

                        if (pixel & (0x80 >> xline)) != 0 {
                            if self.vram[vram_x + vram_y * WIDTH] == 1 {
                                self.registers.v[0xF] |= 1;
                            }

                            self.vram[vram_x + vram_y * WIDTH] ^= 1;
                        }
                    }
                }

                self.vram_changed = true;
            }
            0xE000 => self.op_ex(opcode, x),
            0xF000 => self.op_fx(opcode, x),
            _ => self.unsupported_opcode(opcode),
        };
    }

    fn op_8xy(&mut self, opcode: u16, x: usize, y: usize) {
        match opcode & 0x000F {
            // Vx = Vy
            0 => {
                self.registers.v[x] = self.registers.v[y];
            }
            // Vx = Vx | Vy
            1 => {
                self.registers.v[x] |= self.registers.v[y];
            }
            // Vx = Vx & Vy
            2 => {
                self.registers.v[x] &= self.registers.v[y];
            }
            // Vx = Vx ^ Vy
            3 => {
                self.registers.v[x] ^= self.registers.v[y];
            }
            // Vx += Vy
            4 => {
                let val = self.registers.v[x] as u16 + self.registers.v[y] as u16;

                self.registers.v[x] = val as u8;
                self.registers.v[0xF] = if val > 0xFF { 1 } else { 0 };
            }
            // Vx -= Vy
            5 => {
                self.registers.v[0xF] = if self.registers.v[x] > self.registers.v[y] {
                    1
                } else {
                    0
                };
                self.registers.v[x] = self.registers.v[x].wrapping_sub(self.registers.v[y]);
            }
            // Vx = Vx >> 1
            6 => {
                self.registers.v[0xF] = self.registers.v[x] & 1;
                self.registers.v[x] >>= 1;
            }
            // Vx = Vy - Vx
            7 => {
                self.registers.v[0xF] = if self.registers.v[x] > self.registers.v[y] {
                    0
                } else {
                    1
                };
                self.registers.v[x] = self.registers.v[y].wrapping_sub(self.registers.v[x]);
            }
            // Vx = Vx << 1
            0xE => {
                self.registers.v[0xF] = (self.registers.v[x] >> 7) & 1;
                self.registers.v[x] <<= 1;
            }
            _ => self.unsupported_opcode(opcode),
        };
    }

    fn op_ex(&mut self, opcode: u16, x: usize) {
        match (opcode & 0x00FF) as u8 {
            // if (key() == Vx)
            0x009E => {
                if self.keypad.state[self.registers.v[x] as usize] {
                    self.registers.pc += 2;
                }
            }
            // if (key() == Vx)
            0x00A1 => {
                if !self.keypad.state[self.registers.v[x] as usize] {
                    self.registers.pc += 2;
                }
            }
            _ => self.unsupported_opcode(opcode),
        };
    }

    fn op_fx(&mut self, opcode: u16, x: usize) {
        return match (opcode & 0x00FF) as u8 {
            // Vx = get_delay()
            0x0007 => {
                self.registers.v[x] = self.registers.delay;
            }
            // Vx = get_key()
            0x000A => {
                self.keypad.waiting = true;
                self.keypad.register = x;
            }
            // delay_timer(Vx)
            0x0015 => {
                self.registers.delay = self.registers.v[x];
            }
            // sound_timer(Vx)
            0x0018 => {
                self.registers.sound = self.registers.v[x];
            }
            // I += Vx
            0x001E => {
                self.registers.i += self.registers.v[x] as usize;
                self.registers.v[0xF] = if self.registers.i > 0x0F00 { 1 } else { 0 };
            }
            // I = sprite_addr[Vx]
            0x0029 => {
                self.registers.i = mmu::FONT_BASE_ADDR + (self.registers.v[x] as usize) * 5;
            }
            // set_BCD(Vx)
            // *(I+0) = BCD(3)
            // *(I+1) = BCD(2)
            // *(I+2) = BCD(1)
            0x0033 => {
                let val = self.registers.v[x];

                self.mmu.write_byte(self.registers.i, val / 100);
                self.mmu.write_byte(self.registers.i + 1, (val % 100) / 10);
                self.mmu.write_byte(self.registers.i + 2, val % 10);
            }
            // reg_dump(Vx, &I)
            0x0055 => {
                for i in 0..=x {
                    self.mmu
                        .write_byte(self.registers.i + i, self.registers.v[i]);
                }
            }
            // reg_load(Vx, &I)
            0x0065 => {
                for i in 0..=x {
                    self.registers.v[i] = self.mmu.read_byte(self.registers.i + i);
                }
            }
            _ => self.unsupported_opcode(opcode),
        };
    }

    fn unsupported_opcode(&self, opcode: u16) {
        panic!(
            "unsupported opcode 0x{:04X} @ ${:04X}\n{:?}",
            opcode, self.registers.pc, self
        );
    }
}

impl fmt::Debug for CPU {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Registers:\n{:?}\
            Keypad:\n{:?}",
            self.registers, self.keypad
        )
    }
}
