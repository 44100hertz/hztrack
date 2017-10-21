use std::sync::Arc;

use mixer::*;

use base32;

pub struct Track {
    seq:        Vec<Vec<Field>>,
    row:        usize,
    tick_count: u32,
    bpm:        u8,
    tick_rate:  u8,
    pcm:        Arc<Vec<i8>>,
    output:     MixerIn,
}

impl Track {
    pub fn new(seq: Vec<Vec<Field>>) -> Self {
        Track {
            seq: seq,
            row: 0,
            tick_count: 0,
            bpm: 120,
            tick_rate: 6,
            pcm: Arc::new((0..256)
                .map(|i| ((i as f64 / 128.0 * 3.1415).sin() * 127.0) as i8)
                .collect()),
            output: MixerIn::new(),
        }
    }
}

impl Controller for Track {
    fn next(&mut self) -> MixerIn {
        const DEFAULT_CHAN: ChannelIn = ChannelIn {
            note:       60<<8,
            pcm_off:    0,
            pcm_len:    256,
            pcm_rate:   256,
            vol:        0,
        };

        self.output.chan.resize(self.seq[self.row].len(), DEFAULT_CHAN);
        for i in 0..self.width() {
            let chan = &mut self.output.chan[i];
            let field = &self.seq[self.row][i];
            match field.note {
                Note::On(n) => {
                    chan.note = (n as u16)<<8;
                    chan.vol  = 64;
                }
                Note::Off => chan.vol = 0,
                Note::Hold => {},
            }
            // todo: handle field command
        }

        self.row = (self.row + 1) % self.seq.len();

        MixerIn {
            tick_rate: self.bpm as u16 * self.tick_rate as u16,
            pcm: self.pcm.clone(),
            chan: self.output.chan.clone(),
        }
    }
}
impl Track {
    pub fn width(&self) -> usize { self.seq[self.row].len() }
    pub fn row(&mut self) -> &mut Vec<Field> { &mut self.seq[self.row] }
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
    pub fn hi(&self) -> u8 { self.data >> 4 }
    pub fn lo(&self) -> u8 { self.data & 0xf }
    pub fn set_hi(&mut self, v: u8) { self.data = self.lo() + (v << 4) }
    pub fn set_lo(&mut self, v: u8) { self.data = (self.data & 0xf0) + (v & 0xf) }
}
//      '0' => {
//          arp = match self.tick_count % 3 {
//              0 => 0,
//              1 => field.cmd.hi() as u16,
//              2 => field.cmd.lo() as u16,
//              _ => unreachable!(),
//          };
//      }
//      '1' => chan.note += (field.cmd.data as u16)<<4,
//      '2' => chan.note =
//          match chan.note.checked_sub((field.cmd.data as u16)<<4) {
//              Some(v) => v,
//              None => 0,
//          },
//      'F' => {
//          if field.cmd.data < 32 {
//              self.tick_rate = field.cmd.data + 1
//          } else {
//              self.bpm = field.cmd.data
//          }
//      }
//      'B' => self.ctrl.lock().unwrap().jump_pos(field.cmd.data),
//      c @ _ => eprintln!("unknown command id: {}", c),
