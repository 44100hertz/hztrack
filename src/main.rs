extern crate sdl2;

mod base32;
mod mixer;
mod ui;

use mixer::control::*;

fn main() {
    let seq = vec![Field{
        note: Note::Hold,
        cmd: Some(Command::from_str("z06"))}];
    let ctrl = Controller::new(seq);
    let sdl = sdl2::init().unwrap();
    let mixer = mixer::run(&sdl, ctrl.clone());
    ui::run(&sdl, ctrl.clone());
}
