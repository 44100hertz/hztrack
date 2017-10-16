extern crate sdl2;

mod base32;
mod mixer;
mod ui;

use mixer::control::*;
use ui::sequence::*;

fn main() {
    let seq = vec![
        vec![
            Field{
                note: Note::Hold,
                cmd: Some(Command::from_str("z06"))
            },
            Field{
                note: Note::Hold,
                cmd: None,
            },
        ]];
    let ctrl = Sequence::new(seq);
    let sdl = sdl2::init().unwrap();
    let mixer = mixer::run(&sdl, ctrl.clone());
    ui::run(&sdl, ctrl.clone());
}
