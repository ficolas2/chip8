#[cfg(test)]
use crate::assembler;
use rand::Rng;

use crate::{
    keyboard::Keyboard, memory::{self, Memory}, screen::Screen, timers::Timers
};

pub struct Cpu {
    v: [u8; 16],
    pc: usize,
    stack_pointer: usize,
    i: u16,

    y_shift: bool,
}

impl Cpu {
    pub fn new(flags: &[String]) -> Cpu {
        Cpu {
            v: [0; 16],                // Registers
            pc: memory::PROGRAM_START, // Program counter
            stack_pointer: 0,          // Stack pointer
            i: 0,                      // Index register
            y_shift: flags.iter().any(|s| s == "--yshift"),
        }
    }

    fn read_opcode(&self, memory: &Memory) -> u16 {
        let p = self.pc;
        let most_significant = memory[p] as u16;
        let least_significant = memory[p + 1] as u16;

        most_significant << 8 | least_significant
    }

    #[rustfmt::skip]
    pub fn run(&mut self, memory: &mut Memory, screen: &mut Screen, keyboard: &mut Keyboard, timers: &mut Timers) -> bool {
        let opcode = self.read_opcode(memory);
        self.pc += 2;

        let c = ((opcode & 0xF000) >> 12) as u8;
        let x = ((opcode & 0x0F00) >> 8) as u8;
        let y = ((opcode & 0x00F0) >> 4) as u8;
        let d = (opcode & 0x000F) as u8;

        let n = (opcode & 0x000F) as u8;
        let nn = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        let x_val = self.v[x as usize];
        let y_val = self.v[y as usize];

        match (c, x, y, d) {
            (  0,   0,   0,   0) => { return false;}
            (  0,   0, 0xE,   0) => self.clear_screen(screen),
            (  0,   0, 0xE, 0xE) => self.ret(memory),
            (0x1,   _,   _,   _) => self.jump(nnn),
            (0x2,   _,   _,   _) => self.call(nnn, memory),
            (0x3,   _,   _,   _) => self.skip_if_eq(x_val, nn),
            (0x4,   _,   _,   _) => self.skip_if_neq(x_val, nn),
            (0x5,   _,   _,   0) => self.skip_if_eq(x_val, y_val),
            (0x6,   _,   _,   _) => self.v[x as usize] = nn, // vX := nn
            (0x7,   _,   _,   _) => { 
                self.v[x as usize] = self.v[x as usize].wrapping_add(nn) 
            }, // add vX nn
            (0x8,   _,   _, 0x0) => self.v[x as usize] = self.v[y as usize],
            (0x8,   _,   _, 0x1) => self.v[x as usize] = x_val | y_val, // or
            (0x8,   _,   _, 0x2) => self.v[x as usize] = x_val & y_val, // and
            (0x8,   _,   _, 0x3) => self.v[x as usize] = x_val ^ y_val, // xor
            (0x8,   _,   _, 0x4) => self.add_xy(x, y),
            (0x8,   _,   _, 0x5) => self.sub_xy(x, y),
            (0x8,   _,   _, 0x6) => self.shift_right(x, y),
            (0x8,   _,   _, 0x7) => self.rsb_xy(x, y),
            (0x8,   _,   _, 0xE) => self.shift_left(x, y),
            (0x9,   _,   _,   _) => self.skip_if_neq(x_val, y_val),
            (0xA,   _,   _,   _) => self.i = nnn, // i := nnn
            (0xB,   _,   _,   _) => self.pc = nnn as usize + self.v[0] as usize,
            (0xC,   _,   _,   _) => self.rand(x, nn),
            (0xD,   _,   _,   _) => self.draw_xyn(memory, screen, x, y, n),
            (0xE,   _, 0x9, 0xE) => self.skip_if_key(keyboard, x_val, true),
            (0xE,   _, 0xA, 0x1) => self.skip_if_key(keyboard, x_val, false),
            (0xF,   _, 0x1, 0xE) => self.i += x_val as u16,
            (0xF,   _, 0x0, 0xA) => self.wait_for(keyboard[x_val as usize]),
            (0xF,   _, 0x2, 0x9) => self.set_i_to_font_addr(x_val),
            (0xF,   _, 0x3, 0x3) => self.bcd_x_to_i(memory, x),
            (0xF,   _, 0x5, 0x5) => self.store_reg_at_i(memory, x),
            (0xF,   _, 0x6, 0x5) => self.load_reg_at_i(memory, x),
            (0xF,   _, 0x0, 0x7) => self.v[x as usize] = timers.delay,
            (0xF,   _, 0x1, 0x5) => timers.delay = x_val,
            (0xF,   _, 0x1, 0x8) => timers.sound = x_val,
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
        self.pc = memory.get_stack_addr(self.stack_pointer) as usize;
    }

    fn jump(&mut self, addr: u16) {
        self.pc = addr as usize;
    }

    fn call(&mut self, addr: u16, memory: &mut Memory) {
        if self.stack_pointer > memory::STACK_SIZE {
            panic!("Stack overflow");
        }

        memory.set_stack_addr(self.stack_pointer, self.pc as u16);
        self.stack_pointer += 1;
        self.pc = addr as usize;
    }

    fn skip_if_eq<T>(&mut self, x: T, nn: T) 
    where 
        T: PartialEq
    {
        if x == nn {
            self.pc += 2;
        }
    }

    fn skip_if_neq<T>(&mut self, x: T, nn: T) 
    where 
        T: PartialEq
    {
        if x != nn {
            self.pc += 2;
        }
    }

    fn skip_if_key(&mut self, keyboard: &mut Keyboard, key: u8, val: bool) {
        self.skip_if_eq(keyboard[key as usize], val);
        keyboard[key as usize] = false;
    }

    fn wait_for(&mut self, val: bool) {
        if !val {
            self.pc -= 2;
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let x_val = self.v[x as usize];
        let y_val = self.v[y as usize];

        let (result, overflow) = x_val.overflowing_add(y_val);

        self.v[x as usize] = result;
        self.v[0xF] = overflow as u8;
    }

    fn sub_xy(&mut self, x: u8, y: u8) {
        let x_val = self.v[x as usize];
        let y_val = self.v[y as usize];
        let (result, overflow) = x_val.overflowing_sub(y_val);

        self.v[x as usize] = result;
        self.v[0xF] = !overflow as u8;
    }

    fn rsb_xy(&mut self, x: u8, y: u8) {
        let x_val = self.v[x as usize];
        let y_val = self.v[y as usize];
        let (result, overflow) = y_val.overflowing_sub(x_val);

        self.v[x as usize] = result;
        self.v[0xF] = !overflow as u8;
    }

    fn shift_left(&mut self, x: u8, y: u8) {
        let val = if self.y_shift {
            self.v[y as usize]
        } else {
            self.v[x as usize]
        };

        self.v[x as usize] = val << 1;
        self.v[0xF] = val >> 7;
    }

    fn shift_right(&mut self, x: u8, y: u8) {
        let val = if self.y_shift {
            self.v[y as usize]
        } else {
            self.v[x as usize]
        };

        self.v[x as usize] = val >> 1;
        self.v[0xF] = val & 1;
    }

    fn bcd_x_to_i(&self, memory: &mut Memory, x: u8) {
        let x_val = self.v[x as usize];

        memory[self.i as usize] = (x_val / 100) % 10;
        memory[self.i as usize + 1] = (x_val / 10) % 10;
        memory[self.i as usize + 2] = x_val % 10;
    }

    fn draw_xyn(&mut self, memory: &Memory, screen: &mut Screen, x: u8, y: u8, n: u8) {
        self.v[0xF] = 0;
        let x_val = self.v[x as usize] as usize % 64;
        let y_val = self.v[y as usize] as usize % 32;

        for row in 0..n as usize {
            let sprite = memory[row + self.i as usize];

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
                    self.v[0xF] = 1;
                }
                screen[screen_x][screen_y] = bit ^ screen_state;
            }
        }
    }

    fn store_reg_at_i(&self, memory: &mut Memory, x: u8) {
        for i in 0..=x as usize {
            memory[self.i as usize + i] = self.v[i];
        }
    }

    fn load_reg_at_i(&mut self, memory: &Memory, x: u8) {
        for i in 0..=x as usize {
            self.v[i] = memory[self.i as usize + i];
        }
    }

    fn set_i_to_font_addr(&mut self, x: u8) {
        self.i = (memory::FONT_START as u16) + (x as u16) * 5;
    }

    fn rand(&mut self, x: u8, nn: u8) {
        self.v[x as usize] = rand::thread_rng().gen_range(0..=nn);
    }
}

#[cfg(test)]
macro_rules! cpu_test {
    ($asm:literal [ $($reg_in:expr),+  $(,)?] => [ $($reg_out:expr),+  $(,)?]) =>

    {
    #[allow(unused_assignments)]
    {
        let mut cpu = Cpu {
            v: [0; 16],
            pc: memory::PROGRAM_START,
            ..Cpu::new(&[])
        };

        let mut memory = Memory::new();
        let mut screen = Screen::new();
        let mut keyboard = Keyboard::new();

        let mut p = 0;
        $(
        cpu.v[p] = $reg_in;
        p += 1;
        )+

        let code = assembler::assemble($asm);
        memory.load_program(&code);

        let mut count = 0;
        while cpu.run(&mut memory, &mut screen, &mut keyboard) {
            count += 1;
            if count > 10000 {
                panic!("Looped for too long for a test (10000 iterations)");
            }
        }

        p = 0;
        $(
            p += 1;

            assert_eq!(
                cpu.v[p - 1], $reg_out,
                "v{} is not equal to 0x{:x}, it is 0x{:x}",
                p - 1,
                $reg_out,
                cpu.v[p-1],
            );
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
        [10, 20] => [30, 20]
    );
}

#[test]
fn test_skip() {
    cpu_test!("skeq v0 0x10; add v1 0x1" [0x10, 0x00] => [0x10, 0x00]);
    cpu_test!("skeq v0 v1;   add v1 0x2" [0x01, 0x01] => [0x01, 0x01]);
    cpu_test!("skne v0 0x10; add v1 0x1" [0x10, 0x00] => [0x10, 0x01]);
    cpu_test!("skne v0 v1;   add v1 0x2" [0x01, 0x01] => [0x01, 0x03]);

    cpu_test!("skeq v0 0x10; add v1 0x1" [0x11, 0x00] => [0x11, 0x01]);
    cpu_test!("skeq v0 v1;   add v1 0x2" [0x02, 0x01] => [0x02, 0x03]);
    cpu_test!("skne v0 0x10; add v1 0x1" [0x11, 0x00] => [0x11, 0x00]);
    cpu_test!("skne v0 v1;   add v1 0x2" [0x02, 0x01] => [0x02, 0x01]);
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
    cpu_test!("add v0 v1" [0x10, 0x20] => [0x30, 0x20]);
    cpu_test!("add v0 v1" [0xFF, 0x01] => [
        0x00, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01
    ]); //Overflow
    cpu_test!("add v0 v1; add v0 v1" [0xFF, 0x01] => [
        0x01, 0x01, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00
    ]); //Overflow
}

#[test]
fn test_sub_xy() {
    cpu_test!("sub v0 v1" [0x20, 0x10] => [
        0x10, 0x10, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01
    ]);
    cpu_test!("sub v0 v1" [0x10, 0x20] => [
        0xF0, 0x20, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00
    ]);
    cpu_test!("rsb v0 v1" [0x20, 0x10] => [
        0xf0, 0x10, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00
    ]);
    cpu_test!("rsb v0 v1" [0x10, 0x20] => [
        0x10, 0x20, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01
    ]);
}

#[test]
fn test_bitwise() {
    cpu_test!("or v0 v1" [0b001, 0b011] => [0b011]);
    cpu_test!("and v0 v1" [0b001, 0b011] => [0b001]);
    cpu_test!("xor v0 v1" [0b001, 0b011] => [0b010]);
}

#[test]
fn test_shift() {
    cpu_test!("shr v0" [0b011] => [
        0b01, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01
    ]);
    cpu_test!("shl v0" [0b10000001] => [
        0b10, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x01
    ]);
}

#[test]
fn test_load_and_store() {
    cpu_test!(r#"
            mvi 0x250
            str v2
            mov v0 0x00
            mov v1 0x00
            mov v2 0x00
            ldr v2
        "#
        [1, 2,  3] => [1, 2, 3]
    )
}

#[test]
fn test_bce() {
    cpu_test!(r#"
            mvi 0x250
            bcd v0
            ldr v2
        "#
        [123] => [1, 2, 3]
    )
}
