extern crate sdl2;

use std::sync::{Mutex, Arc};

mod base32;
mod mixer;
mod display;

use mixer::control::*;

fn main() {
    let seq: Vec<_> = (0..9)
        .map(|i| Field{
            cmd: if i < 2 {Some(Command::from_str("2ff"))} else {None},
            note: if i % 2 == 0 {Some(i+60 as u8)} else {None},
        }).collect();
    let ctrl = Arc::new(Mutex::new(Controller::new(seq)));
    let sdl = sdl2::init().unwrap();
    let mixer = mixer::run(&sdl, ctrl.clone());
    display::run(&sdl, ctrl.clone());
}
