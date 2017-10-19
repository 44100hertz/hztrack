use std::sync::Arc;

extern crate sdl2;

mod mixer;
use mixer::*;

struct Dummy {
    pcm: Arc<Vec<i8>>
}
impl Controller for Dummy {
    fn next(&mut self) -> MixerIn {
        MixerIn {
            tick_rate: 120,
            pcm: self.pcm.clone(),
            chan: vec![
                ChannelIn {
                    note:       60<<8,
                    pcm_off:    0,
                    pcm_len:    256,
                    pcm_rate:   256,
                    vol:        40,
                }
            ]
        }
    }
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let pcm: Vec<_> = (0u32..256)
        .map(|i| ((i as f64 / 128.0 * 3.1415).sin() * 127.0) as i8)
        .collect();
    let mixer = mixer::run(&sdl, Dummy{
        pcm: Arc::new(pcm),
    });
    std::thread::sleep(std::time::Duration::from_secs(2));
}
