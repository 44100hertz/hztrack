use mixer::control::*;
use std::sync::{Arc, Mutex};

const NUM_FIELDS: u32 = 4;

pub struct Sequence {
    pub pattern: Vec<Vec<Field>>,
    row: usize,
    col: usize,
    pub play: bool,
}

impl Controller for Sequence {
    fn next(&mut self) -> Vec<Field> {
        if self.play {
            self.move_cursor(0, 1);
        }
        self.pattern[self.row].clone()
    }
}

impl Sequence {
    pub fn new(seq: Vec<Vec<Field>>) -> Arc<Mutex<Self>> {
        let ctrl = Self {
            play: false,
            pattern: seq,
            row: 0,
            col: 0,
        };
        Arc::new(Mutex::new(ctrl))
    }
    pub fn row(&self) -> usize { self.row }
    pub fn col(&self) -> usize { self.col }
    pub fn num_cols(&self) -> u32 { self.pattern[0].len() as u32 * NUM_FIELDS }
    pub fn width(&self) -> u32 { self.pattern[0].len() as u32 * self.field_w() }
    pub fn height(&self) -> u32 { self.pattern.len() as u32 }
    pub fn field_w(&self) -> u32 { 6 }
    pub fn set_note(&mut self, note: Note) {
        if self.col % 4 == 0 {
            self.pattern[self.row][self.col/4].note = note;
        }
    }
    pub fn cursor(&self) -> (i32, i32, u32, u32) {
        let x = (self.col() + ((self.col()+3)/4)*2) as i32;
        let y = self.row() as i32;
        let w = match self.col % NUM_FIELDS as usize {
            0 => 3,
            _ => 1,
        };
        let h = 1;
        (x, y, w, h)
    }
    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        let modulus = |a, b| {
            ((a % b + b) % b) as usize
        };
        self.row = modulus(self.row as i32 + dy, self.height() as i32);
        self.col = modulus(self.col as i32 + dx, self.num_cols() as i32);
    }
    pub fn insert(&mut self) {
        let blank_row: Vec<_> = ::std::iter::repeat(Field{note: Note::Hold, cmd: None})
            .take(self.pattern[0].len())
            .collect();
        self.pattern.insert(self.row+1, blank_row);
        self.row += 1;
    }
    pub fn remove(&mut self) {
        if self.pattern.len() > 1 {
            self.pattern.remove(self.row);
            self.row = if self.row == 0 {0} else {self.row - 1}
        }
    }
}
