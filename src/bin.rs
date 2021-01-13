extern crate minifb;
extern crate rodio;

use minifb::{Key, Scale, Window, WindowOptions};
use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::process;
use structopt::StructOpt;

use libchipolata::chip8;

#[derive(StructOpt)]
struct Cli {
    /// Enable debug mode (debugger).
    #[structopt(short, long)]
    debug: bool,
    /// The path to a ROM.
    #[structopt(parse(from_os_str))]
    rom_name: std::path::PathBuf,
    #[structopt(default_value = "5", long)]
    speed: u8,
}

fn read_keypad(window: &Window) -> [bool; 16] {
    // 1 2 3 C -> 1 2 3 4
    // 4 5 6 D -> Q W E R
    // 7 8 9 E -> A S D F
    // A 0 B F -> Z X C V
    let keypad: [bool; 16] = [
        window.is_key_down(Key::X),    // 0
        window.is_key_down(Key::Key1), // 1
        window.is_key_down(Key::Key2), // 2
        window.is_key_down(Key::Key3), // 3
        window.is_key_down(Key::Q),    // 4
        window.is_key_down(Key::W),    // 5
        window.is_key_down(Key::E),    // 6
        window.is_key_down(Key::A),    // 7
        window.is_key_down(Key::S),    // 8
        window.is_key_down(Key::D),    // 9
        window.is_key_down(Key::Z),    // A
        window.is_key_down(Key::C),    // B
        window.is_key_down(Key::Key4), // C
        window.is_key_down(Key::R),    // D
        window.is_key_down(Key::F),    // E
        window.is_key_down(Key::V),    // F
    ];
    keypad
}

fn main() {
    // CLI
    let args = Cli::from_args();
    let rom_name = args.rom_name;
    let mut file = File::open(&rom_name).unwrap();
    let mut rom = Vec::new();
    file.read_to_end(&mut rom).unwrap();
    let speed = args.speed;

    // Chip8
    let mut interpreter = chip8::Interpreter::new(rom);

    // Debugger
    let mut stepping = false;
    let mut boot = true;
    let mut address_breakpoints = HashSet::new();
    let mut opcode_breakpoints: HashSet<u16> = HashSet::new();

    // Graphics
    let mut window = Window::new(
        format!("chipolata - {} - ESC to exit", rom_name.to_str().unwrap()).as_str(),
        chip8::WIDTH,
        chip8::HEIGHT,
        WindowOptions {
            borderless: false,
            resize: false,
            scale: Scale::X8,
            title: true,
            ..WindowOptions::default()
        },
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut buffer: Vec<u32> = vec![0; chip8::WIDTH * chip8::HEIGHT];

    // Audio
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    let source = rodio::source::SineWave::new(400);
    sink.append(source);
    sink.pause();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut redraw = false;

        if args.debug && boot {
            boot = false;
        } else {
            for _ in 0..speed {
                interpreter.update_keypad(read_keypad(&window));
                interpreter.step();

                if interpreter.should_redraw() {
                    redraw = true;
                }
            }
        }

        if (args.debug && interpreter.cpu.get_pc() == 0x200)
            || address_breakpoints.contains(&interpreter.cpu.get_pc())
            || opcode_breakpoints.contains(&interpreter.cpu.fetch_instruction())
            || window.is_key_down(Key::O)
        {
            stepping = true;
            interpreter.cpu.enable_debug();
            println!("Breakpoint hit at 0x{:04X}", interpreter.cpu.get_pc());
        }

        if stepping {
            loop {
                let mut input = String::new();
                print!(">>> ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut input).unwrap();
                input = input[..input.len() - 1].to_string();

                if input.len() == 0 {
                    continue;
                } else if input == "q" {
                    print!("Exiting... o/");
                    process::exit(0);
                } else if input == "p cpu" {
                    println!("{:?}", interpreter.cpu);
                } else if input.starts_with("p ") {
                    if let Ok(address) = u16::from_str_radix(&input[2..], 16) {
                        println!("0x{:04X}", interpreter.cpu.read_byte(address));
                    } else {
                        println!("Invalid address: {:?}", &input[2..]);
                    }
                } else if input == "s" {
                    let keypad = read_keypad(&window);
                    interpreter.cpu.step(keypad);
                } else if input.starts_with("s ") {
                    if let Ok(mut n) = u16::from_str_radix(&input[2..], 10) {
                        while n > 0 {
                            let keypad = read_keypad(&window);
                            interpreter.cpu.step(keypad);
                            n -= 1;
                        }
                    } else {
                        println!("Invalid number: {:?}", &input[3..]);
                    }
                } else if input == "c" {
                    interpreter.cpu.disable_debug();
                    stepping = false;
                    break;
                } else if input.starts_with("ba ") {
                    if let Ok(address) = u16::from_str_radix(&input[3..], 16) {
                        println!("Added breakpoint for address 0x{:04X}", address);
                        address_breakpoints.insert(address);
                    } else {
                        println!("Invalid address: {:?}", &input[3..]);
                    }
                } else if input.starts_with("bo ") {
                    if let Ok(op) = u16::from_str_radix(&input[3..], 16) {
                        println!("Added breakpoint for opcode 0x{:04X}", op);
                        opcode_breakpoints.insert(op);
                    } else {
                        println!("Invalid opcode: {:?}", &input[3..]);
                    }
                } else if input == "clear" {
                    address_breakpoints.clear();
                    opcode_breakpoints.clear();
                    println!("cleared breakpoints!");
                } else if input == "r" {
                    interpreter.reset();
                    println!("reset!");
                } else {
                    if input != "help" {
                        print!("Invalid command. ");
                    }
                    println!("Available commands:");
                    println!("");
                    println!("  ba [u16] : set breakpoint at address [u16]");
                    println!("  bo [u8]  : set breakpoint for opcode [u8]");
                    println!("  c        : continue");
                    println!("  clear    : clear breakpoints");
                    println!("  p cpu    : print cpu info");
                    println!("  q        : exit");
                    println!("  s        : step");
                    println!("  s [u16]  : step [u16] times");
                    println!("  r        : reset");
                }
            }
        }

        if redraw {
            for x in 0..chip8::WIDTH {
                for y in 0..chip8::HEIGHT {
                    let i = x + (y * chip8::WIDTH);
                    buffer[i] = if interpreter.get_vram()[i] == 1 {
                        0xFFFFFF
                    } else {
                        0x0
                    };
                }
            }
        }

        if interpreter.should_beep() {
            sink.play();
        } else {
            sink.pause();
        }

        interpreter.update_timers();

        window
            .update_with_buffer(&buffer, chip8::WIDTH, chip8::HEIGHT)
            .unwrap();
    }
}
