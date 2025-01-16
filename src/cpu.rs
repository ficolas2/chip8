use crate::{
    memory::{self, Memory},
    screen::Screen,
    assembler
};

pub struct Cpu {
    pub registers: [u8; 16],
    pub program_counter: usize,
    pub stack_pointer: usize,
    pub i_register: u16,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            program_counter: memory::PROGRAM_START,
            stack_pointer: 0,
            i_register: 0,
        }
    }

    fn read_opcode(&self, memory: &Memory) -> u16 {
        let p = self.program_counter;
        let most_significant = memory[p] as u16;
        let least_significant = memory[p + 1] as u16;

        most_significant << 8 | least_significant
    }

    #[rustfmt::skip]
    pub fn run(&mut self, memory: &mut Memory, screen: &mut Screen) -> bool {
        let opcode = self.read_opcode(memory);
        self.program_counter += 2;

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match (c, x, y, d) {
            (  0,   0,   0,   0) => { return false;}
            (  0,   0, 0xE,   0) => self.clear_screen(screen),
            (  0,   0, 0xE, 0xE) => self.ret(memory),
            (0x1,   _,   _,   _) => self.jump(nnn),
            (0x2,   _,   _,   _) => self.call(nnn, memory),
            (0x6,   _,   _,   _) => self.set_x(x, nn),
            (0x7,   _,   _,   _) => self.add_x(x, nn),
            (0x8,   _,   _, 0x4) => self.add_xy(x, y),
            (0xA,   _,   _,   _) => self.set_i(nnn),
            (0xD,   _,   _,   _) => self.draw_xyn(memory, screen, x, y, n),
            _ => {}
            // _ => panic!("Unknown opcode: {:x}", opcode),
        };
        true
    }

    fn clear_screen(&mut self, screen: &mut Screen) {
        screen.clear();
    }

    fn ret(&mut self, memory: &mut Memory) {
        if self.stack_pointer == 0 {
            panic!("Stack underflow");
        }

        self.stack_pointer -= 1;
        self.program_counter = memory.get_stack_addr(self.stack_pointer) as usize;
    }

    fn jump(&mut self, addr: u16) {
        self.program_counter = addr as usize;
    }

    fn call(&mut self, addr: u16, memory: &mut Memory) {
        if self.stack_pointer > memory::STACK_SIZE {
            panic!("Stack overflow");
        }

        memory.set_stack_addr(self.stack_pointer, self.program_counter as u16);
        self.stack_pointer += 1;
        self.program_counter = addr as usize;
    }

    fn skip_if_eq(&mut self, x: u8, nn: u8) {
        if x == nn {
            self.program_counter += 2;
        }
    }

    fn skip_if_neq(&mut self, x: u8, nn: u8) {
        if x != nn {
            self.program_counter += 2;
        }
    }

    fn set_x(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] = nn;
    }

    fn add_x(&mut self, x: u8, nn: u8) {
        self.registers[x as usize] += nn;
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let x_val = self.registers[x as usize];
        let y_val = self.registers[y as usize];

        let (result, overflow) = x_val.overflowing_add(y_val);

        self.registers[x as usize] = result;
        self.registers[1] = overflow as u8;
    }

    fn set_i(&mut self, nnn: u16) {
        self.i_register = nnn;
    }

    fn draw_xyn(&mut self, memory: &Memory, screen: &mut Screen, x: u8, y: u8, n: u8) {
        self.registers[0xF] = 0;
        let x_val = self.registers[x as usize] as usize % 64;
        let y_val = self.registers[y as usize] as usize % 32;

        for row in 0..n as usize {
            let sprite = memory[row + self.i_register as usize];

            for col in 0..8 {
                let screen_x = x_val + col;
                let screen_y = y_val + row;

                if screen_x >= 64 {
                    break;
                }
                if screen_y >= 32 {
                    return;
                }

                let bit = ((sprite >> (7 - col)) & 1) != 0;
                let screen_state = screen[screen_x][screen_y];

                if bit && screen_state {
                    self.registers[0xF] = 1;
                }
                screen[screen_x][screen_y] = bit ^ screen_state;
            }
        }
    }
}

#[cfg(test)]
macro_rules! cpu_test {
    ($asm:literal [ $($reg_in:expr),+  $(,)?] => [ $($reg_out:expr),+  $(,)?]) =>

    {
    #[allow(unused_assignments)]
    {
        let mut cpu = Cpu {
            registers: [0; 16],
            program_counter: memory::PROGRAM_START,
            ..Cpu::new()
        };

        let mut memory = Memory::new();
        let mut screen = Screen::new();

        let mut p = 0;
        $(
        cpu.registers[p] = $reg_in;
        p += 1;
        )+

        let code = assembler::assemble($asm);
        memory.load_program(&code);

        let mut count = 0;
        while cpu.run(&mut memory, &mut screen) {
            count += 1;
            if count > 10000 {
                panic!("Looped for too long for a test (10000 iterations)");
            }
        }

        p = 0;
        $(
            p += 1;
            assert_eq!(cpu.registers[p - 1], $reg_out);
        )+
    }};
}

#[test]
fn test_call_and_ret() {
    cpu_test!(r#"
            jsr 0x204
            end
            add v0 v1
            rts
            add v0 0x2
        "#
        [10, 20] => [30, 00]
    );
}

#[test]
fn test_jump() {
    cpu_test!(r#"
            jmp 0x206
            add v0 0x2
            add v0 0x5
        "# 
        [0x01, 0x01] => [0x01, 0x01]
    );
}

#[test]
fn test_set_x() {
    cpu_test!("mov v2 0x10" [0x01, 0x02] => [0x01, 0x02, 0x10]);
}

#[test]
fn test_add_x() {
    cpu_test!("add v0 0x10" [0x10, 0x20] => [0x20, 0x20]);
}

#[test]
fn test_add_xy() {
    cpu_test!("add v0 v1" [0x10, 0x20] => [0x30, 0x00]);
    cpu_test!("add v0 v1" [0xFF, 0x01] => [0x00, 0x01]); //Overflow
    cpu_test!("add v0 v1; add v0 v1" [0xFF, 0x01] => [0x01, 0x00]); //Overflow

    cpu_test!(r#"
            add v0 v1
            add v0 v2
            add v0 v3
        "#
        [0x01, 0x02, 0x03, 0x04] => [0x0A, 0x00, 0x03, 0x04]
    );
}
