use sdl2;

mod keyboard;

use std::sync::{Mutex, Arc};
use sequence::{Field, Note, Command};
use track::Track;
use mixer::{Controller, MixerIn};

#[derive(Clone)]
struct Ui {
    track: Arc<Mutex<Track>>,
}
impl Controller for Ui {
    fn next(&mut self) -> MixerIn {
        let mut track = self.track.lock().unwrap();
        track.next()
    }
}
pub fn run() {
    let sdl = sdl2::init().unwrap();

    let track = Track::new(vec![vec![
        Field{note: Note::Off, cmd: Command::zero()}
    ]]);
    let ui = Ui{
        track: Arc::new(Mutex::new(track)),
    };

    let video_subsys = sdl.video().unwrap();
    let _win = video_subsys.window("rusttracker", 800, 600)
        .position_centered()
        .opengl()
        .resizable()
        .build().unwrap();
    let _mixer = ::mixer::run(&sdl, ui.clone());
    let mut event_pump = sdl.event_pump().unwrap();

    'main: loop {
        use sdl2::event::Event;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit{..}  => break 'main,
                Event::KeyDown{scancode, ..} => {
                    // HACK: play note, bring into audible octave
                    let mut track = ui.track.lock().unwrap();
                    track.seq.fields[0][0].note = keyboard::to_note(scancode.unwrap());
                    if let Note::On(n) = track.seq.fields[0][0].note {
                        track.seq.fields[0][0].note = Note::On(n+48);
                    }
                }
                _ => {},
            }
        }
    }
}
