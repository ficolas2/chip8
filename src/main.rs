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
    let mut chip8 = Chip8::new();
    chip8.run();
}
