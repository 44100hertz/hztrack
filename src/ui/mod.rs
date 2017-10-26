use sdl2;

use std::sync::{Mutex, Arc};

use mixer::*;
use track::*;

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
    let track = Track::new(
        vec![
        vec![
            Field{note: Note::On(60), cmd: Command::zero()}
        ],
        vec![
            Field{note: Note::On(80), cmd: Command::from_str("310")}
        ],
        vec![
            Field{note: Note::Hold, cmd: Command::from_str("300")}
        ]]);
    let ui = Ui{
        track: Arc::new(Mutex::new(track)),
    };
    let video_subsys = sdl.video().unwrap();
    let win = video_subsys.window("rusttracker", 800, 600)
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
                _ => {},
            }
        }
    }
}
