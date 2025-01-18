use std::ops::{Index, IndexMut};

use termion::input::TermRead;

pub struct Keyboard {
    keys: [bool; 16],
    keys_iter: termion::input::Keys<termion::AsyncReader>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            keys: [false; 16],
            keys_iter: termion::async_stdin().keys(),
        }
    }

    pub fn update(&mut self) {
        for key in self.keys_iter.by_ref() {
            match key.unwrap() {
                termion::event::Key::Char('1') => self.keys[0x1] = true,
                termion::event::Key::Char('2') => self.keys[0x2] = true,
                termion::event::Key::Char('3') => self.keys[0x3] = true,
                termion::event::Key::Char('4') => self.keys[0xC] = true,
                termion::event::Key::Char('q') => self.keys[0x4] = true,
                termion::event::Key::Char('w') => self.keys[0x5] = true,
                termion::event::Key::Char('e') => self.keys[0x6] = true,
                termion::event::Key::Char('r') => self.keys[0xD] = true,
                termion::event::Key::Char('a') => self.keys[0x7] = true,
                termion::event::Key::Char('s') => self.keys[0x8] = true,
                termion::event::Key::Char('d') => self.keys[0x9] = true,
                termion::event::Key::Char('f') => self.keys[0xE] = true,
                termion::event::Key::Char('z') => self.keys[0xA] = true,
                termion::event::Key::Char('x') => self.keys[0x0] = true,
                termion::event::Key::Char('c') => self.keys[0xB] = true,
                termion::event::Key::Char('v') => self.keys[0xF] = true,

                termion::event::Key::Ctrl('c') => std::process::exit(0),
                _ => {}
            }
        }
    }
}

impl Index<usize> for Keyboard {
    type Output = bool;

    fn index(&self, index: usize) -> &bool {
        &self.keys[index]
    }
}

impl IndexMut<usize> for Keyboard {
    fn index_mut(&mut self, index: usize) -> &mut bool {
        &mut self.keys[index]
    }
}
