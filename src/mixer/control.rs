use base32;
use mixer::Mixer;

#[derive(Clone)]
pub enum Note {
    On(u8),
    Off,
    Hold,
}

#[derive(Clone)]
pub struct Field {
    pub cmd: Option<Command>,
    pub note: Note,
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

pub struct Controller {
    pub sequence: Vec<Field>,
    pos: usize,
    pub play: bool,
}
impl Controller {
    pub fn new(seq: Vec<Field>) -> Controller {
        let pos = 0;
        Controller {
            play: false,
            sequence: seq,
            pos: pos,
        }
    }
    pub fn next(&mut self) -> Field {
        if self.play {
            self.scroll(1);
        }
        self.sequence[self.pos].clone()
    }
    pub fn pos(&self) -> usize { self.pos }
    pub fn set_note(&mut self, note: Note) {
        self.sequence[self.pos].note = note;
    }
    pub fn scroll(&mut self, amt: i64) {
        let pos = self.pos as i64 + amt;
        self.pos = if pos < 0 {
            self.sequence.len() - 1
        } else {
            pos as usize % self.sequence.len()
        }
    }
    pub fn insert(&mut self) {
        self.sequence.insert(self.pos+1, Field{note: Note::Hold, cmd: None});
        self.pos += 1;
    }
    pub fn remove(&mut self) {
        if self.sequence.len() > 1 {
            self.sequence.remove(self.pos);
            self.pos = if self.pos == 0 {0} else {self.pos - 1}
        }
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
