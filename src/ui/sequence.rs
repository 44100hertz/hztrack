use sdl2::keyboard::Scancode;

use mixer::control::*;
use std::sync::{Arc, Mutex};

use base32;
use ui::keyboard;

pub struct Sequence {
    pub pattern: Vec<Vec<Field>>,
    row: usize,
    col: usize,
    pub play: bool,
}

pub enum Column {
    Note,
    CommandId,
    CommandHi,
    CommandLo,
}
impl Column {
    fn width(&self) -> u32 {
        match *self {
            Column::Note => 3,
            _ => 1,
        }
    }
    fn handle_key(&self, seq: &mut Sequence, sc: Scancode) {
        match *self {
            Column::Note => {
                let note = keyboard::to_note(sc);
                match note {
                    Note::Hold => {}
                    _ => seq.set_note(note + 60),
                }
            },
            Column::CommandId => {
                let name = sc.name();
                if name.len() == 1 {
                    let id = base32::from_char(name.chars().next().unwrap());
                    if base32::contains(id as char) {
                        seq.set_cmd_id(id);
                    }
                }
            }
            _ => eprintln!("unimplemented"),
        }
    }
}

impl Controller for Sequence {
    fn next(&mut self) -> Vec<Field> {
        let ret = self.pattern[self.row].clone();
        if self.play {
            self.move_cursor(0, 1);
        }
        ret
    }
    fn jump_pos(&mut self, row: u8) {
        if (row as u32) < self.height() {
            self.row = row as usize;
        } else {
            self.error("attempt to jump to non-existant row");
        }
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

    pub fn field_w(&self) -> u32 { 6 }
    pub fn width(&self) -> u32 {
        self.pattern[0].len() as u32 * self.field_w()
    }
    pub fn height(&self) -> u32 { self.pattern.len() as u32 }
    pub fn num_cols(&self) -> u32 {
        self.pattern[0].len() as u32 * 4
    }
    pub fn col_type(&self) -> Column {
        match self.col() % 4 {
            0 => Column::Note,
            1 => Column::CommandId,
            2 => Column::CommandHi,
            3 => Column::CommandLo,
            _ => panic!("You're in a negative column..."),
        }
    }
    pub fn cursor(&self) -> (i32, i32, u32, u32) {
        let x = (self.col() + ((self.col()+3)/4)*2) as i32;
        let y = self.row() as i32;
        let w = self.col_type().width();
        let h = 1;
        (x, y, w, h)
    }

    pub fn set_note(&mut self, note: Note) {
        self.pattern[self.row][self.col/4].note = note;
    }
    pub fn set_cmd_id(&mut self, id: u8) {
        let cmd = &mut self.pattern[self.row][self.col/4].cmd;
        match *cmd {
            Some(ref mut c) => c.id = id,
            None => *cmd = Some(Command{ id: id, data: 0 }),
        }
    }
    pub fn move_cursor(&mut self, dx: i32, dy: i32) {
        let modulus = |a, b| {
            ((a % b + b) % b) as usize
        };
        self.row = modulus(self.row as i32 + dy, self.height() as i32);
        self.col = modulus(self.col as i32 + dx, self.num_cols() as i32);
    }
    pub fn insert(&mut self) {
        let mut blank_row = vec![];
        blank_row.resize(
            self.pattern[0].len(),
            Field{note: Note::Hold, cmd: None});
        self.pattern.insert(self.row+1, blank_row);
        self.row += 1;
    }
    pub fn remove(&mut self) {
        if self.pattern.len() > 1 {
            self.pattern.remove(self.row);
            self.row = if self.row == 0 {0} else {self.row - 1}
        }
    }

    pub fn error(&self, err: &str) {
        eprintln!("Error on row: {}\n\t{}", self.row(), err)
    }

    pub fn handle_key(&mut self, sc: Scancode) {
        match sc {
//            Scancode::PageUp    => self.octave_up(),
//            Scancode::PageDown  => self.octave_down(),

            Scancode::Up    => self.move_cursor(0, -1),
            Scancode::Down  => self.move_cursor(0, 1),
            Scancode::Left  => self.move_cursor(-1, 0),
            Scancode::Right => self.move_cursor(1, 0),

            Scancode::Insert => self.insert(),
            Scancode::Delete => self.remove(),

            Scancode::Space => self.play = !self.play,

            _ => self.col_type().handle_key(self, sc),
        };
    }
}
