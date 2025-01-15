use std::env;

use cpu::Cpu;
use memory::Memory;

mod cpu;
mod memory;

struct Chip8 {
    cpu: Cpu,
    memory: Memory,
    screen: [[bool; 32]; 64],
}

impl Chip8 {
    pub fn new() -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
            screen: [[false; 32]; 64],
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

    for (i, byte) in rom.iter().enumerate() {
        chip8.memory[i + 0x200] = *byte;
    }
    println!();

    loop {
        chip8.run();
        for y in 0..32 {
            for x in 0..64 {
                print!("{}", if chip8.screen[x][y] { "██" } else { "  " });
            }
            println!();
        }
    }

}
