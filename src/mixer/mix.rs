use std::sync::Arc;

use mixer::*;
use std::num::Wrapping;

const PBITS: u32 = 8; // Bits of fixed-point precision for phase.

pub struct Mixer<C> {
    srate:      u32,
    samp_count: Wrapping<u32>, // sample count; used for ticking
    next_tick:  Wrapping<u32>, // will tick again when sample count reaches this
    chan:       Vec<Channel>,
    ctrl:       C,
    input:      MixerIn,
}

#[derive(Clone)]
pub struct Channel {
    phase:      u32,
    phase_inc:  u32,
}
impl Channel {
    fn new() -> Self {
        Channel {
            phase: 0,
            phase_inc: 0,
        }
    }
}

impl<C: Controller + Send> AudioCallback for Mixer<C> {
    type Channel = i16;
    fn callback(&mut self, out: &mut [i16]) {
        for v in out.iter_mut() {
            *v = self.get_point()
        }
    }
}

impl<C: Controller> Mixer<C> {
    pub fn new(srate: i32, ctrl: C) -> Mixer<C> {
        Mixer {
            srate:      srate as u32,
            samp_count: Wrapping(0),
            next_tick:  Wrapping(0),
            ctrl:       ctrl,
            chan:       vec![],
            input: MixerIn {
                tick_rate:  0,
                pcm:        Arc::new(vec![]),
                chan:       vec![],
            }
        }
    }
    fn get_point(&mut self) -> i16 {
        if self.samp_count == self.next_tick {
            for (chan, inchan) in self.chan.iter_mut().zip(&mut self.input.chan) {
                let pbitsf = (1<<PBITS) as f64;
                let fnote = inchan.note as f64 / 2f64.powi(8);
                let pitch = (2.0f64).powf((fnote - 60.0) / 12.0) * 440.0;
                chan.phase_inc =
                    (pitch * pbitsf * inchan.pcm_rate as f64) as u32 / self.srate;
            }
            self.input = self.ctrl.next();
            self.chan.resize(self.input.chan.len(), Channel::new());
            let tick_len = self.srate * 60 / self.input.tick_rate as u32;
            self.next_tick += Wrapping(tick_len);
        }
        let mut total = 0i16;
        for (chan, inchan) in self.chan.iter_mut().zip(&mut self.input.chan) {
            chan.phase  = chan.phase % (inchan.pcm_len<<PBITS);
            let pcm_off = inchan.pcm_off + (chan.phase>>PBITS) as usize;
            let point   = self.input.pcm[pcm_off];
            chan.phase  += chan.phase_inc;
            total = total.saturating_add(point as i16 * inchan.vol)
        }
        self.samp_count += Wrapping(1);
        total
    }
}
