extern crate sdl2;

use std::sync::{Mutex, Arc};

mod base32;
mod mixer;
mod ui;

use mixer::control::*;

fn main() {
    let seq = vec![Field{
        note: Note::Hold,
        cmd: Some(Command::from_str("z06"))}];
    let ctrl = Arc::new(Mutex::new(Controller::new(seq)));
    let sdl = sdl2::init().unwrap();
    let mixer = mixer::run(&sdl, ctrl.clone());
    ui::run(&sdl, ctrl.clone());
}
