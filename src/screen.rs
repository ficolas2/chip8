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
    stdout: Option<termion::raw::RawTerminal<std::io::Stdout>>,
    size: (u16, u16),
    full_redraw: bool,
}

impl Screen {
    pub fn new(raw: bool) -> Screen {
        let stdout = if raw {
            let mut stdout = stdout().into_raw_mode().unwrap();
            write!(stdout, "{}", clear::All).unwrap();
            Some(stdout)
        } else {
            None
        };
        Screen {
            current: [[false; 32]; 64],
            previous: [[false; 32]; 64],
            redraw: false,
            last_draw: std::time::Instant::now(),
            stdout,
            size: termion::terminal_size().unwrap(),
            full_redraw: true,
        }
    }

    pub fn draw(&mut self) {
        if self.stdout.is_none() {
            return;
        }
        let stdout = self.stdout.as_mut().unwrap();
        write!(
            stdout,
            "{}{} x {}",
            cursor::Goto(1, 33),
            self.size.0,
            self.size.1
        )
        .unwrap();

        if self.last_draw.elapsed().as_millis() < 1000 / 60 {
            return;
        }
        self.last_draw = std::time::Instant::now();
        if !self.redraw {
            return;
        }
        for y in 0..32 {
            for x in 0..64 {
                if self.current[x][y] != self.previous[x][y] || self.full_redraw {
                    write!(
                        stdout,
                        "{}{}",
                        cursor::Goto((x * 2 + 1) as u16, (y + 1) as u16),
                        if self.current[x][y] { "██" } else { "  " }
                    )
                    .unwrap();
                    self.previous[x][y] = self.current[x][y];
                }
            }
        }
        stdout.flush().unwrap();

        if let Ok(size) = termion::terminal_size() {
            if size != self.size {
                self.size = size;
                self.full_redraw = true;
                self.redraw = true;
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
