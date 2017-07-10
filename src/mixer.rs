use sdl2::audio::*;

pub struct Channel {
    phase: u32,
    phase_inc: u32,
    pcm_off: usize,
    pcm_len: usize,
    vol: i16,
}

impl Channel {
    fn calc_pitch(&mut self, samp_rate: u32) {
        self.phase_inc = DENOM * 256 * 440 / samp_rate;
    }
    fn get_point(&mut self, pcm: &[i8]) -> i16 {
        let point = pcm[self.phase as usize / DENOM as usize %
                        self.pcm_len + self.pcm_off];
        self.phase = self.phase.wrapping_add(self.phase_inc);
        point as i16 * self.vol
    }
}

pub struct Mixer {
    samp_rate: u32,
    samp_count: u32,
    last_tick: u32,
    next_tick: u32,
    bpm: u32,
    tick_rate: u32,
    tick_len: u32,
    pcm: Vec<i8>,
    chan: Vec<Channel>,
}

const DENOM: u32 = 1024;

impl Mixer {
    fn tick(&mut self) {
        self.tick_len = self.samp_rate / self.bpm / self.tick_rate;
        for chan in &mut self.chan {
            chan.calc_pitch(self.samp_rate);
        }
        self.next_tick = self.last_tick.wrapping_add(self.tick_len);
        self.last_tick = self.samp_count;
    }
    pub fn set_tickrate(&mut self, tick_rate: u32) {
        self.tick_rate = tick_rate;
    }
    pub fn set_bpm(&mut self, bpm: u32) {
        self.bpm = bpm;
    }
    pub fn new(samp_rate: i32) -> Mixer {
        let mut mixer = Mixer {
            samp_rate: samp_rate as u32,
            samp_count: 0,
            last_tick: 0,
            next_tick: 0,
            bpm: 120,
            tick_rate: 6,
            tick_len: 0,
            chan: Vec::new(),
            pcm: ::std::iter::repeat(()).take(255).enumerate()
                .map(|i| ((i.0 as f64 / 128.0 * 3.14159).sin() * 127.0) as i8)
                .collect(),
        };
        mixer.chan.push(Channel {
            phase: 0,
            phase_inc: 0,
            pcm_off: 0,
            pcm_len: 255,
            vol: 255,
        });
        mixer.tick();
        mixer
    }
}

impl AudioCallback for Mixer {
    type Channel = i16;
    fn callback(&mut self, out: &mut [i16]) {
        for v in out.iter_mut() {
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
