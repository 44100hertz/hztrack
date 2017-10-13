extern crate sdl2;

use std::sync::{Mutex, Arc};

mod base32;
mod mixer;
mod display;

use mixer::control::*;

fn main() {
    let seq: Vec<_> = (1..10)
        .map(|i| Field{
            cmd: Some(Command::from_str("2ff")),
            note: Some(i as u8),
        }).collect();
    let ctrl = Arc::new(Mutex::new(Controller::new(seq)));
    let sdl = sdl2::init().unwrap();
    let mixer = mixer::run(&sdl, ctrl.clone());
    display::run(&sdl, ctrl.clone());
}
