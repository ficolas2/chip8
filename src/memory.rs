use std::ops::{Index, IndexMut};

const STACK_START: usize = 0x000;
pub const STACK_SIZE: usize = 16;

pub struct Memory([u8; 0x1000]);

impl Index<usize> for Memory {
    type Output = u8;

    fn index(&self, index: usize) -> &u8 {
        &self.0[index]
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut u8 {
        &mut self.0[index]
    }
}

impl Memory {

    pub fn new() -> Memory {
        Memory([0; 0x1000])
    }

    pub fn set_stack_addr(&mut self, stack_pointer: usize, value: u16) {
        self.set_u16(STACK_START + stack_pointer * 2, value);
    }
    
    pub fn get_stack_addr(&self, stack_pointer: usize) -> u16 {
        self.get_u16(STACK_START + stack_pointer * 2)
    }

    pub fn get_u16(&self, addr: usize) -> u16 {
        let most_significant = self[addr] as u16;
        let least_significant = self[addr + 1] as u16;

        most_significant << 8 | least_significant
    }

    pub fn set_u16(&mut self, addr: usize, value: u16) {
        self[addr] = (value >> 8) as u8;
        self[addr + 1] = value as u8;
    }

}

