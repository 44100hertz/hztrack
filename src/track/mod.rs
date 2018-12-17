use std::sync::Arc;

use mixer::{Controller, MixerIn, ChannelIn};
use sequence::{Sequence, Field, Command, Note};

pub struct Track {
    pub seq:    Sequence,
    chan:       Vec<Channel>,
    row_jump:   Option<usize>,
    row:        usize,
    tick_count: u8,
    tick_rate:  u8,
    bpm:        u8,
    pcm:        Arc<Vec<i8>>,
}

#[derive(Clone)]
pub struct Channel {
    note: u16,
    add_note: u16,
    porta_note: u8,
    cmd: Command,
    vol: i16,
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
    pub fn new(fields: Vec<Vec<Field>>) -> Self {
        Track {
            seq: Sequence::new(fields),
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
    fn channel_beat(&mut self, i: usize) {
        let field = &self.seq.get_field(self.row, i);
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
        let field = &self.seq.get_field(self.row, i);
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
                let porta_note = (chan.porta_note as u16)<<8;
                let rate = (chan.cmd.data as u16)<<4;
                let diff = chan.note - porta_note;
                if diff.abs() < rate {
                    chan.note = porta_note;
                } else if diff > 0 {
                    chan.note -= rate;
                } else {
                    chan.note += rate;
                }
            }
            b'F' => {
                match chan.cmd.data {
                    0...31 => self.tick_rate = chan.cmd.data + 1,
                    32...255 => self.bpm = chan.cmd.data,
                    _ => unreachable!(),
                }
            }
            b'B' => self.row_jump = Some(chan.cmd.data as usize),
            c @ _ => panic!("unknown command id: {}", c as char),
        }
    }
}

impl Controller for Track {
    fn next(&mut self) -> MixerIn {
        let width = self.seq.width();
        self.chan.resize(width, Channel::new());
        if self.tick_count == self.tick_rate {
            self.tick_count = 0;
            self.row = self.row_jump.unwrap_or(
                (self.row + 1) % self.seq.len());
            self.row_jump = None;
        }
        if self.tick_count == 0 {
            for i in 0..width {
                self.channel_beat(i);
            }
        }
        for i in 0..width {
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
