extern crate sdl2;
use sdl2::audio::*;
use std::time::Duration;

const DENOM: u32 = 1024;

struct Mixer {
    samp_rate: u32,
    samp_count: u32,
    last_tick: u32,
    next_tick: u32,
    bpm: u32,
    tick_rate: u32,
    tick_len: u32,
    phase: u32,
    pcm: Vec<i8>,
}

impl Mixer {
    fn calc_next_tick(&mut self) {
        self.next_tick = self.last_tick.wrapping_add(self.tick_len);
        self.last_tick = self.samp_count;
    }
    fn tick(&mut self) {
        // recalc pitches, envelopes
        self.calc_next_tick();
    }
    fn retick(&mut self) {
        self.tick_len = self.samp_rate / self.bpm / self.tick_rate;
        self.calc_next_tick();
    }
    fn set_tickrate(&mut self, tick_rate: u32) {
        self.tick_rate = tick_rate;
        self.calc_next_tick();
    }
    fn set_bpm(&mut self, bpm: u32) {
        self.bpm = bpm;
        self.calc_next_tick();
    }
    fn new(samp_rate: i32) -> Mixer {
        let mut mixer = Mixer {
            samp_rate: samp_rate as u32,
            samp_count: 0,
            last_tick: 0,
            next_tick: 0,
            bpm: 120,
            tick_rate: 6,
            tick_len: 0,
            phase: 0,
            pcm: std::iter::repeat(()).take(255).enumerate()
                .map(|i| ((i.0 as f64 / 128.0 * 3.14159).sin() * 127.0) as i8)
                .collect(),
        };
        mixer.retick();
        mixer
    }
}

impl AudioCallback for Mixer {
    type Channel = i16;
    fn callback(&mut self, out: &mut [i16]) {
        for v in out.iter_mut() {
            *v = {
                self.phase = self.phase.wrapping_add(DENOM * 256 * 440 / self.samp_rate);
                self.pcm[self.phase as usize / DENOM as usize % self.pcm.len()] as i16 * 127
            };
            self.samp_count += 1;
        }
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let audio_subsys = sdl.audio().unwrap();
    let desired = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),
        samples: None,
    };
    let device = audio_subsys.open_playback(None, &desired, |spec| {
        Mixer::new(spec.freq)
    }).unwrap();

    device.resume();
    std::thread::sleep(Duration::from_millis(2000));
}
