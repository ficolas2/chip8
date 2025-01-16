use std::env;

use cpu::Cpu;
use memory::Memory;
use screen::Screen;

mod assembler;


mod fonts;
mod cpu;
mod memory;
mod screen;

struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    screen: Screen,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            screen: Screen::new(),
        }
    }

    pub fn run(&mut self) {
        self.cpu.run(&mut self.memory, &mut self.screen);
    }
}

fn main() {
    // Get first argument as ROM file
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("Usage: chip8 <rom file>");
    }

    let mut chip8 = Chip8::new();
    let rom = std::fs::read(&args[1]).expect("Failed to read ROM file");

    chip8.memory.load_fonts(&fonts::FONT);
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
