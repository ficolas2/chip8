use std::io::Write;
use std::{
    io::stdout,
    ops::{Index, IndexMut},
};
use termion::{clear, cursor, raw::IntoRawMode};

pub struct Screen {
    current: [[bool; 32]; 64],
    previous: [[bool; 32]; 64],
    redraw: bool,
    last_draw: std::time::Instant,
    stdout: termion::raw::RawTerminal<std::io::Stdout>,
}

impl Screen {
    pub fn new() -> Screen {
        let mut stdout = stdout().into_raw_mode().unwrap();
        write!(stdout, "{}", clear::All).unwrap();
        Screen {
            current: [[false; 32]; 64],
            previous: [[false; 32]; 64],
            redraw: false,
            last_draw: std::time::Instant::now(),
            stdout,
        }
    }

    pub fn draw(&mut self) {
        if self.last_draw.elapsed().as_millis() < 1000 / 60 {
            return;
        }
        self.last_draw = std::time::Instant::now();
        if !self.redraw {
            return;
        }
        for y in 0..32 {
            for x in 0..64 {
                if self.current[x][y] != self.previous[x][y] {
                    write!(
                        self.stdout,
                        "{}{}",
                        cursor::Goto((x * 2 + 1) as u16, (y + 1) as u16),
                        if self.current[x][y] { "██" } else { "  " }
                    )
                    .unwrap();
                    self.previous[x][y] = self.current[x][y];
                }
            }
        }
        self.stdout.flush().unwrap();
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
