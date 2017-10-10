extern crate sdl2;

use std::sync::{Mutex, Arc};

mod base32;
mod mixer;
mod display;

use mixer::control::*;

fn main() {
    let mut ctrl = Controller::new();
    {
        for i in 1..100 {
            ctrl.sequence.push_back(
                Field{
                    cmd: None,
                    note: Some(i as u8),
                });
        }
    }
    let sdl = sdl2::init().unwrap();
    let mixer = mixer::run(&sdl, Arc::new(Mutex::new(ctrl)));
    display::run(&sdl);
}
