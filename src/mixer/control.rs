use base32;
use mixer::Mixer;

#[derive(Clone)]
pub struct Field {
    pub cmd: Option<Command>,
    pub note: Option<u8>,
}

impl Field {
    pub fn string(&self) -> String {
        let mut out = String::new();
        match self.note {
            Some(ref note) => {
                const NOTE_NAME: &'static str = "C-C#D-D#E-F-F#G-G#A-A#B-";
                let name = *note as usize % 12;
                out.push_str(&NOTE_NAME[name*2..name*2+2]);
                let octave = note / 12;
                out.push_str(&format!("{}", octave));
            }
            None => out.push_str("   "),
        }
        match self.cmd {
            Some(ref cmd) => out.push_str(
                &format!("{}{:02X}", cmd.id as char, cmd.data)),
            None => out.push_str("   "),
        }
        out
    }
}

pub struct Controller {
    pub sequence: Vec<Field>,
    pos: usize,
}

impl Controller {
    pub fn new(seq: Vec<Field>) -> Controller {
        let pos = seq.len() - 1;
        Controller {
            sequence: seq,
            pos: pos,
        }
    }
    pub fn next(&mut self) -> Field {
        self.pos = (self.pos + 1) % self.sequence.len();
        self.sequence[self.pos].clone()
    }
    pub fn pos(&self) -> usize { self.pos }
    pub fn set_note(&mut self, note: Option<u8>) {
        self.sequence[self.pos].note = note;
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
