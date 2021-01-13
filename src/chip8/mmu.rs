// Memory map:
//
// 0x000-0x1FF - Chip 8 interpreter, which contains the fontset at: 0x050-0x0A0
// 0x200-0xFFF - Program ROM and work RAM

pub const FONT_BASE_ADDR: usize = 0x050;
pub const ROM_BASE_ADDR: usize = 0x200;

const RAM_SIZE: usize = 0x1000;

pub struct MMU {
    rom: Vec<u8>,
    ram: [u8; RAM_SIZE],
}

impl MMU {
    pub fn new(rom: Vec<u8>) -> Self {
        let mut mmu = MMU {
            rom,
            ram: [0; RAM_SIZE],
        };
        mmu.reset();
        mmu
    }

    pub fn load_rom(&mut self, rom: Vec<u8>) {
        self.rom = rom;
        self.reset();
    }

    pub fn reset(&mut self) {
        self.ram = [0; RAM_SIZE];
        // Load fontset.
        for (i, b) in [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ]
        .iter()
        .enumerate()
        {
            self.ram[FONT_BASE_ADDR + i] = *b;
        }
        // Load the ROM in the "work RAM".
        for (i, b) in self.rom.iter().enumerate() {
            self.ram[ROM_BASE_ADDR + i] = *b;
        }
    }

    pub fn read_byte(&mut self, addr: usize) -> u8 {
        self.ram[addr]
    }

    pub fn write_byte(&mut self, addr: usize, value: u8) {
        self.ram[addr] = value
    }

    pub fn read_word(&mut self, addr: usize) -> u16 {
        ((self.read_byte(addr) as u16) << 8) | (self.read_byte(addr + 1) as u16)
    }

    pub fn get_ram_ptr(&self) -> *const u8 {
        self.ram.as_ptr()
    }
}
