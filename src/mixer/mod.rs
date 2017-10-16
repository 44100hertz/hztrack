use std::sync::{Arc, Mutex};

use sdl2;
use sdl2::audio::*;

pub mod control;
use self::control::*;

const PBITS: u32 = 8; // Bits of fixed-point precision for phase.
const PBITSF: f64 = (1<<PBITS) as f64;

// hack: must use the returned audiodevice for scope reasons.
pub fn run(sdl: &sdl2::Sdl, ctrl: Arc<Mutex<Controller>>) -> AudioDevice<Mixer> {
    let audio_subsys = sdl.audio().unwrap();
    let desired = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),
        samples: None,
    };
    let device = audio_subsys.open_playback(None, &desired, |spec| {
        Mixer::new(spec.freq, ctrl)
    }).unwrap();
    device.resume();
    device
}

#[derive(Clone)]
pub struct Channel {
    phase: u32,
    phase_inc: u32,
    note: u8,
    tune: i8,
    pcm_off: usize,
    pcm_len: u32,
    pcm_speed: u32,
    vol: i16,
}

impl Channel {
    fn new() -> Self {
        Self {
            phase: 0,
            phase_inc: 0,
            pcm_off: 0,
            pcm_len: 255,
            pcm_speed: 256,
            note: 0,
            tune: 0,
            vol: 0,
        }
    }
    fn calc_pitch(&mut self, srate: u32) {
        let fine_note = self.note as f64 + self.tune as f64/2f64.powi(8);
        let note = (2.0f64).powf((fine_note - 60.0) / 12.0) * 440.0;
        self.phase_inc = self.pcm_speed * (note * PBITSF) as u32 / srate;
    }
    fn set_whole_note(&mut self, note: i16) {
        self.note = (note>>8) as u8;
        self.tune = (note & ((1<<8) - 1)) as i8;
    }
    fn get_point(&mut self, pcm: &[i8]) -> i16 {
        self.phase = self.phase % (self.pcm_len<<PBITS);
        let point = pcm[(self.phase>>PBITS) as usize];
        self.phase = self.phase.wrapping_add(self.phase_inc);
        point as i16 * self.vol
    }
}

pub struct Mixer {
    srate: u32,         // sampling rate
    samp_count: u32,    // sample count; used for ticking
    next_tick: u32,     // will tick again when sample count reaches this
    bpm: u8,
    tick_rate: u8,      // number of ticks per beat
    pcm: Vec<i8>,
    chan: Vec<Channel>,
    ctrl: Arc<Mutex<Controller>>,
}

impl Mixer {
    pub fn new(srate: i32, ctrl: Arc<Mutex<Controller>>) -> Mixer {
        let mut mixer = Mixer {
            srate: srate as u32,
            samp_count: 0,
            next_tick: 0,
            bpm: 120,
            tick_rate: 6,
            chan: Vec::new(),
            ctrl: ctrl,
            pcm: (0..255)
                .map(|i| ((i as f64 / 128.0 * 3.1415).sin() * 127.0) as i8)
                .collect()
        };
        mixer
    }
    fn tick(&mut self) {
        {
            let cc = self.ctrl.clone();
            let mut ctrl = cc.lock().unwrap();
            let next = ctrl.next();
            self.chan.resize(next.len(), Channel::new());
            for (i, field) in next.iter().enumerate() {
                if let Some(ref cmd) = field.cmd {
                    cmd.execute(self);
                }
                match field.note {
                    Note::On(note) => {
                        self.chan[i].note = note;
                        self.chan[i].vol = 127;
                    },
                    Note::Off => self.chan[i].vol = 0,
                    Note::Hold => {}
                }
                if let Note::On(note) = field.note {
                    self.chan[i].note = note
                }
            }
        }
        for chan in &mut self.chan {
            chan.calc_pitch(self.srate);
        }
        let tick_len = self.srate * 60 / self.bpm as u32 / self.tick_rate as u32;
        self.next_tick = self.next_tick.wrapping_add(tick_len);
    }
}

impl AudioCallback for Mixer {
    type Channel = i16;
    fn callback(&mut self, out: &mut [i16]) {
        for v in out.iter_mut() {
            if self.samp_count == self.next_tick { self.tick(); }
            *v = {
                let mut total: i16 = 0;
                for chan in &mut self.chan {
                    let pcm_from = chan.pcm_off as usize;
                    let pcm_to = pcm_from + chan.pcm_len as usize;
                    let pcm = &self.pcm[pcm_from..pcm_to];
                    total = total.saturating_add(chan.get_point(pcm));
                }
                total
            };
            self.samp_count += 1;
        }
    }

}
