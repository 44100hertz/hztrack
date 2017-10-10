use std::sync::{Arc, Mutex};

use sdl2;
use sdl2::audio::*;

pub mod control;
use self::control::*;

const PBITS: u32 = 8;
const PBITSF: f64 = (1<<PBITS) as f64;

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

pub struct Channel {
    phase: u32,
    phase_inc: u32,
    note: u8,
    pcm_off: usize,
    pcm_len: u32,
    pcm_speed: u32,
    vol: i16,
}

impl Channel {
    fn calc_pitch(&mut self, srate: u32) {
        let note = (2.0f64).powf((self.note as f64 - 60.0) / 12.0) * 440.0;
        self.phase_inc = self.pcm_speed * (note * PBITSF) as u32 / srate;
    }
    fn get_point(&mut self, pcm: &[i8]) -> i16 {
        self.phase = self.phase % (self.pcm_len<<PBITS);
        let point = pcm[(self.phase>>PBITS) as usize + self.pcm_off];
        self.phase = self.phase.wrapping_add(self.phase_inc);
        point as i16 * self.vol
    }
}

pub struct Mixer {
    srate: u32,
    samp_count: u32,
    next_tick: u32,
    bpm: u8,
    tick_rate: u8,
    tick_len: u32,
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
            tick_len: 0,
            chan: Vec::new(),
            pcm: ::std::iter::repeat(()).take(255).enumerate()
                .map(|i| ((i.0 as f64 / 128.0 * 3.14159).sin() * 127.0) as i8)
                .collect(),
            ctrl: ctrl,
        };
        mixer.chan.push(Channel {
            phase: 0,
            phase_inc: 0,
            pcm_off: 0,
            pcm_len: 255,
            pcm_speed: 256,
            note: 60,
            vol: 127,
        });
        mixer
    }
    fn tick(&mut self) {
        let cc = self.ctrl.clone(); 
        let mut ctrl = cc.lock().unwrap();
        if let Some(field) = ctrl.sequence.pop_front() {
            if let Some(cmd) = field.cmd {
                cmd.execute(self);
            }
            if let Some(note) = field.note {
                self.chan[0].note = note
            }
        }
        self.tick_len = self.srate * 60 / self.bpm as u32 / self.tick_rate as u32;
        for chan in &mut self.chan {
            chan.calc_pitch(self.srate);
        }
        self.next_tick = self.next_tick.wrapping_add(self.tick_len);
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
                    total = total.saturating_add(
                        chan.get_point(&self.pcm[..]));
                }
                total
            };
            self.samp_count += 1;
        }
    }

}
