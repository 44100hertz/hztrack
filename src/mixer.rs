use sdl2;
use sdl2::audio::*;
use std::time::Duration;

const PBITS: u32 = 8;
const PBITSF: f64 = (1<<PBITS) as f64;

pub struct Channel {
    phase: u32,
    phase_inc: u32,
    note: i32,
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
    bpm: u32,
    tick_rate: u32,
    tick_len: u32,
    pcm: Vec<i8>,
    chan: Vec<Channel>,
}

impl Mixer {
    fn tick(&mut self) {
        self.tick_len = self.srate * 60 / self.bpm / self.tick_rate;
        self.chan[0].note += 1;
        for chan in &mut self.chan {
            chan.calc_pitch(self.srate);
        }
        self.next_tick = self.next_tick.wrapping_add(self.tick_len);
    }
    pub fn new(srate: i32) -> Mixer {
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
        mixer.chan.push(Channel {
            phase: 0,
            phase_inc: 0,
            pcm_off: 0,
            pcm_len: 255,
            pcm_speed: 256,
            note: 72,
            vol: 127,
        });
        mixer
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

pub fn run() {
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
    ::std::thread::sleep(Duration::from_millis(20000));
}
