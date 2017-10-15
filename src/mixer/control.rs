use std::sync::{Arc, Mutex};
use base32;
use mixer::Mixer;

pub struct Controller {
    pub sequence: Vec<Field>,
    row: usize,
    pub play: bool,
}

#[derive(Clone)]
pub struct Field {
    pub note: Note,
    pub cmd: Option<Command>,
}

#[derive(Clone)]
pub enum Note {
    On(u8),
    Off,
    Hold,
}

impl Controller {
    pub fn new(seq: Vec<Field>) -> Arc<Mutex<Controller>> {
        let ctrl = Controller {
            play: false,
            sequence: seq,
            row: 0,
        };
        Arc::new(Mutex::new(ctrl))
    }
    pub fn next(&mut self) -> Field {
        if self.play {
            self.scroll(1);
        }
        self.sequence[self.row].clone()
    }
    pub fn row(&self) -> usize { self.row }
    pub fn set_note(&mut self, note: Note) {
        self.sequence[self.row].note = note;
    }
    pub fn scroll(&mut self, amt: i64) {
        let row = self.row as i64 + amt;
        self.row = if row < 0 {
            self.sequence.len() - 1
        } else {
            row as usize % self.sequence.len()
        }
    }
    pub fn insert(&mut self) {
        self.sequence.insert(self.row+1, Field{note: Note::Hold, cmd: None});
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
