use std::fmt;
use base32;

pub struct Sequence {
    // TODO: make private
    pub fields: Vec<Vec<Field>>,
}

#[derive(Clone)]
pub struct Field {
    pub note: Note,
    pub cmd:  Command,
}
#[derive(Clone)]
pub enum Note {
    On(u8),
    Off,
    Hold,
}
#[derive(Clone)]
pub struct Command {
    pub id: u8,
    pub data: u8,
}

impl Sequence {
    pub fn new(fields: Vec<Vec<Field>>) -> Self {
        Sequence {
            fields: fields
        }
    }
    pub fn get_field(&self, row: usize, col: usize) -> &Field {
        &self.fields[row][col]
    }
    pub fn width(&self) -> usize {
        self.fields[0].len()
    }
    pub fn len(&self) -> usize {
        self.fields.len()
    }
}

impl fmt::Display for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{:02X}", self.note, self.cmd.id as char, self.cmd.data)
    }
}

impl ::std::ops::Add<u8> for Note {
    type Output = Note;
    fn add(self, with: u8) -> Note {
        match self {
            Note::On(v) => Note::On(v + with),
            _ => self
        }
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Note::On(ref note) => {
                const NOTE_NAME: &'static str = "C-C#D-D#E-F-F#G-G#A-A#B-";
                let name = *note as usize % 12;
                let octave = note / 12;
                write!(f, "{}{}", &NOTE_NAME[name*2..name*2+2], octave)
            }
            Note::Off => write!(f, "---"),
            Note::Hold => write!(f, "   "),
        }
    }
}

impl Command {
    pub fn zero() -> Command { Command { id: '0' as u8, data: 0 } }
    pub fn from_str(raw: &str) -> Command {
        let mut chars = raw.chars();
        Command {
            id: base32::from_char(chars.next().unwrap()).unwrap(),
            data: u8::from_str_radix(chars.as_str(), 16).unwrap(),
        }
    }
    pub fn hi(&self) -> u8 { self.data >> 4 }
    pub fn lo(&self) -> u8 { self.data & 0xf }
    pub fn set_hi(&mut self, v: u8) { self.data = self.lo() + (v << 4) }
    pub fn set_lo(&mut self, v: u8) { self.data = (self.data & 0xf0) + (v & 0xf) }
}
