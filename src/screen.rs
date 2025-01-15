use std::ops::{Index, IndexMut};


pub struct Screen {
    current: [[bool; 32]; 64],
    redraw: bool,
}

impl Screen {
    pub fn new() -> Screen {
        Screen {
            current: [[false; 32]; 64],
            redraw: false,
        }
    }

    pub fn draw(&mut self) {
        if !self.redraw {
            return;
        }
        print!("\x1b[2J");

        for y in 0..32 {
            for x in 0..64 {
                print!("\x1b[{};{}H", y + 1, x * 2 + 1);
                print!("{}", if self.current[x][y] { "██" } else { "  " });
            }
        }
    }

    pub fn clear(&mut self) {
        for x in 0..64 {
            for y in 0..32 {
                self.current[x][y] = false;
            }
        }
    }

}

impl Index<usize> for Screen {
    type Output = [bool; 32];

    fn index(&self, index: usize) -> &[bool; 32] {
        &self.current[index]
    }
}

impl IndexMut<usize> for Screen {
    fn index_mut(&mut self, index: usize) -> &mut [bool; 32] {
        self.redraw = true;
        &mut self.current[index]
    }
}
