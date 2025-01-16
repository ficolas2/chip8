use std::env;

use cpu::Cpu;
use memory::Memory;
use screen::Screen;

mod assembler;

mod cpu;
mod fonts;
mod memory;
mod screen;

const USAGE: &str = r#"
Usage: chip8 <rom file>
Flags:
    --yshift: allows specifying a vY register for the 8xy6 and 8xyE instructions
"#;

struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    screen: Screen,
}

impl Chip8 {
    pub fn new(flags: Vec<String>) -> Chip8 {
        Chip8 {
            cpu: Cpu::new(flags),
            memory: Memory::new(),
            screen: Screen::new(),
        }
    }

    pub fn run(&mut self) {
        self.cpu.run(&mut self.memory, &mut self.screen);
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

    let rom_path = args
        .iter()
        .find(|str| !str.starts_with("-"))
        .expect("No ROM file specified");
    let mut chip8 = Chip8::new(flags);
    let rom = std::fs::read(rom_path).expect("Failed to read ROM file");

    println!("Loading ROM {}", rom_path);

    chip8.memory.load_fonts(fonts::FONT);
    chip8.memory.load_program(&rom);

    let mut last_draw = std::time::Instant::now();
    loop {
        chip8.run();
        std::thread::sleep(std::time::Duration::from_millis(2));
        if last_draw.elapsed().as_millis() < 1000 / 60 {
            continue;
        }
        last_draw = std::time::Instant::now();
        chip8.screen.draw();
    }
}
