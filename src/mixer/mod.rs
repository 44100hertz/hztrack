use std::sync::{Arc, Mutex};

use sdl2;
use sdl2::audio::*;

pub mod control;
use self::control::*;

const PBITS: u32 = 8; // Bits of fixed-point precision for phase.
const PBITSF: f64 = (1<<PBITS) as f64;

pub fn run<C: Controller + Send>(sdl: &sdl2::Sdl, ctrl: Arc<Mutex<C>>) ->
AudioDevice<Mixer<C>>
{
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
    note: u16,
    pcm_off: usize,
    pcm_len: u32,
    pcm_speed: u32,
    vol: i16,
    arp: [u8; 2],
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
            vol: 0,
            arp: [0; 2],
        }
    }
    fn calc_pitch(&mut self, srate: u32, arp: u8) {
        let fine_note = (self.note + ((arp as u16)<<8)) as f64 / 2f64.powi(8);
        let note = (2.0f64).powf((fine_note - 60.0) / 12.0) * 440.0;
        self.phase_inc = self.pcm_speed * (note * PBITSF) as u32 / srate;
    }
    fn set_note(&mut self, note: u8) {
        self.note = (note as u16)<<8;
    }
    fn get_point(&mut self, pcm: &[i8]) -> i16 {
        self.phase = self.phase % (self.pcm_len<<PBITS);
        let point = pcm[(self.phase>>PBITS) as usize];
        self.phase = self.phase.wrapping_add(self.phase_inc);
        point as i16 * self.vol
    }
}

pub struct Mixer<C> {
    srate: u32,         // sampling rate
    samp_count: u32,    // sample count; used for ticking
    next_tick: u32,     // will tick again when sample count reaches this
    bpm: u8,
    tick_rate: u8,      // number of ticks per beat
    tick_count: u32,
    pcm: Vec<i8>,
    chan: Vec<Channel>,
    ctrl: Arc<Mutex<C>>,
}

impl<C: Controller> Mixer<C> {
    pub fn new(srate: i32, ctrl: Arc<Mutex<C>>) -> Mixer<C> {
        let mixer = Mixer {
            srate: srate as u32,
            samp_count: 0,
            next_tick: 0,
            bpm: 120,
            tick_rate: 6,
            tick_count: 0,
            chan: Vec::new(),
            ctrl: ctrl,
            pcm: (0..255)
                .map(|i| ((i as f64 / 128.0 * 3.1415).sin() * 127.0) as i8)
                .collect()
        };
        mixer
    }
    fn beat(&mut self) {
        let next = self.ctrl.lock().unwrap().next();
        self.chan.resize(next.len(), Channel::new());
        for (i, field) in next.iter().enumerate() {
            match field.note {
                Note::On(note) => {
                    self.chan[i].vol = 63;
                    self.chan[i].set_note(note);
                },
                Note::Off => self.chan[i].vol = 0,
                Note::Hold => {}
            }
            self.chan[i].arp = [0; 2];
            field.cmd.execute(self, i);
        }
    }
    fn tick(&mut self) {
        let tick_count = self.tick_count as usize % self.tick_rate as usize;
        if tick_count == 0 {
            self.beat();
        }
        self.tick_count += 1;
        for chan in &mut self.chan {
            let arp_v = match tick_count % 3 {
                0 => 0,
                1 => chan.arp[0],
                2 => chan.arp[1],
                _ => panic!(),
            };
            chan.calc_pitch(self.srate, arp_v);
        }
        let tick_len = self.srate * 60 / self.bpm as u32 / (self.tick_rate+1) as u32;
        self.next_tick = self.next_tick.wrapping_add(tick_len);
    }
}

impl<C: Controller + Send> AudioCallback for Mixer<C> {
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
