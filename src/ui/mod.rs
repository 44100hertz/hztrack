use sdl2;

use std::sync::{Mutex, Arc};

use mixer::*;
use track::*;

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
        vec![vec![
            Field{note: Note::On(80), cmd: Command::from_str("000")}
        ],vec![
            Field{note: Note::Hold, cmd: Command::from_str("281")}
        ]]);
    let ui = Ui{
        track: Arc::new(Mutex::new(track)),
    };
    let mixer = ::mixer::run(&sdl, ui);
    ::std::thread::sleep(::std::time::Duration::from_secs(2));
}
