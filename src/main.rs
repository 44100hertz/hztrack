extern crate sdl2;

mod base32;

mod mixer;

mod track;
use track::*;

fn main() {
    let sdl = sdl2::init().unwrap();
    let mixer = mixer::run(&sdl, Track::new(
            vec![vec![
                Field{note: Note::On(80), cmd: Command::zero()}
            ],vec![
                Field{note: Note::On(60), cmd: Command::zero()}
            ]
            ]));
    std::thread::sleep(std::time::Duration::from_secs(2));
}
