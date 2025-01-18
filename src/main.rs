use std::{env, io::Write, thread, time::Duration};

use cpu::Cpu;
use keyboard::Keyboard;
use memory::Memory;
use screen::Screen;

mod assembler;

mod cpu;
mod fonts;
mod keyboard;
mod memory;
mod screen;
mod timers;

const USAGE: &str = r#"
Usage: chip8 <rom file>
Flags:
    --assemble=<asm file>: create <rom file> from <asm file>
    --yshift: allows specifying a vY register for the 8xy6 and 8xyE instructions
    --clock-speed=n: allows specifying the clock speed (n) in Hz
"#;

struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    screen: Screen,
    keyboard: keyboard::Keyboard,
    timers: timers::Timers,
    clock_speed: u64,
}

impl Chip8 {
    pub fn new(flags: Vec<String>) -> Chip8 {
        let mut chip8 = Chip8 {
            cpu: Cpu::new(&flags),
            memory: Memory::new(),
            screen: Screen::new(true),
            keyboard: Keyboard::new(),
            timers: timers::Timers::new(),
            clock_speed: 700,
        };
        if let Some(clock_speed_str) = flags.iter().find(|f| f.starts_with("--clock-speed=")) {
            chip8.clock_speed = clock_speed_str
                .strip_prefix("--clock-speed=")
                .unwrap()
                .parse()
                .expect("Invalid clock speed");
        }

        chip8
    }

    pub fn run(&mut self) {
        loop {
            thread::sleep(Duration::from_nanos(1_000_000_000 / self.clock_speed));

            self.timers.update();
            self.keyboard.update();
            let cont = self
                .cpu
                .run(&mut self.memory, &mut self.screen, &mut self.keyboard, &mut self.timers);
            if !cont {
                break;
            }

            self.screen.draw();
        }
    }
}

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.is_empty() {
        println!("{}", USAGE);
        return;
    }

    let flags: Vec<String> = args
        .iter()
        .filter(|str| str.starts_with("-"))
        .cloned()
        .collect();

    if flags.iter().any(|f| f.starts_with("--assemble=")) {
        let asm_path = flags
            .iter()
            .find(|f| f.starts_with("--assemble="))
            .unwrap()
            .strip_prefix("--assemble=")
            .unwrap();
        let rom_path = args
            .iter()
            .find(|str| !str.starts_with("-"))
            .expect("No ROM file specified");
        let asm_str = std::fs::read_to_string(asm_path).expect("Failed to read ASM file");
        let bytecode = assembler::assemble(&asm_str);

        println!("Writing ROM to {}", rom_path);
        let mut file = std::fs::File::create(rom_path).expect("Failed to create ROM file");
        file.write_all(&bytecode).expect("Failed to write ROM file");
        return;
    }

    let rom_path = args
        .iter()
        .find(|str| !str.starts_with("-"))
        .expect("No ROM file specified");
    let mut chip8 = Chip8::new(flags);
    let rom = std::fs::read(rom_path).expect("Failed to read ROM file");

    println!("Loading ROM {}", rom_path);

    chip8.memory.load_fonts(fonts::FONT);
    chip8.memory.load_program(&rom);

    chip8.run();
}
