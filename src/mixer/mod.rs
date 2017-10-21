use sdl2;
use sdl2::audio::*;

use std::sync::Arc;

mod mix;
use self::mix::*;

#[derive(Clone)]
pub struct MixerIn {
    pub tick_rate:  u16,    // ticks per minute
    pub pcm:        Arc<Vec<i8>>,
    pub chan:       Vec<ChannelIn>,
}
impl MixerIn {
    pub fn new() -> Self {
        MixerIn {
            tick_rate:  0,
            pcm:        Arc::new(vec![]),
            chan:       vec![],
        }
    }
}

#[derive(Clone)]
pub struct ChannelIn {
    pub note:       u16,    // NNTT = 8bit note, 8bit tuning.
    pub pcm_off:    usize,  // sample offset within data
    pub pcm_len:    u32,    // sample size
    pub pcm_rate:   u32,    // per-sample sampling rate
    pub vol:        i16,
}

pub trait Controller {
    fn next(&mut self) -> MixerIn;
}

pub fn run<C: Controller + Send>(sdl: &sdl2::Sdl, ctrl: C) ->
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

