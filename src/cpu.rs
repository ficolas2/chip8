use crate::memory::{self, Memory};

pub struct Cpu {
    pub registers: [u8; 16],
    pub program_counter: usize,
    pub stack_pointer: usize,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: [0; 16],
            program_counter: 0,
            stack_pointer: 0,
        }
    }

    fn read_opcode(&self, memory: &Memory) -> u16 {
        let p = self.program_counter;
        let most_significant = memory[p] as u16;
        let least_significant = memory[p + 1] as u16;

        most_significant << 8 | least_significant
    }

    pub fn run(&mut self, memory: &mut Memory, screen: &mut [[bool; 32]; 64]) {
        loop {
            let opcode = self.read_opcode(memory);
            self.program_counter += 2;

            let c = ((opcode & 0xF000) >> 12) as u8;
            let x = ((opcode & 0x0F00) >> 8) as u8;
            let y = ((opcode & 0x00F0) >> 4) as u8;
            let d = (opcode & 0x000F) as u8;

            let n = (opcode & 0x000F) as u8;
            let nn = (opcode & 0x00FF) as u8;
            let nnn = opcode & 0x0FFF;

            #[rustfmt::skip]
            match (c, x, y, d) {
                (  0,   0,   0,   0) => { return; }
                (  0,   0, 0xE,   0) => self.clear_screen(screen),
                (  0,   0, 0xE, 0xE) => self.ret(memory),
                (0x1,   _,   _,   _) => self.jump(nnn),
                (0x2,   _,   _,   _) => self.call(nnn, memory),
                (0x6,   _,   _,   _) => self.set_x(x, nn),
                (0x7,   _,   _,   _) => self.add_x(x, nn),
                (0x8,   _,   _, 0x4) => self.add_xy(x, y),
                _ => panic!("Unknown opcode: {:x}", opcode),
            };
        }
    }

    fn clear_screen(&mut self, screen: &mut [[bool; 32]; 64]) {
        screen
            .iter_mut()
            .for_each(|r| r.iter_mut().for_each(|p| *p = false));
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
}

#[cfg(test)]
macro_rules! cpu_test {
    ({$($op:expr),+ $(,)?} [ $($reg_in:expr),+  $(,)?] => [ $($reg_out:expr),+  $(,)?]) =>

    {
    #[allow(unused_assignments)]
    {
        let mut cpu = Cpu {
            registers: [0; 16],
            program_counter: 0x100,
            ..Cpu::new()
        };

        let mut memory = Memory::new();
        let mut screen = [[false; 32]; 64];

        let mut p = 0;
        $(
        cpu.registers[p] = $reg_in;
        p += 1;
        )+

        p = cpu.program_counter;
        $(
            memory[p] = (($op & 0xFF00) >> 8) as u8;
            memory[p + 1] = ($op & 0x00FF) as u8;
            p += 2;
        )+
        cpu.run(&mut memory, &mut screen);

        p = 0;
        $(
            p += 1;
            assert_eq!(cpu.registers[p - 1], $reg_out);
        )+
    }};
}

#[cfg(test)]
macro_rules! op {
    (CALL $loc:literal) => {
        0x2000 | $loc
    };
    (END) => {
        0x0000
    };
    (ADD $reg0:literal $reg1:literal) => {
        0x8004 | ($reg0 << 8) | ($reg1 << 4)
    };
    (RET) => {
        0x00EE
    };
    ($lit:literal) => {
        $lit
    };
}

#[test]
fn test_add_xy() {
    cpu_test!({ 0x8014 }         [0x10, 0x20] => [0x30, 0x00]);
    cpu_test!({ 0x8014 }         [0xFF, 0x01] => [0x00, 0x01]); //Overflow
    cpu_test!({ 0x8014, 0x8014 } [0xFF, 0x01] => [0x01, 0x00]); //Overflow

    cpu_test!({ 0x8014, 0x8024, 0x8034 } [0x01, 0x02, 0x03, 0x04] => [0x0A, 0x00, 0x03, 0x04]);
}

#[test]
fn call_and_ret() {
    println!("{:x}", op!(ADD 0x0 0x1));
    cpu_test!(
        {
            op!(CALL 0x106),
            op!(END),
            op!(0xFFFF),
            op!(ADD 0x0 0x1),
            op!(RET),
        }
        [10, 20] => [30, 00]
    );
}
