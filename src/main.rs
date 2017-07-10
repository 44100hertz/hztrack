extern crate sdl2;

pub mod mixer;

use sdl2::audio::AudioSpecDesired;
use std::time::Duration;

fn main() {
    let sdl = sdl2::init().unwrap();
    let audio_subsys = sdl.audio().unwrap();
    let desired = AudioSpecDesired {
        freq: Some(48000),
        channels: Some(1),
        samples: None,
    };
    let device = audio_subsys.open_playback(None, &desired, |spec| {
        mixer::Mixer::new(spec.freq)
    }).unwrap();

    device.resume();
    std::thread::sleep(Duration::from_millis(2000));
}
