use std::sync::Arc;

use mixer::{Controller, MixerIn, ChannelIn};

use base32;

pub struct Track {
    pub seq:    Vec<Vec<Field>>,
    chan:       Vec<Channel>,
    row_jump:   Option<usize>,
    row:        usize,
    tick_count: u8,
    tick_rate:  u8,
    bpm:        u8,
    pcm:        Arc<Vec<i8>>,
}

#[derive(Clone)]
pub struct Field {
    pub note:   Note,
    pub cmd:    Command,
}

#[derive(Clone)]
pub enum Note {
    On(u8),
    Off,
    Hold,
}

#[derive(Clone)]
pub struct Command {
    pub id:     u8,
    pub data:   u8,
}
#[derive(Clone)]
pub struct Channel {
    note:  u16,
    add_note:   u16,
    porta_note: u8,
    cmd:        Command,
    vol:        i16,
}

impl Channel {
    fn new() -> Self {
        Channel {
            note: 0,
            add_note: 0,
            porta_note: 0,
            cmd: Command::zero(),
            vol: 0,
        }
    }
}
impl Track {
    pub fn new(seq: Vec<Vec<Field>>) -> Self {
        Track {
            seq: seq,
            chan: vec![],
            row: 0,
            row_jump: None,
            tick_count: 0,
            tick_rate: 6,
            bpm: 120,
            pcm: Arc::new((0..256)
                .map(|i| ((i as f64 / 128.0 * 3.1415).sin() * 127.0) as i8)
                .collect()),
        }
    }
    pub fn width(&self) -> usize { self.seq[self.row].len() }
    fn channel_beat(&mut self, i: usize) {
        let field = &self.seq[self.row][i];
        let chan = &mut self.chan[i];
        match field.note {
            Note::On(n) => {
                match field.cmd.id {
                    b'3' => chan.porta_note = n,
                    _ => chan.note = (n as u16)<<8,
                }
                chan.vol = 0x40;
            }
            Note::Off => chan.vol = 0,
            Note::Hold => {},
        }

        // effect memory: Only overwrite command data on a new id,
        // or on nonzero data.
        if field.cmd.data != 0 || field.cmd.id != chan.cmd.id {
            chan.cmd.data = field.cmd.data;
        }
        chan.cmd.id = field.cmd.id;
    }
    fn channel_tick(&mut self, i: usize) {
        let chan = &mut self.chan[i];
        let field = &self.seq[self.row][i];
        match field.cmd.id {
            b'0' => {
                chan.add_note =
                    // arpeggio has no effect memory;
                    // use the immediate command data.
                    match self.tick_count % 3 {
                        0 => 0,
                        1 => (field.cmd.hi() as u16)<<8,
                        2 => (field.cmd.lo() as u16)<<8,
                        _ => unreachable!(),
                    };
            }
            b'1' => chan.note = chan.note
                .saturating_add((chan.cmd.data as u16)<<4),
            b'2' => chan.note = chan.note
                .saturating_sub((chan.cmd.data as u16)<<4),
            b'3' => {
                use std::cmp::*;
                let pn = (chan.porta_note as u16)<<8;
                let rate = (chan.cmd.data as u16)<<4;
                if chan.note < pn {
                    chan.note = min(chan.note + rate, pn);
                } else if chan.note > pn {
                    chan.note = max(chan.note - rate, pn);
                }
            }
            b'F' => {
                if chan.cmd.data < 32 {
                    self.tick_rate = chan.cmd.data + 1
                } else {
                    self.bpm = chan.cmd.data
                }
            }
            b'B' => self.row_jump = Some(chan.cmd.data as usize),
            c @ _ => eprintln!("unknown command id: {}", c as char),
        }
    }
}

impl Controller for Track {
    fn next(&mut self) -> MixerIn {
        {
            let w = self.width();
            self.chan.resize(w, Channel::new());
        }
        if self.tick_count == self.tick_rate {
            self.tick_count = 0;
            self.row = self.row_jump.unwrap_or(
                (self.row + 1) % self.seq.len());
            self.row_jump = None;
        }
        if self.tick_count == 0 {
            for i in 0..self.width() {
                self.channel_beat(i);
            }
        }
        for i in 0..self.width() {
            self.channel_tick(i)
        }
        self.tick_count += 1;
        MixerIn {
            tick_rate: self.bpm as u16 * self.tick_rate as u16,
            pcm: self.pcm.clone(),
            chan: self.chan.iter().map(|c|
                ChannelIn{
                    note: c.note + c.add_note,
                    pcm_off: 0,
                    pcm_len: 256,
                    pcm_rate: 256,
                    vol: c.vol,
                }).collect(),
        }
    }
}

impl Field {
    pub fn string(&self) -> String {
        let note = match self.note {
            Note::On(ref note) => {
                const NOTE_NAME: &'static str = "C-C#D-D#E-F-F#G-G#A-A#B-";
                let name = *note as usize % 12;
                let octave = note / 12;
                format!("{}{}", &NOTE_NAME[name*2..name*2+2], octave)
            }
            Note::Off => String::from("---"),
            Note::Hold => String::from("   "),
        };
        format!("{}{}{:02X}", note, self.cmd.id as char, self.cmd.data)
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
