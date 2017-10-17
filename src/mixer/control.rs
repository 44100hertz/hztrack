use base32;
use mixer::Mixer;

pub trait Controller {
    fn next(&mut self) -> Vec<Field>;
    fn jump_pos(&mut self, row: u8);
}

#[derive(Clone)]
pub struct Field {
    pub note: Note,
    pub cmd: Command,
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
            }
            Note::Off => out.push_str("---"),
            Note::Hold => out.push_str("   "),
        }
        out.push_str(&format!("{}{:02X}", self.cmd.id as char, self.cmd.data));
        out
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
impl Command {
    pub fn zero() -> Command { Command { id: '0' as u8, data: 0 } }
    pub fn from_str(raw: &str) -> Command {
        let mut chars = raw.chars();
        Command {
            id: base32::from_char(chars.next().unwrap()),
            data: u8::from_str_radix(chars.as_str(), 16).unwrap(),
        }
    }
    pub fn execute<C: Controller>(&self, m: &mut Mixer<C>, chan: usize) {
        let mut c = m.ctrl.lock().unwrap();
        match self.id as char {
            '0' => m.chan[chan].arp = [self.hi(), self.lo()],
            '2' => {
                if self.data < 32 {
                    m.tick_rate = self.data
                } else {
                    m.bpm = self.data
                }},
            'B' => c.jump_pos(self.data),
            c @ _ => eprintln!("unknown command id: {}", c),
        }
    }
    pub fn hi(&self) -> u8 { self.data >> 4 }
    pub fn lo(&self) -> u8 { self.data & 0xf }
    pub fn set_hi(&mut self, v: u8) { self.data = self.lo() + (v << 4) }
    pub fn set_lo(&mut self, v: u8) { self.data = (self.data & 0xf0) + (v & 0xf) }
}
