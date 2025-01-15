use cpu::Cpu;
use memory::Memory;

mod cpu;
mod memory;

struct Chip8 {
    cpu: Cpu,
    memory: Memory,
}

impl Chip8 {
    pub fn new () -> Chip8 {
        Chip8 {
            cpu: Cpu::new(),
            memory: Memory::new(),
        }
    }

    pub fn run(&mut self) {
        self.cpu.run(&mut self.memory);
    }
}

fn main() {

    let mut chip8 = Chip8::new();
    chip8.run();
}
