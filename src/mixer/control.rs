use std::sync::{Arc, Mutex};
use base32;
use mixer::Mixer;

pub struct Controller {
    pub sequence: Vec<Vec<Field>>,
    row: usize,
    col: usize,
    pub play: bool,
}

#[derive(Clone)]
pub struct Field {
    pub note: Note,
    pub cmd: Option<Command>,
}

const NUM_FIELDS: u32 = 4;

#[derive(Clone)]
pub enum Note {
    On(u8),
    Off,
    Hold,
}

impl Controller {
    pub fn new(seq: Vec<Vec<Field>>) -> Arc<Mutex<Controller>> {
        let ctrl = Controller {
            play: false,
            sequence: seq,
            row: 0,
            col: 0,
        };
        Arc::new(Mutex::new(ctrl))
    }
    pub fn next(&mut self) -> Vec<Field> {
        if self.play {
            self.move_cursor(0, 1);
        }
        self.sequence[self.row].clone()
    }
    pub fn row(&self) -> usize { self.row }
    pub fn col(&self) -> usize { self.col }
    pub fn num_cols(&self) -> u32 { self.sequence[0].len() as u32 * NUM_FIELDS }
    pub fn width(&self) -> u32 { self.sequence[0].len() as u32 * self.field_w() }
    pub fn height(&self) -> u32 { self.sequence.len() as u32 }
    pub fn field_w(&self) -> u32 { 6 }
    pub fn set_note(&mut self, note: Note) {
        if self.col % 4 == 0 {
            self.sequence[self.row][self.col/4].note = note;
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
            .take(self.sequence[0].len())
            .collect();
        self.sequence.insert(self.row+1, blank_row);
        self.row += 1;
    }
    pub fn remove(&mut self) {
        if self.sequence.len() > 1 {
            self.sequence.remove(self.row);
            self.row = if self.row == 0 {0} else {self.row - 1}
        }
    }
}

impl Field {
    pub fn string(&self) -> String {
        let mut out = String::new();
        match self.note {
            Note::On(ref note) => {
                const NOTE_NAME: &'static str = "C-C#D-D#E-F-F#G-G#A-A#B-";
                let name = *note as usize % 12;
                out.push_str(&NOTE_NAME[name*2..name*2+2]);
                let octave = note / 12;
                out.push_str(&format!("{}", octave));
            },
            Note::Off => out.push_str("---"),
            Note::Hold => out.push_str("   "),
        }
        match self.cmd {
            Some(ref cmd) => out.push_str(
                &format!("{}{:02X}", cmd.id as char, cmd.data)),
            None => out.push_str("   "),
        }
        out
    }
}

#[derive(Clone)]
pub struct Command {
    id: u8,
    data: u8,
}

impl Command {
    pub fn from_str(raw: &str) -> Command {
        let mut chars = raw.chars();
        Command {
            id: base32::from_char(chars.next().unwrap()),
            data: u8::from_str_radix(chars.as_str(), 16).unwrap(),
        }
    }
    pub fn execute(&self, m: &mut Mixer) {
        match self.id as char {
            '2' => {
                if self.data < 32 {
                    m.tick_rate = self.data
                } else {
                    m.bpm = self.data }},
            _ => eprintln!("invalid command!"),
        }
    }
}
